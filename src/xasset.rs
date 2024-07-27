use crate::*;

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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Deserialize)]
pub(crate) struct XAssetList<'a> {
    pub strings: FatPointerCountFirstU32<'a, XString<'a>>,
    pub assets: FatPointerCountFirstU32<'a, XAssetRaw<'a>>,
}
assert_size!(XAssetList, 16);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
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

impl<'a> XFileInto<XAsset, ()> for XAssetRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<XAsset> {
        let asset_type = num::FromPrimitive::from_u32(self.asset_type)
            .ok_or(Error::BadFromPrimitive(self.asset_type as _))?;
        Ok(match asset_type {
            XAssetType::PHYSPRESET => XAsset::PhysPreset(
                self.asset_data
                    .cast::<xmodel::PhysPresetRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::PHYSCONSTRAINTS => XAsset::PhysConstraints(
                self.asset_data
                    .cast::<xmodel::PhysConstraintsRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::DESTRUCTIBLEDEF => XAsset::DestructibleDef(
                self.asset_data
                    .cast::<destructible::DestructibleDefRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::XANIMPARTS => XAsset::XAnimParts(
                self.asset_data
                    .cast::<xanim::XAnimPartsRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::XMODEL => XAsset::XModel(
                self.asset_data
                    .cast::<xmodel::XModelRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::MATERIAL => XAsset::Material(
                self.asset_data
                    .cast::<techset::MaterialRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::TECHNIQUE_SET => XAsset::TechniqueSet(
                self.asset_data
                    .cast::<techset::MaterialTechniqueSetRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::IMAGE => XAsset::Image(
                self.asset_data
                    .cast::<techset::GfxImageRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::SOUND => XAsset::Sound(
                self.asset_data
                    .cast::<sound::SndBankRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::SOUND_PATCH => XAsset::SoundPatch(
                self.asset_data
                    .cast::<sound::SndPatchRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::COMWORLD => XAsset::ComWorld(
                self.asset_data
                    .cast::<com_world::ComWorldRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::GAMEWORLD_SP => XAsset::GameWorldSp(
                self.asset_data
                    .cast::<gameworld::GameWorldSpRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::GAMEWORLD_MP => XAsset::GameWorldMp(
                self.asset_data
                    .cast::<gameworld::GameWorldMpRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::MAP_ENTS => {
                XAsset::MapEnts(self.asset_data.cast::<MapEntsRaw>().xfile_into(de, ())?)
            }
            XAssetType::LIGHT_DEF => XAsset::LightDef(
                self.asset_data
                    .cast::<light::GfxLightDefRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::FONT => {
                XAsset::Font(self.asset_data.cast::<font::FontRaw>().xfile_into(de, ())?)
            }
            XAssetType::MENULIST => XAsset::MenuList(
                self.asset_data
                    .cast::<menu::MenuListRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::MENU => XAsset::Menu(
                self.asset_data
                    .cast::<menu::MenuDefRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::LOCALIZE_ENTRY => XAsset::LocalizeEntry(
                self.asset_data
                    .cast::<LocalizeEntryRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::WEAPON => XAsset::Weapon(
                self.asset_data
                    .cast::<weapon::WeaponVariantDefRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::SNDDRIVER_GLOBALS => XAsset::SndDriverGlobals(
                self.asset_data
                    .cast::<sound::SndDriverGlobalsRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::FX => XAsset::Fx(
                self.asset_data
                    .cast::<fx::FxEffectDefRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::IMPACT_FX => XAsset::ImpactFx(
                self.asset_data
                    .cast::<fx::FxImpactTableRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::RAWFILE => {
                XAsset::RawFile(self.asset_data.cast::<RawFileRaw>().xfile_into(de, ())?)
            }
            XAssetType::STRINGTABLE => XAsset::StringTable(
                self.asset_data
                    .cast::<StringTableRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::PACKINDEX => {
                XAsset::PackIndex(self.asset_data.cast::<PackIndexRaw>().xfile_into(de, ())?)
            }
            XAssetType::XGLOBALS => {
                XAsset::XGlobals(self.asset_data.cast::<XGlobalsRaw>().xfile_into(de, ())?)
            }
            XAssetType::DDL => XAsset::Ddl(
                self.asset_data
                    .cast::<ddl::DdlRootRaw>()
                    .xfile_into(de, ())?,
            ),
            XAssetType::GLASSES => {
                XAsset::Glasses(self.asset_data.cast::<GlassesRaw>().xfile_into(de, ())?)
            }
            XAssetType::EMBLEMSET => {
                XAsset::EmblemSet(self.asset_data.cast::<EmblemSetRaw>().xfile_into(de, ())?)
            }
            _ => {
                dbg!(asset_type);
                unimplemented!()
            }
        })
    }
}
