use core::mem::transmute;

use alloc::{boxed::Box, vec::Vec};

use crate::{
    Error, ErrorKind, FatPointer, FatPointerCountFirstU16, FatPointerCountFirstU32, Ptr32, Result,
    ScriptString, T5XFileDeserialize, T5XFileSerialize, XFileDeserializeInto, XFileSerialize,
    XString, XStringRaw, assert_size,
    common::{Vec2, Vec3},
    file_line_col,
};

use bitflags::bitflags;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GameWorldSpRaw<'a> {
    pub name: XStringRaw<'a>,
    pub path: PathDataRaw<'a>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GameWorldSp {
    pub name: XString,
    pub path: PathData,
}

impl<'a> XFileDeserializeInto<GameWorldSp, ()> for GameWorldSpRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GameWorldSp> {
        Ok(GameWorldSp {
            name: self.name.xfile_deserialize_into(de, ())?,
            path: self.path.xfile_deserialize_into(de, ())?,
        })
    }
}

impl XFileSerialize<()> for GameWorldSp {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());

        let nodes = Ptr32::from_slice(&self.path.nodes);
        let basenodes = Ptr32::from_slice(&self.path.basenodes);
        let chain_node_for_node = Ptr32::from_slice(&self.path.chain_node_for_node);
        let node_for_chain_node = Ptr32::from_slice(&self.path.node_for_chain_node);
        let path_vis = FatPointerCountFirstU32::new(
            Ptr32::from_slice(&self.path.path_vis),
            self.path.path_vis.len(),
        );
        let node_tree = FatPointerCountFirstU32::new(
            Ptr32::from_slice(&self.path.node_tree),
            self.path.node_tree.len(),
        );
        let path = PathDataRaw {
            node_count: self.path.nodes.len() as _,
            nodes,
            basenodes,
            chain_node_count: self.path.nodes.len() as _,
            chain_node_for_node,
            node_for_chain_node,
            path_vis,
            node_tree,
        };

        let gameworld = GameWorldSpRaw { name, path };

        ser.store_into_xfile(gameworld)?;
        self.name.xfile_serialize(ser, ())?;

        self.path.nodes.xfile_serialize(ser, ())?;
        self.path.basenodes.xfile_serialize(ser, ())?;
        self.path.chain_node_for_node.xfile_serialize(ser, ())?;
        self.path.node_for_chain_node.xfile_serialize(ser, ())?;
        self.path.path_vis.xfile_serialize(ser, ())?;
        self.path.node_tree.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GameWorldMpRaw<'a> {
    pub name: XStringRaw<'a>,
    pub path: PathDataRaw<'a>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GameWorldMp {
    pub name: XString,
    pub path: PathData,
}

impl<'a> XFileDeserializeInto<GameWorldMp, ()> for GameWorldMpRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GameWorldMp> {
        Ok(GameWorldMp {
            name: self.name.xfile_deserialize_into(de, ())?,
            path: self.path.xfile_deserialize_into(de, ())?,
        })
    }
}

