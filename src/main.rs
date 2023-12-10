#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![feature(seek_stream_len)]

mod common;
mod techset;
mod xmodel;

use num_derive::FromPrimitive;
use serde::{
    de::{DeserializeOwned, Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{
    ffi::{CString, OsString},
    fmt::{self, Debug, Display},
    fs::File,
    io::{BufReader, Read, Seek, Write},
    marker::PhantomData,
    mem::{size_of, transmute},
    path::Path,
    str::FromStr,
    sync::OnceLock,
};

// FastFiles (internally known as XFiles) are structured as follows:
//
// ----------------------------------------------------------------------------
// | Offset    | Size | Field       | Description                             |
// ----------------------------------------------------------------------------
// | 0x0000000 | 4    | Magic       | Magic value to identify the file as an  |
// |           |      |             | XFile. Will always be ASCII "IWff".     |
// ----------------------------------------------------------------------------
// | 0x0000004 | 1    | Compression | Magic values to identify the            |
// |           |      |             | compression method used. Will always be |
// |           |      |             | ASCII 'u' or '0' for T5 on PC.          |
// ----------------------------------------------------------------------------
// | 0x0000005 | 3    | Unknown     | Exact meaning unknown. Maybe it was     |
// |           |      |             | supposed to represent some version info |
// |           |      |             | info? Will always be ASCII "100".       |
// ----------------------------------------------------------------------------
// | 0x0000008 | 4    | Version     | The real version. For reasons explained |
// |           |      |             | below, XFiles are neither backward- nor |
// |           |      |             | forward-compatible for deserialization  |
// |           |      |             | purposes. It is **imperative** that the |
// |           |      |             | XFile version match the version the     |
// |           |      |             | deserializer is expecting. For the      |
// |           |      |             | latest version of T5, the value is      |
// |           |      |             | 0x000001D9                              |
// ----------------------------------------------------------------------------
// | 0x000000C | *    | Blob        | The rest of the file is a DEFLATE-      |
// |           |      |             | compressed blob. To get the "real"      |
// |           |      |             | contents of the file, it must be        |
// |           |      |             | inflated.                               |
// ----------------------------------------------------------------------------
//
// The inflated blob is structured as follows:
//
// ----------------------------------------------------------------------------
// | Offset    | Size | Field       | Description                             |
// ----------------------------------------------------------------------------
// | 0x0000000 | 36   | XFile       | See the [`XFile`] struct below.         |
// ----------------------------------------------------------------------------
// | 0x0000024 | 16   | XAssetList  | See the [`XAssetList`] struct below.    |
// ----------------------------------------------------------------------------
// | 0x0000034 | *    | XAssets     | The XAssets.                            |
// ----------------------------------------------------------------------------
//
// The XAssetList essentially contains two fat pointers: first, to a string
// array, then an asset array. And herein comes the first major annoyance
// with XFiles - the assets are essentially just the structs used by the engine
// serialzed into a file. Any pointers in said structs become offsets in the
// file. Occasionally the offsets are NULL or a "real" value, but most of the
// time they're 0xFFFFFFFF, which indicates that, instead of being at a
// specific offset, they come immediately after the current struct. This means
// basically nothing in the file is relocatable, and removing or adding a
// single byte will corrupt everything after.
//
// In addition, if the structures' sizes or alignments don't match exactly what
// the serializer used, or if new structures are added, the file is basically
// un-parseable (this is why, as mentioned above, the versions must match
// exactly). Pulling out only assets of a specific type is also impossible,
// because you can't know where they're at in the file until you pull out
// everything before it too. For this reason, you're more or less forced into
// deserializng everything at once and then grabbing the assets you need
// afterwards.

// ============================================================================
//
// [`MaterialTechniqueSetRaw`] (see below) contains an array with 130 elements.
// However, [`Deserialize`] isn't implemented for arrays of that size (wanna
// say 24 is the max?), so we have to do it ourselves here.

trait BigArray<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

#[macro_export]
macro_rules! big_array {
    ($($len:expr,)+) => {
        $(
            impl<'de, T> BigArray<'de> for [T; $len]
                where T: Default + Copy + Deserialize<'de>
            {
                fn deserialize<D>(
                    deserializer: D
                ) -> Result<[T; $len], D::Error>
                    where D: Deserializer<'de>
                {
                    struct ArrayVisitor<T> {
                        element: PhantomData<T>,
                    }

                    impl<'de, T> Visitor<'de> for ArrayVisitor<T>
                        where T: Default + Copy + Deserialize<'de>
                    {
                        type Value = [T; $len];

                        fn expecting(
                            &self, formatter: &mut fmt::Formatter
                        ) -> fmt::Result {
                            formatter
                                .write_str(
                                    concat!("an array of length ", $len)
                                )
                        }

                        fn visit_seq<A>(
                            self, mut seq: A
                        ) -> Result<[T; $len], A::Error>
                            where A: SeqAccess<'de>
                        {
                            let mut arr = [T::default(); $len];
                            for i in 0..$len {
                                arr[i] = seq.next_element()?
                                    .ok_or_else(
                                        || Error::invalid_length(i, &self)
                                    )?;
                            }
                            Ok(arr)
                        }
                    }

                    let visitor = ArrayVisitor { element: PhantomData };
                    deserializer.deserialize_tuple($len, visitor)
                }
            }
        )+
    }
}

big_array! {
    130,
}
// ============================================================================

/// Helper macro to ensure the structs we're deserializing are the correct
/// size.
#[macro_export]
macro_rules! assert_size {
    ($t:ty, $n:literal) => {
        const _: fn() = || {
            let _ = core::mem::transmute::<$t, [u8; $n]>;
        };
    };
}

/// Helper function to deserialze [`T`] from [`xfile`].
pub fn load_from_xfile<T: DeserializeOwned>(xfile: impl Read + Seek) -> T {
    bincode::deserialize_from::<_, T>(xfile).unwrap()
}

/// Trait to deserialize [`Self`] from [`xfile`], then convert [`Self`] to
/// [`T`].
///
/// [`Self`] may have [`repr`] attributes ([`C`], [`packed`]) or members
/// ([`Ptr32`], [`FlexibleArrayU16`]/[`FlexibleArrayU32`], etc.) that make
/// them very unergonomic to use. Since, if we were to deserialze them without
/// any such conversion, we'd probably end up converting them separately later
/// anyways, it's a nice touch to have both done in one go.
trait XFileInto<T> {
    /// Deserialize [`Self`] from [`xfile`], then convert [`Self`] to [`T`].
    ///
    /// [`Self`] may have [`repr`] attributes ([`C`], [`packed`]) or members
    /// ([`Ptr32`], [`FlexibleArrayU16`]/[`FlexibleArrayU32`], etc.) that make
    /// them very unergonomic to use. Since, if we were to deserialze them
    /// without any such conversion, we'd probably end up converting them
    /// separately later anyways, it's a nice touch to have both done in one
    /// go.
    fn xfile_into(&self, xfile: impl Read + Seek) -> T;
}

/// Newtype to handle pointer members of serialized structs.
///
/// We use this instead of a [`u32`] for two reasons. One, to differentiate
/// between actual [`u32`]s and offsets. And two, so that we can implement
/// [`XFileInto`] to retrieve the pointed-to data.
///
/// We can't use [`*const T`] or [`*mut T`] for three reasons.
/// * Pointer members of the serialzed structs are converted to offsets
/// within the XFile during serialization (as noted above), so they wouldn't
/// be valid pointers. Also, they're often [`0xFFFFFFFF`] anyways, so again,
/// invalid pointers.
/// * T5 and its associated tools are all 32-bit programs using 4-byte
/// pointers, and [`*const T`]/[`*mut T`] are probably going to be 8 bytes
/// on any machine this is compiled for.
/// * We couldn't point them to the data in the file since 1) that data
/// is read buffered and will eventually get overwritten, and 2) even if it
/// weren't, we don't want their lifetime tied to the lifetime of the XFile.
///
/// Also, pointers are unsafe and just annoying to use compared to a [`u32`].
#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(transparent)]
struct Ptr32<'a, T>(u32, PhantomData<&'a mut T>);

impl<'a, T> Default for Ptr32<'a, T> {
    fn default() -> Self {
        Self(0, PhantomData::default())
    }
}

impl<'a, T> Ptr32<'a, T> {
    fn cast<U>(self) -> Ptr32<'a, U> {
        Ptr32::<'a, U>(self.0, PhantomData)
    }

    fn to_array(self, size: usize) -> Ptr32Array<'a, T> {
        Ptr32Array { p: self, size }
    }
}

