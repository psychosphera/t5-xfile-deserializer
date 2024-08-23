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
// serialzed into a file. Any pointers in said structs become offsets in the
// file. Occasionally the offsets are NULL or a "real" value, but most of the
// time they're 0xFFFFFFFF or 0xFFFFFFFE, which indicates that, instead of being at a
// specific offset, they come immediately after the current struct. This means
// basically nothing in the file is relocatable.
//
// In addition, if the structures' sizes or alignments don't match exactly what
// the serializer used, or if new structures are added, the file is basically
// un-parseable (this is why, as mentioned above, the versions must match
// exactly). Pulling out only assets of a specific type or by name is also impossible,
// because you can't know where a given asset is at in the file until you pull
// out everything before it too. For this reason, you're more or less forced
// into deserializng everything at once and then grabbing the assets you need
// afterwards. Which, in fairness, makes sense in the context of a game engine (you're
// never going to need to load *half*, or some other fraction, of a level), but it
// *really* doesn't make writing a deserializer fun.

#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::missing_transmute_annotations)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::from_over_into)]
#![allow(clippy::needless_borrows_for_generic_args)]

pub mod clipmap;
pub mod com_world;
pub mod common;
pub mod ddl;
pub mod destructible;
pub mod font;
pub mod fx;
pub mod gameworld;
pub mod gfx_world;
pub mod light;
pub mod menu;
pub mod misc;
pub mod sound;
pub mod techset;
pub mod util;
pub mod weapon;
pub mod xanim;
pub mod xasset;
pub mod xmodel;

use std::{
    collections::VecDeque,
    ffi::CString,
    fmt::{Debug, Display},
    io::{Cursor, Read, Seek, Write},
    marker::PhantomData,
    path::Path,
};

use bincode::{
    config::{BigEndian, FixintEncoding, LittleEndian, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use num_derive::FromPrimitive;
use serde::{de::DeserializeOwned, Deserialize};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "d3d9")]
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;

pub use misc::*;
use util::{StreamLen, *};
use xasset::{XAsset, XAssetList, XAssetRaw, XAssetType};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XFileHeader {
    pub magic: [u8; 8],
    pub version: u32,
}
assert_size!(XFileHeader, 12);

impl XFileHeader {
    pub fn magic_string(&self) -> String {
        self.magic.iter().map(|c| *c as char).collect()
    }

