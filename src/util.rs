use crate::*;

use std::{
    ffi::CString,
    fmt::{self, Debug},
    io::{Read, Seek, SeekFrom},
    marker::PhantomData,
};

use serde::{
    de::{DeserializeOwned, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
#[cfg(feature = "serde")]
use serde::{ser::SerializeTuple, Serializer};

/// Helper macro to ensure the structs we're deserializing are the correct
/// size.
#[macro_export]
macro_rules! assert_size {
    ($t:ty, $n:literal) => {
        const _: fn() = || {
            let _ = core::mem::transmute::<$t, [u8; $n]>;
        };
    };
    ($t:ty, $e:expr) => {
        const _: fn() = || {
            let _ = core::mem::transmute::<$t, [u8; $e]>;
        };
    };
}

/// C-like `sizeof`. Accepts types and values.
#[macro_export]
macro_rules! sizeof {
    ($t:ty) => {
        core::mem::size_of::<$t>()
    };
    ($e:expr) => {
        core::mem::size_of_val($e)
    };
}

// ============================================================================
// A couple of the structs we have to deserialize have arrays bigger than
// serialize and deserialize are implemented for by default, so we have to
// implement that ourselves.

pub(crate) trait BigArray<'de>: Sized {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error>;
    #[cfg(feature = "serde")]
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>;
}

pub(crate) struct ArrayVisitor<T, const N: usize> {
    element: PhantomData<[T; N]>,
}

impl<T, const N: usize> ArrayVisitor<T, N> {
    pub fn new() -> Self {
        Self {
            element: PhantomData,
        }
    }
}

impl<'de, T: Default + Copy + Deserialize<'de>, const N: usize> Visitor<'de>
    for ArrayVisitor<T, N>
{
    type Value = [T; N];

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("an array of length {}", N))
    }

    #[allow(clippy::needless_range_loop)]
    fn visit_seq<A>(self, mut seq: A) -> core::result::Result<[T; N], A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut arr = [T::default(); N];
        for i in 0..N {
            arr[i] = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
        }
        Ok(arr)
    }
}

#[macro_export]
macro_rules! big_array {
    ($($len:expr,)+) => {
        $(
            impl<'de, T: Default + Copy + Deserialize<'de> + Serialize> BigArray<'de> for [T; $len] {
                fn deserialize<D>(
                    deserializer: D
                ) -> core::result::Result<[T; $len], D::Error>
                    where D: Deserializer<'de>
                {
                    let visitor = ArrayVisitor::<T, $len>::new();
                    deserializer.deserialize_tuple($len, visitor)
                }

                #[cfg(feature = "serde")]
                fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
                    let mut st = serializer.serialize_tuple($len)?;
                    for t in self {
                        st.serialize_element(&t)?;
                    };
                    st.end()
                }
            }
        )+
    }
}

big_array! {
    64,
    130,
}
// ============================================================================

// ============================================================================
/// [`Seek::stream_len`]` isn't stable yet, so we implement it manually here
pub(crate) trait StreamLen: Seek {
    fn stream_len(&mut self) -> std::io::Result<u64> {
        let pos = self.stream_position()?;
        let len = self.seek(SeekFrom::End(0))?;
        self.seek(SeekFrom::Start(pos))?;
        Ok(len)
    }
}

impl<T: Seek> StreamLen for T {}
// ============================================================================

#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<String> {
        //dbg!(*self);

        if self.0.is_null() {
            return Ok(String::new());
        }

        if self.as_u32() != 0xFFFFFFFF {
            eprintln!("ignoring offset");
            return Ok(String::new());
        }

        xfile.seek_and(std::io::SeekFrom::Start(self.as_u32() as _), |f| {
            xfile_read_string(f)
        })?
    }
}

pub(crate) fn xfile_read_string(mut xfile: impl Read + Seek) -> Result<String> {
    let mut string_buf = Vec::new();

    loop {
        let c = load_from_xfile::<u8>(&mut xfile)?;

        if !c.is_ascii() {
            return Err(Error::BrokenInvariant(format!(
                "{}: XString: c ({c:#02X}) is not valid ASCII",
                file_line_col!()
            )));
        }

        string_buf.push(c);
        if c == b'\0' {
            break;
        }
    }

    //dbg!(xfile.stream_position()?);
    Ok(CString::from_vec_with_nul(string_buf)
        .unwrap()
        .to_string_lossy()
        .to_string())
}

/// Trait to deserialize [`Self`] from [`xfile`], then convert [`Self`] to
/// [`T`].
///
/// [`Self`] may have members ([`Ptr32`], [`FlexibleArrayU16`]/[`FlexibleArrayU32`], etc.)
/// that make them very unergonomic to use. Since, if we were to deserialze them without
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
    fn xfile_into(&self, xfile: impl Read + Seek, data: U) -> Result<T>;
}

