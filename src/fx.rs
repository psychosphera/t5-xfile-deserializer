use crate::{
    common::{Vec2, Vec3, Vec4},
    *,
};

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
    #[derive(Clone, Debug)]
    pub struct FxEffectDefFlags: u8 {
        const NEEDS_LIGHTING = 0x01;
        const IS_SEE_THRU_DECAL = 0x02;
    }
}

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

impl<'a> XFileInto<FxEffectDef> for FxEffectDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> FxEffectDef {
        FxEffectDef {
            name: self.name.xfile_into(&mut xfile),
            flags: FxEffectDefFlags::from_bits(self.flags).unwrap(),
            ef_priority: self.ef_priority,
            total_size: self.total_size as _,
            msec_looping_life: self.msec_looping_life,
            elem_def_count_emission: self.elem_def_count_emission,
            elem_def_count_looping: self.elem_def_count_looping,
            elem_def_count_one_shot: self.elem_def_count_one_shot,
            elem_defs: self
                .elem_defs
                .to_array(
                    self.elem_def_count_looping as usize
                        + self.elem_def_count_one_shot as usize
                        + self.elem_def_count_emission as usize,
                )
                .to_vec(&mut xfile)
                .into_iter()
                .map(|d| d.xfile_into(&mut xfile))
                .collect(),
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
    pub atlas: FxElemAtlas,
    pub wind_influence: f32,
    pub elem_type: u8,
    pub visual_count: u8,
    pub vel_interval_count: u8,
    pub vis_state_interval_count: u8,
    pub vel_samples: Ptr32<'a, FxElemVelStateSample>,
    pub vis_samples: Ptr32<'a, FxElemVisStateSampleRaw>,
    pub visuals: Ptr32<'a, ()>,
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

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum FxElemType {
    TRAIL = 0x05,
    MODEL = 0x07,
    OMNI_LIGHT = 0x08,
    SPOT_LIGHT = 0x09,
    SOUND = 0x0A,
    DECAL = 0x0B,
    RUNNER = 0x0C,
}

#[derive(Clone, Debug)]
pub enum FxElemDefUnion {
    Billboard(FxBillboardTrim),
    CloudDensityRange(FxIntRange),
}

#[derive(Clone, Debug)]
pub struct FxBillboardTrim {
    pub top_width: f32,
    pub bottom_width: f32,
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct FxEffectDefRefRaw<'a>(Ptr32<'a, ()>);

impl<'a> XFileInto<FxEffectDefRef> for FxEffectDefRefRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> FxEffectDefRef {
        FxEffectDefRef::Name(XString::from_u32(self.0.as_u32()).xfile_into(xfile))
    }
}

#[derive(Clone, Debug)]
pub enum FxEffectDefRef {
    Name(String),
    Handle(Option<Box<FxEffectDef>>),
}

#[derive(Clone, Debug)]
pub enum FxElemVisuals {
    Material(Option<Box<techset::Material>>),
    Model(Option<Box<xmodel::XModel>>),
    EffectDef(FxEffectDefRef),
    SoundName(String),
}

#[derive(Clone, Debug)]
pub enum FxElemDefVisuals {
    MarkArray(Vec<FxElemMarkVisuals>),
    Array(Vec<FxElemVisuals>),
    Instance(FxElemVisuals),
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemMarkVisualsRaw<'a> {
    pub materials: [Ptr32<'a, techset::MaterialRaw<'a>>; 2],
}

#[derive(Clone, Debug)]
pub struct FxElemMarkVisuals {
    pub materials: [Option<Box<techset::Material>>; 2],
}

impl<'a> XFileInto<FxElemMarkVisuals> for FxElemMarkVisualsRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> FxElemMarkVisuals {
        FxElemMarkVisuals {
            materials: [
                self.materials[0].xfile_into(&mut xfile),
                self.materials[1].xfile_into(xfile),
            ],
        }
    }
}

