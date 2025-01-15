use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    Error, ErrorKind, FatPointer, FatPointerCountFirstU32, MapEnts, MapEntsRaw, Ptr32, Result,
    ScriptString, T5XFileDeserialize, XFileDeserializeInto, XStringRaw, assert_size,
    common::{Mat3, Vec3, Vec4},
    file_line_col,
    fx::{FxEffectDef, FxEffectDefRaw},
    techset::{Material, MaterialRaw},
    xmodel::{
        CBrushSide, CBrushSideRaw, CPlane, CPlaneRaw, PhysConstraint, PhysConstraintRaw,
        PhysPreset, PhysPresetRaw, XModel, XModelPieces, XModelPiecesRaw, XModelRaw,
    },
};

use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ClipMapRaw<'a> {
    pub name: XStringRaw<'a>,
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ClipMap {
    pub name: String,
    pub is_in_use: bool,
    pub planes: Vec<CPlane>,
    pub static_model_list: Vec<CStaticModel>,
    pub materials: Vec<DMaterial>,
    pub brushsides: Vec<CBrushSide>,
    pub nodes: Vec<CNode>,
    pub leafs: Vec<CLeaf>,
    pub leafbrush_nodes: Vec<CLeafBrushNode>,
    pub leafbrushes: Vec<u16>,
    pub leafsurfaces: Vec<u32>,
    pub verts: Vec<Vec3>,
    pub brush_verts: Vec<Vec3>,
    pub uinds: Vec<u16>,
    pub tri_count: i32,
    pub tri_indices: Vec<u16>,
    pub tri_edge_is_walkable: Vec<u8>,
    pub borders: Vec<CollisionBorder>,
    pub partitions: Vec<CollisionPartition>,
    pub aabb_trees: Vec<CollisionAabbTree>,
    pub cmodels: Vec<CModel>,
    pub brushes: Vec<CBrush>,
    pub num_clusters: i32,
    pub cluster_bytes: i32,
    pub visibility: Vec<u8>,
    pub vised: bool,
    pub map_ents: Option<Box<MapEnts>>,
    pub box_brush: Option<Box<CBrush>>,
    pub box_model: CModel,
    pub original_dyn_ent_count: u16,
    pub dyn_ent_count: [u16; 4],
    pub dyn_ent_def_list: [Vec<DynEntityDef>; 2],
    pub dyn_ent_pose_list: [Vec<DynEntityPose>; 2],
    pub dyn_ent_client_list: [Vec<DynEntityClient>; 2],
    pub dyn_ent_server_list: [Vec<DynEntityServer>; 2],
    pub dyn_ent_coll_list: [Vec<DynEntityColl>; 4],
    pub constraints: Vec<PhysConstraint>,
    pub ropes: Vec<Rope>,
    pub checksum: u32,
}