impl XFileSerialize<()> for GameWorldMp {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());

        let nodes = Ptr32::from_slice(&self.path.nodes);
        let basenodes = Ptr32::from_slice(&self.path.basenodes);
        let chain_node_for_node = Ptr32::from_slice(&self.path.chain_node_for_node);
        let node_for_chain_node = Ptr32::from_slice(&self.path.node_for_chain_node);
        let path_vis = FatPointerCountFirstU32::new(
            Ptr32::from_slice(&self.path.path_vis),
            self.path.path_vis.len(),
        );
        let node_tree = FatPointerCountFirstU32::new(
            Ptr32::from_slice(&self.path.node_tree),
            self.path.node_tree.len(),
        );
        let path = PathDataRaw {
            node_count: self.path.nodes.len() as _,
            nodes,
            basenodes,
            chain_node_count: self.path.nodes.len() as _,
            chain_node_for_node,
            node_for_chain_node,
            path_vis,
            node_tree,
        };

        let gameworld = GameWorldMpRaw { name, path };

        ser.store_into_xfile(gameworld)?;
        self.name.xfile_serialize(ser, ())?;

        self.path.nodes.xfile_serialize(ser, ())?;
        self.path.basenodes.xfile_serialize(ser, ())?;
        self.path.chain_node_for_node.xfile_serialize(ser, ())?;
        self.path.node_for_chain_node.xfile_serialize(ser, ())?;
        self.path.path_vis.xfile_serialize(ser, ())?;
        self.path.node_tree.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PathDataRaw<'a> {
    pub node_count: u32,
    pub nodes: Ptr32<'a, PathNodeRaw<'a>>,
    pub basenodes: Ptr32<'a, PathBaseNodeRaw>,
    pub chain_node_count: u32,
    pub chain_node_for_node: Ptr32<'a, u16>,
    pub node_for_chain_node: Ptr32<'a, u16>,
    pub path_vis: FatPointerCountFirstU32<'a, u8>,
    pub node_tree: FatPointerCountFirstU32<'a, PathNodeTreeRaw>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PathData {
    pub nodes: Vec<PathNode>,
    pub basenodes: Vec<PathBaseNode>,
    pub chain_node_for_node: Vec<u16>,
    pub node_for_chain_node: Vec<u16>,
    pub path_vis: Vec<u8>,
    pub node_tree: Vec<PathNodeTree>,
}

impl<'a> XFileDeserializeInto<PathData, ()> for PathDataRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<PathData> {
        let nodes = self
            .nodes
            .to_array(self.node_count as usize + 128)
            .xfile_deserialize_into(de, ())?;
        let basenodes = self
            .basenodes
            .to_array(self.node_count as usize + 128)
            .to_vec_into(de)?;
        let chain_node_for_node = self
            .chain_node_for_node
            .to_array(self.node_count as _)
            .to_vec(de)?;
        let node_for_chain_node = self
            .node_for_chain_node
            .to_array(self.node_count as _)
            .to_vec(de)?;
        let path_vis = self.path_vis.to_vec(de)?;
        let node_tree = self.node_tree.xfile_deserialize_into(de, ())?;

        Ok(PathData {
            nodes,
            basenodes,
            chain_node_for_node,
            node_for_chain_node,
            path_vis,
            node_tree,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PathNodeRaw<'a> {
    pub constant: PathNodeConstantRaw<'a>,
    pub dynamic: PathNodeDynamicRaw,
    pub transient: PathNodeTransientRaw<'a>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PathNode {
    pub constant: PathNodeConstant,
    pub dynamic: PathNodeDynamic,
    pub transient: PathNodeTransient,
}

impl<'a> XFileDeserializeInto<PathNode, ()> for PathNodeRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<PathNode> {
        Ok(PathNode {
            constant: self.constant.xfile_deserialize_into(de, ())?,
            dynamic: self.dynamic.into(),
            transient: self.transient.into(),
        })
    }
}

impl XFileSerialize<()> for PathNode {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let links = FatPointerCountFirstU16::from_slice(&self.constant.links);
        let constant = PathNodeConstantRaw {
            type_: self.constant.type_ as _,
            spawnflags: self.constant.spawnflags.bits(),
            targetname: ser.get_or_insert_script_string(self.constant.targetname.get())?,
            script_linkname: ser
                .get_or_insert_script_string(self.constant.script_linkname.get())?,
            script_noteworthy: ser
                .get_or_insert_script_string(self.constant.script_noteworthy.get())?,
            target: ser.get_or_insert_script_string(self.constant.target.get())?,
            animscript: ser.get_or_insert_script_string(self.constant.animscript.get())?,
            animscriptfunc: self.constant.animscriptfunc,
            origin: self.constant.origin.get(),
            angle: self.constant.angle,
            forward: self.constant.forward.get(),
            radius: self.constant.radius,
            min_use_dist_sq: self.constant.min_use_dist_sq,
            overlap_node: self.constant.overlap_node,
            chain_id: self.constant.chain_id,
            chain_depth: self.constant.chain_depth,
            chain_parent: self.constant.chain_parent,
            links,
        };

