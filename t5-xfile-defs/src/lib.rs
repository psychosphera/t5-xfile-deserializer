// FastFiles (internally known as XFiles) are structured as follows (all
// values native endian - that is, little endian for Windows and macOS, big
// endian for Xbox 360, PS3, and presumably Wii):
//
// ----------------------------------------------------------------------------
// | Offset    | Size | Field       | Description                             |
// ----------------------------------------------------------------------------
// | 0x0000000 | 4    | Magic       | Magic value to identify the file as an  |
// |           |      |             | XFile. Will always be ASCII "IWff".     |
// ----------------------------------------------------------------------------
// | 0x0000004 | 1    | Compression | Magic value to identify the             |
// |           |      |             | compression method used. Will always be |
// |           |      |             | ASCII '0' for Xbox 360 and PS3, and     |
// |           |      |             | *seems* to always be 'u' for Windows    |
// |           |      |             | (might be different for, e.g., modded   |
// |           |      |             | maps). Unsure for Wii, macOS is         |
// |           |      |             | presumably the same as Windows.         |
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
// |           |      |             | deserializer is expecting. For all      |
// |           |      |             | release builds of T5, that value is     |
// |           |      |             | 0x000001D9.                             |
// ----------------------------------------------------------------------------
// | 0x000000C | *    | Blob        | The rest of the file is a DEFLATE-      |
// |           |      |             | compressed blob. To get the "real"      |
// |           |      |             | contents of the file, it must be        |
// |           |      |             | inflated.                               |
// ----------------------------------------------------------------------------
//
// XFiles don't contain an easy way to detect the platform they're compiled
// for. The endianness of the Version field can serve as a simple sanity
// check (i.e., if the expected platform is Windows but Version is
// big-endian, then the platform is obviously wrong), but since both
// little- and big-endian have multiple potential platforms, the correct
// platform can't be derived for certain, and even if the endianness matches
// the expected platform, that's no guarantee the expected platform is correct.
//
// (In theory, one could probably use structure differences between platforms
// or other known values that differ between platforms to verify the correct
// platform, but someone else can do that.)
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
// serialzed into a file. Pointers in said structs are either set to 0xFFFFFFFF
// or 0xFFFFFFFE (unsure of the difference between the two), which indicates
// that the data for said pointers comes after the current struct and any
// previous 0xFFFFFFFF-pointers in said struct, NULL, or to a "real" value,
// which is used by T5 as a pointer into a buffer allocated by the XFile
// loader. This buffer seems to act as a sort of ".bss" section, for pointers
// that should be allocated, but that it's the engine's job to initialize. One
// large buffer is presumably used for efficiency purposes (one single
// allocation versus many if each object is allocated separately, no memory
// fragmentation, etc.) and because, assuming the data is valid and the engine
// is in a valid state, there's no concern about clobbering other objects in
// the buffer, but in principle there's no reason it can't be done on a
// per-object basis.
//
// In addition, if the structures' sizes or alignments don't match exactly what
// the serializer used, or if new structures are added, the file is basically
// un-parseable (this is why, as mentioned above, the versions must match
// exactly). Pulling out only assets of a specific type or by name is also
// impossible, because you can't know where a given asset is at in the file
// until you pull out everything before it too. For this reason, you're more or
// less forced into deserializng everything at once and then grabbing the
// assets you need afterwards. Which, in fairness, makes sense in the context
// of a game engine (you're never going to need to load *half*, or some other
// fraction, of a level), but it *really* doesn't make writing a deserializer
// fun. Makes the serializer pretty easy tho -_-

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::missing_transmute_annotations)]
#![allow(clippy::from_over_into)]
#![allow(clippy::needless_lifetimes)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod clipmap;
pub mod com_world;
pub mod common;
pub mod ddl;
pub mod destructible;
pub mod emblem;
pub mod font;
pub mod fx;
pub mod gameworld;
pub mod gfx_world;
pub mod glass;
pub mod light;
pub mod menu;
pub mod misc;
mod prelude;
pub mod sound;
pub mod techset;
pub mod util;
pub mod weapon;
pub mod xanim;
pub mod xasset;
pub mod xmodel;

use alloc::{
    fmt::{Debug, Display},
    string::String,
};

use serde::{Deserialize, de::DeserializeOwned};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "d3d9")]
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;

pub use misc::*;
pub use util::*;
use xasset::XAssetType;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XFileHeader {
    pub magic: [u8; 8],
    pub version: u32,
}
assert_size!(XFileHeader, 12);

pub const XFILE_HEADER_MAGIC_U: &str = "IWffu100";
pub const XFILE_HEADER_MAGIC_0: &str = "IWff0100";
pub const XFILE_HEADER_MAGIC_U_RAW: [u8; 8] = *b"IWffu100";
pub const XFILE_HEADER_MAGIC_0_RAW: [u8; 8] = *b"IWff0100";