impl<'a> XFileDeserializeInto<ClipMap, ()> for ClipMapRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ClipMap> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let planes = self.planes.to_vec_into(de)?;
        let static_model_list = self.static_model_list.xfile_deserialize_into(de, ())?;
        let materials = self.materials.to_vec_into(de)?;
        let brushsides = self.brushsides.xfile_deserialize_into(de, ())?;
        let nodes = self.nodes.xfile_deserialize_into(de, ())?;
        let leafs = self.leafs.to_vec_into(de)?;
        let leafbrush_nodes = self.leafbrush_nodes.xfile_deserialize_into(de, ())?;
        let leafbrushes = self.leafbrushes.to_vec(de)?;
        let leafsurfaces = self.leafsurfaces.to_vec(de)?;
        let verts = self.verts.to_vec_into(de)?;
        let brush_verts = self.brush_verts.to_vec_into(de)?;
        let uinds = self.uinds.to_vec(de)?;
        let tri_indices = self
            .tri_indices
            .to_array(self.tri_count as usize * 3)
            .to_vec(de)?;
        let tri_edge_is_walkable = self
            .tri_edge_is_walkable
            .to_array((((self.tri_count as usize * 3) + 31) >> 5) * 4)
            .to_vec(de)?;
        let borders = self.borders.to_vec_into(de)?;
        let partitions = self.partitions.xfile_deserialize_into(de, ())?;
        let aabb_trees = self.aabb_trees.to_vec_into(de)?;
        let cmodels = self.cmodels.to_vec_into(de)?;
        let brushes = self.brushes.xfile_deserialize_into(de, ())?;
        let visibility = self
            .visibility
            .to_array(self.cluster_bytes as usize * self.num_clusters as usize)
            .to_vec(de)?;
        let map_ents = self.map_ents.xfile_deserialize_into(de, ())?;
        let box_brush = self.box_brush.xfile_deserialize_into(de, ())?;
        let box_model = self.box_model.into();
        let dyn_ent_def_list = [
            self.dyn_ent_def_list[0]
                .to_array(self.dyn_ent_count[0] as usize)
                .xfile_deserialize_into(de, ())?,
            self.dyn_ent_def_list[1]
                .to_array(self.dyn_ent_count[1] as usize)
                .xfile_deserialize_into(de, ())?,
        ];
        let dyn_ent_pose_list = [
            self.dyn_ent_pose_list[0]
                .to_array(self.dyn_ent_count[0] as usize)
                .to_vec_into(de)?,
            self.dyn_ent_pose_list[1]
                .to_array(self.dyn_ent_count[1] as usize)
                .to_vec_into(de)?,
        ];
        let dyn_ent_client_list = [
            self.dyn_ent_client_list[0]
                .to_array(self.dyn_ent_count[0] as usize)
                .to_vec(de)?,
            self.dyn_ent_client_list[1]
                .to_array(self.dyn_ent_count[1] as usize)
                .to_vec(de)?,
        ];
        let dyn_ent_server_list = [
            self.dyn_ent_server_list[0]
                .to_array(self.dyn_ent_count[0] as usize)
                .to_vec(de)?,
            self.dyn_ent_server_list[1]
                .to_array(self.dyn_ent_count[1] as usize)
                .to_vec(de)?,
        ];
        let dyn_ent_coll_list = [
            self.dyn_ent_coll_list[0]
                .to_array(self.dyn_ent_count[0] as usize)
                .to_vec_into(de)?,
            self.dyn_ent_coll_list[1]
                .to_array(self.dyn_ent_count[1] as usize)
                .to_vec_into(de)?,
            self.dyn_ent_coll_list[2]
                .to_array(self.dyn_ent_count[2] as usize)
                .to_vec_into(de)?,
            self.dyn_ent_coll_list[3]
                .to_array(self.dyn_ent_count[3] as usize)
                .to_vec_into(de)?,
        ];
        let constraints = self.constraints.xfile_deserialize_into(de, ())?;
        let ropes = self.ropes.xfile_deserialize_into(de, ())?;

        Ok(ClipMap {
            name,
            is_in_use: self.is_in_use != 0,
            planes,
            static_model_list,
            materials,
            brushsides,
            nodes,
            leafs,
            leafbrush_nodes,
            leafbrushes,
            leafsurfaces,
            verts,
            brush_verts,
            uinds,
            tri_count: self.tri_count,
            tri_indices,
            tri_edge_is_walkable,
            borders,
            partitions,
            aabb_trees,
            cmodels,
            brushes,
            num_clusters: self.num_clusters,
            cluster_bytes: self.cluster_bytes,
            visibility,
            vised: self.vised != 0,
            map_ents,
            box_brush,
            box_model,
            original_dyn_ent_count: self.original_dyn_ent_count,
            dyn_ent_count: self.dyn_ent_count,
            dyn_ent_def_list,
            dyn_ent_pose_list,
            dyn_ent_client_list,
            dyn_ent_server_list,
            dyn_ent_coll_list,
            constraints,
            ropes,
            checksum: self.checksum,
        })
    }
}

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CStaticModel {
    pub writable: CStaticModelWritable,
    pub xmodel: Option<Box<XModel>>,
    pub origin: Vec3,
    pub inv_scaled_axis: Mat3,
    pub absmin: Vec3,
    pub absmax: Vec3,
}

