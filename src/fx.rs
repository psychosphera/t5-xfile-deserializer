use crate::{
    common::{Vec2, Vec3, Vec4},
    *,
};

use bitflags::bitflags;
use num_derive::FromPrimitive;
use techset::MaterialRaw;
use xmodel::XModelRaw;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxEffectDefRaw<'a> {
    pub name: XString<'a>,
    pub flags: u8,
    pub ef_priority: u8,
    #[allow(dead_code)]
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
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Clone, Debug)]
    pub struct FxEffectDefFlags: u8 {
        const NEEDS_LIGHTING = 0x01;
        const IS_SEE_THRU_DECAL = 0x02;
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
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

impl<'a> XFileInto<FxEffectDef, ()> for FxEffectDefRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<FxEffectDef> {
        dbg!(self);

        let name = self.name.xfile_into(de, ())?;
        dbg!(&name);
        let elem_defs = self
            .elem_defs
            .to_array(
                self.elem_def_count_looping as usize
                    + self.elem_def_count_one_shot as usize
                    + self.elem_def_count_emission as usize,
            )
            .xfile_into(de, ())?;
        dbg!(&elem_defs);

        let flags = FxEffectDefFlags::from_bits(self.flags).ok_or(Error::new(
            file_line_col!(),
            ErrorKind::BadBitflags(self.flags as _),
        ))?;
        Ok(FxEffectDef {
            name,
            flags,
            ef_priority: self.ef_priority,
            total_size: self.total_size as _,
            msec_looping_life: self.msec_looping_life,
            elem_def_count_emission: self.elem_def_count_emission,
            elem_def_count_looping: self.elem_def_count_looping,
            elem_def_count_one_shot: self.elem_def_count_one_shot,
            elem_defs,
            bounding_box_dim: self.bounding_box_dim.into(),
            bounding_sphere: self.bounding_sphere.into(),
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxElemDefRaw<'a> {
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
    pub atlas: FxElemAtlas,
    pub wind_influence: f32,
    pub elem_type: u8,
    pub visual_count: u8,
    pub vel_interval_count: u8,
    pub vis_state_interval_count: u8,
    pub vel_samples: Ptr32<'a, FxElemVelStateSample>,
    pub vis_samples: Ptr32<'a, FxElemVisStateSampleRaw>,
    pub visuals: FxElemDefVisualsRaw<'a>,
    pub coll_mins: [f32; 3],
    pub coll_maxs: [f32; 3],
    pub effect_on_impact: FxEffectDefRefRaw<'a>,
    pub effect_on_death: FxEffectDefRefRaw<'a>,
    pub effect_emitted: FxEffectDefRefRaw<'a>,
    pub emit_dist: FxFloatRange,
    pub emit_dist_variance: FxFloatRange,
    pub effect_attached: FxEffectDefRefRaw<'a>,
    pub trail_def: Ptr32<'a, FxTrailDefRaw<'a>>,
    pub sort_order: u8,
    pub lighting_frac: u8,
    #[allow(dead_code)]
    unused: [u8; 2],
    pub alpha_fade_time_msec: u16,
    pub max_wind_strength: u16,
    pub spawn_interval_at_max_wind: u16,
    pub lifespan_at_max_wind: u16,
    #[allow(dead_code)]
    pub u: [u8; 8],
    pub spawn_sound: FxElemSpawnSoundRaw<'a>,
    pub billboard_pivot: [f32; 2],
}
assert_size!(FxElemDefRaw, 292);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum FxElemType {
    UNKNOWN = 0x00,
    TRAIL = 0x05,
    CLOUD = 0x06,
    MODEL = 0x07,
    OMNI_LIGHT = 0x08,
    SPOT_LIGHT = 0x09,
    SOUND = 0x0A,
    DECAL = 0x0B,
    RUNNER = 0x0C,
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Clone, Debug)]
    pub struct FxElemFlags: u32 {
        const SPAWN_RELATIVE_TO_EFFECT = 0x00000002;
        const SPAWN_FRUSTUM_CULL = 0x00000004;
        const RUNNER_USES_RAND_ROT = 0x00000008;
        const SPAWN_OFFSET_SPHERE = 0x00000010;
        const SPAWN_OFFSET_CYLINDER = 0x00000020;
        const RUN_RELATIVE_TO_SPAWN = 0x00000040;
        const RUN_RELATIVE_TO_EFFECT = 0x00000080;
        const USE_COLLISION = 0x00000100;
        const DIE_ON_TOUCH = 0x00000200;
        const DRAW_PAST_FOG = 0x00000400;
        const BLOCK_SIGHT = 0x00001000;
        const USE_ITEM_CLIP = 0x00002000;
        const UNKNOWN_00004000 = 0x00004000;
        const USE_BILLBOARD_PIVOT = 0x00200000;
        const USE_GAUSSIAN_CLOUD = 0x00400000;
        const USE_ROTATIONAXIS = 0x00800000;
        const HAS_VELOCITY_GRAPH_LOCAL = 0x01000000;
        const HAS_VELOCITY_GRAPH_WORLD = 0x02000000;
        const HAS_GRAVITY = 0x04000000;
        const USE_MODEL_PHYSICS = 0x08000000;
        const NONUNIFORM_SCALE = 0x10000000;
        const HAS_REFLECTION = 0x40000000;
        const IS_MATURE_CONTENT = 0x80000000;
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
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
    pub atlas: FxElemAtlas,
    pub wind_influence: f32,
    pub elem_type: FxElemType,
    pub visual_count: u8,
    pub vel_interval_count: u8,
    pub vis_state_interval_count: u8,
    pub vel_samples: Vec<FxElemVelStateSample>,
    pub vis_samples: Vec<FxElemVisStateSample>,
    pub visuals: Option<FxElemDefVisuals>,
    pub coll_mins: Vec3,
    pub coll_maxs: Vec3,
    pub effect_on_impact: FxEffectDefRef,
    pub effect_on_death: FxEffectDefRef,
    pub effect_emitted: FxEffectDefRef,
    pub emit_dist: FxFloatRange,
    pub emit_dist_variance: FxFloatRange,
    pub effect_attached: FxEffectDefRef,
    pub trail_def: Option<Box<FxTrailDef>>,
    pub sort_order: u8,
    pub lighting_frac: u8,
    pub alpha_fade_time_msec: u16,
    pub max_wind_strength: u16,
    pub spawn_interval_at_max_wind: u16,
    pub lifespan_at_max_wind: u16,
    pub u: Option<FxElemDefUnion>,
    pub spawn_sound: FxElemSpawnSound,
    pub billboard_pivot: Vec2,
}

impl<'a> XFileInto<FxElemDef, ()> for FxElemDefRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<FxElemDef> {
        dbg!(self);

        let vel_samples = if self.vel_samples.is_null() {
            vec![]
        } else {
            self.vel_samples
                .to_array(self.vel_interval_count as usize + 1)
                .to_vec(de)?
        };
        let vis_samples = if self.vis_samples.is_null() {
            vec![]
        } else {
            self.vis_samples
                .to_array(self.vis_state_interval_count as usize + 1)
                .to_vec_into(de)?
        };
        let visuals = self
            .visuals
            .xfile_into(de, (self.elem_type, self.visual_count))?;
        let effect_on_impact = self.effect_on_impact.xfile_into(de, ())?;
        let effect_on_death = self.effect_on_death.xfile_into(de, ())?;
        let effect_emitted = self.effect_emitted.xfile_into(de, ())?;
        let effect_attached = self.effect_attached.xfile_into(de, ())?;
        let trail_def = self.trail_def.xfile_into(de, ())?;
        let spawn_sound = self.spawn_sound.xfile_into(de, ())?;

        let flags = FxElemFlags::from_bits(self.flags as _).ok_or(Error::new(
            file_line_col!(),
            ErrorKind::BadBitflags(self.flags as _),
        ))?;
        let elem_type = num::FromPrimitive::from_u8(self.elem_type).ok_or(Error::new(
            file_line_col!(),
            ErrorKind::BadFromPrimitive(self.elem_type as _),
        ))?;

        Ok(FxElemDef {
            flags,
            spawn: self.spawn,
            spawn_range: self.spawn_range,
            fade_in_range: self.fade_in_range,
            fade_out_range: self.fade_out_range,
            spawn_frustum_cull_radius: self.spawn_frustum_cull_radius,
            spawn_delay_msec: self.spawn_delay_msec,
            life_span_msec: self.life_span_msec,
            spawn_origin: self.spawn_origin,
            spawn_offset_radius: self.spawn_offset_radius,
            spawn_offset_height: self.spawn_offset_height,
            spawn_angles: self.spawn_angles,
            angular_velocity: self.angular_velocity,
            initial_rotation: self.initial_rotation,
            rotation_axis: self.rotation_axis,
            gravity: self.gravity,
            reflection_factor: self.reflection_factor,
            atlas: self.atlas,
            wind_influence: self.wind_influence,
            elem_type,
            visual_count: self.visual_count,
            vel_interval_count: self.vel_interval_count,
            vis_state_interval_count: self.vis_state_interval_count,
            vel_samples,
            vis_samples,
            visuals,
            coll_mins: self.coll_mins.into(),
            coll_maxs: self.coll_maxs.into(),
            effect_on_impact,
            effect_on_death,
            effect_emitted,
            emit_dist: self.emit_dist,
            emit_dist_variance: self.emit_dist_variance,
            effect_attached,
            trail_def,
            sort_order: self.sort_order,
            lighting_frac: self.lighting_frac,
            alpha_fade_time_msec: self.alpha_fade_time_msec,
            max_wind_strength: self.max_wind_strength,
            spawn_interval_at_max_wind: self.spawn_interval_at_max_wind,
            lifespan_at_max_wind: self.lifespan_at_max_wind,
            u: None,
            spawn_sound,
            billboard_pivot: self.billboard_pivot.into(),
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum FxElemDefUnion {
    Billboard(FxBillboardTrim),
    CloudDensityRange(FxIntRange),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FxBillboardTrim {
    pub top_width: f32,
    pub bottom_width: f32,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct FxEffectDefRefRaw<'a>(Ptr32<'a, ()>);

impl<'a> XFileInto<FxEffectDefRef, ()> for FxEffectDefRefRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<FxEffectDefRef> {
        dbg!(self);

        let name = XString::from_u32(self.0.as_u32()).xfile_into(de, ())?;
        dbg!(&name);

        Ok(FxEffectDefRef::Name(name))
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum FxEffectDefRef {
    Name(String),
    Handle(Option<Box<FxEffectDef>>),
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxElemDefVisualsRaw<'a>(Ptr32<'a, ()>);
assert_size!(FxElemDefVisualsRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum FxElemDefVisuals {
    MarkArray(Vec<FxElemMarkVisuals>),
    Array(Vec<FxElemVisuals>),
    Instance(Option<FxElemVisuals>),
}

impl<'a> XFileInto<Option<FxElemDefVisuals>, (u8, u8)> for FxElemDefVisualsRaw<'a> {
    fn xfile_into(
        &self,
        de: &mut T5XFileDeserializer,
        (elem_type, visual_count): (u8, u8),
    ) -> Result<Option<FxElemDefVisuals>> {
        dbg!(self);

        if elem_type == FxElemType::DECAL as u8 {
            let mark_array = self
                .0
                .cast::<FxElemMarkVisualsRaw>()
                .to_array(visual_count as _)
                .xfile_into(de, ())?;
            Ok(Some(FxElemDefVisuals::MarkArray(mark_array)))
        } else if visual_count < 2 {
            let instance = self
                .0
                .cast::<FxElemVisualsRaw>()
                .xfile_into(de, elem_type)?;
            Ok(Some(FxElemDefVisuals::Instance(
                instance.map(|p| *p).flatten(),
            )))
        } else if !self.0.is_null() {
            let array = self
                .0
                .cast::<FxElemVisualsRaw>()
                .to_array(elem_type as _)
                .xfile_into(de, elem_type)?
                .into_iter()
                .flatten()
                .collect();
            Ok(Some(FxElemDefVisuals::Array(array)))
        } else {
            Ok(None)
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxElemMarkVisualsRaw<'a> {
    pub materials: [Ptr32<'a, techset::MaterialRaw<'a>>; 2],
}
assert_size!(FxElemMarkVisualsRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FxElemMarkVisuals {
    pub materials: [Option<Box<techset::Material>>; 2],
}

impl<'a> XFileInto<FxElemMarkVisuals, ()> for FxElemMarkVisualsRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<FxElemMarkVisuals> {
        dbg!(self);

        Ok(FxElemMarkVisuals {
            materials: [
                self.materials[0].xfile_into(de, ())?,
                self.materials[1].xfile_into(de, ())?,
            ],
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxElemVisualsRaw<'a>(Ptr32<'a, ()>);
assert_size!(FxElemVisualsRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum FxElemVisuals {
    Material(Option<Box<techset::Material>>),
    Model(Option<Box<xmodel::XModel>>),
    EffectDef(FxEffectDefRef),
    SoundName(String),
}

impl<'a> XFileInto<Option<FxElemVisuals>, u8> for FxElemVisualsRaw<'a> {
    fn xfile_into(
        &self,
        de: &mut T5XFileDeserializer,
        elem_type: u8,
    ) -> Result<Option<FxElemVisuals>> {
        dbg!(self);

        if elem_type == FxElemType::MODEL as _ {
            let model = self.0.cast::<XModelRaw>().xfile_into(de, ())?;
            Ok(Some(FxElemVisuals::Model(model)))
        } else if elem_type == FxElemType::RUNNER as _ {
            let effect_def = self.0.cast::<FxEffectDefRefRaw>().xfile_into(de, ())?;
            Ok(Some(FxElemVisuals::EffectDef(*effect_def.unwrap())))
        } else if elem_type == FxElemType::SOUND as _ {
            let sound = XString::from_u32(self.0.as_u32()).xfile_into(de, ())?;
            dbg!(&sound);
            Ok(Some(FxElemVisuals::SoundName(sound)))
        } else if elem_type != FxElemType::OMNI_LIGHT as _
            && elem_type != FxElemType::SPOT_LIGHT as _
        {
            let material = self.0.cast::<MaterialRaw>().xfile_into(de, ())?;
            Ok(Some(FxElemVisuals::Material(material)))
        } else {
            Ok(None)
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxFloatRange {
    pub base: f32,
    pub amplitude: f32,
}
assert_size!(FxFloatRange, 8);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxIntRange {
    pub base: i32,
    pub amplitude: i32,
}
assert_size!(FxIntRange, 8);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemAtlas {
    pub behavior: u8,
    pub index: u8,
    pub fps: u8,
    pub loop_count: u8,
    pub col_index_bits: u8,
    pub row_index_bits: u8,
    pub entry_count_and_index_range: u16,
}
assert_size!(FxElemAtlas, 8);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVelStateSample {
    pub local: FxElemVelStateInFrame,
    pub world: FxElemVelStateInFrame,
}
assert_size!(FxElemVelStateSample, 96);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVelStateInFrame {
    pub velocity: FxElemVec3Range,
    pub total_delta: FxElemVec3Range,
}
assert_size!(FxElemVelStateInFrame, 48);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVec3Range {
    pub base: [f32; 3],
    pub amplitude: [f32; 3],
}
assert_size!(FxElemVec3Range, 24);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxElemVisStateSampleRaw {
    pub base: FxElemVisualStateRaw,
    pub amplitude: FxElemVisualStateRaw,
}
assert_size!(FxElemVisStateSampleRaw, 48);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FxElemVisStateSample {
    pub base: FxElemVisualState,
    pub amplitude: FxElemVisualState,
}

impl From<FxElemVisStateSampleRaw> for FxElemVisStateSample {
    fn from(value: FxElemVisStateSampleRaw) -> Self {
        Self {
            base: value.base.into(),
            amplitude: value.amplitude.into(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxElemVisualStateRaw {
    pub color: [u8; 4],
    pub rotation_delta: f32,
    pub rotation_total: f32,
    pub size: [f32; 2],
    pub scale: f32,
}
assert_size!(FxElemVisualStateRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FxElemVisualState {
    pub color: [u8; 4],
    pub rotation_delta: f32,
    pub rotation_total: f32,
    pub size: Vec2,
    pub scale: f32,
}

impl Into<FxElemVisualState> for FxElemVisualStateRaw {
    fn into(self) -> FxElemVisualState {
        FxElemVisualState {
            color: self.color,
            rotation_delta: self.rotation_delta,
            rotation_total: self.rotation_total,
            size: self.size.into(),
            scale: self.scale,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxTrailDefRaw<'a> {
    pub scroll_time_msec: i32,
    pub repeat_dist: i32,
    pub split_dist: i32,
    pub verts: FatPointerCountFirstU32<'a, FxTrailVertexRaw>,
    pub inds: FatPointerCountFirstU32<'a, u16>,
}
assert_size!(FxTrailDefRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FxTrailDef {
    pub scroll_time_msec: i32,
    pub repeat_dist: i32,
    pub split_dist: i32,
    pub verts: Vec<FxTrailVertex>,
    pub inds: Vec<u16>,
}

impl<'a> XFileInto<FxTrailDef, ()> for FxTrailDefRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<FxTrailDef> {
        dbg!(self);

        Ok(FxTrailDef {
            scroll_time_msec: self.scroll_time_msec,
            repeat_dist: self.repeat_dist,
            split_dist: self.split_dist,
            verts: self.verts.to_vec_into(de)?,
            inds: self.inds.to_vec(de)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxTrailVertexRaw {
    pub pos: [f32; 2],
    pub normal: [f32; 2],
    pub tex_coord: f32,
}
assert_size!(FxTrailVertexRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FxTrailVertex {
    pub pos: Vec2,
    pub normal: Vec2,
    pub tex_coord: f32,
}

impl From<FxTrailVertexRaw> for FxTrailVertex {
    fn from(value: FxTrailVertexRaw) -> Self {
        Self {
            pos: value.pos.into(),
            normal: value.normal.into(),
            tex_coord: value.tex_coord,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxElemSpawnSoundRaw<'a> {
    pub spawn_sound: XString<'a>,
}
assert_size!(FxElemSpawnSoundRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FxElemSpawnSound {
    pub spawn_sound: String,
}

impl<'a> XFileInto<FxElemSpawnSound, ()> for FxElemSpawnSoundRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<FxElemSpawnSound> {
        dbg!(self);

        let spawn_sound = self.spawn_sound.xfile_into(de, ())?;
        dbg!(&spawn_sound);

        Ok(FxElemSpawnSound { spawn_sound })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxImpactTableRaw<'a> {
    pub name: XString<'a>,
    pub table: Ptr32ArrayConst<'a, FxImpactEntryRaw<'a>, 21>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FxImpactTable {
    pub name: String,
    pub table: Vec<FxImpactEntry>,
}

impl<'a> XFileInto<FxImpactTable, ()> for FxImpactTableRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<FxImpactTable> {
        dbg!(self);

        let name = self.name.xfile_into(de, ())?;
        dbg!(&name);

        let table = self.table.xfile_into(de, ())?;

        Ok(FxImpactTable { name, table })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FxImpactEntryRaw<'a> {
    pub nonflesh: [Ptr32<'a, FxEffectDefRaw<'a>>; 31],
    pub flesh: [Ptr32<'a, FxEffectDefRaw<'a>>; 4],
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FxImpactEntry {
    pub nonflesh: [Option<Box<FxEffectDef>>; 31],
    pub flesh: [Option<Box<FxEffectDef>>; 4],
}

impl<'a> XFileInto<FxImpactEntry, ()> for FxImpactEntryRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<FxImpactEntry> {
        dbg!(self);

        let nonflesh = self
            .nonflesh
            .iter()
            .map(|p| p.xfile_into(de, ()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .unwrap();

        let flesh = self
            .flesh
            .iter()
            .map(|p| p.xfile_into(de, ()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .unwrap();

        Ok(FxImpactEntry { nonflesh, flesh })
    }
}