impl XFileHeader {
    pub const fn new(platform: XFilePlatform) -> Self {
        let magic = if platform.is_console() {
            XFILE_HEADER_MAGIC_0_RAW
        } else {
            XFILE_HEADER_MAGIC_U_RAW
        };

        let version = XFileVersion::from_platform(platform).as_u32();

        Self { magic, version }
    }

    pub fn magic_string(&self) -> String {
        self.magic.iter().map(|c| *c as char).collect()
    }

    pub const fn magic_is_valid(&self) -> bool {
        // won't work in a const fn
        // self.magic == XFILE_HEADER_MAGIC_U_RAW || self.magic == XFILE_HEADER_MAGIC_0_RAW
        self.magic[0] == b'I'
            && self.magic[1] == b'W'
            && self.magic[2] == b'f'
            && self.magic[3] == b'f'
            && (self.magic[4] == b'u' || self.magic[4] == b'0')
            && self.magic[5] == b'1'
            && self.magic[6] == b'0'
            && self.magic[7] == b'0'
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XFile {
    pub size: u32,
    pub external_size: u32,
    pub block_size: [u32; 7],
}
assert_size!(XFile, 36);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct ScriptString(u16);

impl ScriptString {
    pub fn to_string(self, de: &mut impl T5XFileDeserialize) -> Result<String> {
        de.get_script_string(self)?.ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadScriptString(self.0),
        ))
    }

    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

const XFILE_VERSION: u32 = 0x000001D9u32;
const XFILE_VERSION_LE: u32 = XFILE_VERSION.to_le();
const XFILE_VERSION_BE: u32 = XFILE_VERSION.to_be();

#[cfg(target_endian = "little")]
const XFILE_VERSION_OE: u32 = XFILE_VERSION_BE;

#[cfg(target_endian = "big")]
const XFILE_VERSION_OE: u32 = XFILE_VERSION_LE;

#[repr(u32)]
pub enum XFileVersion {
    LE = XFILE_VERSION_LE,
    BE = XFILE_VERSION_BE,
}

impl XFileVersion {
    pub const fn is_valid(version: u32, platform: XFilePlatform) -> bool {
        let version = if let Some(v) = Self::from_u32(version) {
            v.as_u32()
        } else {
            return false;
        };

        version == Self::from_platform(platform).as_u32()
    }

    pub const fn is_other_endian(version: u32) -> bool {
        version == XFILE_VERSION_OE
    }

    pub const fn from_u32(value: u32) -> Option<Self> {
        match value {
            XFILE_VERSION_LE => Some(Self::LE),
            XFILE_VERSION_BE => Some(Self::BE),
            _ => None,
        }
    }

    pub const fn from_platform(platform: XFilePlatform) -> Self {
        match platform {
            XFilePlatform::Windows | XFilePlatform::macOS => XFileVersion::LE,
            XFilePlatform::Xbox360 | XFilePlatform::PS3 => XFileVersion::BE,
            XFilePlatform::Wii => unreachable!(), // safe since the deserializer rejects Wii
                                                  // before this function ever gets called
        }
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::LE => XFILE_VERSION_LE,
            Self::BE => XFILE_VERSION_BE,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum XFilePlatform {
    Windows,
    macOS,
    Xbox360,
    PS3,
    Wii,
}

impl Display for XFilePlatform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Self::Windows => "Windows",
            Self::macOS => "macOS",
            Self::Xbox360 => "Xbox 360",
            Self::PS3 => "PS3",
            Self::Wii => "Wii",
        };
        write!(f, "{}", s)
    }
}

impl XFilePlatform {
    pub const fn is_le(&self) -> bool {
        match self {
            Self::Windows | Self::macOS => true,
            Self::Xbox360 | Self::PS3 => false,
            Self::Wii => unreachable!(), // safe since the deserializer rejects Wii
                                         // before this function ever gets called
        }
    }

    pub const fn is_be(&self) -> bool {
        !self.is_le()
    }

    pub const fn is_console(&self) -> bool {
        match self {
            Self::Xbox360 | Self::PS3 | Self::Wii => true,
            Self::Windows | Self::macOS => false,
        }
    }

    pub const fn is_pc(&self) -> bool {
        !self.is_console()
    }
}

pub struct XFileOffset(u32);

impl XFileOffset {
    pub const fn from_u32(offset: u32) -> Self {
        Self(offset)
    }

    pub const fn block(&self) -> u8 {
        (((self.0 - 1) >> 29) & 0x00000007) as _
    }

    pub const fn offset(&self) -> u32 {
        (self.0 - 1) & 0x1FFFFFFF
    }
}

