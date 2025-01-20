use alloc::{boxed::Box, vec::Vec};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::prelude::*;

use crate::{
    Error, ErrorKind, FatPointerCountFirstU32, LocalizeEntry, LocalizeEntryRaw, MapEnts,
    MapEntsRaw, PackIndex, PackIndexRaw, Ptr32, RawFile, RawFileRaw, Result, StringTable,
    StringTableRaw, T5XFileDeserialize, T5XFileSerialize, XFileDeserializeInto, XFilePlatform,
    XFileSerialize, XGlobals, XGlobalsRaw, XString, XStringRaw, assert_size,
    clipmap::{ClipMap, ClipMapRaw},
    com_world::{ComWorld, ComWorldRaw},
    ddl::{DdlRoot, DdlRootRaw},
    destructible::{DestructibleDef, DestructibleDefRaw},
    emblem::{EmblemSet, EmblemSetRaw},
    file_line_col,
    font::{Font, FontRaw},
    fx::{FxEffectDef, FxEffectDefRaw, FxImpactTable, FxImpactTableRaw},
    gameworld::{GameWorldMp, GameWorldMpRaw, GameWorldSp, GameWorldSpRaw},
    gfx_world::{GfxWorld, GfxWorldRaw},
    glass::{Glasses, GlassesRaw},
    light::{GfxLightDef, GfxLightDefRaw},
    menu::{MenuDef, MenuDefRaw, MenuList, MenuListRaw},
    sound::{SndBank, SndBankRaw, SndDriverGlobals, SndDriverGlobalsRaw, SndPatch, SndPatchRaw},
    techset::{
        GfxImage, GfxImageRaw, Material, MaterialRaw, MaterialTechniqueSet, MaterialTechniqueSetRaw,
    },
    weapon::{WeaponVariantDef, WeaponVariantDefRaw},
    xanim::{XAnimParts, XAnimPartsRaw},
    xmodel::{PhysConstraints, PhysConstraintsRaw, PhysPreset, PhysPresetRaw, XModel, XModelRaw},
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAsset {
    PC(XAssetGeneric<1>),
    Console(XAssetGeneric<4>),
}

impl XFileSerialize<()> for XAsset {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        match self {
            Self::PC(a) => a.xfile_serialize(ser, ()),
            Self::Console(a) => a.xfile_serialize(ser, ()),
        }
    }
}