trait SeekAnd: Read + Seek {
    fn seek_and<T>(
        &mut self,
        from: std::io::SeekFrom,
        predicate: impl FnOnce(&mut Self) -> T,
    ) -> std::io::Result<T> {
        let pos = self.stream_position()?;

        if let std::io::SeekFrom::Start(p) = from {
            if p != 0xFFFFFFFF {
                let (_, off) = convert_offset_to_ptr(p as _);
                assert!(off as u64 <= self.stream_len().unwrap(), "p = {p:#08X}");
                self.seek(std::io::SeekFrom::Start(off as _))?;
            }
        } else if let std::io::SeekFrom::Current(p) = from {
            assert!(
                pos as i64 + p <= self.stream_len().unwrap() as i64,
                "p = {p:#08X}"
            );
            self.seek(from)?;
        } else {
            unimplemented!()
        }

        let t = predicate(self);

        if let std::io::SeekFrom::Start(p) = from {
            if p != 0xFFFFFFFF {
                self.seek(std::io::SeekFrom::Start(pos))?;
            }
        } else if let std::io::SeekFrom::Current(p) = from {
            self.seek(std::io::SeekFrom::Current(-p))?;
        } else {
            unimplemented!()
        }

        Ok(t)
    }
}

impl<S: Read + Seek> SeekAnd for S {}

