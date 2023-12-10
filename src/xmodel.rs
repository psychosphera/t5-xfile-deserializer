use crate::{
    common::{Mat3, Vec3, Vec4},
    *,
};

#[derive(Clone, Default, Debug, Deserialize)]
pub struct XModelRaw<'a> {
    pub name: XString<'a>,
    pub num_bones: u8,
    pub num_root_bones: u8,
    pub numsurfs: u8,
    pub lod_ramp_type: u8,
    pub bone_names: Ptr32<'a, ScriptString>,
    pub parent_list: Ptr32<'a, u8>,
    pub quats: Ptr32<'a, i16>,
    pub trans: Ptr32<'a, f32>,
    pub part_classification: Ptr32<'a, u8>,
    pub base_mat: Ptr32<'a, DObjAnimMatRaw>,
    pub surfs: Ptr32<'a, XSurfaceRaw<'a>>,
    pub material_handles: Ptr32<'a, Ptr32<'a, techset::MaterialRaw<'a>>>,
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
    pub collmaps: FatPointerCountFirstU32<'a, CollmapRaw<'a>>,
    pub phys_constraints: Ptr32<'a, PhysConstraintsRaw<'a>>,
}
assert_size!(XModelRaw, 252);

#[derive(Clone, Default, Debug)]
pub struct XModel {
    pub name: String,
    pub num_bones: usize,
    pub num_root_bones: usize,
    pub numsurfs: usize,
    pub lod_ramp_type: u8,
    pub bone_names: Vec<String>,
    pub parent_list: Vec<u8>,
    pub quats: Vec<i16>,
    pub trans: Vec<f32>,
    pub part_classification: Vec<u8>,
    pub base_mat: Vec<Box<DObjAnimMat>>,
    pub surfs: Vec<Box<XSurface>>,
    pub material_handles: Vec<Box<techset::Material>>,
    pub lod_info: [XModelLodInfo; 4],
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
    pub collmaps: Vec<Box<Collmap>>,
    pub phys_constraints: Option<Box<PhysConstraints>>,
}

