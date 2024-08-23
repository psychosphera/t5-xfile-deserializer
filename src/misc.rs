use common::{Mat3, Vec2, Vec3, Vec4};
use serde::Deserialize;

use crate::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct RawFileRaw<'a> {
    pub name: XString<'a>,
    pub len: i32,
    pub buffer: Ptr32<'a, u8>,
}
assert_size!(RawFileRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RawFile {
    pub name: String,
    pub buffer: Vec<u8>,
}

impl<'a> XFileInto<RawFile, ()> for RawFileRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<RawFile> {
        //dbg!(&self);
        let name = self.name.xfile_into(de, ())?;
        let buffer = self.buffer.to_array(self.len as usize + 1).to_vec(de)?;
        Ok(RawFile { name, buffer })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct StringTableRaw<'a> {
    pub name: XString<'a>,
    pub column_count: i32,
    pub row_count: i32,
    pub values: Ptr32<'a, StringTableCellRaw<'a>>,
    pub cell_index: Ptr32<'a, i16>,
}
assert_size!(StringTableRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct StringTable {
    pub name: String,
    pub column_count: usize,
    pub row_count: usize,
    pub values: Vec<StringTableCell>,
    pub cell_index: Vec<i16>,
}

impl<'a> XFileInto<StringTable, ()> for StringTableRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<StringTable> {
        let size = self.column_count as usize * self.row_count as usize;

        Ok(StringTable {
            name: self.name.xfile_into(de, ())?,
            column_count: self.column_count as _,
            row_count: self.row_count as _,
            values: self.values.to_array(size).xfile_into(de, ())?,
            cell_index: self.cell_index.to_array(size).to_vec(de)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct StringTableCellRaw<'a> {
    pub name: XString<'a>,
    pub hash: i32,
}
assert_size!(StringTableCellRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct StringTableCell {
    pub name: String,
    pub hash: i32,
}

impl<'a> XFileInto<StringTableCell, ()> for StringTableCellRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<StringTableCell> {
        Ok(StringTableCell {
            name: self.name.xfile_into(de, ())?,
            hash: self.hash,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexRaw<'a> {
    pub name: XString<'a>,
    pub header: PackIndexHeaderRaw,
    pub entries: Ptr32<'a, PackIndexEntryRaw>,
}
assert_size!(PackIndexRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PackIndex {
    pub name: String,
    pub header: PackIndexHeader,
    pub entries: Vec<PackIndexEntry>,
}

impl<'a> XFileInto<PackIndex, ()> for PackIndexRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<PackIndex> {
        Ok(PackIndex {
            name: self.name.xfile_into(de, ())?,
            header: self.header.into(),
            entries: self
                .entries
                .to_array(self.header.count as _)
                .to_vec_into(de)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexHeaderRaw {
    pub magic: u32,
    pub timestamp: u32,
    pub count: u32,
    pub alignment: u32,
    pub data_start: u32,
}
assert_size!(PackIndexHeaderRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PackIndexHeader {
    pub magic: u32,
    pub timestamp: u32,
    pub count: usize,
    pub alignment: usize,
    pub data_start: usize,
}

impl Into<PackIndexHeader> for PackIndexHeaderRaw {
    fn into(self) -> PackIndexHeader {
        PackIndexHeader {
            magic: self.magic,
            timestamp: self.timestamp,
            count: self.count as _,
            alignment: self.alignment as _,
            data_start: self.data_start as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexEntryRaw {
    pub hash: u32,
    pub offset: u32,
    pub size: u32,
}
assert_size!(PackIndexEntryRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PackIndexEntry {
    pub hash: u32,
    pub offset: usize,
    pub size: usize,
}

impl From<PackIndexEntryRaw> for PackIndexEntry {
    fn from(value: PackIndexEntryRaw) -> Self {
        Self {
            hash: value.hash,
            offset: value.offset as _,
            size: value.size as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct MapEntsRaw<'a> {
    pub name: XString<'a>,
    pub entity_string: FatPointerCountLastU32<'a, u8>,
}
assert_size!(MapEntsRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MapEnts {
    pub name: String,
    pub entity_string: String,
}

impl<'a> XFileInto<MapEnts, ()> for MapEntsRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<MapEnts> {
        let name = self.name.xfile_into(de, ())?;

        let mut chars = self.entity_string.to_vec(de)?;
        if chars.is_empty() {
            return Ok(MapEnts {
                name,
                entity_string: String::new(),
            });
        }

        if chars.bytes().last().unwrap().unwrap() != b'\0' {
            chars.push(b'\0');
        }

        let entity_string = CString::from_vec_with_nul(chars)
            .unwrap()
            .to_string_lossy()
            .to_string();

        Ok(MapEnts {
            name,
            entity_string,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct LocalizeEntryRaw<'a> {
    pub value: XString<'a>,
    pub name: XString<'a>,
}
assert_size!(LocalizeEntryRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct LocalizeEntry {
    pub value: String,
    pub name: String,
}

impl<'a> XFileInto<LocalizeEntry, ()> for LocalizeEntryRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<LocalizeEntry> {
        let value = self.value.xfile_into(de, ())?;
        let name = self.name.xfile_into(de, ())?;
        //dbg!(&value, &name);
        Ok(LocalizeEntry { value, name })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct XGlobalsRaw<'a> {
    pub name: XString<'a>,
    pub xanim_stream_buffer_size: i32,
    pub cinematic_max_width: i32,
    pub cinematic_max_height: i32,
    pub extracam_resolution: i32,
    pub gump_reserve: i32,
    pub screen_clear_color: [f32; 4],
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XGlobals {
    pub name: String,
    pub xanim_stream_buffer_size: i32,
    pub cinematic_max_width: i32,
    pub cinematic_max_height: i32,
    pub extracam_resolution: i32,
    pub gump_reserve: i32,
    pub screen_clear_color: Vec4,
}

impl<'a> XFileInto<XGlobals, ()> for XGlobalsRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<XGlobals> {
        Ok(XGlobals {
            name: self.name.xfile_into(de, ())?,
            xanim_stream_buffer_size: self.xanim_stream_buffer_size,
            cinematic_max_width: self.cinematic_max_width,
            cinematic_max_height: self.cinematic_max_height,
            extracam_resolution: self.extracam_resolution,
            gump_reserve: self.gump_reserve,
            screen_clear_color: self.screen_clear_color.into(),
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GlassesRaw<'a> {
    name: XString<'a>,
    glasses: FatPointerCountFirstU32<'a, GlassRaw<'a>>,
    work_memory: FatPointerCountLastU32<'a, u8>,
    small_allocator_blocks: u32,
    max_groups: u32,
    max_shards: u32,
    max_physics: u32,
    shard_memory_size: u32,
    max_free_cmd: u32,
    num_slots: u32,
    num_verts: u32,
    num_indices: u32,
}
assert_size!(GlassesRaw, 56);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Glasses {
    pub name: String,
    pub glasses: Vec<Glass>,
    pub work_memory: Vec<u8>,
    pub small_allocator_blocks: u32,
    pub max_groups: u32,
    pub max_shards: u32,
    pub max_physics: u32,
    pub shard_memory_size: u32,
    pub max_free_cmd: u32,
    pub num_slots: u32,
    pub num_verts: u32,
    pub num_indices: u32,
}

impl<'a> XFileInto<Glasses, ()> for GlassesRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<Glasses> {
        let name = self.name.xfile_into(de, ())?;
        let glasses = self.glasses.xfile_into(de, ())?;
        let work_memory = self.work_memory.to_vec(de)?;

        Ok(Glasses {
            name,
            glasses,
            work_memory,
            small_allocator_blocks: self.small_allocator_blocks,
            max_groups: self.max_groups,
            max_shards: self.max_shards,
            max_physics: self.max_physics,
            shard_memory_size: self.shard_memory_size,
            max_free_cmd: self.max_free_cmd,
            num_slots: self.num_slots,
            num_verts: self.num_verts,
            num_indices: self.num_indices,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GlassRaw<'a> {
    pub glass_def: Ptr32<'a, GlassDefRaw<'a>>,
    pub index: u32,
    pub brush_model: u32,
    pub origin: [f32; 3],
    pub angles: [f32; 3],
    pub absmin: [f32; 3],
    pub absmax: [f32; 3],
    pub is_planar: bool,
    pub num_outline_verts: u8,
    pub outline: Ptr32<'a, [f32; 2]>,
    pub outline_axis: [[f32; 3]; 3],
    pub outline_origin: [f32; 3],
    pub uv_scale: f32,
    pub thickness: f32,
}
assert_size!(GlassRaw, 124);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Glass {
    pub glass_def: Option<Box<GlassDef>>,
    pub index: u32,
    pub brush_model: u32,
    pub origin: Vec3,
    pub angles: Vec3,
    pub absmin: Vec3,
    pub absmax: Vec3,
    pub is_planar: bool,
    pub outline: Vec<Vec2>,
    pub outline_axis: Mat3,
    pub outline_origin: Vec3,
    pub uv_scale: f32,
    pub thickness: f32,
}

impl<'a> XFileInto<Glass, ()> for GlassRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<Glass> {
        let glass_def = self.glass_def.xfile_into(de, ())?;
        let origin = self.origin.into();
        let angles = self.angles.into();
        let absmin = self.absmin.into();
        let absmax = self.absmax.into();
        let outline = self
            .outline
            .to_array(self.num_outline_verts as _)
            .to_vec_into(de)?;
        let outline_axis = self.outline_axis.into();
        let outline_origin = self.outline_origin.into();

        Ok(Glass {
            glass_def,
            index: self.index,
            brush_model: self.brush_model,
            origin,
            angles,
            absmin,
            absmax,
            is_planar: self.is_planar,
            outline,
            outline_axis,
            outline_origin,
            uv_scale: self.uv_scale,
            thickness: self.thickness,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GlassDefRaw<'a> {
    pub name: XString<'a>,
    pub max_health: i32,
    pub thickness: f32,
    pub min_shard_size: f32,
    pub max_shard_size: f32,
    pub shard_life_probability: f32,
    pub max_shards: i32,
    pub pristine_material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub cracked_material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub shard_material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub crack_sound: XString<'a>,
    pub shatter_sound: XString<'a>,
    pub auto_shatter_sound: XString<'a>,
    pub crack_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub shatter_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
}
assert_size!(GlassDefRaw, 60);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GlassDef {
    pub name: String,
    pub max_health: i32,
    pub thickness: f32,
    pub min_shard_size: f32,
    pub max_shard_size: f32,
    pub shard_life_probability: f32,
    pub max_shards: i32,
    pub pristine_material: Option<Box<techset::Material>>,
    pub cracked_material: Option<Box<techset::Material>>,
    pub shard_material: Option<Box<techset::Material>>,
    pub crack_sound: String,
    pub shatter_sound: String,
    pub auto_shatter_sound: String,
    pub crack_effect: Option<Box<fx::FxEffectDef>>,
    pub shatter_effect: Option<Box<fx::FxEffectDef>>,
}

impl<'a> XFileInto<GlassDef, ()> for GlassDefRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<GlassDef> {
        let name = self.name.xfile_into(de, ())?;
        let pristine_material = self.pristine_material.xfile_into(de, ())?;
        let cracked_material = self.cracked_material.xfile_into(de, ())?;
        let shard_material = self.shard_material.xfile_into(de, ())?;
        let crack_sound = self.crack_sound.xfile_into(de, ())?;
        let shatter_sound = self.shatter_sound.xfile_into(de, ())?;
        let auto_shatter_sound = self.auto_shatter_sound.xfile_into(de, ())?;
        let crack_effect = self.crack_effect.xfile_into(de, ())?;
        let shatter_effect = self.shatter_effect.xfile_into(de, ())?;

        Ok(GlassDef {
            name,
            max_health: self.max_health,
            thickness: self.thickness,
            min_shard_size: self.min_shard_size,
            max_shard_size: self.max_shard_size,
            shard_life_probability: self.shard_life_probability,
            max_shards: self.max_shards,
            pristine_material,
            cracked_material,
            shard_material,
            crack_sound,
            shatter_sound,
            auto_shatter_sound,
            crack_effect,
            shatter_effect,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemSetRaw<'a> {
    pub color_count: i32,
    pub layers: FatPointerCountFirstU32<'a, EmblemLayer>,
    pub categories: FatPointerCountFirstU32<'a, EmblemCategoryRaw<'a>>,
    pub icons: FatPointerCountFirstU32<'a, EmblemIconRaw<'a>>,
    pub backgrounds: FatPointerCountFirstU32<'a, EmblemBackgroundRaw<'a>>,
    pub background_lookup: FatPointerCountFirstU32<'a, u16>,
}
assert_size!(EmblemSetRaw, 44);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EmblemSet {
    pub color_count: i32,
    pub layers: Vec<EmblemLayer>,
    pub categories: Vec<EmblemCategory>,
    pub icons: Vec<EmblemIcon>,
    pub backgrounds: Vec<EmblemBackground>,
    pub background_lookup: Vec<u16>,
}

impl<'a> XFileInto<EmblemSet, ()> for EmblemSetRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<EmblemSet> {
        let layers = self.layers.to_vec(de)?;
        let categories = self.categories.xfile_into(de, ())?;
        let icons = self.icons.xfile_into(de, ())?;
        let backgrounds = self.backgrounds.xfile_into(de, ())?;
        let background_lookup = self.background_lookup.to_vec(de)?;

        Ok(EmblemSet {
            color_count: self.color_count,
            layers,
            categories,
            icons,
            backgrounds,
            background_lookup,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct EmblemLayer {
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
}
assert_size!(EmblemLayer, 12);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemCategoryRaw<'a> {
    pub name: XString<'a>,
    pub description: XString<'a>,
}
assert_size!(EmblemCategoryRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EmblemCategory {
    pub name: String,
    pub description: String,
}

impl<'a> XFileInto<EmblemCategory, ()> for EmblemCategoryRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<EmblemCategory> {
        let name = self.name.xfile_into(de, ())?;
        let description = self.description.xfile_into(de, ())?;

        Ok(EmblemCategory { name, description })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemIconRaw<'a> {
    pub image: Ptr32<'a, techset::GfxImageRaw<'a>>,
    pub description: XString<'a>,
    pub outline_size: f32,
    pub default_color: i32,
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
    pub unclassify_at: i32,
    pub sort_key: i32,
    pub category: u32,
}
assert_size!(EmblemIconRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EmblemIcon {
    pub image: Option<Box<techset::GfxImage>>,
    pub description: String,
    pub outline_size: f32,
    pub default_color: i32,
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
    pub unclassify_at: i32,
    pub sort_key: i32,
    pub category: u32,
}

impl<'a> XFileInto<EmblemIcon, ()> for EmblemIconRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<EmblemIcon> {
        let image = self.image.xfile_into(de, ())?;
        let description = self.description.xfile_into(de, ())?;

        Ok(EmblemIcon {
            image,
            description,
            outline_size: self.outline_size,
            default_color: self.default_color,
            cost: self.cost,
            unlock_level: self.unlock_level,
            unlock_plevel: self.unlock_plevel,
            unclassify_at: self.unclassify_at,
            sort_key: self.sort_key,
            category: self.category,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemBackgroundRaw<'a> {
    pub material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub description: XString<'a>,
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
    pub unclassify_at: i32,
}
assert_size!(EmblemBackgroundRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EmblemBackground {
    pub material: Option<Box<techset::Material>>,
    pub description: String,
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
    pub unclassify_at: i32,
}

impl<'a> XFileInto<EmblemBackground, ()> for EmblemBackgroundRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<EmblemBackground> {
        let material = self.material.xfile_into(de, ())?;
        let description = self.description.xfile_into(de, ())?;

        Ok(EmblemBackground {
            material,
            description,
            cost: self.cost,
            unlock_level: self.unlock_level,
            unlock_plevel: self.unlock_plevel,
            unclassify_at: self.unclassify_at,
        })
    }
}