impl<'a, T, U, V, const N: usize> XFileInto<[U; N], V> for [T; N]
where
    U: Debug + 'a,
    [U; N]: TryFrom<&'a [U]>,
    <&'a [U] as TryInto<[U; N]>>::Error: Debug,
    T: DeserializeOwned + Clone + Debug + XFileInto<U, V>,
    V: Copy,
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Result<[U; N]> {
        self.iter()
            .cloned()
            .map(|t| t.xfile_into(&mut xfile, data))
            .collect::<Result<Vec<_>>>()
            .map(|v| TryInto::<[U; N]>::try_into(v).unwrap())
    }
}

/// Newtype to handle pointer members of serialized structs.
///
/// We use this instead of a [`u32`] for two reasons. One, to differentiate
/// between actual [`u32`]s and offsets. And two, so that we can implement
/// [`XFileInto`] to retrieve the pointed-to data.
///
/// We can't use [`*const T`] or [`*mut T`] for two reasons.
/// * Pointer members of the serialzed structs are converted to offsets
///   within the XFile during serialization (as noted above), so they wouldn't
///   be valid pointers. Also, they're often [`0xFFFFFFFF`] anyways, so again,
///   invalid pointers.
/// * T5 and its associated tools are all 32-bit programs using 4-byte
///   pointers, and [`*const T`]/[`*mut T`] are probably going to be 8 bytes
///   on any machine this is compiled for.
///
/// Also, pointers are unsafe and just annoying to use compared to a [`u32`].
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(transparent)]
pub(crate) struct Ptr32<'a, T>(u32, PhantomData<&'a mut T>);

impl<'a, T> Default for Ptr32<'a, T> {
    fn default() -> Self {
        Self(0, PhantomData)
    }
}

impl<'a, T> Ptr32<'a, T> {
    pub fn from_u32(value: u32) -> Self {
        Self(value, PhantomData)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn is_null(&self) -> bool {
        self.as_u32() == 0x00000000
    }

    pub fn cast<U>(self) -> Ptr32<'a, U> {
        Ptr32::<'a, U>(self.0, PhantomData)
    }

    pub fn to_array(self, size: usize) -> Ptr32Array<'a, T> {
        Ptr32Array { p: self, size }
    }
}