impl XAsset {
    pub fn try_get(
        de: &mut impl T5XFileDeserialize,
        xasset_raw: XAssetRaw,
        platform: XFilePlatform,
    ) -> Result<Self> {
        let asset = if platform.is_pc() {
            Self::PC(xasset_raw.xfile_deserialize_into(de, ())?)
        } else {
            Self::Console(xasset_raw.xfile_deserialize_into(de, ())?)
        };
        Ok(asset)
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::PC(a) => a.name(),
            Self::Console(a) => a.name(),
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Self::PC(a) => a.is_some(),
            Self::Console(a) => a.is_some(),
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn is_pc(&self) -> bool {
        matches!(self, Self::PC(_))
    }

    pub fn is_console(&self) -> bool {
        !self.is_pc()
    }

    pub fn asset_type(&self) -> XAssetType {
        match self {
            Self::PC(a) => a.asset_type(),
            Self::Console(a) => a.asset_type(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAssetGeneric<const MAX_LOCAL_CLIENTS: usize = 1> {
    PhysPreset(Option<Box<PhysPreset>>),
    PhysConstraints(Option<Box<PhysConstraints>>),
    DestructibleDef(Option<Box<DestructibleDef>>),
    XAnimParts(Option<Box<XAnimParts>>),
    XModel(Option<Box<XModel>>),
    Material(Option<Box<Material>>),
    TechniqueSet(Option<Box<MaterialTechniqueSet>>),
    Image(Option<Box<GfxImage>>),
    Sound(Option<Box<SndBank>>),
    SoundPatch(Option<Box<SndPatch>>),
    ClipMap(Option<Box<ClipMap>>),
    ClipMapPVS(Option<Box<ClipMap>>),
    ComWorld(Option<Box<ComWorld>>),
    GameWorldSp(Option<Box<GameWorldSp>>),
    GameWorldMp(Option<Box<GameWorldMp>>),
    MapEnts(Option<Box<MapEnts>>),
    GfxWorld(Option<Box<GfxWorld<MAX_LOCAL_CLIENTS>>>),
    LightDef(Option<Box<GfxLightDef>>),
    Font(Option<Box<Font>>),
    MenuList(Option<Box<MenuList<MAX_LOCAL_CLIENTS>>>),
    Menu(Option<Box<MenuDef<MAX_LOCAL_CLIENTS>>>),
    LocalizeEntry(Option<Box<LocalizeEntry>>),
    Weapon(Option<Box<WeaponVariantDef>>),
    SndDriverGlobals(Option<Box<SndDriverGlobals>>),
    Fx(Option<Box<FxEffectDef>>),
    ImpactFx(Option<Box<FxImpactTable>>),
    RawFile(Option<Box<RawFile>>),
    StringTable(Option<Box<StringTable>>),
    PackIndex(Option<Box<PackIndex>>),
    XGlobals(Option<Box<XGlobals>>),
    Ddl(Option<Box<DdlRoot>>),
    Glasses(Option<Box<Glasses>>),
    EmblemSet(Option<Box<EmblemSet>>),
}

impl<const MAX_LOCAL_CLIENTS: usize> XAssetGeneric<MAX_LOCAL_CLIENTS> {
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
            Self::ClipMap(p) => p.is_some(),
            Self::ClipMapPVS(p) => p.is_some(),
            Self::ComWorld(p) => p.is_some(),
            Self::GameWorldSp(p) => p.is_some(),
            Self::GameWorldMp(p) => p.is_some(),
            Self::MapEnts(p) => p.is_some(),
            Self::GfxWorld(p) => p.is_some(),
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
            Self::PhysPreset(p) => p.as_ref().map(|p| p.name.get()),
            Self::PhysConstraints(p) => p.as_ref().map(|p| p.name.get()),
            Self::DestructibleDef(p) => p.as_ref().map(|p| p.name.get()),
            Self::XAnimParts(p) => p.as_ref().map(|p| p.name.get()),
            Self::XModel(p) => p.as_ref().map(|p| p.name.get()),
            Self::Material(p) => p.as_ref().map(|p| p.info.name.get()),
            Self::TechniqueSet(p) => p.as_ref().map(|p| p.name.get()),
            Self::Image(p) => p.as_ref().map(|p| p.name.get()),
            Self::Sound(p) => p.as_ref().map(|p| p.name.get()),
            Self::SoundPatch(p) => p.as_ref().map(|p| p.name.get()),
            Self::ClipMap(p) => p.as_ref().map(|p| p.name.get()),
            Self::ClipMapPVS(p) => p.as_ref().map(|p| p.name.get()),
            Self::ComWorld(p) => p.as_ref().map(|p| p.name.get()),
            Self::GameWorldSp(p) => p.as_ref().map(|p| p.name.get()),
            Self::GameWorldMp(p) => p.as_ref().map(|p| p.name.get()),
            Self::MapEnts(p) => p.as_ref().map(|p| p.name.get()),
            Self::GfxWorld(p) => p.as_ref().map(|p| p.name.get()),
            Self::LightDef(p) => p.as_ref().map(|p| p.name.get()),
            Self::Font(p) => p.as_ref().map(|p| p.font_name.get()),
            Self::MenuList(p) => p.as_ref().map(|p| p.name.get()),
            Self::Menu(p) => p.as_ref().map(|p| p.window.name.get()),
            Self::LocalizeEntry(p) => p.as_ref().map(|p| p.name.get()),
            Self::Weapon(p) => p.as_ref().map(|p| p.internal_name.get()),
            Self::SndDriverGlobals(p) => p.as_ref().map(|p| p.name.get()),
            Self::Fx(p) => p.as_ref().map(|p| p.name.get()),
            Self::ImpactFx(p) => p.as_ref().map(|p| p.name.get()),
            Self::RawFile(p) => p.as_ref().map(|p| p.name.get()),
            Self::StringTable(p) => p.as_ref().map(|p| p.name.get()),
            Self::PackIndex(p) => p.as_ref().map(|p| p.name.get()),
            Self::XGlobals(p) => p.as_ref().map(|p| p.name.get()),
            Self::Ddl(p) => p.as_ref().map(|p| p.name.get()),
            Self::Glasses(p) => p.as_ref().map(|p| p.name.get()),
            Self::EmblemSet(_) => Some("emblemset"),
        }
    }

    pub fn asset_type(&self) -> XAssetType {
        match *self {
            Self::PhysPreset(_) => XAssetType::PHYSPRESET,
            Self::PhysConstraints(_) => XAssetType::PHYSCONSTRAINTS,
            Self::DestructibleDef(_) => XAssetType::DESTRUCTIBLEDEF,
            Self::XAnimParts(_) => XAssetType::XANIMPARTS,
            Self::XModel(_) => XAssetType::XMODEL,
            Self::Material(_) => XAssetType::MATERIAL,
            Self::TechniqueSet(_) => XAssetType::TECHNIQUE_SET,
            Self::Image(_) => XAssetType::IMAGE,
            Self::Sound(_) => XAssetType::SOUND,
            Self::SoundPatch(_) => XAssetType::SOUND,
            Self::ClipMap(_) => XAssetType::CLIPMAP,
            Self::ClipMapPVS(_) => XAssetType::CLIPMAP_PVS,
            Self::ComWorld(_) => XAssetType::COMWORLD,
            Self::GameWorldSp(_) => XAssetType::GAMEWORLD_SP,
            Self::GameWorldMp(_) => XAssetType::GAMEWORLD_MP,
            Self::MapEnts(_) => XAssetType::MAP_ENTS,
            Self::GfxWorld(_) => XAssetType::GFXWORLD,
            Self::LightDef(_) => XAssetType::LIGHT_DEF,
            Self::Font(_) => XAssetType::FONT,
            Self::MenuList(_) => XAssetType::MENULIST,
            Self::Menu(_) => XAssetType::MENU,
            Self::LocalizeEntry(_) => XAssetType::LOCALIZE_ENTRY,
            Self::Weapon(_) => XAssetType::WEAPON,
            Self::SndDriverGlobals(_) => XAssetType::SNDDRIVER_GLOBALS,
            Self::Fx(_) => XAssetType::FX,
            Self::ImpactFx(_) => XAssetType::IMPACT_FX,
            Self::RawFile(_) => XAssetType::RAWFILE,
            Self::StringTable(_) => XAssetType::STRINGTABLE,
            Self::PackIndex(_) => XAssetType::PACKINDEX,
            Self::XGlobals(_) => XAssetType::XGLOBALS,
            Self::Ddl(_) => XAssetType::DDL,
            Self::Glasses(_) => XAssetType::GLASSES,
            Self::EmblemSet(_) => XAssetType::EMBLEMSET,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XAssetListRaw<'a> {
    pub strings: FatPointerCountFirstU32<'a, XStringRaw<'a>>,
    pub assets: FatPointerCountFirstU32<'a, XAssetRaw<'a>>,
}
assert_size!(XAssetListRaw, 16);

#[derive(Clone, Debug, Default)]
pub struct XAssetList {
    pub _strings: Vec<XString>,
    pub assets: Vec<XAsset>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XAssetRaw<'a> {
    pub asset_type: u32,
    pub asset_data: Ptr32<'a, ()>,
}
assert_size!(XAssetRaw, 8);

impl<'a> XFileSerialize<()> for XAssetList {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let script_strings = ser.script_strings();

        let asset_list = XAssetListRaw {
            strings: FatPointerCountFirstU32 {
                size: script_strings.len() as _,
                p: Ptr32::unreal(),
            },
            assets: FatPointerCountFirstU32 {
                size: ser.asset_count() as _,
                p: Ptr32::unreal(),
            },
        };

        let mut script_string_bytes = Vec::new();
        for string in script_strings.iter() {
            for c in string.chars() {
                script_string_bytes.push(c as u8);
            }
            script_string_bytes.push(b'\0');
        }

        let asset_bytes = ser.asset_bytes().map(|a| a.to_vec()).unwrap_or_default();

        ser.store_into_xfile(asset_list)?;
        ser.store_into_xfile(script_string_bytes)?;
        ser.store_into_xfile(asset_bytes)
    }
}

/// T5 doesn't actually use all of these.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug, FromPrimitive)]
#[repr(u32)]
pub enum XAssetType {
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

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<XAssetGeneric<MAX_LOCAL_CLIENTS>, ()>
    for XAssetRaw<'a>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<XAssetGeneric<MAX_LOCAL_CLIENTS>> {
        //dbg!(de.stream_pos()?);
        let asset_type =
            num::FromPrimitive::from_u32(self.asset_type).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::InvalidXAssetType(self.asset_type),
            ))?;
        //println!("type={:?} ({})", asset_type, self.asset_type);
        Ok(match asset_type {
            XAssetType::PHYSPRESET => XAssetGeneric::PhysPreset(
                self.asset_data
                    .cast::<PhysPresetRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::PHYSCONSTRAINTS => XAssetGeneric::PhysConstraints(
                self.asset_data
                    .cast::<PhysConstraintsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::DESTRUCTIBLEDEF => XAssetGeneric::DestructibleDef(
                self.asset_data
                    .cast::<DestructibleDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::XANIMPARTS => XAssetGeneric::XAnimParts(
                self.asset_data
                    .cast::<XAnimPartsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::XMODEL => XAssetGeneric::XModel(
                self.asset_data
                    .cast::<XModelRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::MATERIAL => XAssetGeneric::Material(
                self.asset_data
                    .cast::<MaterialRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::TECHNIQUE_SET => XAssetGeneric::TechniqueSet(
                self.asset_data
                    .cast::<MaterialTechniqueSetRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::IMAGE => XAssetGeneric::Image(
                self.asset_data
                    .cast::<GfxImageRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::SOUND => XAssetGeneric::Sound(
                self.asset_data
                    .cast::<SndBankRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::SOUND_PATCH => XAssetGeneric::SoundPatch(
                self.asset_data
                    .cast::<SndPatchRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::CLIPMAP => XAssetGeneric::ClipMap(
                self.asset_data
                    .cast::<ClipMapRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::CLIPMAP_PVS => XAssetGeneric::ClipMapPVS(
                self.asset_data
                    .cast::<ClipMapRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::COMWORLD => XAssetGeneric::ComWorld(
                self.asset_data
                    .cast::<ComWorldRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::GAMEWORLD_SP => XAssetGeneric::GameWorldSp(
                self.asset_data
                    .cast::<GameWorldSpRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::GAMEWORLD_MP => XAssetGeneric::GameWorldMp(
                self.asset_data
                    .cast::<GameWorldMpRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::MAP_ENTS => XAssetGeneric::MapEnts(
                self.asset_data
                    .cast::<MapEntsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::GFXWORLD => XAssetGeneric::GfxWorld(
                self.asset_data
                    .cast::<GfxWorldRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::LIGHT_DEF => XAssetGeneric::LightDef(
                self.asset_data
                    .cast::<GfxLightDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::FONT => XAssetGeneric::Font(
                self.asset_data
                    .cast::<FontRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::MENULIST => XAssetGeneric::MenuList(
                self.asset_data
                    .cast::<MenuListRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::MENU => XAssetGeneric::Menu(
                self.asset_data
                    .cast::<MenuDefRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::LOCALIZE_ENTRY => XAssetGeneric::LocalizeEntry(
                self.asset_data
                    .cast::<LocalizeEntryRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::WEAPON => XAssetGeneric::Weapon(
                self.asset_data
                    .cast::<WeaponVariantDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::SNDDRIVER_GLOBALS => XAssetGeneric::SndDriverGlobals(
                self.asset_data
                    .cast::<SndDriverGlobalsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::FX => XAssetGeneric::Fx(
                self.asset_data
                    .cast::<FxEffectDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::IMPACT_FX => XAssetGeneric::ImpactFx(
                self.asset_data
                    .cast::<FxImpactTableRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::RAWFILE => XAssetGeneric::RawFile(
                self.asset_data
                    .cast::<RawFileRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::STRINGTABLE => XAssetGeneric::StringTable(
                self.asset_data
                    .cast::<StringTableRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::PACKINDEX => XAssetGeneric::PackIndex(
                self.asset_data
                    .cast::<PackIndexRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::XGLOBALS => XAssetGeneric::XGlobals(
                self.asset_data
                    .cast::<XGlobalsRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::DDL => XAssetGeneric::Ddl(
                self.asset_data
                    .cast::<DdlRootRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::GLASSES => XAssetGeneric::Glasses(
                self.asset_data
                    .cast::<GlassesRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            XAssetType::EMBLEMSET => XAssetGeneric::EmblemSet(
                self.asset_data
                    .cast::<EmblemSetRaw>()
                    .xfile_deserialize_into(de, ())?,
            ),
            _ => {
                //dbg!(asset_type);
                return Err(Error::new_with_offset(
                    file_line_col!(),
                    de.stream_pos()? as _,
                    ErrorKind::UnusedXAssetType(asset_type),
                ));
            }
        })
    }
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileSerialize<()> for XAssetGeneric<MAX_LOCAL_CLIENTS> {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let asset_type = self.asset_type() as _;
        let asset_data = Ptr32::unreal();

        let asset = XAssetRaw {
            asset_type,
            asset_data,
        };

        ser.store_into_xfile(asset)?;
        match self {
            // Self::PhysPreset(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::PhysConstraints(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::DestructibleDef(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::XAnimParts(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::XModel(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::Material(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::TechniqueSet(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::Image(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            // Self::Sound(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::SoundPatch(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::ClipMap(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::ClipMapPVS(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::ComWorld(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::GameWorldSp(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::GameWorldMp(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::MapEnts(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            // Self::GfxWorld(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::LightDef(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::Font(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            // Self::MenuList(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::Menu(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::LocalizeEntry(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            // Self::Weapon(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            // Self::SndDriverGlobals(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::Fx(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            // Self::ImpactFx(p) => if let Some(p) = p { p.xfile_serialize(ser, ()) } else { Ok(()) },
            Self::RawFile(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            Self::StringTable(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            Self::PackIndex(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            Self::XGlobals(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            Self::Ddl(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            Self::Glasses(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            Self::EmblemSet(p) => {
                if let Some(p) = p {
                    p.xfile_serialize(ser, ())
                } else {
                    Ok(())
                }
            }
            _ => todo!(),
        }
    }
}