    pub fn magic_is_valid(&self) -> bool {
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
struct ScriptString(u16);

impl ScriptString {
    pub fn to_string(self, de: &T5XFileDeserializer) -> Result<String> {
        de.script_strings
            .get(self.0 as usize)
            .cloned()
            .ok_or(Error::new(
                file_line_col!(),
                ErrorKind::BadScriptString(self.0),
            ))
    }
}

const XFILE_VERSION: u32 = 0x000001D9u32;
const XFILE_VERSION_LE: u32 = XFILE_VERSION.to_le();
const XFILE_VERSION_BE: u32 = XFILE_VERSION.to_be();

#[repr(u32)]
enum XFileVersion {
    LE = XFILE_VERSION_LE,
    BE = XFILE_VERSION_BE,
}

impl XFileVersion {
    fn is_valid(version: u32, platform: XFilePlatform) -> bool {
        Self::from_u32(version)
            .map(|v| v.as_u32())
            .unwrap_or(0xFFFFFFFF) // sentinel value to make life simple
            == Self::from_platform(platform).as_u32()
    }

    fn is_other_endian(version: u32, platform: XFilePlatform) -> bool {
        if platform.is_le() {
            version == Self::BE.as_u32()
        } else {
            version == Self::LE.as_u32()
        }
    }

    fn from_u32(value: u32) -> Option<Self> {
        match value {
            XFILE_VERSION_LE => Some(Self::LE),
            XFILE_VERSION_BE => Some(Self::BE),
            _ => None,
        }
    }

    fn from_platform(platform: XFilePlatform) -> Self {
        match platform {
            XFilePlatform::Windows | XFilePlatform::macOS => XFileVersion::LE,
            XFilePlatform::Xbox360 | XFilePlatform::PS3 => XFileVersion::BE,
            XFilePlatform::Wii => unreachable!(), // safe since the deserializer rejects Wii
                                                  // before this function ever gets called
        }
    }

    fn as_u32(&self) -> u32 {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    pub fn is_le(&self) -> bool {
        match self {
            Self::Windows | Self::macOS => true,
            Self::Xbox360 | Self::PS3 => false,
            Self::Wii => unreachable!(), // safe since the deserializer rejects Wii
                                         // before this function ever gets called
        }
    }

    pub fn is_be(&self) -> bool {
        !self.is_le()
    }

    pub fn is_console(&self) -> bool {
        match self {
            Self::Xbox360 | Self::PS3 | Self::Wii => true,
            Self::Windows | Self::macOS => false,
        }
    }

    pub fn is_pc(&self) -> bool {
        !self.is_console()
    }
}

#[cfg(feature = "d3d9")]
pub struct D3D9State<'a> {
    pub(crate) device: &'a mut IDirect3DDevice9,
}

#[cfg(not(feature = "d3d9"))]
struct D3D9State<'a>(PhantomData<&'a ()>);

/// Trait to seal [`T5XFileDeserializer`]'s typestates.
pub(crate) trait T5XFileDeserializerTypestate {}

pub enum T5XFileDeserializerUninflated {}
pub enum T5XFileDeserializerInflated {}
pub enum T5XFileDeserializerDeserialize {}

impl T5XFileDeserializerTypestate for T5XFileDeserializerUninflated {}
impl T5XFileDeserializerTypestate for T5XFileDeserializerInflated {}
impl T5XFileDeserializerTypestate for T5XFileDeserializerDeserialize {}

#[allow(private_bounds, private_interfaces)]
pub struct T5XFileDeserializer<'a, T: T5XFileDeserializerTypestate = T5XFileDeserializerDeserialize>
{
    silent: bool,
    xfile: XFile,
    script_strings: Vec<String>,
    file: Option<&'a mut std::fs::File>,
    cache_file: Option<&'a mut std::fs::File>,
    reader: Option<Cursor<Vec<u8>>>,
    xasset_list: XAssetList<'a>,
    xassets_raw: VecDeque<XAssetRaw<'a>>,
    deserialized_assets: usize,
    non_null_assets: usize,
    opts: BincodeOptions,
    platform: XFilePlatform,
    d3d9_state: Option<D3D9State<'a>>,
    _p: PhantomData<T>,
}

/// A simple enum that contains all the possible errors this library can return.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Occurs when a [`std::io`] function returns an error.
    Io(std::io::Error),
    /// Occurs when `bincode` couldn't deserialize an object.
    Bincode(Box<bincode::ErrorKind>),
    /// Occurs when an XFile's blob couldn't be inflated.
    Inflate(String),
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
    /// Occurs when an XFile's platform is unsupported (currently just Wii).
    UnsupportedPlatform(XFilePlatform),
    /// Occurs when some part of the library hasn't yet been implemented.
    Todo(String),
    /// Occurs when a [`ScriptString`] doesn't index [`T5XFileDeserializer::script_strings`].
    BadScriptString(u16),
    /// Occurs when an `XAsset`'s `asset_type` isn't a variant of [`XAssetType`].
    InvalidXAssetType(u32),
    /// Occurs when an `XAsset`'s `asset_type` *is* a variant of [`XAssetType`],
    /// but that `asset_type` isn't used by T5.
    UnusedXAssetType(XAssetType),
    /// Occurs when an error is returned by D3D9.
    #[cfg(feature = "d3d9")]
    Windows(windows::core::Error),
}

impl From<std::io::Error> for ErrorKind {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

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

#[macro_export]
macro_rules! file_line_col {
    () => {
        format!("{}:{}:{}", file!(), line!(), column!())
    };
}

#[derive(Debug)]
pub struct Error {
    where_: String,
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(where_: String, kind: ErrorKind) -> Self {
        Self { where_, kind }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn where_(&self) -> String {
        self.where_.clone()
    }
}

pub type Result<T> = core::result::Result<T, Error>;

type BincodeOptionsLE =
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, FixintEncoding>;
type BincodeOptionsBE =
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, BigEndian>, FixintEncoding>;

#[derive(Clone)]
enum BincodeOptions {
    LE(BincodeOptionsLE),
    BE(BincodeOptionsBE),
}

impl BincodeOptions {
    fn new(little_endian: bool) -> Self {
        if little_endian {
            BincodeOptions::LE(
                DefaultOptions::new()
                    .with_little_endian()
                    .with_fixint_encoding(),
            )
        } else {
            BincodeOptions::BE(
                DefaultOptions::new()
                    .with_big_endian()
                    .with_fixint_encoding(),
            )
        }
    }

