use core::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use alloc::{
    boxed::Box,
    ffi::CString,
    format,
    string::{String, ToString},
    vec::Vec,
};

#[allow(unused_imports)]
use crate::prelude::*;

use crate::{Error, ErrorKind, Result, T5XFileDeserialize, T5XFileSerialize, file_line_col};

use serde::{
    Deserialize, Serialize,
    de::{DeserializeOwned, SeqAccess, Visitor},
};

/// Helper macro to ensure the structs we're deserializing are the correct
/// size.
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

pub(crate) use assert_size;

/// C-like `sizeof`. Accepts types and values.
#[macro_export]
macro_rules! size_of {
    ($t:ty) => {
        core::mem::size_of::<$t>()
    };
    ($e:expr) => {
        core::mem::size_of_val($e)
    };
}

// ============================================================================
#[allow(dead_code)]
pub(crate) struct ArrayVisitor<T, const N: usize> {
    element: PhantomData<[T; N]>,
}

impl<T, const N: usize> ArrayVisitor<T, N> {
    #[allow(dead_code)]
    pub const fn new() -> Self {
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
// ============================================================================

#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XStringRaw<'a>(Ptr32<'a, u8>);
assert_size!(XStringRaw, 4);

impl<'a> XStringRaw<'a> {
    pub const fn from_u32(value: u32) -> Self {
        Self(Ptr32::from_u32(value))
    }

    #[allow(dead_code)]
    pub const fn as_u32(self) -> u32 {
        self.0.as_u32()
    }

    pub fn from_str(s: impl AsRef<str>) -> Self {
        if s.as_ref().is_empty() {
            Self::from_u32(0)
        } else {
            Self::from_u32(0xFFFFFFFF)
        }
    }
}

impl<'a> XFileDeserializeInto<XString, ()> for XStringRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<XString> {
        if self.0.is_null() {
            return Ok(XString::new());
        }

        if self.0.is_real() {
            return Ok(XString::new());
        }

        let mut string_buf = Vec::new();

        loop {
            let c = de.load_from_xfile::<u8>()?;

            // Localized strings use CP1252 for languages that use the latin alphabet.
            // `num::is_ascii` returns false for any values > 127, so valid CP1252 characters
            // have to be manually permitted here.
            //
            // FIXME: come up with a more elegant solution
            if !c.is_ascii() && c != 0xF1 && c != 0xDC && c != 0xAE && c != 0xA9 && c != 0x99 {
                return Err(Error::new_with_offset(
                    file_line_col!(),
                    de.stream_pos()? as _,
                    ErrorKind::BrokenInvariant(format!(
                        "XString: c ({c:#02X}) is not valid EASCII",
                    )),
                ));
            }

            string_buf.push(c);
            if c == b'\0' {
                break;
            }
        }

        //dbg!(xfile.stream_position()?);
        Ok(XString(
            CString::from_vec_with_nul(string_buf)
                .unwrap()
                .to_string_lossy()
                .to_string(),
        ))
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct XString(pub String);

impl XFileSerialize<()> for XString {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let mut bytes = self.0.chars().map(|c| c as u8).collect::<Vec<_>>();
        bytes.push(b'\0');

        ser.store_into_xfile(bytes)
    }
}

impl XString {
    pub fn get(&self) -> &str {
        &self.0
    }

    pub const fn new() -> Self {
        Self(String::new())
    }
}

// ============================================================================
/// Trait to deserialize [`Self`] from [`xfile`], then convert [`Self`] to
/// [`T`].
///
/// [`Self`] may have members ([`Ptr32`], [`FlexibleArrayU16`]/[`FlexibleArrayU32`], etc.)
/// that make them very unergonomic to use. Since, if we were to deserialze them without
/// any such conversion, we'd probably end up converting them separately later
/// anyways, it's a nice touch to have both done in one go.
pub trait XFileDeserializeInto<T, U: Copy> {
    /// Deserialize [`Self`] from [`xfile`], then convert [`Self`] to [`T`].
    ///
    /// [`Self`] may have [`repr`] attributes ([`C`], [`packed`]) or members
    /// ([`Ptr32`], [`FlexibleArrayU16`]/[`FlexibleArrayU32`], etc.) that make
    /// them very unergonomic to use. Since, if we were to deserialze them
    /// without any such conversion, we'd probably end up converting them
    /// separately later anyways, it's a nice touch to have both done in one
    /// go.
    fn xfile_deserialize_into(&self, de: &mut impl T5XFileDeserialize, data: U) -> Result<T>;
}

impl<'a, T, U, V, const N: usize> XFileDeserializeInto<[U; N], V> for [T; N]
where
    U: Debug + 'a,
    [U; N]: TryFrom<&'a [U]>,
    <&'a [U] as TryInto<[U; N]>>::Error: Debug,
    T: DeserializeOwned + Clone + Debug + XFileDeserializeInto<U, V>,
    V: Copy,
{
    fn xfile_deserialize_into(&self, de: &mut impl T5XFileDeserialize, data: V) -> Result<[U; N]> {
        self.iter()
            .cloned()
            .map(|t| t.xfile_deserialize_into(de, data))
            .collect::<Result<Vec<_>>>()
            .map(|v| TryInto::<[U; N]>::try_into(v).unwrap())
    }
}
// ============================================================================

// ============================================================================
pub trait XFileSerialize<T: Copy> {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, data: T) -> Result<()>;
}