impl<'a> XFileInto<XModel> for XModelRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> XModel {
        let name = self.name.xfile_into(&mut xfile);
        let bone_names = self
            .bone_names
            .to_array(self.num_bones as _)
            .to_vec(&mut xfile)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let parent_list = self
            .parent_list
            .to_array(self.num_bones as usize - self.num_root_bones as usize)
            .to_vec(&mut xfile);
        let quats = self
            .quats
            .to_array((self.num_bones as usize - self.num_root_bones as usize) * 4)
            .to_vec(&mut xfile);
        let trans = self
            .trans
            .to_array((self.num_bones as usize - self.num_root_bones as usize) * 4)
            .to_vec(&mut xfile);
        let part_classification = self
            .part_classification
            .to_array(self.num_bones as _)
            .to_vec(&mut xfile);
        let base_mat = self
            .base_mat
            .to_array(self.num_bones as _)
            .to_vec(&mut xfile)
            .into_iter()
            .map(|m| Box::new(m.into()))
            .collect();
        let surfs = self
            .surfs
            .clone()
            .to_array(self.numsurfs as _)
            .xfile_into(&mut xfile)
            .into_iter()
            .map(Box::new)
            .collect();
        let material_handles = self
            .material_handles
            .to_array(self.numsurfs as _)
            .xfile_into(&mut xfile)
            .into_iter()
            .filter_map(|h| h)
            .collect();
        let lod_info = [
            self.lod_info[0].into(),
            self.lod_info[1].into(),
            self.lod_info[2].into(),
            self.lod_info[3].into(),
        ];
        let coll_surfs = self.coll_surfs.xfile_into(&mut xfile);
        let bone_info = self
            .bone_info
            .to_array(self.num_bones as _)
            .to_vec(&mut xfile)
            .into_iter()
            .map(Into::into)
            .collect();
        let stream_info = self.stream_info.xfile_into(&mut xfile);
        let phys_preset = self.phys_preset.xfile_into(&mut xfile);
        let collmaps = self
            .collmaps
            .xfile_into(&mut xfile)
            .into_iter()
            .map(Box::new)
            .collect();
        let phys_constraints = self.phys_constraints.xfile_into(xfile);

        XModel {
            name,
            num_bones: self.num_bones as _,
            num_root_bones: self.num_root_bones as _,
            numsurfs: self.numsurfs as _,
            lod_ramp_type: self.lod_ramp_type,
            bone_names,
            parent_list,
            quats,
            trans,
            part_classification,
            base_mat,
            surfs,
            material_handles,
            lod_info,
            load_dist_auto_generated: self.load_dist_auto_generated,
            coll_surfs,
            contents: self.contents,
            bone_info,
            radius: self.radius,
            mins: self.mins.into(),
            maxs: self.maxs.into(),
            num_lods: self.num_lods,
            coll_lod: self.coll_lod,
            stream_info,
            mem_usage: self.mem_usage,
            flags: self.flags,
            bad: self.bad,
            phys_preset,
            collmaps,
            phys_constraints,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct DObjAnimMatRaw {
    pub quat: [f32; 4],
    pub trans: [f32; 3],
    pub trans_weight: f32,
}
assert_size!(DObjAnimMatRaw, 32);

#[derive(Clone, Default, Debug)]
pub struct DObjAnimMat {
    pub quat: Vec4,
    pub trans: Vec3,
    pub trans_weight: f32,
}

impl Into<DObjAnimMat> for DObjAnimMatRaw {
    fn into(self) -> DObjAnimMat {
        DObjAnimMat {
            quat: self.quat.into(),
            trans: self.trans.into(),
            trans_weight: self.trans_weight,
        }
    }
}

#[cfg(feature = "d3d9")]
pub type IDirect3DVertexBuffer9 = windows::Win32::Graphics::Direct3D9::IDirect3DVertexBuffer9;
#[cfg(not(feature = "d3d9"))]
pub type IDirect3DVertexBuffer9 = ();

#[cfg(feature = "d3d9")]
pub type IDirect3DIndexBuffer9 = windows::Win32::Graphics::Direct3D9::IDirect3DIndexBuffer9;
#[cfg(not(feature = "d3d9"))]
pub type IDirect3DIndexBuffer9 = ();

#[derive(Clone, Default, Debug, Deserialize)]
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

#[derive(Clone, Default, Debug)]
pub struct XSurface {
    pub tile_mode: u8,
    pub flags: u16,
    pub base_tri_index: usize,
    pub base_vert_index: usize,
    pub tri_indices: Vec<u16>,
    pub vert_info: XSurfaceVertexInfo,
    pub verts0: Vec<Box<GfxPackedVertex>>,
    pub vb0: Option<Box<IDirect3DVertexBuffer9>>,
    pub vert_list: Vec<Box<XRigidVertList>>,
    pub index_buffer: Option<Box<IDirect3DIndexBuffer9>>,
    pub part_bits: [i32; 5],
}

impl<'a> XFileInto<XSurface> for XSurfaceRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> XSurface {
        let vert_info = self.vert_info.xfile_into(&mut xfile);
        let verts0 = self
            .verts0
            .to_array(self.vert_count as _)
            .to_vec(&mut xfile)
            .into_iter()
            .map(|v| Box::new(v.into()))
            .collect();
        let vert_list = self
            .vert_list
            .to_array(self.vert_list_count as _)
            .xfile_into(&mut xfile)
            .into_iter()
            .map(Box::new)
            .collect();
        let tri_indices = self.tri_indices.to_array(self.tri_count as _).to_vec(xfile);

        XSurface {
            tile_mode: self.tile_mode,
            flags: self.flags,
            base_tri_index: self.base_tri_index as _,
            base_vert_index: self.base_vert_index as _,
            tri_indices,
            vert_info,
            verts0,
            vb0: None,
            vert_list,
            index_buffer: None,
            part_bits: self.part_bits,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XSurfaceVertexInfoRaw<'a> {
    pub vert_count: [i16; 4],
    pub verts_blend: Ptr32<'a, u16>,
    pub tension_data: Ptr32<'a, f32>,
}
assert_size!(XSurfaceVertexInfoRaw, 16);

#[derive(Clone, Default, Debug)]
pub struct XSurfaceVertexInfo {
    pub vert_count: [i16; 4],
    pub verts_blend: Vec<u16>,
    pub tension_data: Vec<f32>,
}

impl<'a> XFileInto<XSurfaceVertexInfo> for XSurfaceVertexInfoRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> XSurfaceVertexInfo {
        let blend_count = self.vert_count[0] as usize
            + self.vert_count[1] as usize * 3
            + self.vert_count[2] as usize * 5
            + self.vert_count[3] as usize * 7;
        let tension_count = (self.vert_count[0] as usize
            + self.vert_count[1] as usize
            + self.vert_count[2] as usize
            + self.vert_count[3] as usize)
            * 12;

        XSurfaceVertexInfo {
            vert_count: self.vert_count,
            verts_blend: self.verts_blend.to_array(blend_count).to_vec(&mut xfile),
            tension_data: self.tension_data.to_array(tension_count).to_vec(xfile),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct GfxPackedVertexRaw {
    pub xyz: [f32; 3],
    pub binormal_sign: f32,
    pub color: GfxColorRaw,
    pub tex_coord: TexCoordsRaw,
    pub normal: UnitVecRaw,
    pub tangent: UnitVecRaw,
}
assert_size!(GfxPackedVertexRaw, 32);

#[derive(Clone, Default, Debug)]
pub struct GfxPackedVertex {
    pub xyz: Vec3,
    pub binormal_sign: f32,
    pub color: GfxColorRaw,
    pub tex_coord: TexCoordsRaw,
    pub normal: UnitVecRaw,
    pub tangent: UnitVecRaw,
}

impl Into<GfxPackedVertex> for GfxPackedVertexRaw {
    fn into(self) -> GfxPackedVertex {
        GfxPackedVertex {
            xyz: self.xyz.into(),
            binormal_sign: self.binormal_sign,
            color: self.color,
            tex_coord: self.tex_coord,
            normal: self.normal,
            tangent: self.tangent,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct GfxColorRaw(pub [u8; 4]);
assert_size!(GfxColorRaw, 4);

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct TexCoordsRaw(pub u32);
assert_size!(TexCoordsRaw, 4);

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct UnitVecRaw(pub [u8; 4]);
assert_size!(UnitVecRaw, 4);

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XRigidVertListRaw<'a> {
    pub bone_offset: u16,
    pub vert_count: u16,
    pub tri_offset: u16,
    pub tri_count: u16,
    pub collision_tree: Ptr32<'a, XSurfaceCollisionTreeRaw<'a>>,
}
assert_size!(XRigidVertListRaw, 12);

#[derive(Clone, Default, Debug)]
pub struct XRigidVertList {
    pub bone_offset: usize,
    pub vert_count: usize,
    pub tri_offset: usize,
    pub tri_count: usize,
    pub collision_tree: Option<Box<XSurfaceCollisionTree>>,
}

impl<'a> XFileInto<XRigidVertList> for XRigidVertListRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> XRigidVertList {
        XRigidVertList {
            bone_offset: self.bone_offset as _,
            vert_count: self.vert_count as _,
            tri_offset: self.tri_offset as _,
            tri_count: self.tri_count as _,
            collision_tree: self.collision_tree.xfile_into(xfile),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XSurfaceCollisionTreeRaw<'a> {
    pub trans: [f32; 3],
    pub scale: [f32; 3],
    pub nodes: FatPointerCountFirstU32<'a, XSurfaceCollisionNodeRaw>,
    pub leafs: FatPointerCountFirstU32<'a, XSurfaceCollisionLeafRaw>,
}
assert_size!(XSurfaceCollisionTreeRaw, 40);

#[derive(Clone, Default, Debug)]
pub struct XSurfaceCollisionTree {
    pub trans: Vec3,
    pub scale: Vec3,
    pub nodes: Vec<XSurfaceCollisionNode>,
    pub leafs: Vec<XSurfaceCollisionLeaf>,
}

impl<'a> XFileInto<XSurfaceCollisionTree> for XSurfaceCollisionTreeRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> XSurfaceCollisionTree {
        XSurfaceCollisionTree {
            trans: self.trans.into(),
            scale: self.scale.into(),
            nodes: self
                .nodes
                .to_vec(&mut xfile)
                .into_iter()
                .map(Into::into)
                .collect(),
            leafs: self
                .leafs
                .to_vec(xfile)
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XSurfaceCollisionNodeRaw {
    pub aabb: XSurfaceCollisionAabb,
    pub child_begin_index: u16,
    pub child_count: u16,
}
assert_size!(XSurfaceCollisionNodeRaw, 16);

#[derive(Clone, Default, Debug)]
pub struct XSurfaceCollisionNode {
    pub aabb: XSurfaceCollisionAabb,
    pub child_begin_index: usize,
    pub child_count: usize,
}

impl Into<XSurfaceCollisionNode> for XSurfaceCollisionNodeRaw {
    fn into(self) -> XSurfaceCollisionNode {
        XSurfaceCollisionNode {
            aabb: self.aabb,
            child_begin_index: self.child_begin_index as _,
            child_count: self.child_count as _,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XSurfaceCollisionAabb {
    pub mins: [u16; 3],
    pub maxs: [u16; 3],
}
assert_size!(XSurfaceCollisionAabb, 12);

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XSurfaceCollisionLeafRaw {
    pub triangle_begin_index: u16,
}
assert_size!(XSurfaceCollisionLeafRaw, 2);

#[derive(Clone, Default, Debug)]
pub struct XSurfaceCollisionLeaf {
    pub triangle_begin_index: usize,
}

impl Into<XSurfaceCollisionLeaf> for XSurfaceCollisionLeafRaw {
    fn into(self) -> XSurfaceCollisionLeaf {
        XSurfaceCollisionLeaf {
            triangle_begin_index: self.triangle_begin_index as _,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
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

#[derive(Clone, Default, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Lod {
    #[default]
    Zero,
    One,
    Two,
    Three,
}

#[derive(Clone, Default, Debug)]
pub struct XModelLodInfo {
    pub dist: f32,
    pub numsurfs: usize,
    pub surf_index: usize,
    pub part_bits: [i32; 5],
    pub lod: Lod,
    pub smc_index_plus_one: usize,
    pub smc_alloc_bits: u8,
}

impl Into<XModelLodInfo> for XModelLodInfoRaw {
    fn into(self) -> XModelLodInfo {
        XModelLodInfo {
            dist: self.dist,
            numsurfs: self.numsurfs as _,
            surf_index: self.surf_index as _,
            part_bits: self.part_bits,
            lod: num::FromPrimitive::from_u8(self.lod).unwrap(),
            smc_index_plus_one: self.smc_index_plus_one as _,
            smc_alloc_bits: self.smc_alloc_bits,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XModelCollSurfRaw<'a> {
    pub coll_tris: FatPointerCountLastU32<'a, XModelCollTriRaw>,
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub bone_idx: i32,
    pub contents: i32,
    pub surf_flags: i32,
}
assert_size!(XModelCollSurfRaw, 44);

#[derive(Clone, Default, Debug)]
pub struct XModelCollSurf {
    pub coll_tris: Vec<XModelCollTri>,
    pub mins: Vec3,
    pub maxs: Vec3,
    pub bone_idx: usize,
    pub contents: i32,
    pub surf_flags: i32,
}

impl<'a> XFileInto<XModelCollSurf> for XModelCollSurfRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> XModelCollSurf {
        XModelCollSurf {
            coll_tris: self
                .coll_tris
                .to_vec(xfile)
                .into_iter()
                .map(Into::into)
                .collect(),
            mins: self.mins.into(),
            maxs: self.maxs.into(),
            bone_idx: self.bone_idx as _,
            contents: self.contents,
            surf_flags: self.surf_flags,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XModelCollTriRaw {
    pub plane: [f32; 4],
    pub svec: [f32; 4],
    pub tvec: [f32; 4],
}
assert_size!(XModelCollTriRaw, 48);

#[derive(Clone, Default, Debug)]
pub struct XModelCollTri {
    pub plane: Vec4,
    pub svec: Vec4,
    pub tvec: Vec4,
}

impl Into<XModelCollTri> for XModelCollTriRaw {
    fn into(self) -> XModelCollTri {
        XModelCollTri {
            plane: self.plane.into(),
            svec: self.svec.into(),
            tvec: self.tvec.into(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XBoneInfoRaw {
    pub bounds: [[f32; 3]; 2],
    pub offset: [f32; 3],
    pub radius_squared: f32,
    pub collmap: u8,
}
assert_size!(XBoneInfoRaw, 44);

#[derive(Clone, Default, Debug)]
pub struct XBoneInfo {
    pub bounds: [Vec3; 2],
    pub offset: Vec3,
    pub radius_squared: f32,
    pub collmap: u8,
}

impl Into<XBoneInfo> for XBoneInfoRaw {
    fn into(self) -> XBoneInfo {
        XBoneInfo {
            bounds: [self.bounds[0].into(), self.bounds[1].into()],
            offset: self.offset.into(),
            radius_squared: self.radius_squared,
            collmap: self.collmap,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XModelStreamInfoRaw<'a> {
    pub high_mip_bounds: Ptr32<'a, XModelHighMipBoundsRaw>,
}
assert_size!(XModelStreamInfoRaw, 4);

#[derive(Clone, Default, Debug)]
pub struct XModelStreamInfo {
    pub high_mip_bounds: Option<Box<XModelHighMipBounds>>,
}

impl<'a> XFileInto<XModelStreamInfo> for XModelStreamInfoRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> XModelStreamInfo {
        XModelStreamInfo {
            high_mip_bounds: self
                .high_mip_bounds
                .xfile_get(&mut xfile)
                .map(|b| Box::new((*b).into())),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XModelHighMipBoundsRaw {
    pub center: [f32; 3],
    pub himip_radius_sq: f32,
}
assert_size!(XModelHighMipBoundsRaw, 16);

#[derive(Clone, Default, Debug)]
pub struct XModelHighMipBounds {
    pub center: Vec3,
    pub himip_radius_sq: f32,
}

impl Into<XModelHighMipBounds> for XModelHighMipBoundsRaw {
    fn into(self) -> XModelHighMipBounds {
        XModelHighMipBounds {
            center: self.center.into(),
            himip_radius_sq: self.himip_radius_sq,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
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

#[derive(Clone, Default, Debug)]
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

impl<'a> XFileInto<PhysPreset> for PhysPresetRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> PhysPreset {
        PhysPreset {
            name: self.name.xfile_into(&mut xfile),
            flags: self.flags,
            mass: self.mass,
            bounce: self.bounce,
            friction: self.friction,
            bullet_force_scale: self.bullet_force_scale,
            explosive_force_scale: self.explosive_force_scale,
            snd_alias_prefix: self.snd_alias_prefix.xfile_into(&mut xfile),
            pieces_spread_fraction: self.pieces_spread_fraction,
            pieces_upward_velocity: self.pieces_upward_velocity,
            can_float: self.can_float as _,
            gravity_scale: self.gravity_scale,
            center_of_mass_offset: self.center_of_mass_offset.into(),
            buoyancy_box_min: self.buoyancy_box_min.into(),
            buoyancy_box_max: self.buoyancy_box_max.into(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct CollmapRaw<'a> {
    pub geom_list: Ptr32<'a, PhysGeomListRaw<'a>>,
}
assert_size!(CollmapRaw, 4);

#[derive(Clone, Default, Debug)]
pub struct Collmap {
    pub geom_list: Option<Box<PhysGeomList>>,
}

impl<'a> XFileInto<Collmap> for CollmapRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> Collmap {
        Collmap {
            geom_list: self.geom_list.xfile_into(xfile),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct PhysGeomListRaw<'a> {
    pub geoms: FatPointerCountFirstU32<'a, PhysGeomInfoRaw<'a>>,
    pub contents: i32,
}
assert_size!(PhysGeomListRaw, 12);

#[derive(Clone, Default, Debug)]
pub struct PhysGeomList {
    pub geoms: Vec<PhysGeomInfo>,
    pub contents: i32,
}

impl<'a> XFileInto<PhysGeomList> for PhysGeomListRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> PhysGeomList {
        PhysGeomList {
            geoms: self.geoms.xfile_into(xfile),
            contents: self.contents,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct PhysGeomInfoRaw<'a> {
    pub brush: Ptr32<'a, BrushWrapperRaw<'a>>,
    pub type_: i32,
    pub orientation: [[f32; 3]; 3],
    pub offset: [f32; 3],
    pub half_lengths: [f32; 3],
}
assert_size!(PhysGeomInfoRaw, 68);

#[derive(Clone, Default, Debug, FromPrimitive)]
#[repr(i32)]
pub enum PhysGeomType {
    #[default]
    BOX = 0x01,
    BRUSH = 0x02,
    CYLINDER = 0x03,
}

#[derive(Clone, Default, Debug)]
pub struct PhysGeomInfo {
    pub brush: Option<Box<BrushWrapper>>,
    pub type_: PhysGeomType,
    pub orientation: Mat3,
    pub offset: Vec3,
    pub half_lengths: Vec3,
}

impl<'a> XFileInto<PhysGeomInfo> for PhysGeomInfoRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> PhysGeomInfo {
        PhysGeomInfo {
            brush: self.brush.xfile_into(xfile),
            type_: num::FromPrimitive::from_i32(self.type_).unwrap(),
            orientation: self.orientation.into(),
            offset: self.offset.into(),
            half_lengths: self.half_lengths.into(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
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

#[derive(Clone, Default, Debug)]
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

impl<'a> XFileInto<BrushWrapper> for BrushWrapperRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> BrushWrapper {
        BrushWrapper {
            mins: self.mins.into(),
            contents: self.contents,
            maxs: self.maxs.into(),
            sides: self.sides.xfile_into(&mut xfile),
            axial_cflags: self.axial_cflags,
            axial_sflags: self.axial_sflags,
            verts: self
                .verts
                .to_vec(&mut xfile)
                .into_iter()
                .map(Into::into)
                .collect(),
            planes: self
                .planes
                .to_array(self.sides.size as _)
                .to_vec(xfile)
                .into_iter()
                .map(|p| Some(Box::new(p.into())))
                .collect(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct CBrushSideRaw<'a> {
    pub plane: Ptr32<'a, CPlaneRaw>,
    pub cflags: i32,
    pub sflags: i32,
}
assert_size!(CBrushSideRaw, 12);

#[derive(Clone, Default, Debug)]
pub struct CBrushSide {
    pub plane: Option<Box<CPlane>>,
    pub cflags: i32,
    pub sflags: i32,
}

impl<'a> XFileInto<CBrushSide> for CBrushSideRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> CBrushSide {
        CBrushSide {
            plane: self.plane.xfile_get(xfile).map(|p| Box::new((*p).into())),
            cflags: self.cflags,
            sflags: self.sflags,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct CPlaneRaw {
    pub normal: [f32; 3],
    pub dist: f32,
    pub type_: u8,
    pub signbits: u8,
    pad: [u8; 2],
}
assert_size!(CPlaneRaw, 20);

#[derive(Clone, Default, Debug)]
pub struct CPlaneType(u8);

impl CPlaneType {
    fn new(value: u8) -> Self {
        Self(value)
    }

    fn get(self) -> u8 {
        self.0
    }

    fn is_axial(self) -> bool {
        self.0 < 3
    }
}

#[derive(Clone, Default, Debug)]
pub struct CPlaneSignbits(u8);

#[derive(Clone, Default, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Sign {
    #[default]
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

#[derive(Clone, Default, Debug)]
pub struct CPlane {
    pub normal: Vec3,
    pub dist: f32,
    pub type_: CPlaneType,
    pub signbits: CPlaneSignbits,
}

impl Into<CPlane> for CPlaneRaw {
    fn into(self) -> CPlane {
        CPlane {
            normal: self.normal.into(),
            dist: self.dist,
            type_: CPlaneType::new(self.type_),
            signbits: CPlaneSignbits::from_bits(self.signbits),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct PhysConstraintsRaw<'a> {
    name: XString<'a>,
    count: u32,
    data: [PhysConstraintRaw<'a>; 16],
}
assert_size!(PhysConstraintsRaw, 2696);

#[derive(Clone, Default, Debug)]
pub struct PhysConstraints {
    name: String,
    count: usize,
    data: Vec<PhysConstraint>,
}

impl<'a> XFileInto<PhysConstraints> for PhysConstraintsRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> PhysConstraints {
        PhysConstraints {
            name: self.name.xfile_into(&mut xfile),
            count: self.count as _,
            data: self.data.iter().map(|r| r.xfile_into(&mut xfile)).collect(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct PhysConstraintRaw<'a> {
    pub targetname: ScriptString,
    pad: u16,
    pub type_: i32,
    pub attach_point_type1: i32,
    pub target_index1: i32,
    pub target_ent1: ScriptString,
    pad_2: u16,
    pub target_bone1: XString<'a>,
    pub attach_point_type2: i32,
    pub target_index2: i32,
    pub target_ent2: ScriptString,
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

#[derive(Clone, Default, Debug, FromPrimitive)]
#[repr(i32)]
pub enum ConstraintType {
    #[default]
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

#[derive(Clone, Default, Debug, FromPrimitive)]
#[repr(i32)]
pub enum AttachPointType {
    #[default]
    WORLD = 0x00,
    DYNENT = 0x01,
    ENT = 0x02,
    BONE = 0x03,
}

#[derive(Clone, Default, Debug)]
pub struct PhysConstraint {
    pub targetname: String,
    pub type_: ConstraintType,
    pub attach_point_type1: AttachPointType,
    pub target_index1: usize,
    pub target_ent1: String,
    pub target_bone1: String,
    pub attach_point_type2: AttachPointType,
    pub target_index2: usize,
    pub target_ent2: String,
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
    pub rope_index: usize,
    pub centity_num: [i32; 4],
}

impl<'a> XFileInto<PhysConstraint> for PhysConstraintRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> PhysConstraint {
        PhysConstraint {
            targetname: self.targetname.to_string(),
            type_: num::FromPrimitive::from_i32(self.type_).unwrap(),
            attach_point_type1: num::FromPrimitive::from_i32(self.attach_point_type1).unwrap(),
            target_index1: self.target_index1 as _,
            target_ent1: self.target_ent1.to_string(),
            target_bone1: self.target_bone1.xfile_into(&mut xfile),
            attach_point_type2: num::FromPrimitive::from_i32(self.attach_point_type2).unwrap(),
            target_index2: self.target_index2 as _,
            target_ent2: self.target_ent2.to_string(),
            target_bone2: self.target_bone2.xfile_into(&mut xfile),
            offset: self.offset.into(),
            pos: self.pos.into(),
            pos2: self.pos2.into(),
            dir: self.dir.into(),
            flags: self.flags as _,
            timeout: self.timeout as _,
            min_health: self.min_health as _,
            max_health: self.max_health as _,
            distance: self.distance,
            damp: self.damp,
            power: self.power,
            scale: self.scale.into(),
            spin_scale: self.spin_scale,
            min_angle: self.spin_scale,
            max_angle: self.spin_scale,
            material: self.material.xfile_into(xfile),
            constraint_handle: self.constraint_handle,
            rope_index: self.rope_index as _,
            centity_num: self.centity_num,
        }
    }
}
