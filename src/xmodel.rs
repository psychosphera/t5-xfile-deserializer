use crate::{
    common::{Mat3, Vec3, Vec4},
    *,
};

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
    pub base_mat: Ptr32<'a, DObjAnimMatRaw>,
    pub surfs: Ptr32<'a, XSurfaceRaw<'a>>,
    pub material_handles: Ptr32<'a, Ptr32<'a, techset::Material>>,
    pub lod_info: [XModelLodInfoRaw; 4],
    pub load_dist_auto_generated: u8,
    pad: [u8; 3],
    pub coll_surfs: FatPointerCountLastU32<'a, XModelCollSurfRaw<'a>>,
    pub contents: i32,
    pub bone_info: Ptr32<'a, XBoneInfoRaw>,
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
    pub phys_constraints: Ptr32<'a, PhysConstraintsRaw<'a>>,
}
assert_size!(XModelRaw, 252);

pub struct XModel {
    pub name: String,
    pub num_bones: usize,
    pub num_root_bones: usize,
    pub numsurfs: usize,
    pub lod_ramp_type: u8,
    pub bone_names: Vec<u16>,
    pub parent_list: Vec<u8>,
    pub quats: Vec<i16>,
    pub trans: Vec<f32>,
    pub part_classification: Vec<u8>,
    pub base_mat: Option<Box<DObjAnimMat>>,
    pub surfs: Option<Box<XSurface>>,
    pub material_handles: Vec<Box<techset::Material>>,
    pub lod_info: [XModelLodInfoRaw; 4],
    pub load_dist_auto_generated: u8,
    pub coll_surfs: Vec<XModelCollSurf>,
    pub contents: i32,
    pub bone_info: Vec<XBoneInfo>,
    pub radius: f32,
    pub mins: Vec3,
    pub maxs: Vec3,
    pub num_lods: i16,
    pub coll_lod: i16,
    pub stream_info: XModelStreamInfo,
    pub mem_usage: i32,
    pub flags: i32,
    pub bad: bool,
    pub phys_preset: Option<Box<PhysPreset>>,
    pub num_collmaps: u8,
    pub collmaps: Option<Box<Collmap>>,
    pub phys_constraints: Option<Box<PhysConstraints>>,
}

pub struct DObjAnimMatRaw {
    pub quat: [f32; 4],
    pub trans: [f32; 3],
    pub trans_weight: f32,
}
assert_size!(DObjAnimMatRaw, 32);

pub struct DObjAnimMat {
    pub quat: Vec4,
    pub trans: Vec3,
    pub trans_weight: f32,
}

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
    pub verts0: Ptr32<'a, GfxPackedVertexRaw>,
    pub vb0: Ptr32<'a, IDirect3DVertexBuffer9>,
    pub vert_list: Ptr32<'a, XRigidVertListRaw<'a>>,
    pub index_buffer: Ptr32<'a, IDirect3DIndexBuffer9>,
    pub part_bits: [i32; 5],
}
assert_size!(XSurfaceRaw, 68);

pub struct XSurface {
    pub tile_mode: u8,
    pub vert_list_count: usize,
    pub flags: u16,
    pub vert_count: usize,
    pub tri_count: usize,
    pub base_tri_index: usize,
    pub base_vert_index: usize,
    pub tri_indices: Vec<u16>,
    pub vert_info: XSurfaceVertexInfo,
    pub verts0: Option<Box<GfxPackedVertex>>,
    pub vb0: Option<Box<IDirect3DVertexBuffer9>>,
    pub vert_list: Option<Box<XRigidVertList>>,
    pub index_buffer: Option<Box<IDirect3DIndexBuffer9>>,
    pub part_bits: [i32; 5],
}

pub struct XSurfaceVertexInfoRaw<'a> {
    pub vert_count: [i16; 4],
    pub verts_blend: Ptr32<'a, u16>,
    pub tension_data: Ptr32<'a, f32>,
}
assert_size!(XSurfaceVertexInfoRaw, 16);

pub struct XSurfaceVertexInfo {
    pub vert_count: [i16; 4],
    pub verts_blend: Vec<u16>,
    pub tension_data: Vec<f32>,
}

