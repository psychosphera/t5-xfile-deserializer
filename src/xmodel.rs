use bitflags::bitflags;
use num::FromPrimitive;

use crate::{common::*, *};

// 0x6638D (418701)
// 0x65E41 (417345)
// ----------------
//   0x54C   (1356)

// 0x6638D (418701)
// 0x65E75 (417397)
//   0x518    (518)

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XModelRaw<'a> {
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
    pub lod_info: [XModelLodInfoRaw; MAX_LODS],
    pub load_dist_auto_generated: u8,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pad_2: [u8; 3],
    pub phys_preset: Ptr32<'a, PhysPresetRaw<'a>>,
    pub collmaps: FatPointerCountFirstU32<'a, CollmapRaw<'a>>,
    pub phys_constraints: Ptr32<'a, PhysConstraintsRaw<'a>>,
}
assert_size!(XModelRaw, 252);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug, FromPrimitive)]
#[repr(u8)]
pub enum XModelLodRampType {
    #[default]
    RIGID = 0x00,
    SKINNED = 0x01,
    COUNT = 0x02,
}

pub const MAX_LODS: usize = 4;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XModel {
    pub name: String,
    pub num_bones: usize,
    pub num_root_bones: usize,
    pub numsurfs: usize,
    pub lod_ramp_type: XModelLodRampType,
    pub bone_names: Vec<String>,
    pub parent_list: Vec<u8>,
    pub quats: Vec<i16>,
    pub trans: Vec<f32>,
    pub part_classification: Vec<u8>,
    pub base_mat: Vec<DObjAnimMat>,
    pub surfs: Vec<XSurface>,
    pub material_handles: Vec<Box<techset::Material>>,
    pub lod_info: [XModelLodInfo; MAX_LODS],
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
    pub collmaps: Vec<Collmap>,
    pub phys_constraints: Option<Box<PhysConstraints>>,
}

