#![feature(seek_stream_len)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

mod common;
mod destructible;
mod font;
mod fx;
mod gameworld;
mod light;
mod misc;
mod techset;
mod util;
mod xanim;
mod xmodel;

use std::{
    ffi::CString,
    fmt::{Debug, Display},
    io::{Cursor, Read, Seek, Write},
    path::Path,
    sync::{Arc, Mutex},
};

use num_derive::FromPrimitive;
use serde::{de::DeserializeOwned, Deserialize};

use misc::*;
use util::*;

#[derive(Copy, Clone, Default, Debug, Deserialize)]
struct XFileHeader {
    magic: [u8; 8],
    version: u32,
}
assert_size!(XFileHeader, 12);

#[derive(Copy, Clone, Default, Debug, Deserialize)]
struct XFile {
    size: u32,
    external_size: u32,
    block_size: [u32; 7],
}
assert_size!(XFile, 36);

#[derive(Copy, Clone, Default, Debug, Deserialize)]
struct ScriptString(u16);

impl Display for ScriptString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let script_strings = SCRIPT_STRINGS.lock().unwrap();

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

pub struct T5XFileDeserializer {
    xfile: XFile,
    script_strings: Arc<Vec<String>>,
    file: Option<std::fs::File>,
    cache_file: Option<std::fs::File>,
    reader: Option<Cursor<Vec<u8>>>,
}

fn make_de_current(de: &T5XFileDeserializer) -> Result<(), ()> {
    let script_strings = SCRIPT_STRINGS.lock();
    match script_strings {
        Ok(mut s) => {
            *s = Some(de.script_strings.clone());
        }
        Err(_) => return Err(()),
    };

    let xfile = XFILE.lock();
    match xfile {
        Ok(mut f) => {
            *f = Some(de.xfile.clone());
            Ok(())
        }
        Err(_) => Err(()),
    }
}

fn release_de() -> Result<(), ()> {
    let script_strings = SCRIPT_STRINGS.lock();
    match script_strings {
        Ok(mut s) => {
            *s = None;
        }
        Err(_) => return Err(()),
    };

    let xfile = XFILE.lock();
    match xfile {
        Ok(mut f) => {
            *f = None;
            Ok(())
        }
        Err(_) => Err(()),
    }
}