pub struct GfxPackedVertexRaw {
    pub xyz: [f32; 3],
    pub binormal_sign: f32,
    pub color: GfxColorRaw,
    pub tex_coord: TexCoordsRaw,
    pub normal: UnitVecRaw,
    pub tangent: UnitVecRaw,
}
assert_size!(GfxPackedVertexRaw, 32);

pub struct GfxPackedVertex {
    pub xyz: Vec3,
    pub binormal_sign: f32,
    pub color: GfxColorRaw,
    pub tex_coord: TexCoordsRaw,
    pub normal: UnitVecRaw,
    pub tangent: UnitVecRaw,
}

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
    pub collision_tree: Ptr32<'a, XSurfaceCollisionTreeRaw<'a>>,
}
assert_size!(XRigidVertListRaw, 12);

pub struct XRigidVertList {
    pub bone_offset: usize,
    pub vert_count: usize,
    pub tri_offset: usize,
    pub tri_count: usize,
    pub collision_tree: Option<Box<XSurfaceCollisionTree>>,
}

pub struct XSurfaceCollisionTreeRaw<'a> {
    pub trans: [f32; 3],
    pub scale: [f32; 3],
    pub nodes: FatPointerCountFirstU32<'a, XSurfaceCollisionNodeRaw>,
    pub leafs: FatPointerCountFirstU32<'a, XSurfaceCollisionLeafRaw>,
}
assert_size!(XSurfaceCollisionTreeRaw, 40);

pub struct XSurfaceCollisionTree {
    pub trans: Vec3,
    pub scale: Vec3,
    pub nodes: Vec<XSurfaceCollisionNode>,
    pub leafs: Vec<XSurfaceCollisionLeaf>,
}

pub struct XSurfaceCollisionNodeRaw {
    pub aabb: XSurfaceCollisionAabb,
    pub child_begin_index: u16,
    pub child_count: u16,
}
assert_size!(XSurfaceCollisionNodeRaw, 16);

pub struct XSurfaceCollisionNode {
    pub aabb: XSurfaceCollisionAabb,
    pub child_begin_index: usize,
    pub child_count: usize,
}

pub struct XSurfaceCollisionAabb {
    pub mins: [u16; 3],
    pub maxs: [u16; 3],
}
assert_size!(XSurfaceCollisionAabb, 12);

pub struct XSurfaceCollisionLeafRaw {
    pub triangle_begin_index: u16,
}
assert_size!(XSurfaceCollisionLeafRaw, 2);

pub struct XSurfaceCollisionLeaf {
    pub triangle_begin_index: usize,
}

pub struct XModelLodInfoRaw {
    pub dist: f32,
    pub numsurfs: u16,
    pub surf_index: u16,
    pub part_bits: [i32; 5],
    pub lod: u8,
    pub smc_index_plus_one: u8,
    pub smc_alloc_bits: u8,
    unused: u8,
}
assert_size!(XModelLodInfoRaw, 32);

#[repr(u8)]
pub enum Lod {
    Zero,
    One,
    Two,
    Three,
}

pub struct XModelLodInfo {
    pub dist: f32,
    pub numsurfs: u16,
    pub surf_index: usize,
    pub part_bits: [i32; 5],
    pub lod: Lod,
    pub smc_index_plus_one: usize,
    pub smc_alloc_bits: u8,
}

pub struct XModelCollSurfRaw<'a> {
    pub coll_tris: FatPointerCountLastU32<'a, XModelCollTriRaw>,
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub bone_idx: i32,
    pub contents: i32,
    pub surf_flags: i32,
}
assert_size!(XModelCollSurfRaw, 44);

pub struct XModelCollSurf {
    pub coll_tris: Vec<XModelCollTri>,
    pub mins: Vec3,
    pub maxs: Vec3,
    pub bone_idx: usize,
    pub contents: i32,
    pub surf_flags: i32,
}

pub struct XModelCollTriRaw {
    pub plane: [f32; 4],
    pub svec: [f32; 4],
    pub tvec: [f32; 4],
}
assert_size!(XModelCollTriRaw, 48);