impl<'a, T: DeserializeOwned + Clone + Debug + XFileInto<U>, U> XFileInto<Option<Box<U>>>
    for Ptr32<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Option<Box<U>> {
        if self.0 == 0x00000000 {
            return None;
        }

        if self.0 != 0xFFFFFFFF {
            println!("ignoring offset");
            return None;
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.0 as _), |f| {
                bincode::deserialize_from::<_, T>(f).unwrap()
            })
            .ok()
            .map(|t| Box::new(t.xfile_into(xfile)))
    }
}

impl<'a, T: DeserializeOwned + Debug> Ptr32<'a, T> {
    /// Same principle as [`XFileInto::xfile_into`], except it doesn't do any
    /// type conversion. Useful for the rare structs that don't need any such
    /// conversion.
    fn xfile_get(self, mut xfile: impl Read + Seek) -> Option<Box<T>> {
        if self.0 == 0x00000000 {
            return None;
        }

        if self.0 != 0xFFFFFFFF {
            println!("ignoring offset");
            return None;
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.0 as _), |f| {
                bincode::deserialize_from::<_, T>(f).unwrap()
            })
            .ok()
            .map(Box::new)
    }
}

/// Newtype for flexible array members of serialzed structs.
///
/// In C, we might have a struct like:
/// ```c
/// struct S {
///     int something;
///     short count;
///     char bytes[];
/// }
/// ```
/// This can't be easily represented in Rust, so this type encapsulates `count`
/// and `bytes` and allows the correct number of [`T`]s to be deserialized into
/// a [`Vec<T>`] (see [`FlexibleArrayU16::to_vec`]).
///
/// This type and [`FlexibleArrayU32`] are exactly the same except that
/// [`FlexibleArrayU16::count`] is a [`u16`] (as the name implies), and
/// [`FlexibleArrayU32::count`] is a [`u32`].
#[derive(Copy, Clone, Default, Debug, Deserialize)]
#[repr(transparent)]
struct FlexibleArrayU16<T: DeserializeOwned> {
    count: u16,
    _p: PhantomData<T>,
}

impl<T: DeserializeOwned> FlexibleArrayU16<T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        let mut v = vec![0u8; self.count as usize * size_of::<T>()];

        xfile.read_exact(&mut v).unwrap();

        let mut vt = Vec::new();

        for i in 0..self.count as usize {
            let s = &v[i * size_of::<T>()..(i + 1) * size_of::<T>()];
            vt.push(bincode::deserialize(s).unwrap());
        }

        vt
    }
}

