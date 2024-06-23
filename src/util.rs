use crate::*;

use std::{
    ffi::CString,
    fmt::{self, Debug},
    io::{Read, Seek},
    marker::PhantomData,
};

use serde::{
    de::{DeserializeOwned, Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

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

// ============================================================================
//
// [`MaterialTechniqueSetRaw`] (see below) contains an array with 130 elements.
// However, [`Deserialize`] isn't implemented for arrays of that size (wanna
// say 24 is the max?), so we have to do it ourselves here.

pub(crate) trait BigArray<'de>: Sized {
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

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XString<'a>(Ptr32<'a, u8>);
assert_size!(XString, 4);

impl<'a> XString<'a> {
    pub fn from_u32(value: u32) -> Self {
        Self(Ptr32::from_u32(value))
    }

    pub fn as_u32(self) -> u32 {
        self.0.as_u32()
    }
}

impl<'a> XFileInto<String, ()> for XString<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> String {
        //dbg!(*self);

        if self.as_u32() == 0x00000000 {
            return String::new();
        } else if self.as_u32() != 0xFFFFFFFF {
            println!("ignoring offset");
            return String::new();
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.as_u32() as _), |f| {
                file_read_string(f)
            })
            .unwrap()
    }
}

pub(crate) fn file_read_string(mut xfile: impl Read + Seek) -> String {
    let mut string_buf = Vec::new();
    let mut c_buf = [0u8; 1];

    loop {
        xfile.read_exact(&mut c_buf).unwrap();
        let c = c_buf[0];
        assert!(c.is_ascii(), "c={c:#02X}");
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

/// Trait to deserialize [`Self`] from [`xfile`], then convert [`Self`] to
/// [`T`].
///
/// [`Self`] may have [`repr`] attributes ([`C`], [`packed`]) or members
/// ([`Ptr32`], [`FlexibleArrayU16`]/[`FlexibleArrayU32`], etc.) that make
/// them very unergonomic to use. Since, if we were to deserialze them without
/// any such conversion, we'd probably end up converting them separately later
/// anyways, it's a nice touch to have both done in one go.
pub(crate) trait XFileInto<T, U: Copy> {
    /// Deserialize [`Self`] from [`xfile`], then convert [`Self`] to [`T`].
    ///
    /// [`Self`] may have [`repr`] attributes ([`C`], [`packed`]) or members
    /// ([`Ptr32`], [`FlexibleArrayU16`]/[`FlexibleArrayU32`], etc.) that make
    /// them very unergonomic to use. Since, if we were to deserialze them
    /// without any such conversion, we'd probably end up converting them
    /// separately later anyways, it's a nice touch to have both done in one
    /// go.
    fn xfile_into(&self, xfile: impl Read + Seek, data: U) -> T;
}

impl<'a, T, U, V, const N: usize> XFileInto<[U; N], V> for [T; N]
where
    U: Debug + 'a,
    [U; N]: TryFrom<&'a [U]>,
    <&'a [U] as TryInto<[U; N]>>::Error: Debug,
    T: DeserializeOwned + Clone + Debug + XFileInto<U, V>,
    V: Copy,
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> [U; N] {
        self.iter()
            .cloned()
            .map(|t| t.xfile_into(&mut xfile, data))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
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
pub(crate) struct Ptr32<'a, T>(pub(crate) u32, PhantomData<&'a mut T>);

impl<'a, T> Default for Ptr32<'a, T> {
    fn default() -> Self {
        Self(0, PhantomData::default())
    }
}

impl<'a, T> Ptr32<'a, T> {
    pub(crate) fn from_u32(value: u32) -> Self {
        Self(value, PhantomData)
    }

    pub(crate) fn as_u32(&self) -> u32 {
        self.0
    }

    pub(crate) fn cast<U>(self) -> Ptr32<'a, U> {
        Ptr32::<'a, U>(self.0, PhantomData)
    }

    pub(crate) fn to_array(self, size: usize) -> Ptr32Array<'a, T> {
        Ptr32Array { p: self, size }
    }
}