pub struct XModelCollTri {
    pub plane: Vec4,
    pub svec: Vec4,
    pub tvec: Vec4,
}

pub struct XBoneInfoRaw {
    pub bounds: [[f32; 3]; 2],
    pub offset: [f32; 3],
    pub radius_squared: f32,
    pub collmap: u8,
}
assert_size!(XBoneInfoRaw, 44);

pub struct XBoneInfo {
    pub bounds: [Vec3; 2],
    pub offset: Vec3,
    pub radius_squared: f32,
    pub collmap: u8,
}

pub struct XModelStreamInfoRaw<'a> {
    pub high_mip_bounds: Ptr32<'a, XModelHighMipBoundsRaw>,
}
assert_size!(XModelStreamInfoRaw, 4);

pub struct XModelStreamInfo {
    pub high_mip_bounds: Option<Box<XModelHighMipBounds>>,
}

pub struct XModelHighMipBoundsRaw {
    pub center: [f32; 3],
    pub himip_radius_sq: f32,
}
assert_size!(XModelHighMipBoundsRaw, 16);

pub struct XModelHighMipBounds {
    pub center: Vec3,
    pub himip_radius_sq: f32,
}

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

pub struct PhysPreset {
    pub name: String,
    pub flags: i32,
    pub mass: f32,
    pub bounce: f32,
    pub friction: f32,
    pub bullet_force_scale: f32,
    pub explosive_force_scale: f32,
    pub snd_alias_prefix: String,
    pub pieces_spread_fraction: f32,
    pub pieces_upward_velocity: f32,
    pub can_float: i32,
    pub gravity_scale: f32,
    pub center_of_mass_offset: Vec3,
    pub buoyancy_box_min: Vec3,
    pub buoyancy_box_max: Vec3,
}

pub struct CollmapRaw<'a> {
    pub geom_list: Ptr32<'a, PhysGeomListRaw<'a>>,
}
assert_size!(CollmapRaw, 4);

pub struct Collmap {
    pub geom_list: Option<Box<PhysGeomList>>,
}

pub struct PhysGeomListRaw<'a> {
    pub geoms: FatPointerCountFirstU32<'a, PhysGeomInfoRaw<'a>>,
    pub contents: i32,
}
assert_size!(PhysGeomListRaw, 12);

pub struct PhysGeomList {
    pub geoms: Vec<PhysGeomInfo>,
    pub contents: i32,
}

pub struct PhysGeomInfoRaw<'a> {
    pub brush: Ptr32<'a, BrushWrapperRaw<'a>>,
    pub type_: i32,
    pub orientation: [[f32; 3]; 3],
    pub offset: [f32; 3],
    pub half_lengths: [f32; 3],
}
assert_size!(PhysGeomInfoRaw, 68);

#[repr(i32)]
pub enum PhysGeomType {
    BOX = 0x01,
    BRUSH = 0x02,
    CYLINDER = 0x03,
}

pub struct PhysGeomInfo {
    pub brush: Option<Box<BrushWrapper>>,
    pub type_: PhysGeomType,
    pub orientation: Mat3,
    pub offset: Vec3,
    pub half_lengths: Vec3,
}

pub struct BrushWrapperRaw<'a> {
    pub mins: [f32; 3],
    pub contents: i32,
    pub maxs: [f32; 3],
    pub sides: FatPointerCountFirstU32<'a, CBrushSideRaw<'a>>,
    pub axial_cflags: [[i32; 3]; 2],
    pub axial_sflags: [[i32; 3]; 2],
    pub verts: FatPointerCountFirstU32<'a, [f32; 3]>,
    pub planes: Ptr32<'a, CPlaneRaw>,
}
assert_size!(BrushWrapperRaw, 96);

pub struct BrushWrapper {
    pub mins: Vec3,
    pub contents: i32,
    pub maxs: Vec3,
    pub sides: Vec<CBrushSide>,
    pub axial_cflags: [[i32; 3]; 2],
    pub axial_sflags: [[i32; 3]; 2],
    pub verts: Vec<Vec3>,
    pub planes: Vec<Option<Box<CPlane>>>,
}

