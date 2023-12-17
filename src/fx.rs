use crate::{*, common::{Vec4, Vec3, Vec2}};

use bitflags::bitflags;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxEffectDefRaw<'a> {
    pub name: XString<'a>,
    pub flags: u8,
    pub ef_priority: u8,
    reserved: [u8; 2],
    pub total_size: i32,
    pub msec_looping_life: i32,
    pub elem_def_count_looping: i32,
    pub elem_def_count_one_shot: i32,
    pub elem_def_count_emission: i32,
    pub elem_defs: Ptr32<'a, FxElemDefRaw<'a>>,
    pub bounding_box_dim: [f32; 3],
    pub bounding_sphere: [f32; 4],
}
assert_size!(FxEffectDefRaw, 60);

bitflags! {
    pub struct FxEffectDefFlags: u8 {
        const NEEDS_LIGHTING = 0x01;
        const IS_SEE_THRU_DECAL = 0x02;
    }
}

pub struct FxEffectDef {
    pub name: String,
    pub flags: FxEffectDefFlags,
    pub ef_priority: u8,
    pub total_size: usize,
    pub msec_looping_life: i32,
    pub elem_def_count_looping: i32,
    pub elem_def_count_one_shot: i32,
    pub elem_def_count_emission: i32,
    pub elem_defs: Vec<FxElemDef>,
    pub bounding_box_dim: Vec3,
    pub bounding_sphere: Vec4,
}

impl<'a> XFileInto<FxEffectDef> for FxEffectDefRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> FxEffectDef {
        FxEffectDef {
            name: self.name.xfile_into(&mut xfile),
            flags: FxEffectDefFlags::from_bits(self.flags).unwrap(),
            ef_priority: self.ef_priority,
            total_size: self.total_size as _,
            msec_looping_life: self.msec_looping_life,
            elem_def_count_emission: self.elem_def_count_emission,
            elem_def_count_looping: self.elem_def_count_looping,
            elem_def_count_one_shot: self.elem_def_count_one_shot,
            elem_defs: self.elem_defs.to_array(self.elem_def_count_looping as usize + self.elem_def_count_one_shot as usize + self.elem_def_count_emission as usize).to_vec(&mut xfile).into_iter().map(|d| d.xfile_into(&mut xfile)),
            bounding_box_dim: self.bounding_box_dim.into(),
            bounding_sphere: self.bounding_sphere.into(),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemDefRaw<'a> {
    pub flags: i32,
    pub spawn: [i32; 2],
    pub spawn_range: FxFloatRange,
    pub fade_in_range: FxFloatRange,
    pub fade_out_range: FxFloatRange,
    pub spawn_frustum_cull_radius: f32,
    pub spawn_delay_msec: FxIntRange,
    pub life_span_msec: FxIntRange,
    pub spawn_origin: [FxFloatRange; 3],
    pub spawn_offset_radius: FxFloatRange,
    pub spawn_offset_height: FxFloatRange,
    pub spawn_angles: [FxFloatRange; 3],
    pub angular_velocity: [FxFloatRange; 3],
    pub initial_rotation: FxFloatRange,
    pub rotation_axis: u32,
    pub gravity: FxFloatRange,
    pub reflection_factor: FxFloatRange,
    pub atlas: FxElemAtlasRaw,
    pub wind_influence: f32,
    pub elem_type: u8,
    pub visual_count: u8,
    pub vel_interval_count: u8,
    pub vis_state_interval_count: u8,
    pub vel_samples: Ptr32<'a, FxElemVelStateSampleRaw>,
    pub vis_samples: Ptr32<'a, FxElemVisStateSampleRaw>,
    pub visuals: Ptr32<'a, ()>,
    pub coll_mins: [f32; 3],
    pub coll_maxs: [f32; 3],
    pub effect_on_impact: Ptr32<'a, ()>,
    pub effect_on_death: Ptr32<'a, ()>,
    pub effect_emitted: Ptr32<'a, ()>,
    pub emit_dist: FxFloatRange,
    pub emit_dist_variance: FxFloatRange,
    pub effect_attached: Ptr32<'a, ()>,
    pub trail_def: Ptr32<'a, FxTrailDefRaw<'a>>,
    pub sort_order: u8,
    pub lighting_frac: u8,
    unused: [u8; 2],
    pub alpha_fade_time_msec: u16,
    pub max_wind_strength: u16,
    pub spawn_interval_at_max_wind: u16,
    pub lifespan_at_max_wind: u16,
    pub u: [u8; 8],
    pub spawn_sound: FxElemSpawnSoundRaw<'a>,
    pub billboard_pivot: [f32; 2],
}
assert_size!(FxElemDefRaw, 292);


