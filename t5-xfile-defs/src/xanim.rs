use core::mem::transmute;

use alloc::{boxed::Box, vec::Vec};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::prelude::*;

use crate::{
    FatPointer, Ptr32, Result, ScriptStringRaw, T5XFileDeserialize, XFileDeserializeInto, XString,
    XStringRaw, assert_size, common::Vec3,
};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimPartsRaw<'a> {
    pub name: XStringRaw<'a>,
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
    pad: [u8; 3],
    pub random_data_short_count: u32,
    pub index_count: u32,
    pub framerate: f32,
    pub frequency: f32,
    pub primed_length: f32,
    pub loop_entry_time: f32,
    pub names: Ptr32<'a, ScriptStringRaw>,
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
assert_size!(XAnimPartsRaw, 104);

pub const PART_TYPE_ALL: usize = 9;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimParts {
    pub name: XString,
    pub numframes: u16,
    pub loop_: bool,
    pub delta: bool,
    pub left_hand_grip_ik: bool,
    pub streamable: bool,
    pub streamed_file_size: usize,
    pub bone_count: [u8; 10],
    pub notify_count: u8,
    pub asset_type: u8,
    pub is_default: bool,
    pub index_count: u32,
    pub framerate: f32,
    pub frequency: f32,
    pub primed_length: f32,
    pub loop_entry_time: f32,
    pub names: Vec<String>,
    pub data_byte: Vec<u8>,
    pub data_short: Vec<i16>,
    pub data_int: Vec<i32>,
    pub random_data_short: Vec<i16>,
    pub random_data_byte: Vec<u8>,
    pub random_data_int: Vec<i32>,
    pub indices: XAnimIndices,
    pub notify: Vec<XAnimNotifyInfo>,
    pub delta_part: Option<Box<XAnimDeltaPart>>,
}