pub struct CBrushSideRaw<'a> {
    pub plane: Ptr32<'a, CPlaneRaw>,
    pub cflags: i32,
    pub sflags: i32,
}
assert_size!(CBrushSideRaw, 12);

pub struct CBrushSide {
    pub plane: Option<Box<CPlane>>,
    pub cflags: i32,
    pub sflags: i32,
}

pub struct CPlaneRaw {
    pub normal: [f32; 3],
    pub dist: f32,
    pub type_: u8,
    pub signbits: u8,
    pad: [u8; 2],
}
assert_size!(CPlaneRaw, 20);

pub enum CPlaneType {
    Axial(u8),
    NonAxial,
    Other(u8),
}

impl CPlaneType {
    pub fn from_u8(value: u8) -> Self {
        if value < 3 {
            Self::Axial(value)
        } else if value == 3 {
            Self::NonAxial
        } else {
            Self::Other(value)
        }
    }
}

pub struct CPlaneSignbits(u8);

#[repr(u8)]
pub enum Sign {
    POSITIVE = 0,
    NEGATIVE = 1,
}

impl Sign {
    pub fn from_bool(b: bool) -> Self {
        if b {
            Self::NEGATIVE
        } else {
            Self::POSITIVE
        }
    }

    pub fn from_isize(i: isize) -> Self {
        if i == 0 {
            Self::POSITIVE
        } else {
            Self::NEGATIVE
        }
    }
}

impl CPlaneSignbits {
    pub fn bits(self) -> u8 {
        self.0
    }

    pub fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    pub fn from_signs(x: Sign, y: Sign, z: Sign) -> Self {
        Self(x as u8 + (y as u8) << 1 + (z as u8) << 2)
    }
}

pub struct CPlane {
    pub normal: Vec3,
    pub dist: f32,
    pub type_: CPlaneType,
    pub signbits: CPlaneSignbits,
}

pub struct PhysConstraintsRaw<'a> {
    name: XString<'a>,
    count: u32,
    data: [PhysConstraintRaw<'a>; 16],
}
assert_size!(PhysConstraintsRaw, 2696);

pub struct PhysConstraints {
    name: String,
    data: Vec<PhysConstraint>,
}

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
    pub material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub constraint_handle: i32,
    pub rope_index: i32,
    pub centity_num: [i32; 4],
}
assert_size!(PhysConstraintRaw, 168);

#[repr(u32)]
pub enum ConstraintType {
    NONE = 0x00,
    POINT = 0x01,
    DISTANCE = 0x02,
    HINGE = 0x03,
    JOINT = 0x04,
    ACTUATOR = 0x05,
    FAKE_SHAKE = 0x06,
    LAUNCH = 0x07,
    ROPE = 0x08,
    LIGHT = 0x09,
    NUM_TYPES = 0x0A,
}

#[repr(u32)]
pub enum AttachPointType {
    WORLD = 0x00,
    DYNENT = 0x01,
    ENT = 0x02,
    BONE = 0x03,
}

pub struct PhysConstraint {
    pub targetname: u16,
    pub type_: ConstraintType,
    pub attach_point_type1: AttachPointType,
    pub target_index1: usize,
    pub target_ent1: u16,
    pub target_bone1: String,
    pub attach_point_type2: AttachPointType,
    pub target_index2: usize,
    pub target_ent2: u16,
    pub target_bone2: String,
    pub offset: Vec3,
    pub pos: Vec3,
    pub pos2: Vec3,
    pub dir: Vec3,
    pub flags: i32,
    pub timeout: i32,
    pub min_health: i32,
    pub max_health: i32,
    pub distance: f32,
    pub damp: f32,
    pub power: f32,
    pub scale: Vec3,
    pub spin_scale: f32,
    pub min_angle: f32,
    pub max_angle: f32,
    pub material: Option<Box<techset::Material>>,
    pub constraint_handle: i32,
    pub rope_index: i32,
    pub centity_num: [i32; 4],
}