/// Newtype for flexible array members of serialzed structs.
///
/// In C, we might have a struct like:
/// ```c
/// struct S {
///     int something;
///     int count;
///     char bytes[];
/// }
/// ```
/// This can't be easily represented in Rust, so this type encapsulates `count`
/// and `bytes` and allows the correct number of [`T`]s to be deserialized into
/// a [`Vec<T>`] (see [`FlexibleArrayU32::to_vec`]).
///
/// This type and [`FlexibleArrayU16`] are exactly the same except that
/// [`FlexibleArrayU32::count`] is a [`u32`] (as the name implies), and
/// [`FlexibleArrayU16::count`] is a [`u16`].
#[derive(Copy, Clone, Default, Debug, Deserialize)]
#[repr(transparent)]
struct FlexibleArrayU32<T: DeserializeOwned> {
    count: u32,
    _p: PhantomData<T>,
}

impl<T: DeserializeOwned> FlexibleArrayU32<T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        let mut v = vec![0u8; self.count as usize * size_of::<T>()];

        xfile.read_exact(&mut v).unwrap();

        let mut vt = Vec::new();

        for i in 0..self.count as usize {
            let s = &v[i * size_of::<T>()..(i + 1) * size_of::<T>()];
            vt.push(bincode::deserialize(s).unwrap());
        }

        vt
    }
}

/// Newtype for a fat pointer to a `[T]`.
///
/// Represents an offset containing [`Self::size`] [`T`]s.
///
/// Serialized structs often contain these, but sometimes the size comes
/// before the pointer instead of after, and sometimes it's a [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u16`], and comes before the pointer.
#[derive(Clone, Debug, Deserialize)]
struct FatPointerCountFirstU16<'a, T: Debug + Clone> {
    size: u16,
    p: Ptr32<'a, T>,
}

impl<'a, T: DeserializeOwned + Debug + Clone> FatPointerCountFirstU16<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.p.0 as _), |mut f| {
                let mut vt = Vec::new();

                for _ in 0..self.size {
                    vt.push(bincode::deserialize_from::<_, T>(&mut f).unwrap());
                }

                vt
            })
            .ok()
            .unwrap_or_default()
    }
}

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U>, U> XFileInto<Vec<U>>
    for FatPointerCountFirstU16<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile))
            .collect()
    }
}

/// Newtype for a fat pointer to a `[T]`.
///
/// Represents an offset containing [`Self::size`] [`T`]s.
///
/// Serialized structs often contain these, but sometimes the size comes
/// before the pointer instead of after, and sometimes it's a [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u32`], and comes before the pointer.
#[derive(Copy, Clone, Debug, Default, Deserialize)]
struct FatPointerCountFirstU32<'a, T> {
    size: u32,
    p: Ptr32<'a, T>,
}

impl<'a, T: DeserializeOwned + Debug> FatPointerCountFirstU32<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.p.0 as _), |mut f| {
                let mut vt = Vec::new();

                for _ in 0..self.size {
                    vt.push(bincode::deserialize_from::<_, T>(&mut f).unwrap());
                }

                vt
            })
            .ok()
            .unwrap_or_default()
    }
}

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U>, U> XFileInto<Vec<U>>
    for FatPointerCountFirstU32<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile))
            .collect()
    }
}

/// Newtype for a fat pointer to a `[T]`.
///
/// Represents an offset containing [`Self::size`] [`T`]s.
///
/// Serialized structs often contain these, but sometimes the size comes
/// before the pointer instead of after, and sometimes it's a [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u16`], and comes after the pointer.
#[derive(Copy, Clone, Debug, Deserialize)]
struct FatPointerCountLastU16<'a, T> {
    p: Ptr32<'a, T>,
    size: u16,
}

impl<'a, T: DeserializeOwned + Debug> FatPointerCountLastU16<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.p.0 as _), |mut f| {
                let mut vt = Vec::new();

                for _ in 0..self.size {
                    vt.push(bincode::deserialize_from::<_, T>(&mut f).unwrap());
                }

                vt
            })
            .ok()
            .unwrap_or_default()
    }
}

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U>, U> XFileInto<Vec<U>>
    for FatPointerCountLastU16<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile))
            .collect()
    }
}

/// Newtype for a fat pointer to a `[T]`.
///
/// Represents an offset containing [`Self::size`] [`T`]s.
///
/// Serialized structs often contain these, but sometimes the size comes
/// before the pointer instead of after, and sometimes it's a [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u32`], and comes after the pointer.
#[derive(Copy, Clone, Default, Debug, Deserialize)]
struct FatPointerCountLastU32<'a, T> {
    p: Ptr32<'a, T>,
    size: u32,
}