pub enum FxElemDefUnion {
    Billboard(FxBillboardTrim),
    CloudDensityRange(FxIntRange),
}

pub enum FxEffectDefRef {
    Name(String),
    Handle(Option<Box<FxEffectDef>>),
}



pub struct FxElemDef {
    pub flags: FxElemFlags,
    pub spawn: [i32; 2],
    pub spawn_range: FxFloatRange,
    pub fade_in_range: FxFloatRange,
    pub fade_out_range: FxFloatRange,
    pub spawn_frustum_cull_radius: f32,
    pub spawn_delay_msec: FxIntRange,
    pub life_span_msec: FxIntRange,
    pub spawn_origin: [FxFloatRange; 3],
    pub spawn_offset_radius: FxFloatRange,
    pub spawn_offset_height: FxFloatRange,
    pub spawn_angles: [FxFloatRange; 3],
    pub angular_velocity: [FxFloatRange; 3],
    pub initial_rotation: FxFloatRange,
    pub rotation_axis: u32,
    pub gravity: FxFloatRange,
    pub reflection_factor: FxFloatRange,
    pub atlas: FxElemAtlasRaw,
    pub wind_influence: f32,
    pub elem_type: u8,
    pub visual_count: u8,
    pub vel_interval_count: u8,
    pub vis_state_interval_count: u8,
    pub vel_samples: Vec<FxElemVelStateSample>,
    pub vis_samples: Vec<FxElemVisStateSample>,
    pub visuals: Ptr32<'a, ()>,
    pub coll_mins: Vec3,
    pub coll_maxs: Vec3,
    pub effect_on_impact: Ptr32<'a, ()>,
    pub effect_on_death: Ptr32<'a, ()>,
    pub effect_emitted: Ptr32<'a, ()>,
    pub emit_dist: FxFloatRange,
    pub emit_dist_variance: FxFloatRange,
    pub effect_attached: Ptr32<'a, ()>,
    pub trail_def: Option<Box<FxTrailDef>>,
    pub sort_order: u8,
    pub lighting_frac: u8,
    pub alpha_fade_time_msec: u16,
    pub max_wind_strength: u16,
    pub spawn_interval_at_max_wind: u16,
    pub lifespan_at_max_wind: u16,
    pub u: FxElemDefUnion,
    pub spawn_sound: FxElemSpawnSound,
    pub billboard_pivot: Vec2,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxFloatRange {
    pub base: f32,
    pub amplitude: f32,
}
assert_size!(FxFloatRange, 8);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxIntRange {
    pub base: i32,
    pub amplitude: i32,
}
assert_size!(FxIntRange, 8);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemAtlasRaw {
    pub behavior: u8,
    pub index: u8,
    pub fps: u8,
    pub loop_count: u8,
    pub col_index_bits: u8,
    pub row_index_bits: u8,
    pub entry_count_and_index_range: u16,
}
assert_size!(FxElemAtlasRaw, 8);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVelStateSampleRaw {
    pub local: FxElemVelStateInFrameRaw,
    pub world: FxElemVelStateInFrameRaw,
}
assert_size!(FxElemVelStateSampleRaw, 96);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVelStateInFrameRaw {
    pub velocity: FxElemVec3Range,
    pub total_delta: FxElemVec3Range,
}
assert_size!(FxElemVelStateInFrameRaw, 48);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVec3Range {
    pub base: [f32; 3],
    pub amplitude: [f32; 3],
}
assert_size!(FxElemVec3Range, 24);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVisStateSampleRaw {
    pub base: FxElemVisualStateRaw,
    pub amplitude: FxElemVisualStateRaw,
}
assert_size!(FxElemVisStateSampleRaw, 48);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVisualStateRaw {
    pub color: [u8; 4],
    pub rotation_delta: f32,
    pub rotation_total: f32,
    pub size: [f32; 2],
    pub scale: f32,
}
assert_size!(FxElemVisualStateRaw, 24);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxTrailDefRaw<'a> {
    pub scroll_time_msec: i32,
    pub repeat_dist: i32,
    pub split_dist: i32,
    pub verts: FatPointerCountFirstU32<'a, FxTrailVertexRaw>,
    pub inds: FatPointerCountFirstU32<'a, u16>,
}
assert_size!(FxTrailDefRaw, 28);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxTrailVertexRaw {
    pub pos: [f32; 2],
    pub normal: [f32; 2],
    pub tex_coord: f32,
}
assert_size!(FxTrailVertexRaw, 20);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemSpawnSoundRaw<'a> {
    pub spawn_sound: XString<'a>,
}
assert_size!(FxElemSpawnSoundRaw, 4);