impl<T: XFileSerialize<U>, U: Copy> XFileSerialize<U> for Option<T> {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, data: U) -> Result<()> {
        if let Some(t) = self {
            t.xfile_serialize(ser, data)
        } else {
            Ok(())
        }
    }
}

impl<T: XFileSerialize<U>, U: Copy> XFileSerialize<U> for Box<T> {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, data: U) -> Result<()> {
        (**self).xfile_serialize(ser, data)
    }
}

impl<T: XFileSerialize<U>, U: Copy> XFileSerialize<U> for Vec<T> {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, data: U) -> Result<()> {
        for t in self {
            t.xfile_serialize(ser, data)?;
        }

        Ok(())
    }
}

macro_rules! impl_xfile_serialize {
    ($($t:ty,)+) => {
        $(
            impl XFileSerialize<()> for $t {
                fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
                    ser.store_into_xfile(*self)
                }
            }
        )+
    }
}

impl_xfile_serialize!(bool, u8, i8, u16, i16, u32, i32, usize, isize, f32, f64,);

impl<T, U, const N: usize> XFileSerialize<U> for [T; N]
where
    T: Serialize + Clone + Debug + XFileSerialize<U>,
    U: Copy,
{
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, data: U) -> Result<()> {
        for t in self.iter() {
            t.xfile_serialize(ser, data)?;
        }

        Ok(())
    }
}
// ============================================================================

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
pub struct Ptr32<'a, T>(u32, PhantomData<&'a mut T>);

impl<'a, T> Default for Ptr32<'a, T> {
    fn default() -> Self {
        Self(0, PhantomData)
    }
}

impl<'a, T> Ptr32<'a, T> {
    pub const fn from_u32(value: u32) -> Self {
        Self(value, PhantomData)
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    pub const fn is_null(&self) -> bool {
        self.as_u32() == 0x00000000
    }

    /// Checks whether the pointer is a "real" offset.
    ///
    /// "Real" offsets in T5 point into a buffer allocated by the XFile loader,
    /// which seems to be used as a sort of ".bss" section, where data that
    /// should be allocated but it's left up to the engine to initialize
    /// goes.
    ///
    /// Non-"real" offsets (`0xFFFFFFFF` or `0xFFFFFFFE`) mean that the data lies
    /// directly after the `struct` containing them, rather than somewhere
    /// independent.
    ///
    /// (The name of this function could probably be better.)
    pub const fn is_real(&self) -> bool {
        self.as_u32() != 0xFFFFFFFF && self.as_u32() != 0xFFFFFFFE
    }

    pub const fn null() -> Self {
        Self(0x00000000, PhantomData)
    }

    /// (The name of this function could probably be better.)
    pub const fn unreal() -> Self {
        Self(0xFFFFFFFF, PhantomData)
    }

    pub const fn cast<U>(self) -> Ptr32<'a, U> {
        Ptr32::<'a, U>(self.0, PhantomData)
    }

    pub const fn from_box<U>(b: &Option<Box<T>>) -> Ptr32<'a, U> {
        if b.is_some() {
            Ptr32::<'a, U>::unreal()
        } else {
            Ptr32::<'a, U>::null()
        }
    }

    pub const fn from_slice<U>(s: &[T]) -> Ptr32<'a, U> {
        if s.is_empty() {
            Ptr32::<'a, U>::null()
        } else {
            Ptr32::<'a, U>::unreal()
        }
    }

    pub const fn to_array(self, size: usize) -> Ptr32Array<'a, T> {
        Ptr32Array { p: self, size }
    }
}

