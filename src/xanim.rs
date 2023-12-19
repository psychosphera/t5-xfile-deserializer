use crate::{*, common::Vec3};

pub struct XAnimPartsRaw<'a> {
    pub name: XString<'a>,
    pub data_byte_count: u16,
    pub data_short_count: u16,
    pub data_int_count: u16,
    pub random_data_byte_count: u16,
    pub random_data_int_count: u16,
    pub numframes: u16,
    pub loop_: bool,
    pub delta: bool,
    pub left_hand_grip_ik: bool,
    pub streamable: bool,
    pub streamed_file_size: u32,
    pub bone_count: [u8; 10],
    pub notify_count: u8,
    pub asset_type: u8,
    pub is_default: bool,
    pub random_data_short_count: u32,
    pub index_count: u32,
    pub framerate: f32,
    pub frequency: f32,
    pub primed_length: f32,
    pub loop_entry_time: f32,
    pub names: Ptr32<'a, ScriptString>,
    pub data_byte: Ptr32<'a, u8>,
    pub data_short: Ptr32<'a, i16>,
    pub data_int: Ptr32<'a, i32>,
    pub random_data_short: Ptr32<'a, i16>,
    pub random_data_byte: Ptr32<'a, u8>,
    pub random_data_int: Ptr32<'a, i32>,
    pub indices: XAnimIndicesRaw<'a>,
    pub notify: Ptr32<'a, XAnimNotifyInfoRaw>,
    pub delta_part: Ptr32<'a, XAnimDeltaPartRaw<'a>>,
}

pub struct XAnimIndicesRaw<'a>(Ptr32<'a, ()>);

pub enum XAnimIndices {
    _1(Vec<u8>),
    _2(Vec<u16>),
    
}

pub struct XAnimNotifyInfoRaw {
    pub name: ScriptString,
    pub time: f32,
}

pub struct XAnimNotifyInfo {
    pub name: String,
    pub time: f32,
}

pub struct XAnimDeltaPartRaw<'a> {
    pub trans: Ptr32<'a, XAnimPartTransRaw<'a>>,
    pub quat: Ptr32<'a, XAnimDeltaPartQuatRaw<'a>>,
}

pub struct XAnimDeltaPart {
    pub trans: Option<Box<XAnimPartTrans>>,
    pub quat: Option<Box<XAnimDeltaPartQuat>>,
}

pub struct XAnimPartTransRaw<'a> {
    pub size: u16,
    pub small_trans: u8,
    pub u: XAnimPartTransDataRaw<'a>,
}

pub struct XAnimPartTrans {
    pub size: u16,
    pub small_trans: u8,
    pub u: XAnimPartTransData,
}

pub struct XAnimPartTransDataRaw<'a>(Ptr32<'a, ()>);

pub enum XAnimPartTransData {
    Frames(XAnimPartTransFrames),
    Frame0(Vec3),
}

pub struct XAnimPartTransFramesRaw<'a> {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub frames: XAnimDynamicFramesRaw<'a>,
    pub indices: XAnimDynamicIndicesRaw<'a>,
}

pub struct XAnimPartTransFrames {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub frames: XAnimDynamicFrames,
    pub indices: XAnimDynamicIndices,
}

pub struct XAnimDynamicFramesRaw<'a>(Ptr32<'a, ()>);

pub enum XAnimDynamicFrames {
    _1(Vec<[u8; 3]>),
    _2(Vec<[u16; 3]>),
}

pub struct XAnimDynamicIndicesRaw<'a>(Ptr32<'a, ()>);

pub enum XAnimDynamicIndices {
    _1(Vec<u8>),
    _2(Vec<u16>),
}

pub struct XAnimDeltaPartQuatRaw<'a> {
    pub size: u16,
    pub u: XAnimDeltaPartQuatDataRaw<'a>,
}

pub struct XAnimDeltaPartQuat {
    pub size: u16,
    pub u: XAnimDeltaPartQuatData,
}

pub struct XAnimDeltaPartQuatDataRaw<'a>(Ptr32<'a, ()>);

pub enum XAnimDeltaPartQuatData {
    Frames(XAnimDeltaPartQuatDataFrames),
    Frame0([i16; 2]),
}

pub struct XAnimDeltaPartQuatDataFramesRaw<'a> {
    pub frames: Ptr32<'a, [i16; 2]>,
    pub indices: XAnimDynamicIndicesRaw<'a>,
}

pub struct XAnimDeltaPartQuatDataFrames {
    pub frames: Vec<[u16; 2]>,
    pub indices: XAnimDynamicIndices,
}