        let owner = SentientHandleRaw {
            number: self.dynamic.owner.number,
            info_index: self.dynamic.owner.info_index as _,
        };

        let dynamic = PathNodeDynamicRaw {
            owner,
            free_time: self.dynamic.free_time,
            valid_time: self.dynamic.valid_time,
            in_player_los_time: self.dynamic.in_player_los_time,
            link_count: self.dynamic.link_count,
            overlap_count: self.dynamic.overlap_count,
            turret_ent_number: self.dynamic.turret_ent_number,
            user_count: self.dynamic.user_count,
        };

        let transient = PathNodeTransientRaw {
            search_frame: self.transient.search_frame,
            next_open: Ptr32::null(),
            prev_open: Ptr32::null(),
            parent: Ptr32::null(),
            cost: self.transient.cost,
            heuristic: self.transient.heuristic,
            cost_factor: self.transient.cost_factor,
        };

        let node = PathNodeRaw {
            constant,
            dynamic,
            transient,
        };

        ser.store_into_xfile(node)?;
        self.constant.links.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PathNodeConstantRaw<'a> {
    pub type_: u16,
    pub spawnflags: u16,
    pub targetname: ScriptString,
    pub script_linkname: ScriptString,
    pub script_noteworthy: ScriptString,
    pub target: ScriptString,
    pub animscript: ScriptString,
    pub animscriptfunc: i32,
    pub origin: [f32; 3],
    pub angle: f32,
    pub forward: [f32; 2],
    pub radius: f32,
    pub min_use_dist_sq: f32,
    pub overlap_node: [i16; 2],
    pub chain_id: i16,
    pub chain_depth: i16,
    pub chain_parent: u16,
    pub links: FatPointerCountFirstU16<'a, PathLink>,
}
assert_size!(PathNodeConstantRaw, 68);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug, FromPrimitive)]
pub enum NodeType {
    #[default]
    BADNODE = 0x00,
    PATHNODE = 0x01,
    COVER_STAND = 0x02,
    COVER_CROUCH = 0x03,
    COVER_CROUCH_WINDOW = 0x04,
    COVER_PRONE = 0x05,
    COVER_RIGHT = 0x06,
    COVER_LEFT = 0x07,
    COVER_WIDE_RIGHT = 0x08,
    COVER_WIDE_LEFT = 0x09,
    COVER_PILLAR = 0x0A,
    CONCEALMENT_STAND = 0x0B,
    CONCEALMENT_CROUCH = 0x0C,
    CONCEALMENT_PRONE = 0x0D,
    REACQUIRE = 0x0E,
    BALCONY = 0x0F,
    SCRIPTED = 0x10,
    NEGOTIATION_BEGIN = 0x11,
    NEGOTIATION_END = 0x12,
    TURRET = 0x13,
    GUARD = 0x14,
    NUMTYPES = 0x15,
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Clone, Debug)]
    pub struct SpawnFlags: u16 {
        const DONTLINK = 0x0001;
        const NOTCHAIN = 0x0002;
        const DONTSTAND = 0x0004;
        const DONTCROUCH = 0x0008;
        const DONTPRONE = 0x0010;
        const UNKNOWN_0020 = 0x0020;
        const PRIORITY = 0x0040;
        const UNKNOWN_0080 = 0x0080;
        const CAN_PARENT = 0x0100;
        const DISABLED = 0x0200;
        const DONTLEFT = 0x0400;
        const DONTRIGHT = 0x0800;
        const UNKNOWN_1000 = 0x1000;
        const UNKNOWN_2000 = 0x2000;
        const TEMP_LINKS = 0x4000;
        const ANGLEVALID = 0x8000;
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathLink {
    pub dist: f32,
    pub node_num: u16,
    pub disconnect_count: u8,
    pub negotiation_link: u8,
    pub bad_place_count: [u8; 4],
}
assert_size!(PathLink, 12);

impl XFileSerialize<()> for PathLink {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PathNodeConstant {
    pub type_: NodeType,
    pub spawnflags: SpawnFlags,
    pub targetname: XString,
    pub script_linkname: XString,
    pub script_noteworthy: XString,
    pub target: XString,
    pub animscript: XString,
    pub animscriptfunc: i32,
    pub origin: Vec3,
    pub angle: f32,
    pub forward: Vec2,
    pub radius: f32,
    pub min_use_dist_sq: f32,
    pub overlap_node: [i16; 2],
    pub chain_id: i16,
    pub chain_depth: i16,
    pub chain_parent: u16,
    pub links: Vec<PathLink>,
}

impl<'a> XFileDeserializeInto<PathNodeConstant, ()> for PathNodeConstantRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<PathNodeConstant> {
        Ok(PathNodeConstant {
            type_: num::FromPrimitive::from_u16(self.type_).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.type_ as _),
            ))?,
            spawnflags: SpawnFlags::from_bits(self.spawnflags).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadBitflags(self.spawnflags as _),
            ))?,
            targetname: XString(self.targetname.to_string(de).unwrap_or_default()),
            script_linkname: XString(self.script_linkname.to_string(de).unwrap_or_default()),
            script_noteworthy: XString(self.script_noteworthy.to_string(de).unwrap_or_default()),
            target: XString(self.target.to_string(de).unwrap_or_default()),
            animscript: XString(self.animscript.to_string(de).unwrap_or_default()),
            animscriptfunc: self.animscriptfunc,
            origin: self.origin.into(),
            angle: self.angle,
            forward: self.forward.into(),
            radius: self.radius,
            min_use_dist_sq: self.min_use_dist_sq,
            overlap_node: self.overlap_node,
            chain_id: self.chain_id,
            chain_depth: self.chain_depth,
            chain_parent: self.chain_parent,
            links: self.links.to_vec(de)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PathNodeDynamicRaw {
    pub owner: SentientHandleRaw,
    pub free_time: i32,
    pub valid_time: [i32; 3],
    pub in_player_los_time: i32,
    pub link_count: i16,
    pub overlap_count: i16,
    pub turret_ent_number: i16,
    pub user_count: i16,
}
assert_size!(PathNodeDynamicRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PathNodeDynamic {
    pub owner: SentientHandle,
    pub free_time: i32,
    pub valid_time: [i32; 3],
    pub in_player_los_time: i32,
    pub link_count: i16,
    pub overlap_count: i16,
    pub turret_ent_number: i16,
    pub user_count: i16,
}

impl Into<PathNodeDynamic> for PathNodeDynamicRaw {
    fn into(self) -> PathNodeDynamic {
        PathNodeDynamic {
            owner: self.owner.into(),
            free_time: self.free_time,
            valid_time: self.valid_time,
            in_player_los_time: self.in_player_los_time,
            link_count: self.link_count,
            overlap_count: self.overlap_count,
            turret_ent_number: self.turret_ent_number,
            user_count: self.user_count,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SentientHandleRaw {
    pub number: i16,
    pub info_index: i16,
}
assert_size!(SentientHandleRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SentientHandle {
    pub number: i16,
    pub info_index: usize,
}

impl Into<SentientHandle> for SentientHandleRaw {
    fn into(self) -> SentientHandle {
        SentientHandle {
            number: self.number,
            info_index: self.info_index as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PathNodeTransientRaw<'a> {
    pub search_frame: i32,
    #[allow(dead_code)]
    pub next_open: Ptr32<'a, PathNodeRaw<'a>>,
    #[allow(dead_code)]
    pub prev_open: Ptr32<'a, PathNodeRaw<'a>>,
    #[allow(dead_code)]
    pub parent: Ptr32<'a, PathNodeRaw<'a>>,
    pub cost: f32,
    pub heuristic: f32,
    pub cost_factor: f32,
}
assert_size!(PathNodeTransientRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PathNodeTransient {
    pub search_frame: i32,
    pub cost: f32,
    pub heuristic: f32,
    pub cost_factor: f32,
}

impl<'a> Into<PathNodeTransient> for PathNodeTransientRaw<'a> {
    fn into(self) -> PathNodeTransient {
        PathNodeTransient {
            search_frame: self.search_frame,
            cost: self.cost,
            heuristic: self.heuristic,
            cost_factor: self.cost_factor,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PathBaseNodeRaw {
    pub origin: [f32; 3],
    pub type_: u32,
}
assert_size!(PathBaseNodeRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PathBaseNode {
    pub origin: Vec3,
    pub type_: u32,
}

impl From<PathBaseNodeRaw> for PathBaseNode {
    fn from(value: PathBaseNodeRaw) -> Self {
        Self {
            origin: value.origin.into(),
            type_: value.type_,
        }
    }
}

impl XFileSerialize<()> for PathBaseNode {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let node = PathBaseNodeRaw {
            origin: self.origin.get(),
            type_: self.type_,
        };

        ser.store_into_xfile(node)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PathNodeTreeRaw {
    pub axis: i32,
    pub dist: f32,
    pub u: [u8; 8],
}
assert_size!(PathNodeTreeRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum PathNodeTreeInfo {
    S(PathNodeTreeNodes),
    Child((Option<Box<PathNodeTree>>, Option<Box<PathNodeTree>>)),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PathNodeTree {
    pub axis: i32,
    pub dist: f32,
    pub u: PathNodeTreeInfo,
}

impl XFileDeserializeInto<PathNodeTree, ()> for PathNodeTreeRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<PathNodeTree> {
        let u = if self.axis < 0 {
            PathNodeTreeInfo::S(
                unsafe { transmute::<_, PathNodeTreeNodesRaw>(self.u) }
                    .xfile_deserialize_into(de, ())?,
            )
        } else {
            unimplemented!()
        };

        Ok(PathNodeTree {
            axis: self.axis,
            dist: self.dist,
            u,
        })
    }
}

impl XFileSerialize<()> for PathNodeTree {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let u = match &self.u {
            PathNodeTreeInfo::S(nodes) => {
                let p =
                    FatPointerCountFirstU32::<'_, PathNodeTreeNodesRaw>::from_slice(&nodes.nodes);
                unsafe { transmute::<_, [u8; 8]>(p) }
            }
            PathNodeTreeInfo::Child((a, b)) => {
                let p: [Ptr32<'_, PathNodeTree>; 2] = [
                    Ptr32::<'_, PathNodeTree>::from_box(&a),
                    Ptr32::<'_, PathNodeTree>::from_box(&b),
                ];
                unsafe { transmute::<_, [u8; 8]>(p) }
            }
        };

        let tree = PathNodeTreeRaw {
            axis: self.axis,
            dist: self.dist,
            u,
        };

        ser.store_into_xfile(tree)?;
        match &self.u {
            PathNodeTreeInfo::S(nodes) => nodes.nodes.xfile_serialize(ser, ()),
            PathNodeTreeInfo::Child((a, b)) => {
                a.xfile_serialize(ser, ())?;
                b.xfile_serialize(ser, ())
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PathNodeTreeNodesRaw<'a> {
    pub nodes: FatPointerCountFirstU32<'a, u16>,
}
assert_size!(PathNodeTreeNodesRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PathNodeTreeNodes {
    pub nodes: Vec<u16>,
}

impl<'a> XFileDeserializeInto<PathNodeTreeNodes, ()> for PathNodeTreeNodesRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<PathNodeTreeNodes> {
        Ok(PathNodeTreeNodes {
            nodes: self.nodes.to_vec(de)?,
        })
    }
}