impl<'a, T: DeserializeOwned + Clone + Debug + XFileDeserializeInto<U, V>, U, V: Copy>
    XFileDeserializeInto<Option<Box<U>>, V> for Ptr32<'a, T>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        data: V,
    ) -> Result<Option<Box<U>>> {
        if self.is_null() {
            return Ok(None);
        }

        let t = if self.is_real() {
            if self.0 & 0x1FFFFFFF > de.stream_len().unwrap() as u32 {
                return Err(Error::new_with_offset(
                    file_line_col!(),
                    de.stream_pos()? as _,
                    ErrorKind::InvalidSeek {
                        off: self.0 & 0x1FFFFFFF,
                        max: de.stream_len().unwrap() as u32,
                    },
                ));
            }

            //println!("ignoring offset {:#010X}", self.as_u32());
            return Ok(None);
        } else {
            // no need to seek for 0xFFFFFFFF / 0xFFFFFFFE
            let old = de.stream_pos()?;
            let t = de.load_from_xfile::<T>()?;
            let new = de.stream_pos()?;
            // bincode will sometimes deserialize less than sizeof!(T) bytes
            // since it treats all structs as packed. those instances need to be
            // caught and fixed, so this is how we catch them
            assert!(
                new == old + size_of!(T) as u64,
                "new ({new}) - old ({old}) = {}, expected {}",
                new - old,
                size_of!(T)
            );
            t
        };

        t.xfile_deserialize_into(de, data).map(Box::new).map(Some)
    }
}

impl<'a, T: DeserializeOwned + Debug> Ptr32<'a, T> {
    /// Same principle as [`XFileInto::xfile_into`], except it doesn't do any
    /// type conversion. Useful for the rare structs that don't need any such
    /// conversion.
    pub(crate) fn xfile_get(self, de: &mut impl T5XFileDeserialize) -> Result<Option<T>> {
        if self.is_null() {
            return Ok(None);
        }

        let t = if self.is_real() {
            //eprintln!("ignoring offset {:#010X}", self.as_u32());
            return Ok(None);
        } else {
            // no need to seek for 0xFFFFFFFF / 0xFFFFFFFE
            let old = de.stream_pos()?;
            let t = de.load_from_xfile::<T>();
            let new = de.stream_pos()?;
            // bincode will sometimes deserialize less than sizeof!(T) bytes
            // since it treats all structs as packed. those instances need to be
            // caught and fixed, so this is how we catch them
            assert!(
                new == old + size_of!(T) as u64,
                "new ({new}) - old ({old}) = {}, expected {}",
                new - old,
                size_of!(T)
            );
            t
        };

        t.map(Some)
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

    fn new(count: usize) -> Self;

    fn to_vec(&self, de: &mut impl T5XFileDeserialize) -> Result<Vec<T>> {
        let mut vt = Vec::new();

        let old = de.stream_pos()?;
        for _ in 0..self.count() {
            vt.push(de.load_from_xfile()?);
        }
        let new = de.stream_pos()?;
        // bincode will sometimes deserialize less than sizeof!(T) bytes
        // since it treats all structs as packed. those instances need to be
        // caught and fixed, so this is how we catch them
        assert!(new == old + size_of!(T) as u64 * self.count() as u64);

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

                fn new(count: usize) -> Self {
                    Self {
                        count: count as _,
                        _p: PhantomData,
                    }
                }
            }
        )+
    }
}

impl_flexible_array!(FlexibleArrayU16, FlexibleArrayU32,);

pub trait FatPointer<'a, T: DeserializeOwned + 'a>: Sized {
    fn size(&self) -> usize;
    fn p(&self) -> Ptr32<'a, T>;