pub(crate) trait SeekAnd: Read + Seek {
    fn seek_and<T>(
        &mut self,
        from: std::io::SeekFrom,
        predicate: impl FnOnce(&mut Self) -> T,
    ) -> std::io::Result<T> {
        let pos = self.stream_position()?;

        if let std::io::SeekFrom::Start(p) = from {
            if p != 0xFFFFFFFF && p != 0xFFFFFFFE {
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
            if p != 0xFFFFFFFF && p != 0xFFFFFFFE {
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

impl<'a, T: DeserializeOwned + Clone + Debug + XFileInto<U, V>, U, V: Copy>
    XFileInto<Option<Box<U>>, V> for Ptr32<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Option<Box<U>> {
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
            .map(|t| Box::new(t.xfile_into(xfile, data)))
    }
}

impl<'a, T: DeserializeOwned + Debug> Ptr32<'a, T> {
    /// Same principle as [`XFileInto::xfile_into`], except it doesn't do any
    /// type conversion. Useful for the rare structs that don't need any such
    /// conversion.
    pub(crate) fn xfile_get(self, mut xfile: impl Read + Seek) -> Option<Box<T>> {
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
pub(crate) struct FlexibleArrayU16<T: DeserializeOwned> {
    count: u16,
    _p: PhantomData<T>,
}

impl<T: DeserializeOwned> FlexibleArrayU16<T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    pub(crate) fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
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
pub(crate) struct FlexibleArrayU32<T: DeserializeOwned> {
    count: u32,
    _p: PhantomData<T>,
}

impl<T: DeserializeOwned> FlexibleArrayU32<T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    pub(crate) fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
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
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FatPointerCountFirstU16<'a, T: Debug + Clone> {
    size: u16,
    p: Ptr32<'a, T>,
}

impl<'a, T: DeserializeOwned + Debug + Clone> FatPointerCountFirstU16<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    pub(crate) fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        if self.p.0 != 0xFFFFFFFF {
            println!("ignoring offset");
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

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U, V>, U, V: Copy> XFileInto<Vec<U>, V>
    for FatPointerCountFirstU16<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile, data))
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
pub(crate) struct FatPointerCountFirstU32<'a, T> {
    pub size: u32,
    pub p: Ptr32<'a, T>,
}

impl<'a, T: DeserializeOwned + Debug> FatPointerCountFirstU32<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    pub(crate) fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        if self.p.0 != 0xFFFFFFFF {
            println!("ignoring offset");
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

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U, V>, U, V: Copy> XFileInto<Vec<U>, V>
    for FatPointerCountFirstU32<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile, data))
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
pub(crate) struct FatPointerCountLastU16<'a, T> {
    p: Ptr32<'a, T>,
    size: u16,
}

impl<'a, T: DeserializeOwned + Debug> FatPointerCountLastU16<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    pub(crate) fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        if self.p.0 != 0xFFFFFFFF {
            println!("ignoring offset");
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

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U, V>, U, V: Copy> XFileInto<Vec<U>, V>
    for FatPointerCountLastU16<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile, data))
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
pub(crate) struct FatPointerCountLastU32<'a, T> {
    p: Ptr32<'a, T>,
    size: u32,
}

impl<'a, T: DeserializeOwned + Debug> FatPointerCountLastU32<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    pub(crate) fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        if self.p.0 != 0xFFFFFFFF {
            println!("ignoring offset");
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

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U, V>, U, V: Copy> XFileInto<Vec<U>, V>
    for FatPointerCountLastU32<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile, data))
            .collect()
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct Ptr32Array<'a, T> {
    p: Ptr32<'a, T>,
    size: usize,
}

impl<'a, T: DeserializeOwned + Debug> Ptr32Array<'a, T> {
    /// Deserializes [`self.count`] [`T`]s into a [`Vec<T>`].
    pub(crate) fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        if self.p.0 != 0xFFFFFFFF {
            println!("ignoring offset");
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

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U, V>, U, V: Copy> XFileInto<Vec<U>, V>
    for Ptr32Array<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile, data))
            .collect()
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct Ptr32ArrayConst<'a, T, const N: usize>(Ptr32<'a, T>);

impl<'a, T: Clone + DeserializeOwned + Debug, const N: usize> Ptr32ArrayConst<'a, T, N> {
    /// Deserializes [`N`] [`T`]s into a [`Vec<T>`].
    pub(crate) fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.0.as_u32() == 0x00000000 {
            return Vec::new();
        }

        if self.0.as_u32() != 0xFFFFFFFF {
            println!("ignoring offset");
            return Vec::new();
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.0.as_u32() as _), |mut f| {
                let mut vt = Vec::new();

                for _ in 0..N {
                    vt.push(bincode::deserialize_from::<_, T>(&mut f).unwrap());
                }

                vt
            })
            .ok()
            .unwrap_or_default()
    }
}

impl<'a, T: DeserializeOwned + Debug + Clone + XFileInto<U, V>, U, V: Copy, const N: usize>
    XFileInto<Vec<U>, V> for Ptr32ArrayConst<'a, T, N>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Vec<U> {
        self.clone()
            .to_vec(&mut xfile)
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile, data))
            .collect()
    }
}