impl<'a> XFileDeserializeInto<CStaticModel, ()> for CStaticModelRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<CStaticModel> {
        let xmodel = self.xmodel.xfile_deserialize_into(de, ())?;
        let origin = self.origin.into();
        let inv_scaled_axis = self.inv_scaled_axis.into();
        let absmin = self.absmin.into();
        let absmax = self.absmax.into();

        Ok(CStaticModel {
            writable: self.writable,
            xmodel,
            origin,
            inv_scaled_axis,
            absmin,
            absmax,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct CStaticModelWritable {
    pub next_model_in_world_sector: u16,
}
assert_size!(CStaticModelWritable, 2);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct DMaterialName(#[serde(with = "serde_arrays")] [u8; 64]);

impl core::fmt::Display for DMaterialName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DMaterial {
    pub material: String,
    pub surface_flags: i32,
    pub content_flags: i32,
}

impl From<DMaterialRaw> for DMaterial {
    fn from(value: DMaterialRaw) -> Self {
        Self {
            material: value.material.to_string(),
            surface_flags: value.surface_flags,
            content_flags: value.content_flags,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CNodeRaw<'a> {
    pub plane: Ptr32<'a, CPlaneRaw>,
    pub children: [i16; 2],
}
assert_size!(CNodeRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CNode {
    pub plane: Option<Box<CPlane>>,
    pub children: [i16; 2],
}

impl<'a> XFileDeserializeInto<CNode, ()> for CNodeRaw<'a> {
    fn xfile_deserialize_into(&self, de: &mut impl T5XFileDeserialize, _data: ()) -> Result<CNode> {
        let plane = self.plane.xfile_get(de)?.map(Into::into).map(Box::new);

        Ok(CNode {
            plane,
            children: self.children,
        })
    }
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CLeaf {
    pub first_coll_aabb_index: usize,
    pub coll_aabb_count: usize,
    pub brush_contents: i32,
    pub terrain_contents: i32,
    pub mins: Vec3,
    pub maxs: Vec3,
    pub leaf_brush_node: i32,
    pub cluster: u16,
}

impl From<CLeafRaw> for CLeaf {
    fn from(value: CLeafRaw) -> Self {
        Self {
            first_coll_aabb_index: value.first_coll_aabb_index as _,
            coll_aabb_count: value.coll_aabb_count as _,
            brush_contents: value.brush_contents,
            terrain_contents: value.terrain_contents,
            mins: value.mins.into(),
            maxs: value.maxs.into(),
            leaf_brush_node: value.leaf_brush_node,
            cluster: value.cluster,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CLeafBrushNodeRaw<'a> {
    pub axis: u8,
    pub leaf_brush_count: i16,
    pub contents: i32,
    pub data: CLeafBrushNodeDataRaw<'a>,
}
assert_size!(CLeafBrushNodeRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CLeafBrushNode {
    pub axis: u8,
    pub leaf_brush_count: usize,
    pub contents: i32,
    pub data: Option<CLeafBrushNodeData>,
}

impl<'a> XFileDeserializeInto<CLeafBrushNode, ()> for CLeafBrushNodeRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<CLeafBrushNode> {
        let data = self
            .data
            .xfile_deserialize_into(de, self.leaf_brush_count)?;

        Ok(CLeafBrushNode {
            axis: self.axis,
            leaf_brush_count: self.leaf_brush_count as _,
            contents: self.contents,
            data,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CLeafBrushNodeDataRaw<'a>(Ptr32<'a, ()>);
assert_size!(CLeafBrushNodeDataRaw, 4);

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

impl<'a> XFileDeserializeInto<Option<CLeafBrushNodeData>, i16> for CLeafBrushNodeDataRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        leaf_brush_count: i16,
    ) -> Result<Option<CLeafBrushNodeData>> {
        if leaf_brush_count < 1 {
            Ok(None)
        } else {
            let Some(leaf) = self
                .0
                .cast::<CLeafBrushNodeLeafRaw>()
                .xfile_deserialize_into(de, leaf_brush_count)?
                .map(|l| CLeafBrushNodeData::Leaf(*l))
            else {
                return Ok(None);
            };
            Ok(Some(leaf))
        }
    }
}

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

impl<'a> XFileDeserializeInto<CLeafBrushNodeLeaf, i16> for CLeafBrushNodeLeafRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        leaf_brush_count: i16,
    ) -> Result<CLeafBrushNodeLeaf> {
        let brushes = self.brushes.to_array(leaf_brush_count as _).to_vec(de)?;

        Ok(CLeafBrushNodeLeaf { brushes })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct CLeafBrushNodeChildren {
    pub dist: f32,
    pub range: f32,
    pub child_offset: [u16; 2],
}
assert_size!(CLeafBrushNodeChildren, 12);

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CollisionBorder {
    pub dist_eq: Vec3,
    pub z_slope: f32,
    pub z_base: f32,
    pub start: f32,
    pub length: f32,
}

impl From<CollisionBorderRaw> for CollisionBorder {
    fn from(value: CollisionBorderRaw) -> Self {
        Self {
            dist_eq: value.dist_eq.into(),
            z_slope: value.z_slope,
            z_base: value.z_base,
            start: value.start,
            length: value.length,
        }
    }
}

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CollisionPartition {
    pub tri_count: u8,
    pub border_count: u8,
    pub first_tri: i32,
    pub nuinds: i32,
    pub fuind: i32,
    pub borders: Option<Box<CollisionBorder>>,
}

impl<'a> XFileDeserializeInto<CollisionPartition, ()> for CollisionPartitionRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<CollisionPartition> {
        let borders = self.borders.xfile_get(de)?.map(Into::into).map(Box::new);

        Ok(CollisionPartition {
            tri_count: self.tri_count,
            border_count: self.border_count,
            first_tri: self.first_tri,
            nuinds: self.nuinds,
            fuind: self.fuind,
            borders,
        })
    }
}

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CollisionAabbTree {
    pub origin: Vec3,
    pub material_index: usize,
    pub child_count: usize,
    pub half_size: Vec3,
    pub index: usize,
}

impl From<CollisionAabbTreeRaw> for CollisionAabbTree {
    fn from(value: CollisionAabbTreeRaw) -> Self {
        Self {
            origin: value.origin.into(),
            material_index: value.material_index as _,
            child_count: value.child_count as _,
            half_size: value.half_size.into(),
            index: value.index as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct CModelRaw {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub radius: f32,
    pub leaf: CLeafRaw,
}
assert_size!(CModelRaw, 72);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CModel {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub radius: f32,
    pub leaf: CLeaf,
}

impl From<CModelRaw> for CModel {
    fn from(value: CModelRaw) -> Self {
        Self {
            mins: value.mins.into(),
            maxs: value.maxs.into(),
            radius: value.radius,
            leaf: value.leaf.into(),
        }
    }
}

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CBrush {
    pub mins: Vec3,
    pub contents: i32,
    pub maxs: Vec3,
    pub sides: Vec<CBrushSide>,
    pub axial_cflags: [[i32; 3]; 2],
    pub axial_sflags: [[i32; 3]; 2],
    pub verts: Vec<Vec3>,
}

impl<'a> XFileDeserializeInto<CBrush, ()> for CBrushRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<CBrush> {
        let mins = self.mins.into();
        let maxs = self.maxs.into();
        let sides = self.sides.xfile_deserialize_into(de, ())?;
        let verts = self.verts.to_vec_into(de)?;

        Ok(CBrush {
            mins,
            contents: self.contents,
            maxs,
            sides,
            axial_cflags: self.axial_cflags,
            axial_sflags: self.axial_sflags,
            verts,
        })
    }
}

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
#[derive(Copy, Clone, Default, Debug, Deserialize, PartialEq, Eq, FromPrimitive)]
pub enum DynEntityType {
    #[default]
    INVALID = 0,
    CLUTTER = 1,
    DESTRUCT = 2,
    COUNT = 3,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DynEntityDef {
    pub type_: DynEntityType,
    pub pose: GfxPlacement,
    pub xmodel: Option<Box<XModel>>,
    pub destroyed_xmodel: Option<Box<XModel>>,
    pub brush_model: u16,
    pub physics_brush_model: u16,
    pub destroy_fx: Option<Box<FxEffectDef>>,
    pub destroy_sound: u32,
    pub destroy_pieces: Option<Box<XModelPieces>>,
    pub phys_preset: Option<Box<PhysPreset>>,
    pub phys_constraints: [i16; 4],
    pub health: i32,
    pub flags: i32,
    pub contents: i32,
    pub targetname: String,
    pub target: String,
}

impl<'a> XFileDeserializeInto<DynEntityDef, ()> for DynEntityDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<DynEntityDef> {
        let type_ = FromPrimitive::from_i32(self.type_).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.type_ as _),
        ))?;
        let pose = self.pose.into();
        let xmodel = self.xmodel.xfile_deserialize_into(de, ())?;
        let destroyed_xmodel = self.destroyed_xmodel.xfile_deserialize_into(de, ())?;
        let destroy_fx = self.destroy_fx.xfile_deserialize_into(de, ())?;
        let destroy_pieces = self.destroy_pieces.xfile_deserialize_into(de, ())?;
        let phys_preset = self.phys_preset.xfile_deserialize_into(de, ())?;
        let targetname = self.targetname.to_string(de)?;
        let target = self.target.to_string(de)?;

        Ok(DynEntityDef {
            type_,
            pose,
            xmodel,
            destroyed_xmodel,
            brush_model: self.brush_model,
            physics_brush_model: self.physics_brush_model,
            destroy_fx,
            destroy_sound: self.destroy_sound,
            destroy_pieces,
            phys_preset,
            phys_constraints: self.phys_constraints,
            health: self.health,
            flags: self.flags,
            contents: self.contents,
            targetname,
            target,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GfxPlacementRaw {
    pub quat: [f32; 4],
    pub origin: [f32; 3],
}
assert_size!(GfxPlacementRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxPlacement {
    pub quat: Vec4,
    pub origin: Vec3,
}

impl From<GfxPlacementRaw> for GfxPlacement {
    fn from(value: GfxPlacementRaw) -> Self {
        Self {
            quat: value.quat.into(),
            origin: value.origin.into(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct DynEntityPoseRaw {
    pub pose: GfxPlacementRaw,
    pub radius: f32,
}
assert_size!(DynEntityPoseRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DynEntityPose {
    pub pose: GfxPlacement,
    pub radius: f32,
}

impl From<DynEntityPoseRaw> for DynEntityPose {
    fn from(value: DynEntityPoseRaw) -> Self {
        Self {
            pose: value.pose.into(),
            radius: value.radius,
        }
    }
}

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DynEntityColl {
    pub sector: u16,
    pub next_ent_in_sector: u16,
    pub link_mins: Vec3,
    pub link_maxs: Vec3,
    pub contents: i32,
}

impl From<DynEntityCollRaw> for DynEntityColl {
    fn from(value: DynEntityCollRaw) -> Self {
        Self {
            sector: value.sector,
            next_ent_in_sector: value.next_ent_in_sector,
            link_mins: value.link_mins.into(),
            link_maxs: value.link_maxs.into(),
            contents: value.contents,
        }
    }
}

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Rope {
    pub m_particles: [Par; 25],
    pub m_constraints: [Constraint; 30],
    pub m_entity_anchors: [i32; 3],
    pub m_num_particles: i32,
    pub m_num_constraints: i32,
    pub m_num_entity_anchors: i32,
    pub m_num_draw_verts: i32,
    pub m_client_verts: RopeClientVerts,
    pub m_min: Vec3,
    pub m_max: Vec3,
    pub m_start: Vec3,
    pub m_end: Vec3,
    pub m_in_use: bool,
    pub m_visible: bool,
    pub m_dist_constraint: i32,
    pub m_flags: i32,
    pub m_material: Option<Box<Material>>,
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
}

impl<'a> XFileDeserializeInto<Rope, ()> for RopeRaw<'a> {
    fn xfile_deserialize_into(&self, de: &mut impl T5XFileDeserialize, _data: ()) -> Result<Rope> {
        let m_material = self.m_material.xfile_deserialize_into(de, ())?;

        Ok(Rope {
            m_particles: self.m_particles.map(Into::into),
            m_constraints: self
                .m_constraints
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?
                .try_into()
                .unwrap(),
            m_entity_anchors: self.m_entity_anchors,
            m_num_particles: self.m_num_particles,
            m_num_constraints: self.m_num_constraints,
            m_num_entity_anchors: self.m_num_entity_anchors,
            m_num_draw_verts: self.m_num_draw_verts,
            m_client_verts: self.m_client_verts.into(),
            m_min: self.m_min.into(),
            m_max: self.m_max.into(),
            m_start: self.m_start.into(),
            m_end: self.m_end.into(),
            m_in_use: self.m_in_use != 0,
            m_visible: self.m_visible != 0,
            m_dist_constraint: self.m_dist_constraint,
            m_flags: self.m_flags,
            m_material,
            m_seglen: self.m_seglen,
            m_length: self.m_length,
            m_width: self.m_width,
            m_scale: self.m_scale,
            m_force_scale: self.m_force_scale,
            m_health: self.m_health,
            m_frame: self.m_frame,
            m_stable_count: self.m_stable_count,
            m_static_rope: self.m_static_rope,
            m_lighting_handle: self.m_lighting_handle,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ParRaw {
    pub p: [f32; 3],
    pub p0: [f32; 3],
    pub p_prev: [f32; 3],
    pub flags: i32,
}
assert_size!(ParRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Par {
    pub p: Vec3,
    pub p0: Vec3,
    pub p_prev: Vec3,
    pub flags: i32,
}

impl From<ParRaw> for Par {
    fn from(value: ParRaw) -> Self {
        Self {
            p: value.p.into(),
            p0: value.p0.into(),
            p_prev: value.p_prev.into(),
            flags: value.flags,
        }
    }
}

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Constraint {
    pub p: Vec3,
    pub type_: RopeConstraint,
    pub enetiy_index: usize,
    pub bone_name_hash: i32,
    pub pi1: u8,
    pub pi2: u8,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize, PartialEq, Eq, FromPrimitive)]
pub enum RopeConstraint {
    #[default]
    PAIR = 0,
    WORLD = 1,
    DENTITY = 2,
    CENTITY = 3,
}

impl TryFrom<ConstraintRaw> for Constraint {
    type Error = crate::Error;

    fn try_from(value: ConstraintRaw) -> Result<Self> {
        Ok(Self {
            p: value.p.into(),
            type_: FromPrimitive::from_i32(value.type_).ok_or(Error::new_with_offset(
                file_line_col!(),
                0,
                ErrorKind::BadFromPrimitive(value.type_ as _),
            ))?,
            enetiy_index: value.enetiy_index as _,
            bone_name_hash: value.bone_name_hash,
            pi1: value.pi1,
            pi2: value.pi2,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct RopeClientVertsRaw {
    pub frame_verts: [RopeFrameVertsRaw; 2],
    pub frame_index: u32,
}
assert_size!(RopeClientVertsRaw, 1212);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RopeClientVerts {
    pub frame_verts: [RopeFrameVerts; 2],
    pub frame_index: usize,
}

impl From<RopeClientVertsRaw> for RopeClientVerts {
    fn from(value: RopeClientVertsRaw) -> Self {
        Self {
            frame_verts: value.frame_verts.map(Into::into),
            frame_index: value.frame_index as _,
        }
    }
}

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RopeFrameVerts {
    pub num_verts: i32,
    #[serde(with = "serde_arrays")]
    pub v: [Vec3; 50],
}

impl Default for RopeFrameVerts {
    fn default() -> Self {
        Self {
            num_verts: 0,
            v: [Vec3::default(); 50],
        }
    }
}

impl From<RopeFrameVertsRaw> for RopeFrameVerts {
    fn from(value: RopeFrameVertsRaw) -> Self {
        Self {
            num_verts: value.num_verts,
            v: value.v.map(Into::into),
        }
    }
}