    fn from_platform(platform: XFilePlatform) -> Self {
        Self::new(platform.is_le())
    }

    fn deserialize_from<T: DeserializeOwned>(&self, reader: impl Read) -> bincode::Result<T> {
        match self {
            Self::LE(opts) => opts.deserialize_from(reader),
            Self::BE(opts) => opts.deserialize_from(reader),
        }
    }
}

pub enum InflateSuccess {
    NewlyInflated,
    AlreadyInflated,
}

pub enum CacheSuccess {
    CacheCreated,
    CacheOverwritten,
}

pub struct T5XFileDeserializerBuilder<'a> {
    silent: bool,
    file: Option<&'a mut std::fs::File>,
    cache_file: Option<&'a mut std::fs::File>,
    platform: XFilePlatform,
    d3d9_state: Option<D3D9State<'a>>,
}

impl<'a> T5XFileDeserializerBuilder<'a> {
    pub fn from_file(file: &'a mut std::fs::File, platform: XFilePlatform) -> Self {
        Self {
            file: Some(file),
            cache_file: None,
            platform,
            silent: false,
            d3d9_state: None,
        }
    }

    pub fn from_cache_file(cache_file: &'a mut std::fs::File, platform: XFilePlatform) -> Self {
        Self {
            file: None,
            cache_file: Some(cache_file),
            platform,
            silent: false,
            d3d9_state: None,
        }
    }

    pub fn with_silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        self
    }

    #[cfg(feature = "d3d9")]
    pub fn with_d3d9(mut self, d3d9_state: Option<D3D9State<'a>>) -> Self {
        self.d3d9_state = d3d9_state;
        self
    }

    pub fn build(mut self) -> Result<T5XFileDeserializer<'a, T5XFileDeserializerUninflated>> {
        if self.file.is_some() {
            T5XFileDeserializer::from_file(
                self.file.take().unwrap(),
                self.silent,
                self.platform,
                self.d3d9_state,
            )
        } else if self.cache_file.is_some() {
            T5XFileDeserializer::from_cache_file(
                self.cache_file.take().unwrap(),
                self.silent,
                self.platform,
                self.d3d9_state,
            )
        } else {
            unreachable!()
        }
    }
}

impl<'a> T5XFileDeserializer<'a, T5XFileDeserializerUninflated> {
    fn from_file(
        file: &'a mut std::fs::File,
        silent: bool,
        platform: XFilePlatform,
        d3d9_state: Option<D3D9State<'a>>,
    ) -> Result<Self> {
        if platform == XFilePlatform::Wii {
            if !silent {
                println!("Wii Fastfiles aren't supported.");
            }
            return Err(Error::new(
                file_line_col!(),
                ErrorKind::UnsupportedPlatform(platform),
            ));
        }

        if !silent && (platform == XFilePlatform::Xbox360 || platform == XFilePlatform::PS3) {
            println!(
                "Warning: {} Fastfiles might (and probably do) have differences\
                 from Windows Fastfiles that aren't accounted for in this\
                 library. Expect problems.",
                platform
            );
        }

        if !silent && platform == XFilePlatform::macOS {
            println!(
                "Warning: macOS Fastfiles are *presumably* identical to\
                 Windows Fastfiles (being an Aspyr port and all), but the\
                 author of this library hasn't yet verified that to be true.\
                 Problems may arise."
            );
        }

        if !silent {
            println!("Found file, reading header...");
        }

        let opts = BincodeOptions::from_platform(platform);

        let header = opts
            .deserialize_from::<XFileHeader>(&mut *file)
            .map_err(|e| Error::new(file_line_col!(), ErrorKind::Bincode(e)))?;

        // dbg!(&header);

        if !header.magic_is_valid() {
            if !silent {
                println!("Fastfile header magic invalid: valid values are IWffu100 and IWff0100");
            }
            return Err(Error::new(
                file_line_col!(),
                ErrorKind::BadHeaderMagic(header.magic_string()),
            ));
        }

        if XFileVersion::is_other_endian(header.version, platform) {
            if !silent {
                println!(
                    "Fastfile header is valid, but it has the wrong endianness\
                     for {} (probably for a different platform).",
                    platform
                );
            }
            return Err(Error::new(
                file_line_col!(),
                ErrorKind::WrongEndiannessForPlatform(platform),
            ));
        }

        if !XFileVersion::is_valid(header.version, platform) {
            if !silent {
                println!(
                    "Fastfile is wrong version (version={:#010X}, expected {:#010X})",
                    header.version,
                    XFileVersion::from_platform(platform).as_u32()
                );
            }

            return Err(Error::new(
                file_line_col!(),
                ErrorKind::WrongVersion(header.version),
            ));
        }

        if !silent {
            println!("Header verified, reading playload...");
        }

        let de = Self {
            silent,
            xfile: XFile::default(),
            script_strings: Vec::default(),
            file: Some(file),
            cache_file: None,
            reader: None,
            xasset_list: XAssetList::default(),
            xassets_raw: VecDeque::new(),
            deserialized_assets: 0,
            non_null_assets: 0,
            opts,
            platform,
            d3d9_state,
            _p: PhantomData,
        };

        Ok(de)
    }

