use crate::*;

pub struct XModelRaw<'a> {
    pub name: XString<'a>,
    pub num_bones: u8,
    pub num_root_bones: u8,
    pub numsurfs: u8,
    pub lod_ramp_type: u8,
    pub bone_names: Ptr32<'a, u16>,
    pub parent_list: Ptr32<'a, u8>,
    pub quats: Ptr32<'a, i16>,
    pub trans: Ptr32<'a, f32>,
    pub part_classification: Ptr32<'a, u8>,
    pub base_mat: Ptr32<'a, DObjAnimMat>,
    pub surfs: Ptr32<'a, XSurfaceRaw<'a>>,
    pub material_handles: Ptr32<'a, Ptr32<'a, techset::Material>>,
    pub lod_info: [XModelLodInfo; 4],
    pub load_dist_auto_generated: u8,
    pad: [u8; 3],
    pub coll_surfs: FatPointerCountLastU32<'a, XModelCollSurf<'a>>,
    pub contents: i32,
    pub bone_info: Ptr32<'a, XBoneInfo>,
    pub radius: f32,
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub num_lods: i16,
    pub coll_lod: i16,
    pub stream_info: XModelStreamInfoRaw<'a>,
    pub mem_usage: i32,
    pub flags: i32,
    pub bad: bool,
    pad_2: [u8; 3],
    pub phys_preset: Ptr32<'a, PhysPresetRaw<'a>>,
    pub num_collmaps: u8,
    pad_3: [u8; 3],
    pub collmaps: Ptr32<'a, CollmapRaw<'a>>,
    pub phys_constraints: Ptr32<'a, PhysConstraintRaw<'a>>,
}
assert_size!(XModelRaw, 252);

pub struct DObjAnimMat {
    pub quat: [f32; 4],
    pub trans: [f32; 3],
    pub trans_weight: f32,
}
assert_size!(DObjAnimMat, 32);

pub type IDirect3DVertexBuffer9 = ();
pub type IDirect3DIndexBuffer9 = ();

pub struct XSurfaceRaw<'a> {
    pub tile_mode: u8,
    pub vert_list_count: u8,
    pub flags: u16,
    pub vert_count: u16,
    pub tri_count: u16,
    pub base_tri_index: u16,
    pub base_vert_index: u16,
    pub tri_indices: Ptr32<'a, u16>,
    pub vert_info: XSurfaceVertexInfoRaw<'a>,
    pub verts0: Ptr32<'a, GfxPackedVertex>,
    pub vb0: Ptr32<'a, IDirect3DVertexBuffer9>,
    pub vert_list: Ptr32<'a, XRigidVertListRaw<'a>>,
    pub index_buffer: Ptr32<'a, IDirect3DIndexBuffer9>,
    pub part_bits: [i32; 5],
}   
assert_size!(XSurfaceRaw, 68);

pub struct XSurfaceVertexInfoRaw<'a> {
    pub vert_count: [i16; 4],
    pub verts_blend: Ptr32<'a, u16>,
    pub tension_data: Ptr32<'a, f32>,
}
assert_size!(XSurfaceVertexInfoRaw, 16);

pub struct GfxPackedVertex {
    pub xyz: [f32; 3],
    pub binormal_sign: f32,
    pub color: GfxColorRaw,
    pub tex_coord: TexCoordsRaw,
    pub normal: UnitVecRaw,
    pub tangent: UnitVecRaw,
}
assert_size!(GfxPackedVertex, 32);

pub struct GfxColorRaw(pub [u8; 4]);
assert_size!(GfxColorRaw, 4);

pub struct TexCoordsRaw(pub u32);
assert_size!(TexCoordsRaw, 4);

pub struct UnitVecRaw(pub [u8; 4]);
assert_size!(UnitVecRaw, 4);

pub struct XRigidVertListRaw<'a> {
    pub bone_offset: u16,
    pub vert_count: u16,
    pub tri_offset: u16,
    pub tri_count: u16,
    pub collision_tree: Ptr32<'a, XSurfaceCollisionTree<'a>>,
}
assert_size!(XRigidVertListRaw, 12);

pub struct XSurfaceCollisionTree<'a> {
    pub trans: [f32; 3],
    pub scale: [f32; 3],
    pub nodes: FatPointerCountFirstU32<'a, XSurfaceCollisionNode>,
    pub leafs: FatPointerCountFirstU32<'a, XSurfaceCollisionLeaf>,
}
assert_size!(XSurfaceCollisionTree, 40);

pub struct XSurfaceCollisionNode {
    pub aabb: XSurfaceCollisionAabb,
    pub child_begin_index: u16,
    pub child_count: u16,
}
assert_size!(XSurfaceCollisionNode, 16);

pub struct XSurfaceCollisionAabb {
    pub mins: [u16; 3],
    pub maxs: [u16; 3],
}
assert_size!(XSurfaceCollisionAabb, 12);

pub struct XSurfaceCollisionLeaf {
    pub triangle_begin_index: u16,
}
assert_size!(XSurfaceCollisionLeaf, 2);

