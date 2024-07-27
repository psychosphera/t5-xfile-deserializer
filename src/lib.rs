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
// |           |      |             | release builds of T5, the value is      |
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

pub mod com_world;
pub mod common;
pub mod ddl;
pub mod destructible;
pub mod font;
pub mod fx;
pub mod gameworld;
pub mod light;
pub mod menu;
pub mod misc;
pub mod sound;
pub mod techset;
pub mod util;
pub mod weapon;
pub mod xanim;
pub mod xmodel;

use std::{
    collections::VecDeque,
    ffi::CString,
    fmt::{Debug, Display},
    io::{Cursor, Read, Seek, Write},
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
};

use bincode::{
    config::{BigEndian, FixintEncoding, LittleEndian, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use num_derive::FromPrimitive;
use serde::{de::DeserializeOwned, Deserialize};

#[cfg(feature = "serde")]
use serde::Serialize;

pub use misc::*;
use util::{StreamLen, *};

const MAX_LOCAL_CLIENTS: usize = 1;

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

impl Display for ScriptString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let script_strings = SCRIPT_STRINGS.lock().map_err(|_| std::fmt::Error)?;

        let s = script_strings.as_ref().and_then(|v| v.get(self.0 as usize));

        if let Some(s) = s {
            write!(f, "{}", s)
        } else {
            write!(f, "")
        }
    }
}

static XFILE: Mutex<Option<XFile>> = Mutex::new(None);
static SCRIPT_STRINGS: Mutex<Option<Arc<Vec<String>>>> = Mutex::new(None);
static BINCODE_OPTIONS: Mutex<Option<BincodeOptions>> = Mutex::new(None);

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
            .unwrap_or(0xFFFFFFFF)
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

pub struct T5XFileDeserializer<'a> {
    silent: bool,
    xfile: XFile,
    script_strings: Arc<Vec<String>>,
    file: Option<&'a mut std::fs::File>,
    cache_file: Option<&'a mut std::fs::File>,
    reader: Option<Cursor<Vec<u8>>>,
    xassets_raw: VecDeque<XAssetRaw<'a>>,
    deserialized_assets: usize,
    opts: BincodeOptions,
    last_xfile: Option<XFile>,
    last_script_strings: Option<Arc<Vec<String>>>,
    last_opts: Option<BincodeOptions>,
}