    fn from_cache_file(
        file: &'a mut std::fs::File,
        silent: bool,
        platform: XFilePlatform,
        d3d9_state: Option<D3D9State<'a>>,
    ) -> Result<Self> {
        if platform == XFilePlatform::Wii {
            if !silent {
                println!("Wii Fastfiles aren't supported (does Wii even use Fastfiles?)");
            }
            return Err(Error::new(
                file_line_col!(),
                ErrorKind::UnsupportedPlatform(platform),
            ));
        }

        if !silent {
            println!("Found inflated cache file, reading...");
        }

        Ok(T5XFileDeserializer::<'a, T5XFileDeserializerUninflated> {
            silent,
            xfile: XFile::default(),
            script_strings: Vec::default(),
            file: None,
            cache_file: Some(file),
            reader: None,
            xasset_list: XAssetList::default(),
            xassets_raw: VecDeque::new(),
            deserialized_assets: 0,
            non_null_assets: 0,
            opts: BincodeOptions::from_platform(platform),
            platform,
            d3d9_state,
            _p: PhantomData,
        })
    }

    pub fn inflate(mut self) -> Result<T5XFileDeserializer<'a, T5XFileDeserializerInflated>> {
        assert!(self.reader.is_none());

        let reader = if let Some(f) = self.cache_file.take() {
            let mut decompressed_payload = Vec::new();
            f.read_to_end(&mut decompressed_payload)
                .map_err(|e| Error::new(file_line_col!(), ErrorKind::Io(e)))?;
            Cursor::new(decompressed_payload)
        } else if let Some(f) = self.file.take() {
            let mut compressed_payload = Vec::new();
            f.seek(std::io::SeekFrom::Start(sizeof!(XFileHeader) as _))
                .map_err(|e| Error::new(file_line_col!(), ErrorKind::Io(e)))?;
            dbg!(f
                .stream_position()
                .map_err(|e| Error::new(file_line_col!(), ErrorKind::Io(e)))?);
            let bytes_read = f
                .read_to_end(&mut compressed_payload)
                .map_err(|e| Error::new(file_line_col!(), ErrorKind::Io(e)))?;
            if !self.silent {
                println!("Payload read, inflating... (this may take a while)");
            }
            let decompressed_payload = inflate::inflate_bytes_zlib(&compressed_payload)
                .map_err(|e| Error::new(file_line_col!(), ErrorKind::Inflate(e)))?;
            if !self.silent {
                println!(
                    "Payload inflated, compressed size: {} bytes, decompressed size: {} bytes",
                    bytes_read,
                    decompressed_payload.len()
                );
            }
            Cursor::new(decompressed_payload)
        } else {
            unreachable!() // safe since the constructors had to populate at least self.cache_file
        };

        self.reader = Some(reader);

        let xasset_list = {
            let mut file = self.reader.as_mut().unwrap();
            let xfile = self
                .opts
                .deserialize_from::<XFile>(&mut file)
                .map_err(|e| Error::new(file_line_col!(), ErrorKind::Bincode(e)))?;

            dbg!(xfile);
            dbg!(StreamLen::stream_len(&mut file)?);
            self.xfile = xfile;

            dbg!(file
                .stream_position()
                .map_err(|e| Error::new(file_line_col!(), ErrorKind::Io(e)))?);
            let xasset_list = self
                .opts
                .deserialize_from::<XAssetList>(&mut file)
                .map_err(|e| Error::new(file_line_col!(), ErrorKind::Bincode(e)))?;
            dbg!(&xasset_list);
            dbg!(file
                .stream_position()
                .map_err(|e| Error::new(file_line_col!(), ErrorKind::Io(e)))?);
            xasset_list
        };

        if !self.silent {
            println!("Fastfile contains {} assets.", xasset_list.assets.size());
        }

        let de = T5XFileDeserializer::<T5XFileDeserializerInflated> {
            silent: self.silent,
            xfile: self.xfile,
            script_strings: Vec::new(),
            file: self.file,
            cache_file: self.cache_file,
            reader: self.reader,
            xasset_list,
            xassets_raw: VecDeque::new(),
            deserialized_assets: self.deserialized_assets,
            non_null_assets: self.non_null_assets,
            opts: self.opts,
            platform: self.platform,
            d3d9_state: self.d3d9_state,
            _p: PhantomData,
        };

        Ok(de)
    }
}

