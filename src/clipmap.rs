use fx::FxEffectDefRaw;
use techset::MaterialRaw;
use xmodel::{
    CBrushSideRaw, CPlaneRaw, PhysConstraintRaw, PhysPresetRaw, XModelPiecesRaw, XModelRaw,
};

use crate::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ClipMapRaw<'a> {
    pub name: XString<'a>,
    pub is_in_use: i32,
    pub planes: FatPointerCountFirstU32<'a, CPlaneRaw>,
    pub static_model_list: FatPointerCountFirstU32<'a, CStaticModelRaw<'a>>,
    pub materials: FatPointerCountFirstU32<'a, DMaterialRaw>,
    pub brushsides: FatPointerCountFirstU32<'a, CBrushSideRaw<'a>>,
    pub nodes: FatPointerCountFirstU32<'a, CNodeRaw<'a>>,
    pub leafs: FatPointerCountFirstU32<'a, CLeafRaw>,
    pub leafbrush_nodes: FatPointerCountFirstU32<'a, CLeafBrushNodeRaw<'a>>,
    pub leafbrushes: FatPointerCountFirstU32<'a, u16>,
    pub leafsurfaces: FatPointerCountFirstU32<'a, u32>,
    pub verts: FatPointerCountFirstU32<'a, [f32; 3]>,
    pub brush_verts: FatPointerCountFirstU32<'a, [f32; 3]>,
    pub uinds: FatPointerCountFirstU32<'a, u16>,
    pub tri_count: i32,
    pub tri_indices: Ptr32<'a, u16>,
    pub tri_edge_is_walkable: Ptr32<'a, u8>,
    pub borders: FatPointerCountFirstU32<'a, CollisionBorderRaw>,
    pub partitions: FatPointerCountFirstU32<'a, CollisionPartitionRaw<'a>>,
    pub aabb_trees: FatPointerCountFirstU32<'a, CollisionAabbTreeRaw>,
    pub cmodels: FatPointerCountFirstU32<'a, CModelRaw>,
    pub brushes: FatPointerCountFirstU32<'a, CBrushRaw<'a>>,
    pub num_clusters: i32,
    pub cluster_bytes: i32,
    pub visibility: Ptr32<'a, u8>,
    pub vised: i32,
    pub map_ents: Ptr32<'a, MapEntsRaw<'a>>,
    pub box_brush: Ptr32<'a, CBrushRaw<'a>>,
    pub box_model: CModelRaw,
    pub original_dyn_ent_count: u16,
    pub dyn_ent_count: [u16; 4],
    pad: [u8; 2],
    pub dyn_ent_def_list: [Ptr32<'a, DynEntityDefRaw<'a>>; 2],
    pub dyn_ent_pose_list: [Ptr32<'a, DynEntityPoseRaw>; 2],
    pub dyn_ent_client_list: [Ptr32<'a, DynEntityClient>; 2],
    pub dyn_ent_server_list: [Ptr32<'a, DynEntityServer>; 2],
    pub dyn_ent_coll_list: [Ptr32<'a, DynEntityCollRaw>; 4],
    pub constraints: FatPointerCountFirstU32<'a, PhysConstraintRaw<'a>>,
    pub ropes: FatPointerCountFirstU32<'a, RopeRaw<'a>>,
    pub checksum: u32,
}
assert_size!(ClipMapRaw, 332);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CStaticModelRaw<'a> {
    pub writable: CStaticModelWritable,
    pub xmodel: Ptr32<'a, XModelRaw<'a>>,
    pub origin: [f32; 3],
    pub inv_scaled_axis: [[f32; 3]; 3],
    pub absmin: [f32; 3],
    pub absmax: [f32; 3],
}
assert_size!(CStaticModelRaw, 80);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct CStaticModelWritable {
    pub next_model_in_world_sector: u16,
}
assert_size!(CStaticModelWritable, 2);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct DMaterialName(#[serde(with = "serde_arrays")] [u8; 64]);

impl Display for DMaterialName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self
            .0
            .into_iter()
            .enumerate()
            .find(|(_, c)| *c == 0)
            .map(|(i, _)| i)
            .unwrap_or(64);
        let s = self.0[..len].iter().map(|c| *c as char).collect::<String>();
        write!(f, "{}", s)
    }
}

impl Default for DMaterialName {
    fn default() -> Self {
        Self([0u8; 64])
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct DMaterialRaw {
    pub material: DMaterialName,
    pub surface_flags: i32,
    pub content_flags: i32,
}
assert_size!(DMaterialRaw, 72);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CNodeRaw<'a> {
    pub plane: Ptr32<'a, CPlaneRaw>,
    pub children: [i16; 2],
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CLeafRaw {
    pub first_coll_aabb_index: u16,
    pub coll_aabb_count: u16,
    pub brush_contents: i32,
    pub terrain_contents: i32,
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub leaf_brush_node: i32,
    pub cluster: u16,
}
assert_size!(CLeafRaw, 44);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CLeafBrushNodeRaw<'a> {
    pub axis: u8,
    pub leaf_brush_count: u16,
    pub contents: i32,
    pub data: CLeafBrushNodeDataRaw<'a>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CLeafBrushNodeDataRaw<'a>(Ptr32<'a, ()>);
assert_size!(CLeafBrushNodeDataRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CLeafBrushNodeLeafRaw<'a> {
    pub brushes: Ptr32<'a, u16>,
}
assert_size!(CLeafBrushNodeLeafRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct CLeafBrushNodeLeaf {
    pub brushes: Vec<u16>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct CLeafBrushNodeChildren {
    pub dist: f32,
    pub range: f32,
    pub child_offset: [u16; 2],
}
assert_size!(CLeafBrushNodeChildren, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum CLeafBrushNodeData {
    Leaf(CLeafBrushNodeLeaf),
    Children(CLeafBrushNodeChildren),
}

impl Default for CLeafBrushNodeData {
    fn default() -> Self {
        Self::Leaf(CLeafBrushNodeLeaf::default())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CollisionBorderRaw {
    pub dist_eq: [f32; 3],
    pub z_slope: f32,
    pub z_base: f32,
    pub start: f32,
    pub length: f32,
}
assert_size!(CollisionBorderRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CollisionPartitionRaw<'a> {
    pub tri_count: u8,
    pub border_count: u8,
    pub first_tri: i32,
    pub nuinds: i32,
    pub fuind: i32,
    pub borders: Ptr32<'a, CollisionBorderRaw>,
}
assert_size!(CollisionPartitionRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CollisionAabbTreeRaw {
    pub origin: [f32; 3],
    pub material_index: u16,
    pub child_count: u16,
    pub half_size: [f32; 3],
    pub index: i32,
}
assert_size!(CollisionAabbTreeRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CModelRaw {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub radius: f32,
    pub leaf: CLeafRaw,
}
assert_size!(CModelRaw, 72);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CBrushRaw<'a> {
    pub mins: [f32; 3],
    pub contents: i32,
    pub maxs: [f32; 3],
    pub sides: FatPointerCountFirstU32<'a, CBrushSideRaw<'a>>,
    pub axial_cflags: [[i32; 3]; 2],
    pub axial_sflags: [[i32; 3]; 2],
    pub verts: FatPointerCountFirstU32<'a, [f32; 3]>,
    pad: [u8; 4],
}
assert_size!(CBrushRaw, 96);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct DynEntityDefRaw<'a> {
    pub type_: i32,
    pub pose: GfxPlacementRaw,
    pub xmodel: Ptr32<'a, XModelRaw<'a>>,
    pub destroyed_xmodel: Ptr32<'a, XModelRaw<'a>>,
    pub brush_model: u16,
    pub physics_brush_model: u16,
    pub destroy_fx: Ptr32<'a, FxEffectDefRaw<'a>>,
    pub destroy_sound: u32,
    pub destroy_pieces: Ptr32<'a, XModelPiecesRaw<'a>>,
    pub phys_preset: Ptr32<'a, PhysPresetRaw<'a>>,
    pub phys_constraints: [i16; 4],
    pub health: i32,
    pub flags: i32,
    pub contents: i32,
    pub targetname: ScriptString,
    pub target: ScriptString,
}
assert_size!(DynEntityDefRaw, 84);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, FromPrimitive)]
pub enum DynEntityType {
    #[default]
    INVALID = 0,
    CLUTTER = 1,
    DESTRUCT = 2,
    COUNT = 3,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GfxPlacementRaw {
    pub quat: [f32; 4],
    pub origin: [f32; 3],
}
assert_size!(GfxPlacementRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct DynEntityPoseRaw {
    pub pose: GfxPlacementRaw,
    pub radius: f32,
}
assert_size!(DynEntityPoseRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct DynEntityClient {
    pub phys_obj_id: i32,
    pub flags: u16,
    pub lighting_handle: u16,
    pub health: i32,
    pub burn_time: u16,
    pub fade_time: u16,
    pub physics_start_time: i32,
}
assert_size!(DynEntityClient, 20);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct DynEntityServer {
    pub flags: u16,
    pub health: i32,
}
assert_size!(DynEntityServer, 8);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct DynEntityCollRaw {
    pub sector: u16,
    pub next_ent_in_sector: u16,
    pub link_mins: [f32; 3],
    pub link_maxs: [f32; 3],
    pub contents: i32,
}
assert_size!(DynEntityCollRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct RopeRaw<'a> {
    pub m_particles: [ParRaw; 25],
    pub m_constraints: [ConstraintRaw; 30],
    pub m_entity_anchors: [i32; 3],
    pub m_num_particles: i32,
    pub m_num_constraints: i32,
    pub m_num_entity_anchors: i32,
    pub m_num_draw_verts: i32,
    pub m_client_verts: RopeClientVertsRaw,
    pub m_min: [f32; 3],
    pub m_max: [f32; 3],
    pub m_start: [f32; 3],
    pub m_end: [f32; 3],
    pub m_in_use: i32,
    pub m_visible: i32,
    pub m_dist_constraint: i32,
    pub m_flags: i32,
    pub m_material: Ptr32<'a, MaterialRaw<'a>>,
    pub m_seglen: f32,
    pub m_length: f32,
    pub m_width: f32,
    pub m_scale: f32,
    pub m_force_scale: f32,
    pub m_health: i32,
    pub m_frame: i32,
    pub m_stable_count: i32,
    pub m_static_rope: i32,
    pub m_lighting_handle: u16,
    pad: [u8; 2],
}
assert_size!(RopeRaw, 3188);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ParRaw {
    pub p: [f32; 3],
    pub p0: [f32; 3],
    pub p_prev: [f32; 3],
    pub flags: i32,
}
assert_size!(ParRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ConstraintRaw {
    pub p: [f32; 3],
    pub type_: i32,
    pub enetiy_index: i32,
    pub bone_name_hash: i32,
    pub pi1: u8,
    pub pi2: u8,
}
assert_size!(ConstraintRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, FromPrimitive)]
pub enum RopeConstraint {
    #[default]
    PAIR = 0,
    WORLD = 1,
    DENTITY = 2,
    CENTITY = 3,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct RopeClientVertsRaw {
    pub frame_verts: [RopeFrameVertsRaw; 2],
    pub frame_index: u32,
}
assert_size!(RopeClientVertsRaw, 1212);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct RopeFrameVertsRaw {
    pub num_verts: i32,
    #[serde(with = "serde_arrays")]
    pub v: [[f32; 3]; 50],
}
assert_size!(RopeFrameVertsRaw, 604);

impl Default for RopeFrameVertsRaw {
    fn default() -> Self {
        Self {
            num_verts: 0,
            v: [[0.0; 3]; 50],
        }
    }
}