impl T5XFileDeserializer {
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        match std::fs::File::open(path) {
            Ok(f) => Self::from_file(f),
            Err(e) => Err(e),
        }
    }

    pub fn from_file(mut file: std::fs::File) -> std::io::Result<Self> {
        let mut header_bytes = [0u8; 12];
        file.read_exact(&mut header_bytes).unwrap();

        let header = bincode::deserialize::<XFileHeader>(&header_bytes).unwrap();
        assert!(
            xfile_header_magic_is_valid(&header),
            "Fastfile header magic invalid: valid values are IWffu100 and IWff0100"
        );
        assert!(
            xfile_is_correct_version(&header),
            "Fastfile is wrong version (version: 0x{:08x}, correct version: {}",
            header.version,
            XFILE_VERSION
        );

        Ok(Self {
            xfile: XFile::default(),
            script_strings: Arc::default(),
            file: Some(file),
            cache_file: None,
            reader: None,
        })
    }

    pub fn from_cache_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        match std::fs::File::open(path) {
            Ok(f) => Self::from_cache_file(f),
            Err(e) => Err(e),
        }
    }

    pub fn from_cache_file(file: std::fs::File) -> std::io::Result<Self> {
        Ok(Self {
            xfile: XFile::default(),
            script_strings: Arc::default(),
            file: None,
            cache_file: Some(file),
            reader: None,
        })
    }

    pub fn inflate(&mut self) -> Result<(), String> {
        let reader = if let Some(mut f) = self.cache_file.take() {
            let mut decompressed_payload = Vec::new();
            f.read_to_end(&mut decompressed_payload).unwrap();
            Cursor::new(decompressed_payload)
        } else if let Some(mut f) = self.file.take() {
            let mut compressed_payload = Vec::new();
            f.seek(std::io::SeekFrom::Start(size_of::<XFile>() as _))
                .unwrap();
            f.read_to_end(&mut compressed_payload).unwrap();
            let decompressed_payload = inflate::inflate_bytes(&compressed_payload)?;
            Cursor::new(decompressed_payload)
        } else {
            unreachable!()
        };

        self.reader = Some(reader);
        Ok(())
    }

    pub fn cache(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut f = std::fs::File::create(path)?;
        let pos = self.reader.as_ref().unwrap().position();
        let v = self.reader.take().unwrap().into_inner();
        let r = f.write_all(&v);
        self.reader = Some(Cursor::new(v));
        self.reader.as_mut().unwrap().set_position(pos);
        r
    }

    pub fn deserialize(&mut self) -> Vec<XAsset> {
        let xasset_list = {
            let mut file = self.reader.as_mut().unwrap();
            let xfile = bincode::deserialize_from::<_, XFile>(&mut file).unwrap();
            dbg!(xfile);
            dbg!(file.stream_len()).unwrap();
            self.xfile = xfile;

            dbg!(file.stream_position().unwrap());

            let mut xasset_list_buf = [0u8; size_of::<XAssetList>()];
            file.read_exact(&mut xasset_list_buf).unwrap();
            let xasset_list = bincode::deserialize::<XAssetList>(&xasset_list_buf).unwrap();

            dbg!(file.stream_position().unwrap());
            println!("Fastfile contains {} assets.", xasset_list.assets.size);

            let strings = xasset_list
                .strings
                .to_vec(&mut file)
                .into_iter()
                .map(|s| s.xfile_into(&mut file, ()))
                .collect::<Vec<_>>();
            //dbg!(&strings);
            self.script_strings = Arc::new(strings);
            xasset_list
        };

        make_de_current(self).unwrap();

        let mut file = self.reader.as_mut().unwrap();
        let assets = xasset_list.assets.to_vec(&mut file);
        //dbg!(&assets);
        let mut deserialized_assets = Vec::new();

        for asset in assets {
            //dbg!(asset);
            dbg!(file.stream_position().unwrap());
            let a = asset.xfile_into(&mut file, ());
            // if asset.asset_type != XAssetType::TECHNIQUE_SET as _ {
            //     dbg!(&a);
            // }
            assert!(a.is_some());

            deserialized_assets.push(a);
            println!(
                "Successfully deserialized {} asset{}.",
                deserialized_assets.len(),
                if deserialized_assets.len() > 1 {
                    "s"
                } else {
                    ""
                }
            );
        }

        release_de().unwrap();

        unimplemented!()
    }
}

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
    GameWorldSp(Option<Box<gameworld::GameWorldSp>>),
    GameWorldMp(Option<Box<gameworld::GameWorldMp>>),
    MapEnts(Option<Box<MapEnts>>),
    LightDef(Option<Box<light::GfxLightDef>>),
    Font(Option<Box<font::Font>>),
    LocalizeEntry(Option<Box<LocalizeEntry>>),
    Fx(Option<Box<fx::FxEffectDef>>),
    ImpactFx(Option<Box<fx::FxImpactTable>>),
    RawFile(Option<Box<RawFile>>),
    StringTable(Option<Box<StringTable>>),
    PackIndex(Option<Box<PackIndex>>),
    XGlobals(Option<Box<XGlobals>>),
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
            Self::GameWorldSp(p) => p.is_some(),
            Self::GameWorldMp(p) => p.is_some(),
            Self::MapEnts(p) => p.is_some(),
            Self::LightDef(p) => p.is_some(),
            Self::Font(p) => p.is_some(),
            Self::LocalizeEntry(p) => p.is_some(),
            Self::Fx(p) => p.is_some(),
            Self::ImpactFx(p) => p.is_some(),
            Self::RawFile(p) => p.is_some(),
            Self::StringTable(p) => p.is_some(),
            Self::PackIndex(p) => p.is_some(),
            Self::XGlobals(p) => p.is_some(),
        }
    }
}