pub(crate) trait SeekAnd: Read + Seek + StreamLen {
    fn seek_and<T>(
        &mut self,
        from: std::io::SeekFrom,
        predicate: impl FnOnce(&mut Self) -> T,
    ) -> Result<T> {
        let pos = self.stream_position()?;

        if let std::io::SeekFrom::Start(p) = from {
            if p != 0xFFFFFFFF && p != 0xFFFFFFFE {
                let (_, off) = convert_offset_to_ptr(p as _)?;
                let len = StreamLen::stream_len(self)?;
                if off as u64 > len {
                    return Err(Error::InvalidSeek { off, max: len as _ });
                }
                self.seek(std::io::SeekFrom::Start(off as _))?;
            }
        } else if let std::io::SeekFrom::Current(p) = from {
            let len = StreamLen::stream_len(self)?;
            let off = pos as i64 + p;
            if pos as i64 + p > len as i64 {
                return Err(Error::InvalidSeek {
                    off: off as _,
                    max: len as _,
                });
            }
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
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Result<Option<Box<U>>> {
        if self.is_null() {
            return Ok(None);
        }

        if self.as_u32() != 0xFFFFFFFF {
            eprintln!("ignoring offset");
            return Ok(None);
        }

        xfile
            .seek_and(
                std::io::SeekFrom::Start(self.as_u32() as _),
                |f| -> Result<T> { load_from_xfile(f) },
            )??
            .xfile_into(xfile, data)
            .map(Box::new)
            .map(Some)
    }
}

impl<'a, T: DeserializeOwned + Debug> Ptr32<'a, T> {
    /// Same principle as [`XFileInto::xfile_into`], except it doesn't do any
    /// type conversion. Useful for the rare structs that don't need any such
    /// conversion.
    pub(crate) fn xfile_get(self, mut xfile: impl Read + Seek) -> Result<Option<T>> {
        if self.is_null() {
            return Ok(None);
        }

        if self.as_u32() != 0xFFFFFFFF {
            eprintln!("ignoring offset");
            return Ok(None);
        }

        xfile.seek_and(std::io::SeekFrom::Start(self.as_u32() as _), |f| {
            load_from_xfile(f)
        })?
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
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
#[repr(transparent)]
pub(crate) struct FlexibleArrayU16<T: DeserializeOwned> {
    count: u16,
    _p: PhantomData<T>,
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
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
#[repr(transparent)]
pub(crate) struct FlexibleArrayU32<T: DeserializeOwned> {
    count: u32,
    _p: PhantomData<T>,
}

pub(crate) trait FlexibleArray<T: DeserializeOwned> {
    fn count(&self) -> usize;

    fn to_vec(&self, mut xfile: impl Read + Seek) -> Result<Vec<T>> {
        let mut vt = Vec::new();

        for _ in 0..self.count() {
            vt.push(load_from_xfile(&mut xfile)?);
        }

        Ok(vt)
    }
}

macro_rules! impl_flexible_array {
    ($($s:ident,)+) => {
        $(
            impl<T: DeserializeOwned> FlexibleArray<T> for $s<T> {
                fn count(&self) -> usize {
                    self.count as _
                }
            }
        )+
    }
}

impl_flexible_array!(FlexibleArrayU16, FlexibleArrayU32,);

pub(crate) trait FatPointer<'a, T: DeserializeOwned + 'a> {
    fn size(&self) -> usize;
    fn p(&self) -> Ptr32<'a, T>;

    fn is_null(&self) -> bool {
        self.p().is_null()
    }

    fn to_vec(&self, mut xfile: impl Read + Seek) -> Result<Vec<T>> {
        if self.p().is_null() {
            return Ok(Vec::new());
        }

        if self.p().as_u32() != 0xFFFFFFFF {
            eprintln!("ignoring offset");
            return Ok(Vec::new());
        }

        xfile
            .seek_and(std::io::SeekFrom::Start(self.p().as_u32() as _), |mut f| {
                let mut vt = Vec::new();

                for _ in 0..self.size() {
                    vt.push(load_from_xfile(&mut f));
                }

                vt
            })?
            .into_iter()
            .collect::<Result<Vec<_>>>()
    }

    fn to_vec_into<U: From<T>>(&self, xfile: impl Read + Seek) -> Result<Vec<U>> {
        self.to_vec(xfile)
            .map(|v| v.into_iter().map(Into::<U>::into).collect())
    }
}

macro_rules! impl_fat_pointer {
    ($($s:ident,)+) => {
        $(
            impl<'a, T: Debug + Clone + DeserializeOwned + 'a> FatPointer<'a, T>
                for $s<'a, T>
            {
                fn size(&self) -> usize {
                    self.size as _
                }

                fn p(&self) -> Ptr32<'a, T> {
                    self.p.clone()
                }
            }
        )+
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
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FatPointerCountFirstU16<'a, T: Debug + Clone> {
    size: u16,
    p: Ptr32<'a, T>,
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
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct FatPointerCountFirstU32<'a, T> {
    size: u32,
    p: Ptr32<'a, T>,
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
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FatPointerCountLastU16<'a, T> {
    p: Ptr32<'a, T>,
    size: u16,
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
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct FatPointerCountLastU32<'a, T> {
    p: Ptr32<'a, T>,
    size: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct Ptr32Array<'a, T> {
    p: Ptr32<'a, T>,
    size: usize,
}

impl_fat_pointer!(
    FatPointerCountFirstU16,
    FatPointerCountFirstU32,
    FatPointerCountLastU16,
    FatPointerCountLastU32,
    Ptr32Array,
);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct Ptr32ArrayConst<'a, T, const N: usize>(Ptr32<'a, T>);

// Can't use the macro for this since it has the const generic parameter
impl<'a, T: Debug + Clone + DeserializeOwned + 'a, const N: usize> FatPointer<'a, T>
    for Ptr32ArrayConst<'a, T, N>
{
    fn size(&self) -> usize {
        N
    }

    fn p(&self) -> Ptr32<'a, T> {
        self.0.clone()
    }
}

// ===============================================================================
// Trying to implement `XFileInto` generically for all `FatPointer<'_, T>` leads to an
// unconstrained type error, so for now we just implement it individually. FIXME

macro_rules! impl_xfile_into_for_fat_pointer {
    ($($s:ident,)+) => {
        $(
            impl<'a, T, U, V> XFileInto<Vec<U>, V> for $s<'a, T>
            where
                T: DeserializeOwned + Debug + Clone + XFileInto<U, V>,
                V: Copy,
            {
                fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Result<Vec<U>> {
                    self.clone()
                        .to_vec(&mut xfile)?
                        .into_iter()
                        .map(|a| a.xfile_into(&mut xfile, data))
                        .collect()
                }
            }
        )+
    }
}

impl_xfile_into_for_fat_pointer!(
    FatPointerCountFirstU16,
    FatPointerCountFirstU32,
    FatPointerCountLastU16,
    FatPointerCountLastU32,
    Ptr32Array,
);

// Can't use the macro for this since it has the const generic parameter
impl<'a, T, U, V, const N: usize> XFileInto<Vec<U>, V> for Ptr32ArrayConst<'a, T, N>
where
    T: DeserializeOwned + Debug + Clone + XFileInto<U, V>,
    V: Copy,
{
    fn xfile_into(&self, mut xfile: impl Read + Seek, data: V) -> Result<Vec<U>> {
        self.clone()
            .to_vec(&mut xfile)?
            .into_iter()
            .map(|a| a.xfile_into(&mut xfile, data))
            .collect()
    }
}
// ===============================================================================