#[derive(Debug)]
pub enum Error {
    Poison(Box<dyn std::error::Error>),
    Io(std::io::Error),
    Bincode(Box<bincode::ErrorKind>),
    Inflate(String),
    NotInflated,
    BadOffset(u32),
    BadFromPrimitive(i64),
    BadBitflags(u32),
    BadChar(u32),
    BrokenInvariant(String),
    InvalidSeek { off: u32, max: u32 },
    BadHeaderMagic(String),
    WrongVersion(u32),
    WrongEndiannessForPlatform(XFilePlatform),
    UnsupportedPlatform(XFilePlatform),
    Other(Box<dyn std::error::Error>),
    Todo(String),
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Self::Poison(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<Box<bincode::ErrorKind>> for Error {
    fn from(value: Box<bincode::ErrorKind>) -> Self {
        Self::Bincode(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Inflate(value)
    }
}

#[macro_export]
macro_rules! file_line_col {
    () => {
        format!("{}:{}:{}", file!(), line!(), column!())
    };
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

type Locks = (
    MutexGuard<'static, Option<Arc<Vec<String>>>>,
    MutexGuard<'static, Option<XFile>>,
    MutexGuard<'static, Option<BincodeOptions>>,
);

fn make_de_current(de: &mut T5XFileDeserializer) -> Result<Locks> {
    let script_strings = SCRIPT_STRINGS.lock();
    let s = match script_strings {
        Ok(mut s) => {
            de.last_script_strings = s.clone();
            *s = Some(de.script_strings.clone());
            s
        }
        Err(e) => return Err(Error::Poison(Box::new(e))),
    };

    let xfile = XFILE.lock();
    let f = match xfile {
        Ok(mut f) => {
            de.last_xfile = *f;
            *f = Some(de.xfile);
            f
        }
        Err(e) => return Err(Error::Poison(Box::new(e))),
    };

    let opts = BINCODE_OPTIONS.lock();
    let o = match opts {
        Ok(mut o) => {
            de.last_opts = o.clone();
            *o = Some(de.opts.clone());
            o
        }
        Err(e) => return Err(Error::Poison(Box::new(e))),
    };

    Ok((s, f, o))
}

fn release_de(de: &mut T5XFileDeserializer, locks: Locks) {
    let (mut s, mut f, mut o) = locks;
    *s = de.last_script_strings.take();
    *f = de.last_xfile.take();
    *o = de.last_opts.take();
    // locks get dropped at the end of scope
}

fn de_do<T, F: FnOnce(&mut T5XFileDeserializer) -> Result<T>>(
    de: &mut T5XFileDeserializer,
    pred: F,
) -> Result<T> {
    let locks = make_de_current(de)?;
    let t = pred(de);
    release_de(de, locks);
    t
}

impl<'a> T5XFileDeserializer<'a> {
    pub fn from_file(
        mut file: &'a mut std::fs::File,
        silent: bool,
        inflate: bool,
        platform: XFilePlatform,
    ) -> Result<Self> {
        if platform == XFilePlatform::Wii {
            if !silent {
                println!("Wii Fastfiles aren't supported.");
            }
            return Err(Error::UnsupportedPlatform(platform));
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
            println!("Found file, reading header...",);
        }

        let opts = BincodeOptions::from_platform(platform);

        let header = opts.deserialize_from::<XFileHeader>(&mut file)?;

        dbg!(&header);

        if !header.magic_is_valid() {
            if !silent {
                println!("Fastfile header magic invalid: valid values are IWffu100 and IWff0100");
            }
            return Err(Error::BadHeaderMagic(header.magic_string()));
        }

        if XFileVersion::is_other_endian(header.version, platform) {
            if !silent {
                println!(
                    "Fastfile header is valid, but it has the wrong endianness\
                     for {} (probably for a different platform).",
                    platform
                );
            }
            return Err(Error::WrongEndiannessForPlatform(platform));
        }

        if !XFileVersion::is_valid(header.version, platform) {
            if !silent {
                println!(
                    "Fastfile is wrong version (version={:#010X}, expected {:#010X})",
                    header.version,
                    XFileVersion::from_platform(platform).as_u32()
                );
            }

            return Err(Error::WrongVersion(header.version));
        }

        if !silent {
            println!("Header verified, reading playload...");
        }

        let mut de = Self {
            silent,
            xfile: XFile::default(),
            script_strings: Arc::default(),
            file: Some(file),
            cache_file: None,
            reader: None,
            xassets_raw: VecDeque::new(),
            deserialized_assets: 0,
            opts,
            last_xfile: None,
            last_script_strings: None,
            last_opts: None,
        };

        if inflate {
            de.inflate()?;
        }

        Ok(de)
    }

    pub fn from_cache_file(
        file: &'a mut std::fs::File,
        silent: bool,
        platform: XFilePlatform,
    ) -> Result<Self> {
        if platform == XFilePlatform::Wii {
            if !silent {
                println!("Wii Fastfiles aren't supported (does Wii even use Fastfiles?)");
            }
            return Err(Error::UnsupportedPlatform(platform));
        }

        if !silent {
            println!("Found inflated cache file, reading...");
        }

        Ok(Self {
            silent,
            xfile: XFile::default(),
            script_strings: Arc::default(),
            file: None,
            cache_file: Some(file),
            reader: None,
            xassets_raw: VecDeque::new(),
            deserialized_assets: 0,
            opts: BincodeOptions::from_platform(platform),
            last_xfile: None,
            last_script_strings: None,
            last_opts: None,
        })
    }

    pub fn inflate(&mut self) -> Result<()> {
        if self.reader.is_some() {
            if !self.silent {
                println!("Cannot inflate: already inflated.");
            }
            return Ok(());
        }

        let reader = if let Some(f) = self.cache_file.take() {
            let mut decompressed_payload = Vec::new();
            f.read_to_end(&mut decompressed_payload)?;
            Cursor::new(decompressed_payload)
        } else if let Some(f) = self.file.take() {
            let mut compressed_payload = Vec::new();
            f.seek(std::io::SeekFrom::Start(sizeof!(XFileHeader) as _))?;
            dbg!(f.stream_position()?);
            let bytes_read = f.read_to_end(&mut compressed_payload)?;
            if !self.silent {
                println!("Payload read, inflating... (this may take a while)");
            }
            let decompressed_payload = inflate::inflate_bytes_zlib(&compressed_payload)?;
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
            let xfile = self.opts.deserialize_from::<XFile>(&mut file)?;

            dbg!(xfile);
            dbg!(StreamLen::stream_len(&mut file))?;
            self.xfile = xfile;

            dbg!(file.stream_position()?);
            let xasset_list = self.opts.deserialize_from::<XAssetList>(&mut file)?;
            dbg!(&xasset_list);
            dbg!(file.stream_position()?);
            xasset_list
        };

        if !self.silent {
            println!("Fastfile contains {} assets.", xasset_list.assets.size());
        }

        de_do(self, |de| {
            let mut file = de.reader.as_mut().unwrap();
            let strings = xasset_list
                .strings
                .to_vec(&mut file)?
                .into_iter()
                .map(|s| s.xfile_into(&mut file, ()))
                .collect::<Result<Vec<_>>>()?;
            //dbg!(&strings);
            de.script_strings = Arc::new(strings);

            let assets = xasset_list.assets.to_vec(de.reader.as_mut().unwrap())?;

            de.xassets_raw = VecDeque::from_iter(assets);
            Ok(())
        })
    }

    pub fn cache(&mut self, path: impl AsRef<Path>) -> Result<()> {
        if !self.silent {
            println!("Caching decompressed payload to disk...");
        }

        let mut f = std::fs::File::create(path)?;
        let pos = self.reader.as_ref().unwrap().position();
        let v = self.reader.take().unwrap().into_inner();
        f.write_all(&v)?;
        self.reader = Some(Cursor::new(v));
        self.reader.as_mut().unwrap().set_position(pos);

        if !self.silent {
            println!("Decompressed payload cached.");
        }

        Ok(())
    }

    pub fn deserialize_next(&mut self) -> Result<Option<XAsset>> {
        if self.reader.is_none() {
            return Err(Error::NotInflated);
        }

        let Some(asset) = self.xassets_raw.pop_front() else {
            return Ok(None);
        };

        let a = de_do(self, |de| asset.xfile_into(de.reader.as_mut().unwrap(), ()));
        if a.is_ok() {
            self.deserialized_assets += 1;

            if !self.silent {
                println!(
                    "Successfully deserialized {} asset{}.",
                    self.deserialized_assets,
                    if self.deserialized_assets > 1 {
                        "s"
                    } else {
                        ""
                    }
                );
            }
        }

        a.map(Some)
    }

    pub fn deserialize_remaining(mut self) -> Result<Vec<XAsset>> {
        let mut deserialized_assets = Vec::new();

        while let Some(asset) = self.deserialize_next()? {
            deserialized_assets.push(asset);
        }

        Ok(deserialized_assets)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAsset {
    PhysPreset(Option<Box<xmodel::PhysPreset>>),
    PhysConstraints(Option<Box<xmodel::PhysConstraints>>),
    DestructibleDef(Option<Box<destructible::DestructibleDef>>),
    XAnimParts(Option<Box<xanim::XAnimParts>>),
    XModel(Option<Box<xmodel::XModel>>),
    Material(Option<Box<techset::Material>>),
    TechniqueSet(Option<Box<techset::MaterialTechniqueSet>>),
    Image(Option<Box<techset::GfxImage>>),
    Sound(Option<Box<sound::SndBank>>),
    SoundPatch(Option<Box<sound::SndPatch>>),
    ComWorld(Option<Box<com_world::ComWorld>>),
    GameWorldSp(Option<Box<gameworld::GameWorldSp>>),
    GameWorldMp(Option<Box<gameworld::GameWorldMp>>),
    MapEnts(Option<Box<MapEnts>>),
    LightDef(Option<Box<light::GfxLightDef>>),
    Font(Option<Box<font::Font>>),
    MenuList(Option<Box<menu::MenuList>>),
    Menu(Option<Box<menu::MenuDef>>),
    LocalizeEntry(Option<Box<LocalizeEntry>>),
    Weapon(Option<Box<weapon::WeaponVariantDef>>),
    SndDriverGlobals(Option<Box<sound::SndDriverGlobals>>),
    Fx(Option<Box<fx::FxEffectDef>>),
    ImpactFx(Option<Box<fx::FxImpactTable>>),
    RawFile(Option<Box<RawFile>>),
    StringTable(Option<Box<StringTable>>),
    PackIndex(Option<Box<PackIndex>>),
    XGlobals(Option<Box<XGlobals>>),
    Ddl(Option<Box<ddl::DdlRoot>>),
    Glasses(Option<Box<Glasses>>),
    EmblemSet(Option<Box<EmblemSet>>),
}

impl XAsset {
    pub fn is_some(&self) -> bool {
        match self {
            Self::PhysPreset(p) => p.is_some(),
            Self::PhysConstraints(p) => p.is_some(),
            Self::DestructibleDef(p) => p.is_some(),
            Self::XAnimParts(p) => p.is_some(),
            Self::XModel(p) => p.is_some(),
            Self::Material(p) => p.is_some(),
            Self::TechniqueSet(p) => p.is_some(),
            Self::Image(p) => p.is_some(),
            Self::Sound(p) => p.is_some(),
            Self::SoundPatch(p) => p.is_some(),
            Self::ComWorld(p) => p.is_some(),
            Self::GameWorldSp(p) => p.is_some(),
            Self::GameWorldMp(p) => p.is_some(),
            Self::MapEnts(p) => p.is_some(),
            Self::LightDef(p) => p.is_some(),
            Self::Font(p) => p.is_some(),
            Self::MenuList(p) => p.is_some(),
            Self::Menu(p) => p.is_some(),
            Self::LocalizeEntry(p) => p.is_some(),
            Self::Weapon(p) => p.is_some(),
            Self::SndDriverGlobals(p) => p.is_some(),
            Self::Fx(p) => p.is_some(),
            Self::ImpactFx(p) => p.is_some(),
            Self::RawFile(p) => p.is_some(),
            Self::StringTable(p) => p.is_some(),
            Self::PackIndex(p) => p.is_some(),
            Self::XGlobals(p) => p.is_some(),
            Self::Ddl(p) => p.is_some(),
            Self::Glasses(p) => p.is_some(),
            Self::EmblemSet(p) => p.is_some(),
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::PhysPreset(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::PhysConstraints(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::DestructibleDef(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::XAnimParts(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::XModel(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Material(p) => p.as_ref().map(|p| p.info.name.as_str()),
            Self::TechniqueSet(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Image(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Sound(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::SoundPatch(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::ComWorld(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::GameWorldSp(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::GameWorldMp(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::MapEnts(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::LightDef(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Font(p) => p.as_ref().map(|p| p.font_name.as_str()),
            Self::MenuList(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Menu(p) => p.as_ref().map(|p| p.window.name.as_str()),
            Self::LocalizeEntry(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Weapon(p) => p.as_ref().map(|p| p.internal_name.as_str()),
            Self::SndDriverGlobals(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Fx(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::ImpactFx(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::RawFile(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::StringTable(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::PackIndex(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::XGlobals(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Ddl(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::Glasses(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::EmblemSet(_) => Some("emblemset"),
        }
    }
}

/// Helper function to deserialze [`T`] from [`xfile`].
fn load_from_xfile<T: DeserializeOwned>(xfile: impl Read + Seek) -> Result<T> {
    BINCODE_OPTIONS
        .lock()
        .map_err(|e| Error::Poison(Box::new(e)))?
        .as_mut()
        .unwrap()
        .deserialize_from::<T>(xfile)
        .map_err(Error::Bincode)
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Deserialize)]
struct XAssetList<'a> {
    strings: FatPointerCountFirstU32<'a, XString<'a>>,
    assets: FatPointerCountFirstU32<'a, XAssetRaw<'a>>,
}
assert_size!(XAssetList, 16);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
struct XAssetRaw<'a> {
    asset_type: u32,
    asset_data: Ptr32<'a, ()>,
}
assert_size!(XAssetRaw, 8);

/// T5 doesn't actually use all of these.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug, FromPrimitive)]
#[repr(u32)]
enum XAssetType {
    #[default]
    XMODELPIECES = 0x00,
    PHYSPRESET = 0x01,
    PHYSCONSTRAINTS = 0x02,
    DESTRUCTIBLEDEF = 0x03,
    XANIMPARTS = 0x04,
    XMODEL = 0x05,
    MATERIAL = 0x06,
    TECHNIQUE_SET = 0x07,
    IMAGE = 0x08,
    SOUND = 0x09,
    SOUND_PATCH = 0x0A,
    CLIPMAP = 0x0B,
    CLIPMAP_PVS = 0x0C,
    COMWORLD = 0x0D,
    GAMEWORLD_SP = 0x0E,
    GAMEWORLD_MP = 0x0F,
    MAP_ENTS = 0x10,
    GFXWORLD = 0x11,
    LIGHT_DEF = 0x12,
    UI_MAP = 0x13,
    FONT = 0x14,
    MENULIST = 0x15,
    MENU = 0x16,
    LOCALIZE_ENTRY = 0x17,
    WEAPON = 0x18,
    WEAPONDEF = 0x19,
    WEAPON_VARIANT = 0x1A,
    SNDDRIVER_GLOBALS = 0x1B,
    FX = 0x1C,
    IMPACT_FX = 0x1D,
    AITYPE = 0x1E,
    MPTYPE = 0x1F,
    MPBODY = 0x20,
    MPHEAD = 0x21,
    CHARACTER = 0x22,
    XMODELALIAS = 0x23,
    RAWFILE = 0x24,
    STRINGTABLE = 0x25,
    PACKINDEX = 0x26,
    XGLOBALS = 0x27,
    DDL = 0x28,
    GLASSES = 0x29,
    EMBLEMSET = 0x2A,
    STRING = 0x2B,
    ASSETLIST = 0x2C,
}

impl<'a> XFileInto<XAsset, ()> for XAssetRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> Result<XAsset> {
        let asset_type = num::FromPrimitive::from_u32(self.asset_type)
            .ok_or(Error::BadFromPrimitive(self.asset_type as _))?;
        Ok(match asset_type {
            XAssetType::PHYSPRESET => XAsset::PhysPreset(
                self.asset_data
                    .cast::<xmodel::PhysPresetRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::PHYSCONSTRAINTS => XAsset::PhysConstraints(
                self.asset_data
                    .cast::<xmodel::PhysConstraintsRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::DESTRUCTIBLEDEF => XAsset::DestructibleDef(
                self.asset_data
                    .cast::<destructible::DestructibleDefRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::XANIMPARTS => XAsset::XAnimParts(
                self.asset_data
                    .cast::<xanim::XAnimPartsRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::XMODEL => XAsset::XModel(
                self.asset_data
                    .cast::<xmodel::XModelRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::MATERIAL => XAsset::Material(
                self.asset_data
                    .cast::<techset::MaterialRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::TECHNIQUE_SET => XAsset::TechniqueSet(
                self.asset_data
                    .cast::<techset::MaterialTechniqueSetRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::IMAGE => XAsset::Image(
                self.asset_data
                    .cast::<techset::GfxImageRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::SOUND => XAsset::Sound(
                self.asset_data
                    .cast::<sound::SndBankRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::SOUND_PATCH => XAsset::SoundPatch(
                self.asset_data
                    .cast::<sound::SndPatchRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::COMWORLD => XAsset::ComWorld(
                self.asset_data
                    .cast::<com_world::ComWorldRaw>()
                    .xfile_into(xfile, ())?,  
            ),
            XAssetType::GAMEWORLD_SP => XAsset::GameWorldSp(
                self.asset_data
                    .cast::<gameworld::GameWorldSpRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::GAMEWORLD_MP => XAsset::GameWorldMp(
                self.asset_data
                    .cast::<gameworld::GameWorldMpRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::MAP_ENTS => {
                XAsset::MapEnts(self.asset_data.cast::<MapEntsRaw>().xfile_into(xfile, ())?)
            }
            XAssetType::LIGHT_DEF => XAsset::LightDef(
                self.asset_data
                    .cast::<light::GfxLightDefRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::FONT => XAsset::Font(
                self.asset_data
                    .cast::<font::FontRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::MENULIST => XAsset::MenuList(
                self.asset_data
                    .cast::<menu::MenuListRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::MENU => XAsset::Menu(
                self.asset_data
                    .cast::<menu::MenuDefRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::LOCALIZE_ENTRY => XAsset::LocalizeEntry(
                self.asset_data
                    .cast::<LocalizeEntryRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::WEAPON => XAsset::Weapon(
                self.asset_data
                    .cast::<weapon::WeaponVariantDefRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::SNDDRIVER_GLOBALS => XAsset::SndDriverGlobals(
                self.asset_data
                    .cast::<sound::SndDriverGlobalsRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::FX => XAsset::Fx(
                self.asset_data
                    .cast::<fx::FxEffectDefRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::IMPACT_FX => XAsset::ImpactFx(
                self.asset_data
                    .cast::<fx::FxImpactTableRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::RAWFILE => {
                XAsset::RawFile(self.asset_data.cast::<RawFileRaw>().xfile_into(xfile, ())?)
            }
            XAssetType::STRINGTABLE => XAsset::StringTable(
                self.asset_data
                    .cast::<StringTableRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::PACKINDEX => XAsset::PackIndex(
                self.asset_data
                    .cast::<PackIndexRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::XGLOBALS => XAsset::XGlobals(
                self.asset_data
                    .cast::<XGlobalsRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::DDL => XAsset::Ddl(
                self.asset_data
                    .cast::<ddl::DdlRootRaw>()
                    .xfile_into(xfile, ())?,
            ),
            XAssetType::GLASSES => {
                XAsset::Glasses(self.asset_data.cast::<GlassesRaw>().xfile_into(xfile, ())?)
            }
            XAssetType::EMBLEMSET => XAsset::EmblemSet(
                self.asset_data
                    .cast::<EmblemSetRaw>()
                    .xfile_into(xfile, ())?,
            ),
            _ => {
                dbg!(asset_type);
                unimplemented!()
            }
        })
    }
}

pub(crate) fn convert_offset_to_ptr(offset: u32) -> Result<(u8, u32)> {
    let block = ((offset - 1) >> 29) as u8;
    let off = (offset - 1) & 0x1FFFFFFF;

    let block_sizes = XFILE
        .lock()
        .map_err(|e| Error::Poison(Box::new(e)))?
        .unwrap()
        .block_size;
    let start = block_sizes[0..block as usize].iter().sum::<u32>();
    let p = start + off;

    //dbg!(block_sizes, block, off, start, p);

    Ok((block, p))
}