    fn new(p: Ptr32<'a, T>, size: usize) -> Self;

    fn is_null(&self) -> bool {
        self.p().is_null()
    }

    fn to_vec(&self, de: &mut impl T5XFileDeserialize) -> Result<Vec<T>> {
        if self.is_null() {
            return Ok(Vec::new());
        }

        let v = if self.p().is_real() {
            //eprintln!("ignoring offset {:#010X}", self.p().as_u32());
            return Ok(Vec::new());
        } else {
            // no need to seek for 0xFFFFFFFF / 0xFFFFFFFE
            let old = de.stream_pos()?;
            let mut v = Vec::new();
            for _ in 0..self.size() {
                v.push(de.load_from_xfile::<T>()?);
            }
            let new = de.stream_pos()?;
            // bincode will sometimes deserialize less than sizeof!(T) bytes
            // since it treats all structs as packed. those instances need to be
            // caught and fixed, so this is how we catch them
            assert!(
                new == old + size_of!(T) as u64 * self.size() as u64,
                "new ({new}) != old ({old}) + {} ({})",
                size_of!(T) * self.size(),
                new - old
            );
            v
        };

        Ok(v)
    }

    fn to_vec_into<U: From<T>>(&self, de: &mut impl T5XFileDeserialize) -> Result<Vec<U>> {
        self.to_vec(de)
            .map(|v| v.into_iter().map(Into::<U>::into).collect())
    }

    fn from_slice<U>(s: &[U]) -> Self {
        if s.is_empty() {
            Self::new(Ptr32::null(), 0)
        } else {
            Self::new(Ptr32::unreal(), s.len())
        }
    }
}

macro_rules! impl_fat_pointer {
    ($($s:ident,)+) => {
        $(
            impl<'a, T: Debug + Clone + DeserializeOwned + 'a> FatPointer<'a, T>
                for $s<'a, T>
            {
                fn new(p: Ptr32<'a, T>, size: usize) -> Self {
                    Self {
                        p,
                        size: size as _,
                    }
                }

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
/// before the pointer instead of after, and sometimes it's a [`u8`] or [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u8`], and comes before the pointer.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FatPointerCountFirstU8<'a, T: Debug + Clone> {
    pub size: u8,
    pub p: Ptr32<'a, T>,
}

/// Newtype for a fat pointer to a `[T]`.
///
/// Represents an offset containing [`Self::size`] [`T`]s.
///
/// Serialized structs often contain these, but sometimes the size comes
/// before the pointer instead of after, and sometimes it's a [`u8`] or [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u16`], and comes before the pointer.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FatPointerCountFirstU16<'a, T: Debug + Clone> {
    pub size: u16,
    pub p: Ptr32<'a, T>,
}

/// Newtype for a fat pointer to a `[T]`.
///
/// Represents an offset containing [`Self::size`] [`T`]s.
///
/// Serialized structs often contain these, but sometimes the size comes
/// before the pointer instead of after, and sometimes it's a [`u8`] or [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u32`], and comes before the pointer.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub struct FatPointerCountFirstU32<'a, T> {
    pub size: u32,
    pub p: Ptr32<'a, T>,
}

/// Newtype for a fat pointer to a `[T]`.
///
/// Represents an offset containing [`Self::size`] [`T`]s.
///
/// Serialized structs often contain these, but sometimes the size comes
/// before the pointer instead of after, and sometimes it's a [`u8`] or [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u8`], and comes after the pointer.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FatPointerCountLastU8<'a, T> {
    pub p: Ptr32<'a, T>,
    pub size: u8,
}

/// Newtype for a fat pointer to a `[T]`.
///
/// Represents an offset containing [`Self::size`] [`T`]s.
///
/// Serialized structs often contain these, but sometimes the size comes
/// before the pointer instead of after, and sometimes it's a [`u8`] or [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u16`], and comes after the pointer.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FatPointerCountLastU16<'a, T> {
    pub p: Ptr32<'a, T>,
    pub size: u16,
}

/// Newtype for a fat pointer to a `[T]`.
///
/// Represents an offset containing [`Self::size`] [`T`]s.
///
/// Serialized structs often contain these, but sometimes the size comes
/// before the pointer instead of after, and sometimes it's a [`u8`] or [`u16`] instead
/// of a [`u32`].
///
/// In this case, [`Self::size`] is a [`u32`], and comes after the pointer.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct FatPointerCountLastU32<'a, T> {
    pub p: Ptr32<'a, T>,
    pub size: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct Ptr32Array<'a, T> {
    pub p: Ptr32<'a, T>,
    pub size: usize,
}

impl_fat_pointer!(
    FatPointerCountFirstU8,
    FatPointerCountFirstU16,
    FatPointerCountFirstU32,
    FatPointerCountLastU8,
    FatPointerCountLastU16,
    FatPointerCountLastU32,
    Ptr32Array,
);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct Ptr32ArrayConst<'a, T, const N: usize>(Ptr32<'a, T>);

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

    fn new(p: Ptr32<'a, T>, size: usize) -> Self {
        assert!(size == N);
        Self(p)
    }
}

// ===============================================================================
// Trying to implement `XFileInto` generically for all `FatPointer<'_, T>` leads to an
// unconstrained type error, so for now we just implement it individually. FIXME

macro_rules! impl_xfile_into_for_fat_pointer {
    ($($s:ident,)+) => {
        $(
            impl<'a, T, U, V> XFileDeserializeInto<Vec<U>, V> for $s<'a, T>
            where
                T: DeserializeOwned + Debug + Clone + XFileDeserializeInto<U, V>,
                V: Copy,
            {
                fn xfile_deserialize_into(&self, de: &mut impl T5XFileDeserialize, data: V) -> Result<Vec<U>> {
                    self.clone()
                        .to_vec(de)?
                        .into_iter()
                        .map(|a| a.xfile_deserialize_into(de, data))
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
impl<'a, T, U, V, const N: usize> XFileDeserializeInto<Vec<U>, V> for Ptr32ArrayConst<'a, T, N>
where
    T: DeserializeOwned + Debug + Clone + XFileDeserializeInto<U, V>,
    V: Copy,
{
    fn xfile_deserialize_into(&self, de: &mut impl T5XFileDeserialize, data: V) -> Result<Vec<U>> {
        self.clone()
            .to_vec(de)?
            .into_iter()
            .map(|a| a.xfile_deserialize_into(de, data))
            .collect()
    }
}
// ===============================================================================