impl<'a> XFileDeserializeInto<XAnimParts, ()> for XAnimPartsRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<XAnimParts> {
        //dbg!(self);
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        let names = self
            .names
            .to_array(self.bone_count[PART_TYPE_ALL] as _)
            .to_vec(de)?
            .into_iter()
            .map(|s| s.to_string(de))
            .collect::<Result<Vec<_>>>()?;
        //dbg!(&names);
        let notify = self
            .notify
            .to_array(self.notify_count as _)
            .xfile_deserialize_into(de, ())?;
        //dbg!(&notify);
        let delta_part = self.delta_part.xfile_deserialize_into(de, self.numframes)?;
        //dbg!(&delta_part);
        let data_byte = self
            .data_byte
            .to_array(self.data_byte_count as _)
            .to_vec(de)?;
        //dbg!(&data_byte.len());
        let data_short = self
            .data_short
            .to_array(self.data_short_count as _)
            .to_vec(de)?;
        //dbg!(&data_short.len());
        let data_int = self
            .data_int
            .to_array(self.data_int_count as _)
            .to_vec(de)?;
        //dbg!(&data_int.len());
        let random_data_byte = self
            .random_data_byte
            .to_array(self.random_data_byte_count as _)
            .to_vec(de)?;
        //dbg!(&random_data_byte.len());
        let random_data_short = self
            .random_data_short
            .to_array(self.random_data_short_count as _)
            .to_vec(de)?;
        //dbg!(&random_data_short.len());
        let random_data_int = self
            .random_data_int
            .to_array(self.random_data_int_count as _)
            .to_vec(de)?;
        //dbg!(&random_data_int.len());
        let indices = self
            .indices
            .xfile_deserialize_into(de, (self.numframes, self.index_count))?;
        //dbg!(&indices);

        Ok(XAnimParts {
            name,
            numframes: self.numframes,
            loop_: self.loop_,
            delta: self.delta,
            left_hand_grip_ik: self.left_hand_grip_ik,
            streamable: self.streamable,
            streamed_file_size: self.streamed_file_size as _,
            bone_count: self.bone_count,
            notify_count: self.notify_count,
            asset_type: self.asset_type,
            is_default: self.is_default,
            index_count: self.index_count,
            framerate: self.framerate,
            frequency: self.frequency,
            primed_length: self.primed_length,
            loop_entry_time: self.loop_entry_time,
            names,
            data_byte,
            data_short,
            data_int,
            random_data_short,
            random_data_byte,
            random_data_int,
            indices,
            notify,
            delta_part,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimIndicesRaw<'a>(Ptr32<'a, ()>);
assert_size!(XAnimIndicesRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAnimIndices {
    _1(Vec<u8>),
    _2(Vec<u16>),
}

impl<'a> XFileDeserializeInto<XAnimIndices, (u16, u32)> for XAnimIndicesRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        (num_frames, index_count): (u16, u32),
    ) -> Result<XAnimIndices> {
        if num_frames < 256 {
            Ok(XAnimIndices::_1(
                self.0.cast::<u8>().to_array(index_count as _).to_vec(de)?,
            ))
        } else {
            Ok(XAnimIndices::_2(
                self.0.cast::<u16>().to_array(index_count as _).to_vec(de)?,
            ))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimNotifyInfoRaw {
    pub name: ScriptStringRaw,
    pad: [u8; 2],
    pub time: f32,
}
assert_size!(XAnimNotifyInfoRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimNotifyInfo {
    pub name: XString,
    pub time: f32,
}

impl XFileDeserializeInto<XAnimNotifyInfo, ()> for XAnimNotifyInfoRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<XAnimNotifyInfo> {
        Ok(XAnimNotifyInfo {
            name: XString(self.name.to_string(de).unwrap_or_default()),
            time: self.time,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimDeltaPartRaw<'a> {
    pub trans: Ptr32<'a, XAnimPartTransRaw>,
    pub quat: Ptr32<'a, XAnimDeltaPartQuatRaw>,
}
assert_size!(XAnimDeltaPartRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimDeltaPart {
    pub trans: Option<Box<XAnimPartTrans>>,
    pub quat: Option<Box<XAnimDeltaPartQuat>>,
}

impl<'a> XFileDeserializeInto<XAnimDeltaPart, u16> for XAnimDeltaPartRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        numframes: u16,
    ) -> Result<XAnimDeltaPart> {
        //dbg!(self);
        Ok(XAnimDeltaPart {
            trans: self.trans.xfile_deserialize_into(de, numframes)?,
            quat: self.quat.xfile_deserialize_into(de, numframes)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimPartTransRaw {
    pub size: u16,
    pub small_trans: u8,
    pad: [u8; 1],
    pub u: XAnimPartTransDataRaw,
}
assert_size!(XAnimPartTransRaw, 36);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimPartTrans {
    pub size: u16,
    pub small_trans: u8,
    pub u: Option<XAnimPartTransData>,
}

impl XFileDeserializeInto<XAnimPartTrans, u16> for XAnimPartTransRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        numframes: u16,
    ) -> Result<XAnimPartTrans> {
        //dbg!(self);
        Ok(XAnimPartTrans {
            size: self.size,
            small_trans: self.small_trans,
            u: self
                .u
                .xfile_deserialize_into(de, (numframes, self.small_trans, self.size))?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimPartTransDataRaw([u8; 32]);
assert_size!(XAnimPartTransDataRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAnimPartTransData {
    Frames(XAnimPartTransFrames),
    Frame0(Vec3),
}

impl XFileDeserializeInto<Option<XAnimPartTransData>, (u16, u8, u16)> for XAnimPartTransDataRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        (numframes, small_trans, size): (u16, u8, u16),
    ) -> Result<Option<XAnimPartTransData>> {
        if size == 0 {
            Ok(Some(XAnimPartTransData::Frame0(unsafe {
                transmute::<[u8; 12], _>(self.0[..12].try_into().unwrap())
            })))
        } else {
            let frames = unsafe { transmute::<_, XAnimPartTransFramesRaw>(self.0) }
                .xfile_deserialize_into(de, (numframes, small_trans, size))?;
            Ok(Some(XAnimPartTransData::Frames(frames)))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimPartTransFramesRaw<'a> {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub frames: XAnimDynamicFramesRaw<'a>,
    pub indices: XAnimDynamicIndicesRaw<'a>,
}
assert_size!(XAnimPartTransFramesRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimPartTransFrames {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub frames: XAnimDynamicFrames,
    pub indices: XAnimDynamicIndices,
}

impl<'a> XFileDeserializeInto<XAnimPartTransFrames, (u16, u8, u16)>
    for XAnimPartTransFramesRaw<'a>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        (numframes, small_trans, size): (u16, u8, u16),
    ) -> Result<XAnimPartTransFrames> {
        //dbg!(self);
        let indices = self.indices.xfile_deserialize_into(de, (numframes, size))?;
        let frames = self
            .frames
            .xfile_deserialize_into(de, (small_trans, size))?;
        Ok(XAnimPartTransFrames {
            mins: self.mins.into(),
            maxs: self.maxs.into(),
            frames,
            indices,
        })
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimDynamicFramesRaw<'a>(Ptr32<'a, ()>);
assert_size!(XAnimDynamicFramesRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAnimDynamicFrames {
    _1(Vec<[u8; 3]>),
    _2(Vec<[u16; 3]>),
}

impl<'a> XFileDeserializeInto<XAnimDynamicFrames, (u8, u16)> for XAnimDynamicFramesRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        (small_trans, size): (u8, u16),
    ) -> Result<XAnimDynamicFrames> {
        if small_trans == 0 {
            Ok(XAnimDynamicFrames::_2(if size != 0 {
                self.0
                    .cast::<[u16; 3]>()
                    .to_array(size as usize + 1)
                    .to_vec(de)?
            } else {
                Vec::new()
            }))
        } else {
            Ok(XAnimDynamicFrames::_1(if size != 0 {
                self.0
                    .cast::<[u8; 3]>()
                    .to_array(size as usize + 1)
                    .to_vec(de)?
            } else {
                Vec::new()
            }))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimDynamicIndicesRaw<'a>(Ptr32<'a, ()>);
assert_size!(XAnimDynamicFramesRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAnimDynamicIndices {
    _1(Vec<u8>),
    _2(Vec<u16>),
}

impl<'a> XFileDeserializeInto<XAnimDynamicIndices, (u16, u16)> for XAnimDynamicIndicesRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        (numframes, size): (u16, u16),
    ) -> Result<XAnimDynamicIndices> {
        if numframes < 256 {
            Ok(XAnimDynamicIndices::_1(
                self.0.cast::<u8>().to_array(size as usize + 1).to_vec(de)?,
            ))
        } else {
            Ok(XAnimDynamicIndices::_2(
                self.0
                    .cast::<u16>()
                    .to_array(size as usize + 1)
                    .to_vec(de)?,
            ))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimDeltaPartQuatRaw {
    pub size: u16,
    pad: [u8; 2],
    pub u: XAnimDeltaPartQuatDataRaw,
}
assert_size!(XAnimDeltaPartQuatRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimDeltaPartQuat {
    pub size: u16,
    pub u: Option<XAnimDeltaPartQuatData>,
}

impl XFileDeserializeInto<XAnimDeltaPartQuat, u16> for XAnimDeltaPartQuatRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        numframes: u16,
    ) -> Result<XAnimDeltaPartQuat> {
        Ok(XAnimDeltaPartQuat {
            size: self.size,
            u: self.u.xfile_deserialize_into(de, (numframes, self.size))?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimDeltaPartQuatDataRaw([u8; 8]);
assert_size!(XAnimDeltaPartQuatDataRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAnimDeltaPartQuatData {
    Frames(XAnimDeltaPartQuatDataFrames),
    Frame0([i16; 2]),
}

impl<'a> XFileDeserializeInto<Option<XAnimDeltaPartQuatData>, (u16, u16)>
    for XAnimDeltaPartQuatDataRaw
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        (numframes, size): (u16, u16),
    ) -> Result<Option<XAnimDeltaPartQuatData>> {
        if size == 0 {
            let frames = unsafe {
                transmute::<[u8; 4], Ptr32<'a, [i16; 2]>>(self.0[0..4].try_into().unwrap())
            }
            .xfile_get(de)?
            .unwrap_or_default();
            Ok(Some(XAnimDeltaPartQuatData::Frame0(frames)))
        } else {
            Ok(Some(XAnimDeltaPartQuatData::Frames(
                unsafe { transmute::<_, XAnimDeltaPartQuatDataFramesRaw>(self.0) }
                    .xfile_deserialize_into(de, (numframes, size))?,
            )))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimDeltaPartQuatDataFramesRaw<'a> {
    pub frames: Ptr32<'a, [i16; 2]>,
    pub indices: XAnimDynamicIndicesRaw<'a>,
}
assert_size!(XAnimDeltaPartQuatDataFramesRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimDeltaPartQuatDataFrames {
    pub frames: Vec<[i16; 2]>,
    pub indices: XAnimDynamicIndices,
}

impl<'a> XFileDeserializeInto<XAnimDeltaPartQuatDataFrames, (u16, u16)>
    for XAnimDeltaPartQuatDataFramesRaw<'a>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        data: (u16, u16),
    ) -> Result<XAnimDeltaPartQuatDataFrames> {
        let indices = self.indices.xfile_deserialize_into(de, data)?;
        let frames = if data.1 != 0 {
            self.frames.to_array(data.1 as usize + 1).to_vec(de)?
        } else {
            Vec::new()
        };

        Ok(XAnimDeltaPartQuatDataFrames { frames, indices })
    }
}
