use crate::*;

pub struct GameWorldSpRaw<'a> {
    pub name: XString<'a>,
    pub path: PathDataRaw<'a>,
}

pub struct GameWorldMpRaw<'a> {
    pub name: XString<'a>,
    pub path: PathDataRaw<'a>,
}

pub struct PathDataRaw<'a> {
    pub node_count: u32,
    pub nodes: Ptr32<'a, PathNodeRaw<'a>>,
    pub basenodes: Ptr32<'a, PathBaseNodeRaw>,
    pub chain_node_count: u32,
    pub chain_node_for_node: Ptr32<'a, u16>,
    pub node_for_chain_node: Ptr32<'a, u16>,
    pub path_vis: FatPointerCountFirstU32<'a, u8>,
    pub node_tree: FatPointerCountFirstU32<'a, PathNodeTreeRaw<'a>>
}

pub struct PathNodeRaw<'a> {
    pub constant: PathNodeConstantRaw<'a>,
    pub dynamic: PathNodeDynamicRaw,
    pub transient: PathNodeTransientRaw<'a>
}

#[derive(Clone, Debug, Deserialize)]
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
    pub links: FatPointerCountFirstU16<'a, PathLink>
}
assert_size!(PathNodeConstantRaw, 68);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PathLink {
    pub dist: f32,
    pub node_num: u16,
    pub disconnect_count: u8,
    pub negotiation_link: u8,
    pub bad_place_count: [u8; 4],
}
assert_size!(PathLink, 12);

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

pub struct SentientHandleRaw {
    pub number: i16,
    pub info_index: i16
}
assert_size!(SentientHandleRaw, 4);

pub struct PathNodeTransientRaw<'a> {
    pub search_frame: i32,
    pub next_open: Ptr32<'a, PathNodeRaw<'a>>,
    pub prev_open: Ptr32<'a, PathNodeRaw<'a>>,
    pub parent: Ptr32<'a, PathNodeRaw<'a>>,
    pub cost: f32,
    pub heuristic: f32,
    pub cost_factor: f32
}
assert_size!(PathNodeTransientRaw, 28);

pub struct PathBaseNodeRaw {
    pub origin: [f32; 3],
    pub type_: u32,
}
assert_size!(PathBaseNodeRaw, 16);

pub struct PathNodeTreeRaw<'a> {
    pub axis: i32,
    pub dist: f32,
    pub u: Ptr32<'a, ()>
}
assert_size!(PathNodeTreeRaw, 12);

pub struct PathNodeTreeNodesRaw<'a> {
    pub nodes: FatPointerCountFirstU32<'a, u16>,
}
assert_size!(PathNodeTreeNodesRaw, 8);