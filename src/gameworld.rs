use crate::{
    common::{Vec2, Vec3},
    *,
};

use bitflags::bitflags;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GameWorldSpRaw<'a> {
    pub name: XString<'a>,
    pub path: PathDataRaw<'a>,
}

pub struct GameWorldSp {
    pub name: String,
    pub path: PathData,
}

impl<'a> XFileInto<GameWorldSp> for GameWorldSpRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> GameWorldSp {
        GameWorldSp {
            name: self.name.xfile_into(&mut xfile),
            path: self.path.xfile_into(xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GameWorldMpRaw<'a> {
    pub name: XString<'a>,
    pub path: PathDataRaw<'a>,
}

pub struct GameWorldMp {
    pub name: String,
    pub path: PathData,
}

impl<'a> XFileInto<GameWorldMp> for GameWorldMpRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> GameWorldMp {
        GameWorldMp {
            name: self.name.xfile_into(&mut xfile),
            path: self.path.xfile_into(xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathDataRaw<'a> {
    pub node_count: u32,
    pub nodes: Ptr32<'a, PathNodeRaw<'a>>,
    pub basenodes: Ptr32<'a, PathBaseNodeRaw>,
    pub chain_node_count: u32,
    pub chain_node_for_node: Ptr32<'a, u16>,
    pub node_for_chain_node: Ptr32<'a, u16>,
    pub path_vis: FatPointerCountFirstU32<'a, u8>,
    pub node_tree: FatPointerCountFirstU32<'a, PathNodeTreeRaw>,
}

pub struct PathData {
    pub node_count: usize,
    pub nodes: Vec<PathNode>,
    pub basenodes: Vec<PathBaseNode>,
    pub chain_node_count: usize,
    pub chain_node_for_node: Vec<u16>,
    pub node_for_chain_node: Vec<u16>,
    pub path_vis: Vec<u8>,
    pub node_tree: Vec<PathNodeTree>,
}

impl<'a> XFileInto<PathData> for PathDataRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> PathData {
        let nodes = self
            .nodes
            .to_array(self.node_count as usize + 128)
            .to_vec(&mut xfile)
            .into_iter()
            .map(|n| n.xfile_into(&mut xfile))
            .collect();
        let basenodes = self
            .basenodes
            .to_array(self.node_count as usize + 128)
            .to_vec(&mut xfile)
            .into_iter()
            .map(Into::into)
            .collect();
        let chain_node_for_node = self
            .chain_node_for_node
            .to_array(self.node_count as _)
            .to_vec(&mut xfile);
        let node_for_chain_node = self
            .node_for_chain_node
            .to_array(self.node_count as _)
            .to_vec(&mut xfile);
        let path_vis = self.path_vis.to_vec(&mut xfile);
        let node_tree = self
            .node_tree
            .to_vec(&mut xfile)
            .into_iter()
            .map(|t| t.xfile_into(&mut xfile))
            .collect();

        PathData {
            node_count: self.node_count as _,
            nodes,
            basenodes,
            chain_node_count: self.chain_node_count as _,
            chain_node_for_node,
            node_for_chain_node,
            path_vis,
            node_tree,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathNodeRaw<'a> {
    pub constant: PathNodeConstantRaw<'a>,
    pub dynamic: PathNodeDynamicRaw,
    pub transient: PathNodeTransientRaw<'a>,
}

pub struct PathNode {
    pub constant: PathNodeConstant,
    pub dynamic: PathNodeDynamic,
    pub transient: PathNodeTransient,
}

impl<'a> XFileInto<PathNode> for PathNodeRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> PathNode {
        PathNode {
            constant: self.constant.xfile_into(&mut xfile),
            dynamic: self.dynamic.into(),
            transient: self.transient.into(),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathNodeConstantRaw<'a> {
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

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathLink {
    pub dist: f32,
    pub node_num: u16,
    pub disconnect_count: u8,
    pub negotiation_link: u8,
    pub bad_place_count: [u8; 4],
}
assert_size!(PathLink, 12);

pub struct PathNodeConstant {
    pub type_: NodeType,
    pub spawnflags: SpawnFlags,
    pub targetname: String,
    pub script_linkname: String,
    pub script_noteworthy: String,
    pub target: String,
    pub animscript: String,
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

impl<'a> XFileInto<PathNodeConstant> for PathNodeConstantRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> PathNodeConstant {
        PathNodeConstant {
            type_: num::FromPrimitive::from_u16(self.type_).unwrap(),
            spawnflags: SpawnFlags::from_bits(self.spawnflags).unwrap(),
            targetname: self.targetname.to_string(),
            script_linkname: self.script_linkname.to_string(),
            script_noteworthy: self.script_noteworthy.to_string(),
            target: self.target.to_string(),
            animscript: self.animscript.to_string(),
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
            links: self.links.to_vec(xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathNodeDynamicRaw {
    pub owner: SentientHandleRaw,
    pub free_time: i32,
    pub valid_time: [i32; 3],
    pub in_player_los_time: i32,
    pub link_count: i16,
    pub overlap_count: i16,
    pub turrent_ent_number: i16,
    pub user_count: i16,
}
assert_size!(PathNodeDynamicRaw, 32);

pub struct PathNodeDynamic {
    pub owner: SentientHandle,
    pub free_time: i32,
    pub valid_time: [i32; 3],
    pub in_player_los_time: i32,
    pub link_count: i16,
    pub overlap_count: i16,
    pub turrent_ent_number: i16,
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
            turrent_ent_number: self.turrent_ent_number,
            user_count: self.user_count,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct SentientHandleRaw {
    pub number: i16,
    pub info_index: i16,
}
assert_size!(SentientHandleRaw, 4);

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

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathNodeTransientRaw<'a> {
    pub search_frame: i32,
    pub next_open: Ptr32<'a, PathNodeRaw<'a>>,
    pub prev_open: Ptr32<'a, PathNodeRaw<'a>>,
    pub parent: Ptr32<'a, PathNodeRaw<'a>>,
    pub cost: f32,
    pub heuristic: f32,
    pub cost_factor: f32,
}
assert_size!(PathNodeTransientRaw, 28);

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

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathBaseNodeRaw {
    pub origin: [f32; 3],
    pub type_: u32,
}
assert_size!(PathBaseNodeRaw, 16);

pub struct PathBaseNode {
    pub origin: Vec3,
    pub type_: u32,
}

impl Into<PathBaseNode> for PathBaseNodeRaw {
    fn into(self) -> PathBaseNode {
        PathBaseNode {
            origin: self.origin.into(),
            type_: self.type_,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathNodeTreeRaw {
    pub axis: i32,
    pub dist: f32,
    pub u: [u8; 8],
}
assert_size!(PathNodeTreeRaw, 16);

pub enum PathNodeTreeInfo {
    S(PathNodeTreeNodes),
    Child((Option<Box<PathNodeTree>>, Option<Box<PathNodeTree>>)),
}

pub struct PathNodeTree {
    pub axis: i32,
    pub dist: f32,
    pub u: PathNodeTreeInfo,
}

impl<'a> XFileInto<PathNodeTree> for PathNodeTreeRaw {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> PathNodeTree {
        let u = if self.axis < 0 {
            PathNodeTreeInfo::S(
                unsafe { std::mem::transmute::<_, PathNodeTreeNodesRaw>(self.u) }.xfile_into(xfile),
            )
        } else {
            unimplemented!()
        };

        PathNodeTree {
            axis: self.axis,
            dist: self.dist,
            u,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathNodeTreeNodesRaw<'a> {
    pub nodes: FatPointerCountFirstU32<'a, u16>,
}
assert_size!(PathNodeTreeNodesRaw, 8);

pub struct PathNodeTreeNodes {
    pub nodes: Vec<u16>,
}

impl<'a> XFileInto<PathNodeTreeNodes> for PathNodeTreeNodesRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> PathNodeTreeNodes {
        PathNodeTreeNodes {
            nodes: self.nodes.to_vec(xfile),
        }
    }
}