impl<'a> T5XFileDeserializer<'a, T5XFileDeserializerInflated> {
    pub fn cache(
        mut self,
        path: impl AsRef<Path>,
    ) -> Result<(
        T5XFileDeserializer<'a, T5XFileDeserializerDeserialize>,
        CacheSuccess,
    )> {
        if !self.silent {
            println!("Caching decompressed payload to disk...");
        }

        let cache_exists = path.as_ref().exists();

        let mut f = std::fs::File::create(path)
            .map_err(|e| Error::new(file_line_col!(), ErrorKind::Io(e)))?;
        let pos = self.reader.as_ref().unwrap().position();
        let v = self.reader.take().unwrap().into_inner();
        f.write_all(&v)
            .map_err(|e| Error::new(file_line_col!(), ErrorKind::Io(e)))?;
        self.reader = Some(Cursor::new(v));
        self.reader.as_mut().unwrap().set_position(pos);

        if !self.silent {
            println!("Decompressed payload cached.");
        }

        let mut de = T5XFileDeserializer::<'a, T5XFileDeserializerDeserialize> {
            silent: self.silent,
            xfile: self.xfile,
            script_strings: Vec::new(),
            file: self.file,
            cache_file: self.cache_file,
            reader: self.reader,
            xasset_list: self.xasset_list,
            xassets_raw: self.xassets_raw,
            deserialized_assets: self.deserialized_assets,
            non_null_assets: self.non_null_assets,
            opts: self.opts,
            platform: self.platform,
            d3d9_state: self.d3d9_state,
            _p: PhantomData,
        };

        de.get_script_strings_and_assets()?;

        if cache_exists {
            Ok((de, CacheSuccess::CacheOverwritten))
        } else {
            Ok((de, CacheSuccess::CacheCreated))
        }
    }

    pub fn no_cache(self) -> Result<T5XFileDeserializer<'a, T5XFileDeserializerDeserialize>> {
        let mut de = T5XFileDeserializer::<'a, T5XFileDeserializerDeserialize> {
            silent: self.silent,
            xfile: self.xfile,
            script_strings: Vec::new(),
            file: self.file,
            cache_file: self.cache_file,
            reader: self.reader,
            xasset_list: self.xasset_list,
            xassets_raw: self.xassets_raw,
            deserialized_assets: self.deserialized_assets,
            non_null_assets: self.non_null_assets,
            opts: self.opts,
            platform: self.platform,
            d3d9_state: self.d3d9_state,
            _p: PhantomData,
        };

        de.get_script_strings_and_assets()?;

        Ok(de)
    }
}

impl<'a> T5XFileDeserializer<'a, T5XFileDeserializerDeserialize> {
    pub fn deserialize_next(&mut self) -> Result<Option<XAsset>> {
        let Some(asset) = self.xassets_raw.pop_front() else {
            return Ok(None);
        };

        let asset = XAsset::try_get(self, asset, self.platform);
        //dbg!(&asset);
        if let Ok(ref a) = asset {
            self.deserialized_assets += 1;
            if a.is_some() {
                self.non_null_assets += 1;
            }

            if !self.silent {
                println!(
                    "Successfully deserialized {} asset{} ({} non-null).",
                    self.deserialized_assets,
                    if self.deserialized_assets > 1 {
                        "s"
                    } else {
                        ""
                    },
                    self.non_null_assets,
                );
            }
        }

        asset.map(Some)
    }