/// Helper function to deserialze [`T`] from [`xfile`].
fn load_from_xfile<T: DeserializeOwned>(xfile: impl Read + Seek) -> T {
    bincode::deserialize_from::<_, T>(xfile).unwrap()
}

#[derive(Deserialize)]
struct XAssetList<'a> {
    strings: FatPointerCountFirstU32<'a, XString<'a>>,
    assets: FatPointerCountFirstU32<'a, XAssetRaw<'a>>,
}
assert_size!(XAssetList, 16);

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct XAssetRaw<'a> {
    asset_type: u32,
    asset_data: Ptr32<'a, ()>,
}
assert_size!(XAssetRaw, 8);

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
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> XAsset {
        let asset_type = num::FromPrimitive::from_u32(self.asset_type).unwrap();
        match asset_type {
            XAssetType::PHYSPRESET => XAsset::PhysPreset(
                self.asset_data
                    .cast::<xmodel::PhysPresetRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::PHYSCONSTRAINTS => XAsset::PhysConstraints(
                self.asset_data
                    .cast::<xmodel::PhysConstraintsRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::DESTRUCTIBLEDEF => XAsset::DestructibleDef(
                self.asset_data
                    .cast::<destructible::DestructibleDefRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::XANIMPARTS => XAsset::XAnimParts(
                self.asset_data
                    .cast::<xanim::XAnimPartsRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::XMODEL => XAsset::XModel(
                self.asset_data
                    .cast::<xmodel::XModelRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::MATERIAL => XAsset::Material(
                self.asset_data
                    .cast::<techset::MaterialRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::TECHNIQUE_SET => XAsset::TechniqueSet(
                self.asset_data
                    .cast::<techset::MaterialTechniqueSetRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::IMAGE => XAsset::Image(
                self.asset_data
                    .cast::<techset::GfxImageRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::GAMEWORLD_SP => XAsset::GameWorldSp(
                self.asset_data
                    .cast::<gameworld::GameWorldSpRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::GAMEWORLD_MP => XAsset::GameWorldMp(
                self.asset_data
                    .cast::<gameworld::GameWorldMpRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::MAP_ENTS => {
                XAsset::MapEnts(self.asset_data.cast::<MapEntsRaw>().xfile_into(xfile, ()))
            }
            XAssetType::LIGHT_DEF => XAsset::LightDef(
                self.asset_data
                    .cast::<light::GfxLightDefRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::FONT => XAsset::Font(
                self.asset_data
                    .cast::<font::FontRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::LOCALIZE_ENTRY => XAsset::LocalizeEntry(
                self.asset_data
                    .cast::<LocalizeEntryRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::FX => XAsset::Fx(
                self.asset_data
                    .cast::<fx::FxEffectDefRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::IMPACT_FX => XAsset::ImpactFx(
                self.asset_data
                    .cast::<fx::FxImpactTableRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::RAWFILE => {
                XAsset::RawFile(self.asset_data.cast::<RawFileRaw>().xfile_into(xfile, ()))
            }
            XAssetType::STRINGTABLE => XAsset::StringTable(
                self.asset_data
                    .cast::<StringTableRaw>()
                    .xfile_into(xfile, ()),
            ),
            XAssetType::PACKINDEX => {
                XAsset::PackIndex(self.asset_data.cast::<PackIndexRaw>().xfile_into(xfile, ()))
            }
            XAssetType::XGLOBALS => {
                XAsset::XGlobals(self.asset_data.cast::<XGlobalsRaw>().xfile_into(xfile, ()))
            }
            _ => {
                dbg!(asset_type);
                unimplemented!()
            }
        }
    }
}

pub(crate) fn convert_offset_to_ptr(offset: u32) -> (u8, u32) {
    let block = ((offset - 1) >> 29) as u8;
    let off = (offset - 1) & 0x1FFFFFFF;

    let block_sizes = XFILE.lock().unwrap().unwrap().block_size;
    let start = block_sizes[0..block as usize].iter().sum::<u32>();
    let p = start + off;

    //dbg!(block_sizes, block, off, start, p);

    (block, p)
}