/// A simple enum that contains all the possible errors this library can return.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    #[cfg(feature = "std")]
    /// Occurs when a [`std::io`] function returns an error.
    Io(std::io::Error),
    #[cfg(feature = "bincode")]
    /// Occurs when `bincode` couldn't deserialize an object.
    Bincode(Box<bincode::ErrorKind>),
    /// Occurs when an XFile's blob couldn't be inflated.
    Inflate(String),
    /// Occurs when an XFile's blob couldn't be deflated.
    Deflate(String),
    /// Occurs when `num::FromPrimitive::from_*` return [`None`].
    BadFromPrimitive(i64),
    /// Occurs when `bitflags::from_bits` returns [`None`].
    BadBitflags(u32),
    /// Occurs when a character has invalid encoding.
    BadChar(u32),
    /// Occurs when an invariant expected by the deserializer is broken.
    /// Likely indicates the file is corrupt or some deserialization logic is wrong
    BrokenInvariant(String),
    /// Occurs when attempting to seek to an offset beyond the bounds of a file.
    InvalidSeek { off: u32, max: u32 },
    /// Occurs when an XFile's `magic` field is invalid.
    /// Likely indicates the file is corrupt or isn't an XFile.
    BadHeaderMagic(String),
    /// Occurs when an XFile's version doesn't match the expected version ([`XFILE_VERSION`]).
    WrongVersion(u32),
    /// Occurs when an XFile has the wrong endianness for the given platform.
    WrongEndiannessForPlatform(XFilePlatform),
    /// Occurs when an XFile's platform is unimplemented (currently just Wii).
    UnimplementedPlatform(XFilePlatform),
    /// Occurs when an XFile's platform is unsupported (all platforms except Windows).
    UnsupportedPlatform(XFilePlatform),
    /// Occurs when some part of the library hasn't yet been implemented.
    Todo(String),
    /// Occurs when a [`ScriptString`] isn't a valid index.
    BadScriptString(u16),
    /// Occurs when more than [`u16::MAX`] [`ScriptString`]s are present.
    ScriptStringOverflow,
    /// Occurs when an `XAsset`'s `asset_type` isn't a variant of [`XAssetType`].
    InvalidXAssetType(u32),
    /// Occurs when an `XAsset`'s `asset_type` *is* a variant of [`XAssetType`],
    /// but that `asset_type` isn't used by T5.
    UnusedXAssetType(XAssetType),
    /// Occurs when an error is returned by D3D9.
    #[cfg(feature = "d3d9")]
    Windows(windows::core::Error),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for ErrorKind {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[cfg(feature = "bincode")]
impl From<Box<bincode::ErrorKind>> for ErrorKind {
    fn from(value: Box<bincode::ErrorKind>) -> Self {
        Self::Bincode(value)
    }
}

impl From<String> for ErrorKind {
    fn from(value: String) -> Self {
        Self::Inflate(value)
    }
}

#[cfg(feature = "d3d9")]
impl From<windows::core::Error> for ErrorKind {
    fn from(value: windows::core::Error) -> Self {
        Self::Windows(value)
    }
}

macro_rules! file_line_col {
    () => {
        alloc::format!("{}:{}:{}", file!(), line!(), column!())
    };
}

pub(crate) use file_line_col;

#[derive(Debug)]
pub struct Error {
    where_: String,
    kind: ErrorKind,
    off: Option<u32>,
}

impl Error {
    pub const fn new(where_: String, kind: ErrorKind) -> Self {
        Self {
            where_,
            kind,
            off: None,
        }
    }

    pub const fn new_with_offset(where_: String, off: u32, kind: ErrorKind) -> Self {
        Self {
            where_,
            kind,
            off: Some(off),
        }
    }

    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn where_(&self) -> &str {
        &self.where_
    }

    pub const fn off(&self) -> Option<u32> {
        self.off
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait T5XFileDeserialize {
    fn stream_pos(&mut self) -> Result<u64>;
    fn stream_len(&mut self) -> Result<u64>;

    fn load_from_xfile<T: DeserializeOwned>(&mut self) -> Result<T>;

    /// Returns [`Ok(Some)`] if `string` is present, [`Ok(None)`]
    /// if not, or, depending on the implementation, [`Err`].
    fn get_script_string(&mut self, string: ScriptString) -> Result<Option<String>>;
}

pub trait T5XFileSerialize {
    fn store_into_xfile<T: Serialize>(&mut self, t: T) -> Result<()>;

    /// Returns [`Ok(Some)`] when `string` was already present, [`Ok(None)`]
    /// when `string` wasn't already present, or [`Err`] when
    /// [`Error::ScriptStringOverflow`] or some other error occurs.
    fn get_or_insert_script_string(&mut self, string: String) -> Result<Option<String>>;
}