    pub fn deserialize_remaining(mut self) -> Result<Vec<XAsset>> {
        let mut deserialized_assets = Vec::new();

        while let Some(asset) = self.deserialize_next()? {
            deserialized_assets.push(asset);
        }

        Ok(deserialized_assets)
    }

    pub(crate) fn stream_pos(&mut self) -> Result<u64> {
        self.reader
            .as_mut()
            .unwrap()
            .stream_position()
            .map_err(|e| Error::new(file_line_col!(), ErrorKind::Io(e)))
    }

    // pub(crate) fn seek_and<T, F: FnOnce(&mut Self) -> T>(
    //     &mut self,
    //     from: SeekFrom,
    //     predicate: F,
    // ) -> Result<T> {
    //     let pos = self.reader.as_mut().unwrap().stream_position()?;
    //
    //     if let std::io::SeekFrom::Start(p) = from {
    //         if p != 0xFFFFFFFF && p != 0xFFFFFFFE {
    //             let (_, off) = self.convert_offset_to_ptr(p as _)?;
    //             let len = StreamLen::stream_len(self.reader.as_mut().unwrap())?;
    //             if off as u64 > len {
    //                 return Err(Error::InvalidSeek { off, max: len as _ });
    //             }
    //             self.reader
    //                 .as_mut()
    //                 .unwrap()
    //                 .seek(std::io::SeekFrom::Start(off as _))?;
    //         }
    //     } else if let std::io::SeekFrom::Current(p) = from {
    //         if p != 0 {
    //             let len = StreamLen::stream_len(self.reader.as_mut().unwrap())?;
    //             let off = pos as i64 + p;
    //             if pos as i64 + p > len as i64 {
    //                 return Err(Error::InvalidSeek {
    //                     off: off as _,
    //                     max: len as _,
    //                 });
    //             }
    //             self.reader.as_mut().unwrap().seek(from)?;
    //         }
    //     } else {
    //         unimplemented!()
    //     }
    //
    //     let t = predicate(self);
    //
    //     if let std::io::SeekFrom::Start(p) = from {
    //         if p != 0xFFFFFFFF && p != 0xFFFFFFFE {
    //             self.reader
    //                 .as_mut()
    //                 .unwrap()
    //                 .seek(std::io::SeekFrom::Start(pos))?;
    //         }
    //     } else if let std::io::SeekFrom::Current(p) = from {
    //         if p != 0 {
    //             self.reader
    //                 .as_mut()
    //                 .unwrap()
    //                 .seek(std::io::SeekFrom::Current(-p))?;
    //         }
    //     } else {
    //         unimplemented!()
    //     }
    //
    //     Ok(t)
    // }

    pub(crate) fn load_from_xfile<T: DeserializeOwned>(&mut self) -> Result<T> {
        self.opts
            .deserialize_from(self.reader.as_mut().unwrap())
            .map_err(|e| Error::new(file_line_col!(), ErrorKind::Bincode(e)))
    }

    // pub(crate) fn convert_offset_to_ptr(&self, offset: u32) -> Result<(u8, u32)> {
    //     let block = ((offset - 1) >> 29) as u8;
    //     let off = (offset - 1) & 0x1FFFFFFF;
    //
    //     let start = self.xfile.block_size[0..block as usize].iter().sum::<u32>();
    //     let p = start + off;
    //
    //     //dbg!(block_sizes, block, off, start, p);
    //
    //     Ok((block, p))
    // }

    fn get_script_strings_and_assets(&mut self) -> Result<()> {
        let xasset_list = self.xasset_list;

        self.script_strings = xasset_list
            .strings
            .to_vec(self)?
            .into_iter()
            .map(|s| s.xfile_into(self, ()))
            .collect::<Result<Vec<_>>>()?;
        //dbg!(&strings);

        let assets = xasset_list.assets.to_vec(self)?;
        self.xassets_raw = VecDeque::from_iter(assets);

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) const fn create_d3d9(&self) -> bool {
        self.d3d9_state.is_some()
    }

    #[allow(dead_code)]
    pub(crate) fn d3d9_state(&mut self) -> Option<&mut D3D9State<'a>> {
        self.d3d9_state.as_mut()
    }
}