impl<'a, T: DeserializeOwned + Debug> FatPointerCountLastU32<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.p.0 as _), |mut f| {
                let mut vt = Vec::new();

                for _ in 0..self.size {
                    vt.push(bincode::deserialize_from::<_, T>(&mut f).unwrap());
                }

                vt
            })
            .ok()
            .unwrap_or_default()
    }
}

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U>, U> XFileInto<Vec<U>>
    for FatPointerCountLastU32<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile))
            .collect()
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
struct Ptr32Array<'a, T> {
    p: Ptr32<'a, T>,
    size: usize,
}

impl<'a, T: DeserializeOwned + Debug> Ptr32Array<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.p.0 as _), |mut f| {
                let mut vt = Vec::new();

                for _ in 0..self.size {
                    vt.push(bincode::deserialize_from::<_, T>(&mut f).unwrap());
                }

                vt
            })
            .ok()
            .unwrap_or_default()
    }
}

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U>, U> XFileInto<Vec<U>>
    for Ptr32Array<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile))
            .collect()
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct XFileHeader {
    magic: [u8; 8],
    version: u32,
}
assert_size!(XFileHeader, 12);

#[derive(Copy, Clone, Debug, Deserialize)]
struct XFile {
    size: u32,
    external_size: u32,
    block_size: [u32; 7],
}
assert_size!(XFile, 36);

#[derive(Deserialize)]
struct XAssetList<'a> {
    strings: FatPointerCountFirstU32<'a, XString<'a>>,
    assets: FatPointerCountFirstU32<'a, XAssetRaw<'a>>,
}
assert_size!(XAssetList, 16);

fn xfile_header_magic_is_valid(header: &XFileHeader) -> bool {
    header.magic[0] == b'I'
        && header.magic[1] == b'W'
        && header.magic[2] == b'f'
        && header.magic[3] == b'f'
        && (header.magic[4] == b'u' || header.magic[4] == b'0')
        && header.magic[5] == b'1'
        && header.magic[6] == b'0'
        && header.magic[7] == b'0'
}

const XFILE_VERSION: u32 = 0x000001D9u32;

fn xfile_is_correct_version(header: &XFileHeader) -> bool {
    header.version == XFILE_VERSION
}

fn decompress_xfile(filename: impl AsRef<Path>) -> BufReader<File> {
    let mut file = File::open(&filename).unwrap();

    println!(
        "Found file '{}', reading header...",
        filename.as_ref().display()
    );

    let mut header_bytes = [0u8; 12];
    file.read_exact(&mut header_bytes).unwrap();

    println!("Header read, verifying...");

    let header = bincode::deserialize::<XFileHeader>(&header_bytes).unwrap();
    assert!(
        xfile_header_magic_is_valid(&header),
        "Fastfile header magic invalid: valid values are IWffu100 and IWff0100"
    );
    assert!(
        xfile_is_correct_version(&header),
        "Fastfile is wrong version (version: 0x{:08x}, correct version: {}",
        {
            let header = header.version;
            header
        },
        XFILE_VERSION
    );

    println!("Header verified, reading payload...");

    let mut compressed_payload = Vec::new();
    let bytes_read = file.read_to_end(&mut compressed_payload).unwrap();
    assert!(bytes_read as u64 == file.metadata().unwrap().len() - 12);

    println!("Payload read, inflating... (this may take a while)");

    let decompressed_payload = inflate::inflate_bytes_zlib(&compressed_payload).unwrap();

    println!(
        "Payload inflated, compressed size: {} bytes, decompressed size, {} bytes",
        bytes_read,
        decompressed_payload.len()
    );
    println!("Caching decompressed payload to disk...");

    let mut file_out = File::create(filename.as_ref().with_extension("cache")).unwrap();
    file_out.write_all(&decompressed_payload).unwrap();
    file_out.seek(std::io::SeekFrom::Start(0)).unwrap();

    println!("Decompressed payload cached.");
    BufReader::new(file_out)
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct XAssetRaw<'a> {
    asset_type: u32,
    asset_data: Ptr32<'a, ()>,
}
assert_size!(XAssetRaw, 8);

#[derive(Copy, Clone, FromPrimitive)]
#[repr(u32)]
enum XAssetType {
    XMODEL = 0x05,
    TECHNIQUE_SET = 0x07,
}