bitflags! {
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
    pub visuals: FxElemDefVisuals,
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

impl<'a> XFileInto<FxElemDef> for FxElemDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> FxElemDef {
        let vel_samples = self
            .vel_samples
            .to_array(self.vel_interval_count as _)
            .to_vec(&mut xfile);
        let vis_samples = self
            .vis_samples
            .to_array(self.vis_state_interval_count as _)
            .to_vec(&mut xfile)
            .into_iter()
            .map(Into::into)
            .collect();
        let elem_type = num::FromPrimitive::from_u8(self.elem_type).unwrap();
        let visuals = if elem_type == FxElemType::DECAL {
            FxElemDefVisuals::MarkArray(
                self.visuals
                    .cast::<FxElemMarkVisualsRaw>()
                    .to_array(self.visual_count as _)
                    .to_vec(&mut xfile)
                    .into_iter()
                    .map(|v| v.xfile_into(&mut xfile))
                    .collect(),
            )
        } else if self.visual_count < 2 {
            FxElemDefVisuals::Instance(match elem_type {
                FxElemType::MODEL => FxElemVisuals::Model(
                    self.visuals
                        .cast::<xmodel::XModelRaw>()
                        .xfile_into(&mut xfile),
                ),
                FxElemType::RUNNER => unimplemented!(), // FxElemVisuals::EffectDef(FxEffectDefRef::Handle(self.visuals.cast::<FxEffectDefRaw>().xfile_into(&mut xfile))),
                FxElemType::SOUND => FxElemVisuals::SoundName(
                    XString::from_u32(self.visuals.0).xfile_into(&mut xfile),
                ),
                FxElemType::TRAIL => FxElemVisuals::Material(
                    self.visuals
                        .cast::<techset::MaterialRaw>()
                        .xfile_into(&mut xfile),
                ),
                _ => unreachable!(),
            })
        } else if self.visuals.0 != 0 {
            FxElemDefVisuals::Array(
                self.visuals
                    .cast::<Ptr32<'a, ()>>()
                    .to_array(self.visual_count as _)
                    .to_vec(&mut xfile)
                    .into_iter()
                    .map(|v| match elem_type {
                        FxElemType::MODEL => FxElemVisuals::Model(
                            v.cast::<xmodel::XModelRaw>().xfile_into(&mut xfile),
                        ),
                        FxElemType::RUNNER => unimplemented!(), // FxElemVisuals::EffectDef(FxEffectDefRef::Handle(v.cast::<FxEffectDefRaw>().xfile_into(&mut xfile))),
                        FxElemType::SOUND => FxElemVisuals::SoundName(
                            XString::from_u32(self.visuals.0).xfile_into(&mut xfile),
                        ),
                        FxElemType::TRAIL => FxElemVisuals::Material(
                            v.cast::<techset::MaterialRaw>().xfile_into(&mut xfile),
                        ),
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        } else {
            unreachable!()
        };

        FxElemDef {
            flags: FxElemFlags::from_bits(self.flags as _).unwrap(),
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
            effect_on_impact: self.effect_on_impact.xfile_into(&mut xfile),
            effect_on_death: self.effect_on_death.xfile_into(&mut xfile),
            effect_emitted: self.effect_emitted.xfile_into(&mut xfile),
            emit_dist: self.emit_dist,
            emit_dist_variance: self.emit_dist_variance,
            effect_attached: self.effect_attached.xfile_into(&mut xfile),
            trail_def: self.trail_def.xfile_into(&mut xfile),
            sort_order: self.sort_order,
            lighting_frac: self.lighting_frac,
            alpha_fade_time_msec: self.alpha_fade_time_msec,
            max_wind_strength: self.max_wind_strength,
            spawn_interval_at_max_wind: self.spawn_interval_at_max_wind,
            lifespan_at_max_wind: self.lifespan_at_max_wind,
            u: None,
            spawn_sound: self.spawn_sound.xfile_into(xfile),
            billboard_pivot: self.billboard_pivot.into(),
        }
    }
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

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVelStateSample {
    pub local: FxElemVelStateInFrame,
    pub world: FxElemVelStateInFrame,
}
assert_size!(FxElemVelStateSample, 96);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVelStateInFrame {
    pub velocity: FxElemVec3Range,
    pub total_delta: FxElemVec3Range,
}
assert_size!(FxElemVelStateInFrame, 48);

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

#[derive(Clone, Debug)]
pub struct FxElemVisStateSample {
    pub base: FxElemVisualState,
    pub amplitude: FxElemVisualState,
}

impl Into<FxElemVisStateSample> for FxElemVisStateSampleRaw {
    fn into(self) -> FxElemVisStateSample {
        FxElemVisStateSample {
            base: self.base.into(),
            amplitude: self.amplitude.into(),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemVisualStateRaw {
    pub color: [u8; 4],
    pub rotation_delta: f32,
    pub rotation_total: f32,
    pub size: [f32; 2],
    pub scale: f32,
}
assert_size!(FxElemVisualStateRaw, 24);

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

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxTrailDefRaw<'a> {
    pub scroll_time_msec: i32,
    pub repeat_dist: i32,
    pub split_dist: i32,
    pub verts: FatPointerCountFirstU32<'a, FxTrailVertexRaw>,
    pub inds: FatPointerCountFirstU32<'a, u16>,
}
assert_size!(FxTrailDefRaw, 28);

#[derive(Clone, Debug)]
pub struct FxTrailDef {
    pub scroll_time_msec: i32,
    pub repeat_dist: i32,
    pub split_dist: i32,
    pub verts: Vec<FxTrailVertex>,
    pub inds: Vec<u16>,
}

impl<'a> XFileInto<FxTrailDef> for FxTrailDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> FxTrailDef {
        FxTrailDef {
            scroll_time_msec: self.scroll_time_msec,
            repeat_dist: self.repeat_dist,
            split_dist: self.split_dist,
            verts: self
                .verts
                .to_vec(&mut xfile)
                .into_iter()
                .map(Into::into)
                .collect(),
            inds: self.inds.to_vec(xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxTrailVertexRaw {
    pub pos: [f32; 2],
    pub normal: [f32; 2],
    pub tex_coord: f32,
}
assert_size!(FxTrailVertexRaw, 20);

#[derive(Clone, Debug)]
pub struct FxTrailVertex {
    pub pos: Vec2,
    pub normal: Vec2,
    pub tex_coord: f32,
}

impl Into<FxTrailVertex> for FxTrailVertexRaw {
    fn into(self) -> FxTrailVertex {
        FxTrailVertex {
            pos: self.pos.into(),
            normal: self.normal.into(),
            tex_coord: self.tex_coord,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxElemSpawnSoundRaw<'a> {
    pub spawn_sound: XString<'a>,
}
assert_size!(FxElemSpawnSoundRaw, 4);

#[derive(Clone, Debug)]
pub struct FxElemSpawnSound {
    pub spawn_sound: String,
}

impl<'a> XFileInto<FxElemSpawnSound> for FxElemSpawnSoundRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> FxElemSpawnSound {
        FxElemSpawnSound {
            spawn_sound: self.spawn_sound.xfile_into(xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxImpactTableRaw<'a> {
    pub name: XString<'a>,
    pub table: Ptr32ArrayConst<'a, FxImpactEntryRaw<'a>, 21>,
}

#[derive(Clone, Debug)]
pub struct FxImpactTable {
    pub name: String,
    pub table: Vec<FxImpactEntry>
}

impl<'a> XFileInto<FxImpactTable> for FxImpactTableRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> FxImpactTable {
        FxImpactTable { name: self.name.xfile_into(&mut xfile), table: self.table.to_vec(&mut xfile).into_iter().map(|e| e.xfile_into(&mut xfile)).collect() }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FxImpactEntryRaw<'a> {
    pub nonflesh: [Ptr32<'a, FxEffectDefRaw<'a>>; 31],
    pub flesh: [Ptr32<'a, FxEffectDefRaw<'a>>; 4],
}

#[derive(Clone, Debug)]
pub struct FxImpactEntry {
    pub nonflesh: [Option<Box<FxEffectDef>>; 31],
    pub flesh: [Option<Box<FxEffectDef>>; 4],
}

impl<'a> XFileInto<FxImpactEntry> for FxImpactEntryRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> FxImpactEntry {
        let nonflesh = self.nonflesh
            .iter()
            .map(|p| p.xfile_into(&mut xfile))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let flesh = self.flesh
            .iter()
            .map(|p| p.xfile_into(&mut xfile))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        FxImpactEntry { nonflesh, flesh }
    }
}