pub struct XModelLodInfo {
    pub dist: f32,
    pub numsurfs: u16,
    pub surf_index: u16,
    pub part_bits: [i32; 5],
    pub lod: u8,
    pub smc_index_plus_one: u8,
    pub smc_alloc_bits: u8,
    pub unused: u8,
}
assert_size!(XModelLodInfo, 32);

pub struct XModelCollSurf<'a> {
    pub coll_tris: FatPointerCountLastU32<'a, XModelCollTri>,
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub bone_idx: i32,
    pub contents: i32,
    pub surf_flags: i32,
}
assert_size!(XModelCollSurf, 44);

pub struct XModelCollTri {
    pub plane: [f32; 4],
    pub svec: [f32; 4],
    pub tvec: [f32; 4],
}
assert_size!(XModelCollTri, 48);

pub struct XBoneInfo {
    pub bounds: [[f32; 3]; 2],
    pub offset: [f32; 3],
    pub radius_squared: f32,
    pub collmap: u8,
}
assert_size!(XBoneInfo, 44);

pub struct XModelStreamInfoRaw<'a> {
    pub high_mip_bounds: Ptr32<'a, XModelHighMipBounds>,
}
assert_size!(XModelStreamInfoRaw, 4);

pub struct XModelHighMipBounds {
    pub center: [f32; 3],
    pub himip_radius_sq: f32,
}
assert_size!(XModelHighMipBounds, 16);

pub struct PhysPresetRaw<'a> {
    pub name: XString<'a>,
    pub flags: i32,
    pub mass: f32,
    pub bounce: f32,
    pub friction: f32,
    pub bullet_force_scale: f32,
    pub explosive_force_scale: f32,
    pub snd_alias_prefix: XString<'a>,
    pub pieces_spread_fraction: f32,
    pub pieces_upward_velocity: f32,
    pub can_float: i32,
    pub gravity_scale: f32,
    pub center_of_mass_offset: [f32; 3],
    pub buoyancy_box_min: [f32; 3],
    pub buoyancy_box_max: [f32; 3],
}
assert_size!(PhysPresetRaw, 84);

pub struct CollmapRaw<'a> {
    pub geom_list: Ptr32<'a, PhysGeomListRaw<'a>>,
}
assert_size!(CollmapRaw, 4);

pub struct PhysGeomListRaw<'a> {
    pub geoms: FatPointerCountFirstU32<'a, PhysGeomInfoRaw<'a>>,
    pub contents: i32,
}
assert_size!(PhysGeomListRaw, 12);

pub struct PhysGeomInfoRaw<'a> {
    pub brush: Ptr32<'a, BrushWrapperRaw<'a>>,
    pub type_: i32,
    pub orientation: [[f32; 3]; 3],
    pub offset: [f32; 3],
    pub half_lengths: [f32; 3],
}
assert_size!(PhysGeomInfoRaw, 68);

pub struct BrushWrapperRaw<'a> {
    pub mins: [f32; 3],
    pub contents: i32,
    pub maxs: [f32; 3],
    pub sides: FatPointerCountFirstU32<'a, CBrushSideRaw<'a>>,
    pub axial_cflags: [[i32; 3]; 2],
    pub axial_sflags: [[i32; 3]; 2],
    pub verts: FatPointerCountFirstU32<'a, [f32; 3]>,
    pub planes: Ptr32<'a, CPlane>,
}
assert_size!(BrushWrapperRaw, 96);

pub struct CBrushSideRaw<'a> {
    pub plane: Ptr32<'a, CPlane>,
    pub cflags: i32,
    pub sflags: i32
}
assert_size!(CBrushSideRaw, 12);

pub struct CPlane {
    pub normal: [f32; 3],
    pub dist: f32,
    pub type_: u8,
    pub signbits: u8,
    pad: [u8; 2],
}
assert_size!(CPlane, 20);

pub struct PhysConstraintRaw<'a> {
    pub targetname: u16,
    pad: u16,
    pub type_: i32,
    pub attach_point_type1: i32,
    pub target_index1: i32,
    pub target_ent1: u16,
    pad_2: u16,
    pub target_bone1: XString<'a>,
    pub attach_point_type2: i32,
    pub target_index2: i32,
    pub target_ent2: u16,
    pad_3: u16,
    pub target_bone2: XString<'a>,
    pub offset: [f32; 3],
    pub pos: [f32; 3],
    pub pos2: [f32; 3],
    pub dir: [f32; 3],
    pub flags: i32,
    pub timeout: i32,
    pub min_health: i32,
    pub max_health: i32,
    pub distance: f32,
    pub damp: f32,
    pub power: f32,
    pub scale: [f32; 3],
    pub spin_scale: f32,
    pub min_angle: f32,
    pub max_angle: f32,
    pub material: Ptr32<'a, techset::Material>,
    pub constraint_handle: i32,
    pub rope_index: i32,
    pub centity_num: [i32; 4],
}
assert_size!(PhysConstraintRaw, 168);