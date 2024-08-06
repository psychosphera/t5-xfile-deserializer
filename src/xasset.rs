use crate::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAsset {
    PC(XAssetGeneric<1>),
    Console(XAssetGeneric<4>),
}

impl XAsset {
    pub(crate) fn try_get(
        de: &mut T5XFileDeserializer,
        xasset_raw: XAssetRaw,
        platform: XFilePlatform,
    ) -> Result<Self> {
        let asset = if platform.is_pc() {
            Self::PC(xasset_raw.xfile_into(de, ())?)
        } else {
            Self::Console(xasset_raw.xfile_into(de, ())?)
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
        match self {
            Self::PC(_) => true,
            _ => false,
        }
    }

    pub fn is_console(&self) -> bool {
        !self.is_pc()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAssetGeneric<const MAX_LOCAL_CLIENTS: usize = 1> {
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
    ClipMap(Option<Box<clipmap::ClipMap>>),
    ClipMapPVS(Option<Box<clipmap::ClipMap>>),
    ComWorld(Option<Box<com_world::ComWorld>>),
    GameWorldSp(Option<Box<gameworld::GameWorldSp>>),
    GameWorldMp(Option<Box<gameworld::GameWorldMp>>),
    MapEnts(Option<Box<MapEnts>>),
    GfxWorld(Option<Box<gfx_world::GfxWorld<MAX_LOCAL_CLIENTS>>>),
    LightDef(Option<Box<light::GfxLightDef>>),
    Font(Option<Box<font::Font>>),
    MenuList(Option<Box<menu::MenuList<MAX_LOCAL_CLIENTS>>>),
    Menu(Option<Box<menu::MenuDef<MAX_LOCAL_CLIENTS>>>),
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
            Self::ClipMap(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::ClipMapPVS(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::ComWorld(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::GameWorldSp(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::GameWorldMp(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::MapEnts(p) => p.as_ref().map(|p| p.name.as_str()),
            Self::GfxWorld(p) => p.as_ref().map(|p| p.name.as_str()),
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XAssetList<'a> {
    pub strings: FatPointerCountFirstU32<'a, XString<'a>>,
    pub assets: FatPointerCountFirstU32<'a, XAssetRaw<'a>>,
}
assert_size!(XAssetList, 16);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XAssetRaw<'a> {
    pub asset_type: u32,
    pub asset_data: Ptr32<'a, ()>,
}
assert_size!(XAssetRaw, 8);

/// T5 doesn't actually use all of these.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug, FromPrimitive)]
#[repr(u32)]
pub(crate) enum XAssetType {
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

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileInto<XAssetGeneric<MAX_LOCAL_CLIENTS>, ()>
    for XAssetRaw<'a>
{
    fn xfile_into(
        &self,
        de: &mut T5XFileDeserializer,
        _data: (),
    ) -> Result<XAssetGeneric<MAX_LOCAL_CLIENTS>> {
        let asset_type = num::FromPrimitive::from_u32(self.asset_type)
            .ok_or(Error::BadFromPrimitive(self.asset_type as _))?;
        Ok(match asset_type {
            XAssetType::PHYSPRESET => XAssetGeneric::PhysPreset(
                self.asset_data
                    .cast::<xmodel::PhysPresetRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::PHYSCONSTRAINTS => XAssetGeneric::PhysConstraints(
                self.asset_data
                    .cast::<xmodel::PhysConstraintsRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::DESTRUCTIBLEDEF => XAssetGeneric::DestructibleDef(
                self.asset_data
                    .cast::<destructible::DestructibleDefRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::XANIMPARTS => XAssetGeneric::XAnimParts(
                self.asset_data
                    .cast::<xanim::XAnimPartsRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::XMODEL => XAssetGeneric::XModel(
                self.asset_data
                    .cast::<xmodel::XModelRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::MATERIAL => XAssetGeneric::Material(
                self.asset_data
                    .cast::<techset::MaterialRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::TECHNIQUE_SET => XAssetGeneric::TechniqueSet(
                self.asset_data
                    .cast::<techset::MaterialTechniqueSetRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::IMAGE => XAssetGeneric::Image(
                self.asset_data
                    .cast::<techset::GfxImageRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::SOUND => XAssetGeneric::Sound(
                self.asset_data
                    .cast::<sound::SndBankRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::SOUND_PATCH => XAssetGeneric::SoundPatch(
                self.asset_data
                    .cast::<sound::SndPatchRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::CLIPMAP => XAssetGeneric::ClipMap(
                self.asset_data
                    .cast::<clipmap::ClipMapRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::CLIPMAP_PVS => XAssetGeneric::ClipMapPVS(
                self.asset_data
                    .cast::<clipmap::ClipMapRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::COMWORLD => XAssetGeneric::ComWorld(
                self.asset_data
                    .cast::<com_world::ComWorldRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::GAMEWORLD_SP => XAssetGeneric::GameWorldSp(
                self.asset_data
                    .cast::<gameworld::GameWorldSpRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::GAMEWORLD_MP => XAssetGeneric::GameWorldMp(
                self.asset_data
                    .cast::<gameworld::GameWorldMpRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::MAP_ENTS => {
                XAssetGeneric::MapEnts(self.asset_data.cast::<MapEntsRaw>().xfile_into(de, ())?)
            }
            XAssetType::GFXWORLD => XAssetGeneric::GfxWorld(
                self.asset_data
                    .cast::<gfx_world::GfxWorldRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::LIGHT_DEF => XAssetGeneric::LightDef(
                self.asset_data
                    .cast::<light::GfxLightDefRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::FONT => {
                XAssetGeneric::Font(self.asset_data.cast::<font::FontRaw>().xfile_into(de, ())?)
            }
            XAssetType::MENULIST => XAssetGeneric::MenuList(
                self.asset_data
                    .cast::<menu::MenuListRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::MENU => XAssetGeneric::Menu(
                self.asset_data
                    .cast::<menu::MenuDefRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::LOCALIZE_ENTRY => XAssetGeneric::LocalizeEntry(
                self.asset_data
                    .cast::<LocalizeEntryRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::WEAPON => XAssetGeneric::Weapon(
                self.asset_data
                    .cast::<weapon::WeaponVariantDefRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::SNDDRIVER_GLOBALS => XAssetGeneric::SndDriverGlobals(
                self.asset_data
                    .cast::<sound::SndDriverGlobalsRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::FX => XAssetGeneric::Fx(
                self.asset_data
                    .cast::<fx::FxEffectDefRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::IMPACT_FX => XAssetGeneric::ImpactFx(
                self.asset_data
                    .cast::<fx::FxImpactTableRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::RAWFILE => {
                XAssetGeneric::RawFile(self.asset_data.cast::<RawFileRaw>().xfile_into(de, ())?)
            }
            XAssetType::STRINGTABLE => XAssetGeneric::StringTable(
                self.asset_data
                    .cast::<StringTableRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::PACKINDEX => {
                XAssetGeneric::PackIndex(self.asset_data.cast::<PackIndexRaw>().xfile_into(de, ())?)
            }
            XAssetType::XGLOBALS => {
                XAssetGeneric::XGlobals(self.asset_data.cast::<XGlobalsRaw>().xfile_into(de, ())?)
            }
            XAssetType::DDL => XAssetGeneric::Ddl(
                self.asset_data
                    .cast::<ddl::DdlRootRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::GLASSES => {
                XAssetGeneric::Glasses(self.asset_data.cast::<GlassesRaw>().xfile_into(de, ())?)
            }
            XAssetType::EMBLEMSET => {
                XAssetGeneric::EmblemSet(self.asset_data.cast::<EmblemSetRaw>().xfile_into(de, ())?)
            }
            _ => {
                dbg!(asset_type);
                unimplemented!()
            }
        })
    }
}