enum XAsset {
    TechniqueSet(Option<Box<techset::MaterialTechniqueSet>>),
    XModel(Option<Box<xmodel::XModel>>),
}

impl<'a> XFileInto<XAsset> for XAssetRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> XAsset {
        match num::FromPrimitive::from_u32(self.asset_type).unwrap() {
            XAssetType::XMODEL => XAsset::XModel(
                self.asset_data
                    .cast::<xmodel::XModelRaw>()
                    .xfile_into(xfile),
            ),
            XAssetType::TECHNIQUE_SET => XAsset::TechniqueSet(
                self.asset_data
                    .cast::<techset::MaterialTechniqueSetRaw>()
                    .xfile_into(xfile),
            ),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
struct XString<'a>(u32, PhantomData<&'a ()>);
assert_size!(XString, 4);

impl<'a> XFileInto<String> for XString<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> String {
        //dbg!(*self);

        if self.0 != 0xFFFFFFFF && self.0 != 0x00000000 {
            println!("ignoring offset");
            String::new()
        } else {
            xfile
                .seek_and(std::io::SeekFrom::Start(self.0 as _), |f| {
                    file_read_string(f)
                })
                .unwrap()
        }
    }
}

fn file_read_string(mut xfile: impl Read + Seek) -> String {
    let mut string_buf = Vec::new();
    let mut c_buf = [0u8; 1];

    loop {
        xfile.read_exact(&mut c_buf).unwrap();
        let c = c_buf[0];
        string_buf.push(c);
        if c == b'\0' {
            break;
        }
    }

    //dbg!(xfile.stream_position().unwrap());
    CString::from_vec_with_nul(string_buf)
        .unwrap()
        .to_string_lossy()
        .to_string()
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct ScriptString(u16);

impl Display for ScriptString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SCRIPT_STRINGS.get().unwrap()[self.0 as usize])
    }
}

static XFILE: OnceLock<XFile> = OnceLock::new();
static SCRIPT_STRINGS: OnceLock<Vec<String>> = OnceLock::new();

pub fn convert_offset_to_ptr(offset: u32) -> (u8, u32) {
    let block = ((offset - 1) >> 29) as u8;
    let off = (offset - 1) & 0x1FFFFFFF;

    let block_sizes = XFILE.get().unwrap().block_size;
    let start = block_sizes[0..block as usize].iter().sum::<u32>();
    let p = start + off;

    //dbg!(block_sizes, block, off, start, p);

    (block, p)
}

fn main() {
    let filename = std::env::args_os()
        .nth(1)
        .unwrap_or(OsString::from_str("cuba.ff").unwrap());
    let cached_filename = Path::new(&filename).with_extension("cache");

    let mut file = if !Path::new(&filename).with_extension("cache").exists() {
        decompress_xfile(filename)
    } else {
        println!("Found inflated cache file, reading...");

        let mut file = std::fs::File::open(cached_filename).unwrap();
        let mut bytes = Vec::new();
        let bytes_read = file.read_to_end(&mut bytes).unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        assert!(bytes_read as u64 == file.metadata().unwrap().len());

        println!("Cache read, size: {} bytes", bytes_read);

        BufReader::with_capacity(0x8000, file)
    };

    let xfile = bincode::deserialize_from::<_, XFile>(&mut file).unwrap();
    dbg!(xfile);
    dbg!(file.stream_len()).unwrap();
    XFILE.set(xfile).unwrap();

    dbg!(file.stream_position().unwrap());

    let mut xasset_list_buf = [0u8; size_of::<XAssetList>()];
    file.read_exact(&mut xasset_list_buf).unwrap();
    let xasset_list = bincode::deserialize::<XAssetList>(&xasset_list_buf).unwrap();

    dbg!(file.stream_position().unwrap());
    println!("fastfile contains {} assets.", xasset_list.assets.size);

    let strings = xasset_list
        .strings
        .to_vec(&mut file)
        .into_iter()
        .map(|s| s.xfile_into(&mut file))
        .collect::<Vec<_>>();
    dbg!(&strings);
    SCRIPT_STRINGS.set(strings).unwrap();

    let assets = xasset_list.assets.to_vec(&mut file);
    dbg!(&assets);
    let mut deserialized_assets = Vec::new();

    for asset in assets {
        dbg!(asset);

        let a = asset.xfile_into(&mut file);

        deserialized_assets.push(a);
    }
}