impl<'a> XFileInto<XModel, ()> for XModelRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<XModel> {
        dbg!(self);
        //dbg!(xfile.stream_position()?);

        let name = self.name.xfile_into(de, ())?;
        dbg!(&name);
        //dbg!(xfile.stream_position()?);

        if self.num_bones < self.num_root_bones {
            return Err(Error::BrokenInvariant(format!(
                "{}: XModel: num_bones ({}) < num_root_bones ({})",
                file_line_col!(),
                self.num_bones,
                self.num_root_bones
            )));
        }

        if self.lod_ramp_type >= XModelLodRampType::COUNT as u8 {
            return Err(Error::BrokenInvariant(format!(
                "{}: XModel: lod_ramp_type ({}) >= XModelLodRampType::COUNT",
                file_line_col!(),
                self.lod_ramp_type
            )));
        }

        let lod_ramp_type = XModelLodRampType::from_u8(self.lod_ramp_type)
            .ok_or(Error::BadFromPrimitive(self.lod_ramp_type as _))?;

        let bone_names = self
            .bone_names
            .to_array(self.num_bones as _)
            .to_vec(de)?
            .into_iter()
            .map(|s| s.to_string(de).unwrap_or_default())
            .collect();
        dbg!(&bone_names);
        //dbg!(xfile.stream_position()?);
        let parent_list = self
            .parent_list
            .to_array(self.num_bones as usize - self.num_root_bones as usize)
            .to_vec(de)?;
        //dbg!(&parent_list);
        //dbg!(xfile.stream_position()?);
        let quats = self
            .quats
            .to_array((self.num_bones as usize - self.num_root_bones as usize) * 4)
            .to_vec(de)?;
        //dbg!(&quats);
        //dbg!(xfile.stream_position()?);
        let trans = self
            .trans
            .to_array((self.num_bones as usize - self.num_root_bones as usize) * 4)
            .to_vec(de)?;
        //dbg!(&trans);
        //dbg!(xfile.stream_position()?);
        let part_classification = self
            .part_classification
            .to_array(self.num_bones as _)
            .to_vec(de)?;
        //dbg!(&part_classification);
        //dbg!(xfile.stream_position()?);
        let base_mat = self
            .base_mat
            .to_array(self.num_bones as _)
            .to_vec_into(de)?;
        //dbg!(&base_mat);
        //dbg!(xfile.stream_position()?);
        let surfs = self.surfs.to_array(self.numsurfs as _).xfile_into(de, ())?;
        //dbg!(&surfs);
        //dbg!(xfile.stream_position()?);
        let material_handles = self
            .material_handles
            .to_array(self.numsurfs as _)
            .xfile_into(de, ())?
            .into_iter()
            .flatten()
            .collect();
        dbg!(&material_handles);
        //dbg!(xfile.stream_position()?);
        let lod_info = [
            self.lod_info[0].try_into()?,
            self.lod_info[1].try_into()?,
            self.lod_info[2].try_into()?,
            self.lod_info[3].try_into()?,
        ];
        let coll_surfs = self.coll_surfs.xfile_into(de, ())?;
        //dbg!(&coll_surfs);
        //dbg!(xfile.stream_position()?);
        let bone_info = self
            .bone_info
            .to_array(self.num_bones as _)
            .to_vec_into(de)?;
        //dbg!(&bone_info);
        //dbg!(xfile.stream_position()?);

        if self.num_lods > MAX_LODS as i16 {
            return Err(Error::BrokenInvariant(format!(
                "{}: XModel: num_lods ({}) > MAX_LODS",
                file_line_col!(),
                self.num_lods
            )));
        }

        if self.coll_lod > MAX_LODS as i16 {
            return Err(Error::BrokenInvariant(format!(
                "{}: XModel: coll_lod ({}) > MAX_LODS",
                file_line_col!(),
                self.coll_lod
            )));
        }

        let stream_info = self.stream_info.xfile_into(de, self.numsurfs)?;
        //dbg!(&stream_info);
        //dbg!(xfile.stream_position()?);
        let phys_preset = self.phys_preset.xfile_into(de, ())?;
        //dbg!(&phys_preset);
        //dbg!(xfile.stream_position()?);
        let collmaps = self.collmaps.xfile_into(de, ())?;
        //dbg!(&collmaps);
        //dbg!(xfile.stream_position()?);
        let phys_constraints = self.phys_constraints.xfile_into(de, ())?;
        //dbg!(&phys_constraints);
        //dbg!(xfile.stream_position()?);

        Ok(XModel {
            name,
            num_bones: self.num_bones as _,
            num_root_bones: self.num_root_bones as _,
            numsurfs: self.numsurfs as _,
            lod_ramp_type,
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
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct DObjAnimMatRaw {
    pub quat: [f32; 4],
    pub trans: [f32; 3],
    pub trans_weight: f32,
}
assert_size!(DObjAnimMatRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct DObjAnimMat {
    pub quat: Vec4,
    pub trans: Vec3,
    pub trans_weight: f32,
}

impl From<DObjAnimMatRaw> for DObjAnimMat {
    fn from(value: DObjAnimMatRaw) -> Self {
        DObjAnimMat {
            quat: value.quat.into(),
            trans: value.trans.into(),
            trans_weight: value.trans_weight,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XSurfaceRaw<'a> {
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
    #[allow(dead_code)]
    pub vb0: Ptr32<'a, ()>,
    pub vert_list: Ptr32<'a, XRigidVertListRaw<'a>>,
    #[allow(dead_code)]
    pub index_buffer: Ptr32<'a, ()>,
    pub part_bits: [i32; 5],
}
assert_size!(XSurfaceRaw, 68);

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct XSurfaceFlags: u16 {
        const SKINNED  = 0x02;
        const DEFORMED = 0x80;
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XSurface {
    pub tile_mode: u8,
    pub flags: XSurfaceFlags,
    pub base_tri_index: usize,
    pub base_vert_index: usize,
    pub tri_indices: Vec<u16>,
    pub vert_info: XSurfaceVertexInfo,
    pub verts0: Vec<GfxPackedVertex>,
    pub vb0: Option<Box<GfxVertexBuffer>>,
    pub vert_list: Vec<XRigidVertList>,
    pub index_buffer: Option<Box<GfxIndexBuffer>>,
    pub part_bits: [i32; 5],
}

impl<'a> XFileInto<XSurface, ()> for XSurfaceRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<XSurface> {
        dbg!(self);

        let flags =
            XSurfaceFlags::from_bits(self.flags).ok_or(Error::BadBitflags(self.flags as _))?;
        let vert_info = self.vert_info.xfile_into(de, ())?;
        let verts0 = self.verts0.to_array(self.vert_count as _).to_vec_into(de)?;
        let vert_list = self
            .vert_list
            .to_array(self.vert_list_count as _)
            .xfile_into(de, ())?;
        let tri_indices = self
            .tri_indices
            .to_array(self.tri_count as usize * 3)
            .to_vec(de)?;

        Ok(XSurface {
            tile_mode: self.tile_mode,
            flags,
            base_tri_index: self.base_tri_index as _,
            base_vert_index: self.base_vert_index as _,
            tri_indices,
            vert_info,
            verts0,
            vb0: None,
            vert_list,
            index_buffer: None,
            part_bits: self.part_bits,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XSurfaceVertexInfoRaw<'a> {
    pub vert_count: [i16; 4],
    pub verts_blend: Ptr32<'a, u16>,
    pub tension_data: Ptr32<'a, f32>,
}
assert_size!(XSurfaceVertexInfoRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XSurfaceVertexInfo {
    pub vert_count: [i16; 4],
    pub verts_blend: Vec<u16>,
    pub tension_data: Vec<f32>,
}

impl<'a> XFileInto<XSurfaceVertexInfo, ()> for XSurfaceVertexInfoRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<XSurfaceVertexInfo> {
        let blend_count = self.vert_count[0] as usize
            + self.vert_count[1] as usize * 3
            + self.vert_count[2] as usize * 5
            + self.vert_count[3] as usize * 7;
        let tension_count = (self.vert_count[0] as usize
            + self.vert_count[1] as usize
            + self.vert_count[2] as usize
            + self.vert_count[3] as usize)
            * 12;

        Ok(XSurfaceVertexInfo {
            vert_count: self.vert_count,
            verts_blend: self.verts_blend.to_array(blend_count).to_vec(de)?,
            tension_data: self.tension_data.to_array(tension_count).to_vec(de)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GfxPackedVertexRaw {
    pub xyz: [f32; 3],
    pub binormal_sign: f32,
    pub color: GfxColor,
    pub tex_coord: TexCoords,
    pub normal: UnitVec,
    pub tangent: UnitVec,
}
assert_size!(GfxPackedVertexRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct GfxPackedVertex {
    pub xyz: Vec3,
    pub binormal_sign: f32,
    pub color: GfxColor,
    pub tex_coord: TexCoords,
    pub normal: UnitVec,
    pub tangent: UnitVec,
}

impl From<GfxPackedVertexRaw> for GfxPackedVertex {
    fn from(value: GfxPackedVertexRaw) -> Self {
        Self {
            xyz: value.xyz.into(),
            binormal_sign: value.binormal_sign,
            color: value.color,
            tex_coord: value.tex_coord,
            normal: value.normal,
            tangent: value.tangent,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct GfxColor(pub [u8; 4]);
assert_size!(GfxColor, 4);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct TexCoords(pub u32);
assert_size!(TexCoords, 4);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct UnitVec(pub [u8; 4]);
assert_size!(UnitVec, 4);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XRigidVertListRaw<'a> {
    pub bone_offset: u16,
    pub vert_count: u16,
    pub tri_offset: u16,
    pub tri_count: u16,
    pub collision_tree: Ptr32<'a, XSurfaceCollisionTreeRaw<'a>>,
}
assert_size!(XRigidVertListRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XRigidVertList {
    pub bone_offset: usize,
    pub vert_count: usize,
    pub tri_offset: usize,
    pub tri_count: usize,
    pub collision_tree: Option<Box<XSurfaceCollisionTree>>,
}

impl<'a> XFileInto<XRigidVertList, ()> for XRigidVertListRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<XRigidVertList> {
        //dbg!(&self);

        Ok(XRigidVertList {
            bone_offset: self.bone_offset as _,
            vert_count: self.vert_count as _,
            tri_offset: self.tri_offset as _,
            tri_count: self.tri_count as _,
            collision_tree: self.collision_tree.xfile_into(de, ())?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XSurfaceCollisionTreeRaw<'a> {
    pub trans: [f32; 3],
    pub scale: [f32; 3],
    pub nodes: FatPointerCountFirstU32<'a, XSurfaceCollisionNodeRaw>,
    pub leafs: FatPointerCountFirstU32<'a, XSurfaceCollisionLeafRaw>,
}
assert_size!(XSurfaceCollisionTreeRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XSurfaceCollisionTree {
    pub trans: Vec3,
    pub scale: Vec3,
    pub nodes: Vec<XSurfaceCollisionNode>,
    pub leafs: Vec<XSurfaceCollisionLeaf>,
}

impl<'a> XFileInto<XSurfaceCollisionTree, ()> for XSurfaceCollisionTreeRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<XSurfaceCollisionTree> {
        //dbg!(&self);

        Ok(XSurfaceCollisionTree {
            trans: self.trans.into(),
            scale: self.scale.into(),
            nodes: self.nodes.to_vec_into(de)?,
            leafs: self.leafs.to_vec_into(de)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XSurfaceCollisionNodeRaw {
    pub aabb: XSurfaceCollisionAabb,
    pub child_begin_index: u16,
    pub child_count: u16,
}
assert_size!(XSurfaceCollisionNodeRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XSurfaceCollisionNode {
    pub aabb: XSurfaceCollisionAabb,
    pub child_begin_index: usize,
    pub child_count: usize,
}

impl From<XSurfaceCollisionNodeRaw> for XSurfaceCollisionNode {
    fn from(value: XSurfaceCollisionNodeRaw) -> Self {
        Self {
            aabb: value.aabb,
            child_begin_index: value.child_begin_index as _,
            child_count: value.child_count as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XSurfaceCollisionAabb {
    pub mins: [u16; 3],
    pub maxs: [u16; 3],
}
assert_size!(XSurfaceCollisionAabb, 12);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XSurfaceCollisionLeafRaw {
    pub triangle_begin_index: u16,
}
assert_size!(XSurfaceCollisionLeafRaw, 2);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XSurfaceCollisionLeaf {
    pub triangle_begin_index: usize,
}

impl From<XSurfaceCollisionLeafRaw> for XSurfaceCollisionLeaf {
    fn from(value: XSurfaceCollisionLeafRaw) -> Self {
        Self {
            triangle_begin_index: value.triangle_begin_index as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XModelLodInfoRaw {
    pub dist: f32,
    pub numsurfs: u16,
    pub surf_index: u16,
    pub part_bits: [i32; 5],
    pub lod: u8,
    pub smc_index_plus_one: u8,
    pub smc_alloc_bits: u8,
    #[allow(dead_code)]
    unused: u8,
}
assert_size!(XModelLodInfoRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XModelLodInfo {
    pub dist: f32,
    pub numsurfs: usize,
    pub surf_index: usize,
    pub part_bits: [i32; 5],
    pub lod: u8,
    pub smc_index_plus_one: usize,
    pub smc_alloc_bits: u8,
}

impl TryInto<XModelLodInfo> for XModelLodInfoRaw {
    type Error = Error;
    fn try_into(self) -> Result<XModelLodInfo> {
        if self.lod > MAX_LODS as u8 {
            return Err(Error::BrokenInvariant(format!(
                "{}: XModelLodInfo: lod ({}) > MAX_LODS",
                file_line_col!(),
                self.lod
            )));
        }

        if self.smc_alloc_bits != 0 && (self.smc_alloc_bits < 4 || self.smc_alloc_bits > 9) {
            return Err(Error::BrokenInvariant(format!(
                "{}: XModelLodInfo: smc_alloc_bits ({}) != 0, 4..=9",
                file_line_col!(),
                self.smc_alloc_bits
            )));
        }

        Ok(XModelLodInfo {
            dist: self.dist,
            numsurfs: self.numsurfs as _,
            surf_index: self.surf_index as _,
            part_bits: self.part_bits,
            lod: self.lod,
            smc_index_plus_one: self.smc_index_plus_one as _,
            smc_alloc_bits: self.smc_alloc_bits,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XModelCollSurfRaw<'a> {
    pub coll_tris: FatPointerCountLastU32<'a, XModelCollTriRaw>,
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub bone_idx: i32,
    pub contents: i32,
    pub surf_flags: i32,
}
assert_size!(XModelCollSurfRaw, 44);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XModelCollSurf {
    pub coll_tris: Vec<XModelCollTri>,
    pub mins: Vec3,
    pub maxs: Vec3,
    pub bone_idx: usize,
    pub contents: i32,
    pub surf_flags: i32,
}

impl<'a> XFileInto<XModelCollSurf, ()> for XModelCollSurfRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<XModelCollSurf> {
        Ok(XModelCollSurf {
            coll_tris: self.coll_tris.to_vec_into(de)?,
            mins: self.mins.into(),
            maxs: self.maxs.into(),
            bone_idx: self.bone_idx as _,
            contents: self.contents,
            surf_flags: self.surf_flags,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XModelCollTriRaw {
    pub plane: [f32; 4],
    pub svec: [f32; 4],
    pub tvec: [f32; 4],
}
assert_size!(XModelCollTriRaw, 48);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XModelCollTri {
    pub plane: Vec4,
    pub svec: Vec4,
    pub tvec: Vec4,
}

impl From<XModelCollTriRaw> for XModelCollTri {
    fn from(value: XModelCollTriRaw) -> Self {
        Self {
            plane: value.plane.into(),
            svec: value.svec.into(),
            tvec: value.tvec.into(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XBoneInfoRaw {
    pub bounds: [[f32; 3]; 2],
    pub offset: [f32; 3],
    pub radius_squared: f32,
    pub collmap: u8,
}
assert_size!(XBoneInfoRaw, 44);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XBoneInfo {
    pub bounds: [Vec3; 2],
    pub offset: Vec3,
    pub radius_squared: f32,
    pub collmap: u8,
}

impl From<XBoneInfoRaw> for XBoneInfo {
    fn from(value: XBoneInfoRaw) -> Self {
        Self {
            bounds: [value.bounds[0].into(), value.bounds[1].into()],
            offset: value.offset.into(),
            radius_squared: value.radius_squared,
            collmap: value.collmap,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XModelStreamInfoRaw<'a> {
    pub high_mip_bounds: Ptr32<'a, XModelHighMipBoundsRaw>,
}
assert_size!(XModelStreamInfoRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XModelStreamInfo {
    pub high_mip_bounds: Vec<XModelHighMipBounds>,
}

impl<'a> XFileInto<XModelStreamInfo, u8> for XModelStreamInfoRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, numsurfs: u8) -> Result<XModelStreamInfo> {
        Ok(XModelStreamInfo {
            high_mip_bounds: self
                .high_mip_bounds
                .to_array(numsurfs as _)
                .to_vec_into(de)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct XModelHighMipBoundsRaw {
    pub center: [f32; 3],
    pub himip_radius_sq: f32,
}
assert_size!(XModelHighMipBoundsRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct XModelHighMipBounds {
    pub center: Vec3,
    pub himip_radius_sq: f32,
}

impl From<XModelHighMipBoundsRaw> for XModelHighMipBounds {
    fn from(value: XModelHighMipBoundsRaw) -> Self {
        XModelHighMipBounds {
            center: value.center.into(),
            himip_radius_sq: value.himip_radius_sq,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct PhysPresetRaw<'a> {
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub can_float: bool,
    pub gravity_scale: f32,
    pub center_of_mass_offset: Vec3,
    pub buoyancy_box_min: Vec3,
    pub buoyancy_box_max: Vec3,
}

impl<'a> XFileInto<PhysPreset, ()> for PhysPresetRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<PhysPreset> {
        if self.flags > 1 {
            return Err(Error::BrokenInvariant(format!(
                "{}: PhysPreset: flags ({}) > 1",
                file_line_col!(),
                self.flags
            )));
        }

        if self.can_float > 1 {
            return Err(Error::BrokenInvariant(format!(
                "{}: PhysPreset: can_float ({}) > 1",
                file_line_col!(),
                self.can_float
            )));
        }

        let name = self.name.xfile_into(de, ())?;
        dbg!(&name);

        Ok(PhysPreset {
            name,
            flags: self.flags,
            mass: self.mass,
            bounce: self.bounce,
            friction: self.friction,
            bullet_force_scale: self.bullet_force_scale,
            explosive_force_scale: self.explosive_force_scale,
            snd_alias_prefix: self.snd_alias_prefix.xfile_into(de, ())?,
            pieces_spread_fraction: self.pieces_spread_fraction,
            pieces_upward_velocity: self.pieces_upward_velocity,
            can_float: self.can_float != 0,
            gravity_scale: self.gravity_scale,
            center_of_mass_offset: self.center_of_mass_offset.into(),
            buoyancy_box_min: self.buoyancy_box_min.into(),
            buoyancy_box_max: self.buoyancy_box_max.into(),
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CollmapRaw<'a> {
    pub geom_list: Ptr32<'a, PhysGeomListRaw<'a>>,
}
assert_size!(CollmapRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct Collmap {
    pub geom_list: Option<Box<PhysGeomList>>,
}

impl<'a> XFileInto<Collmap, ()> for CollmapRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<Collmap> {
        Ok(Collmap {
            geom_list: self.geom_list.xfile_into(de, ())?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct PhysGeomListRaw<'a> {
    pub geoms: FatPointerCountFirstU32<'a, PhysGeomInfoRaw<'a>>,
    pub contents: i32,
}
assert_size!(PhysGeomListRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct PhysGeomList {
    pub geoms: Vec<PhysGeomInfo>,
    pub contents: i32,
}

impl<'a> XFileInto<PhysGeomList, ()> for PhysGeomListRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<PhysGeomList> {
        Ok(PhysGeomList {
            geoms: self.geoms.xfile_into(de, ())?,
            contents: self.contents,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct PhysGeomInfoRaw<'a> {
    pub brush: Ptr32<'a, BrushWrapperRaw<'a>>,
    pub type_: i32,
    pub orientation: [[f32; 3]; 3],
    pub offset: [f32; 3],
    pub half_lengths: [f32; 3],
}
assert_size!(PhysGeomInfoRaw, 68);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug, FromPrimitive)]
#[repr(i32)]
pub enum PhysGeomType {
    #[default]
    BOX = 0x01,
    BRUSH = 0x02,
    CYLINDER = 0x03,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct PhysGeomInfo {
    pub brush: Option<Box<BrushWrapper>>,
    pub type_: PhysGeomType,
    pub orientation: Mat3,
    pub offset: Vec3,
    pub half_lengths: Vec3,
}

impl<'a> XFileInto<PhysGeomInfo, ()> for PhysGeomInfoRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<PhysGeomInfo> {
        Ok(PhysGeomInfo {
            brush: self.brush.xfile_into(de, ())?,
            type_: num::FromPrimitive::from_i32(self.type_)
                .ok_or(Error::BadFromPrimitive(self.type_ as _))?,
            orientation: self.orientation.into(),
            offset: self.offset.into(),
            half_lengths: self.half_lengths.into(),
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct BrushWrapperRaw<'a> {
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct BrushWrapper {
    pub mins: Vec3,
    pub contents: i32,
    pub maxs: Vec3,
    pub sides: Vec<CBrushSide>,
    pub axial_cflags: [[i32; 3]; 2],
    pub axial_sflags: [[i32; 3]; 2],
    pub verts: Vec<Vec3>,
    pub planes: Vec<CPlane>,
}

impl<'a> XFileInto<BrushWrapper, ()> for BrushWrapperRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<BrushWrapper> {
        Ok(BrushWrapper {
            mins: self.mins.into(),
            contents: self.contents,
            maxs: self.maxs.into(),
            sides: self.sides.xfile_into(de, ())?,
            axial_cflags: self.axial_cflags,
            axial_sflags: self.axial_sflags,
            verts: self.verts.to_vec_into(de)?,
            planes: self
                .planes
                .to_array(self.sides.size() as _)
                .to_vec_into(de)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CBrushSideRaw<'a> {
    pub plane: Ptr32<'a, CPlaneRaw>,
    pub cflags: i32,
    pub sflags: i32,
}
assert_size!(CBrushSideRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct CBrushSide {
    pub plane: Option<Box<CPlane>>,
    pub cflags: i32,
    pub sflags: i32,
}

impl<'a> XFileInto<CBrushSide, ()> for CBrushSideRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<CBrushSide> {
        Ok(CBrushSide {
            plane: self.plane.xfile_get(de)?.map(Into::into).map(Box::new),
            cflags: self.cflags,
            sflags: self.sflags,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CPlaneRaw {
    pub normal: [f32; 3],
    pub dist: f32,
    pub type_: u8,
    pub signbits: u8,
    #[allow(dead_code)]
    pad: [u8; 2],
}
assert_size!(CPlaneRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct CPlaneType(u8);

impl CPlaneType {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn get(self) -> u8 {
        self.0
    }

    pub fn is_axial(self) -> bool {
        self.0 < 3
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct CPlaneSignbits(u8);

#[cfg_attr(feature = "serde", derive(Serialize))]
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
        Self((x as u8 + (y as u8)) << ((1 + (z as u8)) << 2))
    }

    pub fn x(self) -> Sign {
        Sign::from_bool(self.bits() & 0x01 != 0)
    }

    pub fn y(self) -> Sign {
        Sign::from_bool(self.bits() & 0x02 != 0)
    }

    pub fn z(self) -> Sign {
        Sign::from_bool(self.bits() & 0x04 != 0)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct CPlane {
    pub normal: Vec3,
    pub dist: f32,
    pub type_: CPlaneType,
    pub signbits: CPlaneSignbits,
}

impl From<CPlaneRaw> for CPlane {
    fn from(value: CPlaneRaw) -> Self {
        Self {
            normal: value.normal.into(),
            dist: value.dist,
            type_: CPlaneType::new(value.type_),
            signbits: CPlaneSignbits::from_bits(value.signbits),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct PhysConstraintsRaw<'a> {
    pub name: XString<'a>,
    pub count: u32,
    pub data: [PhysConstraintRaw<'a>; 16],
}
assert_size!(PhysConstraintsRaw, 2696);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct PhysConstraints {
    pub name: String,
    pub count: usize,
    pub data: Vec<PhysConstraint>,
}

impl<'a> XFileInto<PhysConstraints, ()> for PhysConstraintsRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<PhysConstraints> {
        let name = self.name.xfile_into(de, ())?;
        dbg!(&name);
        Ok(PhysConstraints {
            name,
            count: self.count as usize,
            data: self
                .data
                .iter()
                .map(|r| r.xfile_into(de, ()))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct PhysConstraintRaw<'a> {
    pub targetname: ScriptString,
    #[allow(dead_code)]
    pad: u16,
    pub type_: i32,
    pub attach_point_type1: i32,
    pub target_index1: i32,
    pub target_ent1: ScriptString,
    #[allow(dead_code)]
    pad_2: u16,
    pub target_bone1: XString<'a>,
    pub attach_point_type2: i32,
    pub target_index2: i32,
    pub target_ent2: ScriptString,
    #[allow(dead_code)]
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug, FromPrimitive)]
#[repr(i32)]
pub enum AttachPointType {
    #[default]
    WORLD = 0x00,
    DYNENT = 0x01,
    ENT = 0x02,
    BONE = 0x03,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl<'a> XFileInto<PhysConstraint, ()> for PhysConstraintRaw<'a> {
    fn xfile_into(&self, de: &mut T5XFileDeserializer, _data: ()) -> Result<PhysConstraint> {
        let targetname = self.targetname.to_string(de).unwrap_or_default();
        let target_ent1 = self.target_ent1.to_string(de).unwrap_or_default();
        let target_bone1 = self.target_bone1.xfile_into(de, ())?;
        let target_ent2 = self.target_ent2.to_string(de).unwrap_or_default();
        let target_bone2 = self.target_bone2.xfile_into(de, ())?;
        dbg!(&targetname);
        dbg!(&target_ent1);
        dbg!(&target_bone1);
        dbg!(&target_ent2);
        dbg!(&target_bone2);

        Ok(PhysConstraint {
            targetname,
            type_: num::FromPrimitive::from_i32(self.type_)
                .ok_or(Error::BadFromPrimitive(self.type_ as _))?,
            attach_point_type1: num::FromPrimitive::from_i32(self.attach_point_type1)
                .ok_or(Error::BadFromPrimitive(self.attach_point_type1 as _))?,
            target_index1: self.target_index1 as _,
            target_ent1,
            target_bone1,
            attach_point_type2: num::FromPrimitive::from_i32(self.attach_point_type2)
                .ok_or(Error::BadFromPrimitive(self.attach_point_type2 as _))?,
            target_index2: self.target_index2 as _,
            target_ent2,
            target_bone2,
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
            min_angle: self.min_angle,
            max_angle: self.max_angle,
            material: self.material.xfile_into(de, ())?,
            constraint_handle: self.constraint_handle,
            rope_index: self.rope_index as _,
            centity_num: self.centity_num,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct XModelDrawInfo {
    pub lod: u16,
    pub surf_id: u16,
}
assert_size!(XModelDrawInfo, 4);
