use crate::{
    FatPointer, FatPointerCountFirstU32, FatPointerCountLastU32, Ptr32, Result, T5XFileDeserialize,
    T5XFileSerialize, XFileDeserializeInto, XFileSerialize, XString, XStringRaw, assert_size,
    common::{Mat3, Vec2, Vec3},
    fx::{FxEffectDef, FxEffectDefRaw},
    techset::{Material, MaterialRaw},
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GlassesRaw<'a> {
    name: XStringRaw<'a>,
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
    pub name: XString,
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

impl<'a> XFileDeserializeInto<Glasses, ()> for GlassesRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<Glasses> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let glasses = self.glasses.xfile_deserialize_into(de, ())?;
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

impl XFileSerialize<()> for Glasses {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let glasses = FatPointerCountFirstU32::from_slice(&self.glasses);
        let work_memory = FatPointerCountLastU32::from_slice(&self.work_memory);

        let glasses = GlassesRaw {
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
        };

        ser.store_into_xfile(glasses)?;
        self.name.xfile_serialize(ser, ())?;
        self.glasses.xfile_serialize(ser, ())?;
        self.work_memory.xfile_serialize(ser, ())
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

impl<'a> XFileDeserializeInto<Glass, ()> for GlassRaw<'a> {
    fn xfile_deserialize_into(&self, de: &mut impl T5XFileDeserialize, _data: ()) -> Result<Glass> {
        let glass_def = self.glass_def.xfile_deserialize_into(de, ())?;
        let outline = self
            .outline
            .to_array(self.num_outline_verts as _)
            .to_vec_into(de)?;

        let origin = self.origin.into();
        let angles = self.angles.into();
        let absmin = self.absmin.into();
        let absmax = self.absmax.into();
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

impl XFileSerialize<()> for Glass {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let glass_def = Ptr32::from_box(&self.glass_def);
        let outline = Ptr32::from_slice(&self.outline);

        let origin = self.origin.get();
        let angles = self.angles.get();
        let absmin = self.absmin.get();
        let absmax = self.absmax.get();
        let outline_axis = self.outline_axis.get();
        let outline_origin = self.outline_origin.get();

        let glass = GlassRaw {
            glass_def,
            index: self.index,
            brush_model: self.brush_model,
            origin,
            angles,
            absmin,
            absmax,
            is_planar: self.is_planar,
            num_outline_verts: self.outline.len() as _,
            outline,
            outline_axis,
            outline_origin,
            uv_scale: self.uv_scale,
            thickness: self.thickness,
        };

        ser.store_into_xfile(glass)?;
        self.glass_def.xfile_serialize(ser, ())?;
        self.outline.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GlassDefRaw<'a> {
    pub name: XStringRaw<'a>,
    pub max_health: i32,
    pub thickness: f32,
    pub min_shard_size: f32,
    pub max_shard_size: f32,
    pub shard_life_probability: f32,
    pub max_shards: i32,
    pub pristine_material: Ptr32<'a, MaterialRaw<'a>>,
    pub cracked_material: Ptr32<'a, MaterialRaw<'a>>,
    pub shard_material: Ptr32<'a, MaterialRaw<'a>>,
    pub crack_sound: XStringRaw<'a>,
    pub shatter_sound: XStringRaw<'a>,
    pub auto_shatter_sound: XStringRaw<'a>,
    pub crack_effect: Ptr32<'a, FxEffectDefRaw<'a>>,
    pub shatter_effect: Ptr32<'a, FxEffectDefRaw<'a>>,
}
assert_size!(GlassDefRaw, 60);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GlassDef {
    pub name: XString,
    pub max_health: i32,
    pub thickness: f32,
    pub min_shard_size: f32,
    pub max_shard_size: f32,
    pub shard_life_probability: f32,
    pub max_shards: i32,
    pub pristine_material: Option<Box<Material>>,
    pub cracked_material: Option<Box<Material>>,
    pub shard_material: Option<Box<Material>>,
    pub crack_sound: XString,
    pub shatter_sound: XString,
    pub auto_shatter_sound: XString,
    pub crack_effect: Option<Box<FxEffectDef>>,
    pub shatter_effect: Option<Box<FxEffectDef>>,
}

impl<'a> XFileDeserializeInto<GlassDef, ()> for GlassDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GlassDef> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let pristine_material = self.pristine_material.xfile_deserialize_into(de, ())?;
        let cracked_material = self.cracked_material.xfile_deserialize_into(de, ())?;
        let shard_material = self.shard_material.xfile_deserialize_into(de, ())?;
        let crack_sound = self.crack_sound.xfile_deserialize_into(de, ())?;
        let shatter_sound = self.shatter_sound.xfile_deserialize_into(de, ())?;
        let auto_shatter_sound = self.auto_shatter_sound.xfile_deserialize_into(de, ())?;
        let crack_effect = self.crack_effect.xfile_deserialize_into(de, ())?;
        let shatter_effect = self.shatter_effect.xfile_deserialize_into(de, ())?;

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

impl XFileSerialize<()> for GlassDef {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let pristine_material = Ptr32::from_box(&self.pristine_material);
        let cracked_material = Ptr32::from_box(&self.cracked_material);
        let shard_material = Ptr32::from_box(&self.shard_material);
        let crack_sound = XStringRaw::from_str(self.crack_sound.get());
        let shatter_sound = XStringRaw::from_str(self.shatter_sound.get());
        let auto_shatter_sound = XStringRaw::from_str(self.auto_shatter_sound.get());
        let crack_effect = Ptr32::from_box(&self.crack_effect);
        let shatter_effect = Ptr32::from_box(&self.shatter_effect);

        let glass_def = GlassDefRaw {
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
            shatter_effect,
            shatter_sound,
            auto_shatter_sound,
            crack_effect,
        };

        ser.store_into_xfile(glass_def)?;
        self.name.xfile_serialize(ser, ())?;
        self.pristine_material.xfile_serialize(ser, ())?;
        self.cracked_material.xfile_serialize(ser, ())?;
        self.shard_material.xfile_serialize(ser, ())?;
        self.crack_sound.xfile_serialize(ser, ())?;
        self.shatter_sound.xfile_serialize(ser, ())?;
        self.auto_shatter_sound.xfile_serialize(ser, ())?;
        self.crack_effect.xfile_serialize(ser, ())?;
        self.shatter_effect.xfile_serialize(ser, ())
    }
}
