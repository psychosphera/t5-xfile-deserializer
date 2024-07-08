use crate::{common::Vec3, *};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimPartsRaw<'a> {
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
assert_size!(XAnimPartsRaw, 104);

pub const PART_TYPE_ALL: usize = 9;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimParts {
    pub name: String,
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

impl<'a> XFileInto<XAnimParts, ()> for XAnimPartsRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<XAnimParts> {
        let name = self.name.xfile_into(&mut xfile, ())?;
        let notify = self
            .notify
            .to_array(self.notify_count as _)
            .to_vec_into(&mut xfile)?;
        let delta_part = self.delta_part.xfile_into(&mut xfile, self.numframes)?;

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
            names: self
                .names
                .to_array(self.bone_count[PART_TYPE_ALL] as _)
                .to_vec(&mut xfile)?
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            data_byte: self
                .data_byte
                .to_array(self.data_byte_count as _)
                .to_vec(&mut xfile)?,
            data_short: self
                .data_short
                .to_array(self.data_short_count as _)
                .to_vec(&mut xfile)?,
            data_int: self
                .data_int
                .to_array(self.data_int_count as _)
                .to_vec(&mut xfile)?,
            random_data_short: self
                .random_data_short
                .to_array(self.random_data_short_count as _)
                .to_vec(&mut xfile)?,
            random_data_byte: self
                .random_data_byte
                .to_array(self.random_data_byte_count as _)
                .to_vec(&mut xfile)?,
            random_data_int: self
                .random_data_int
                .to_array(self.random_data_int_count as _)
                .to_vec(&mut xfile)?,
            indices: self
                .indices
                .xfile_into(&mut xfile, (self.numframes, self.index_count))?,
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

impl<'a> XFileInto<XAnimIndices, (u16, u32)> for XAnimIndicesRaw<'a> {
    fn xfile_into(
        &self,
        xfile: impl Read + Seek,
        (num_frames, index_count): (u16, u32),
    ) -> Result<XAnimIndices> {
        if num_frames < 256 {
            Ok(XAnimIndices::_1(
                self.0
                    .cast::<u8>()
                    .to_array(index_count as _)
                    .to_vec(xfile)?,
            ))
        } else {
            Ok(XAnimIndices::_2(
                self.0
                    .cast::<u16>()
                    .to_array(index_count as _)
                    .to_vec(xfile)?,
            ))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimNotifyInfoRaw {
    pub name: ScriptString,
    pub time: f32,
}
assert_size!(XAnimNotifyInfoRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimNotifyInfo {
    pub name: String,
    pub time: f32,
}

impl From<XAnimNotifyInfoRaw> for XAnimNotifyInfo {
    fn from(value: XAnimNotifyInfoRaw) -> Self {
        Self {
            name: value.name.to_string(),
            time: value.time,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimDeltaPartRaw<'a> {
    pub trans: Ptr32<'a, XAnimPartTransRaw<'a>>,
    pub quat: Ptr32<'a, XAnimDeltaPartQuatRaw<'a>>,
}
assert_size!(XAnimDeltaPartRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimDeltaPart {
    pub trans: Option<Box<XAnimPartTrans>>,
    pub quat: Option<Box<XAnimDeltaPartQuat>>,
}

impl<'a> XFileInto<XAnimDeltaPart, u16> for XAnimDeltaPartRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, numframes: u16) -> Result<XAnimDeltaPart> {
        Ok(XAnimDeltaPart {
            trans: self.trans.xfile_into(&mut xfile, numframes)?,
            quat: self.quat.xfile_into(xfile, numframes)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimPartTransRaw<'a> {
    pub size: u16,
    pub small_trans: u8,
    pub u: XAnimPartTransDataRaw<'a>,
    pad: [u8; 28],
}
assert_size!(XAnimPartTransRaw, 36);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimPartTrans {
    pub size: u16,
    pub small_trans: u8,
    pub u: Option<XAnimPartTransData>,
}

impl<'a> XFileInto<XAnimPartTrans, u16> for XAnimPartTransRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, numframes: u16) -> Result<XAnimPartTrans> {
        Ok(XAnimPartTrans {
            size: self.size,
            small_trans: self.small_trans,
            u: self
                .u
                .xfile_into(xfile, (numframes, self.small_trans, self.size))?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimPartTransDataRaw<'a>(Ptr32<'a, ()>);
assert_size!(XAnimPartTransDataRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAnimPartTransData {
    Frames(XAnimPartTransFrames),
    Frame0(Vec3),
}

impl<'a> XFileInto<Option<XAnimPartTransData>, (u16, u8, u16)> for XAnimPartTransDataRaw<'a> {
    fn xfile_into(
        &self,
        xfile: impl Read + Seek,
        (numframes, small_trans, size): (u16, u8, u16),
    ) -> Result<Option<XAnimPartTransData>> {
        if self.0.is_null() {
            return Ok(None);
        }

        if size == 0 {
            Ok(Some(XAnimPartTransData::Frame0(
                self.0
                    .cast::<[f32; 3]>()
                    .xfile_get(xfile)?
                    .map(|v| (*v).into())
                    .unwrap(),
            )))
        } else {
            Ok(Some(XAnimPartTransData::Frames(
                *self
                    .0
                    .cast::<XAnimPartTransFramesRaw>()
                    .xfile_into(xfile, (numframes, small_trans, size))?
                    .unwrap(),
            )))
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

impl<'a> XFileInto<XAnimPartTransFrames, (u16, u8, u16)> for XAnimPartTransFramesRaw<'a> {
    fn xfile_into(
        &self,
        mut xfile: impl Read + Seek,
        (numframes, small_trans, size): (u16, u8, u16),
    ) -> Result<XAnimPartTransFrames> {
        let indices = self.indices.xfile_into(&mut xfile, (numframes, size))?;
        Ok(XAnimPartTransFrames {
            mins: self.mins.into(),
            maxs: self.maxs.into(),
            frames: self.frames.xfile_into(xfile, (small_trans, size))?,
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

impl<'a> XFileInto<XAnimDynamicFrames, (u8, u16)> for XAnimDynamicFramesRaw<'a> {
    fn xfile_into(
        &self,
        xfile: impl Read + Seek,
        (small_trans, size): (u8, u16),
    ) -> Result<XAnimDynamicFrames> {
        if small_trans == 0 {
            Ok(XAnimDynamicFrames::_2(if size != 0 {
                self.0
                    .cast::<[u16; 3]>()
                    .to_array(size as usize + 1)
                    .to_vec(xfile)?
            } else {
                Vec::new()
            }))
        } else {
            Ok(XAnimDynamicFrames::_1(if size != 0 {
                self.0
                    .cast::<[u8; 3]>()
                    .to_array(size as usize + 1)
                    .to_vec(xfile)?
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

impl<'a> XFileInto<XAnimDynamicIndices, (u16, u16)> for XAnimDynamicIndicesRaw<'a> {
    fn xfile_into(
        &self,
        xfile: impl Read + Seek,
        (numframes, size): (u16, u16),
    ) -> Result<XAnimDynamicIndices> {
        if numframes < 256 {
            Ok(XAnimDynamicIndices::_1(
                self.0
                    .cast::<u8>()
                    .to_array(size as usize + 1)
                    .to_vec(xfile)?,
            ))
        } else {
            Ok(XAnimDynamicIndices::_2(
                self.0
                    .cast::<u16>()
                    .to_array(size as usize + 1)
                    .to_vec(xfile)?,
            ))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimDeltaPartQuatRaw<'a> {
    pub size: u16,
    pub u: XAnimDeltaPartQuatDataRaw<'a>,
    pad: [u8; 4],
}
assert_size!(XAnimDeltaPartQuatRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XAnimDeltaPartQuat {
    pub size: u16,
    pub u: Option<XAnimDeltaPartQuatData>,
    pad: [u8; 4],
}

impl<'a> XFileInto<XAnimDeltaPartQuat, u16> for XAnimDeltaPartQuatRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, numframes: u16) -> Result<XAnimDeltaPartQuat> {
        Ok(XAnimDeltaPartQuat {
            size: self.size,
            u: self.u.xfile_into(xfile, (numframes, self.size))?,
            pad: [0; 4],
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct XAnimDeltaPartQuatDataRaw<'a>(Ptr32<'a, ()>);
assert_size!(XAnimDeltaPartQuatDataRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum XAnimDeltaPartQuatData {
    Frames(XAnimDeltaPartQuatDataFrames),
    Frame0([i16; 2]),
}

impl<'a> XFileInto<Option<XAnimDeltaPartQuatData>, (u16, u16)> for XAnimDeltaPartQuatDataRaw<'a> {
    fn xfile_into(
        &self,
        xfile: impl Read + Seek,
        (numframes, size): (u16, u16),
    ) -> Result<Option<XAnimDeltaPartQuatData>> {
        if self.0.is_null() {
            return Ok(None);
        }

        if numframes == 0 {
            Ok(Some(XAnimDeltaPartQuatData::Frame0(
                self.0
                    .cast::<[i16; 2]>()
                    .xfile_get(xfile)?
                    .map(|d| *d)
                    .unwrap(),
            )))
        } else {
            Ok(Some(XAnimDeltaPartQuatData::Frames(
                self.0
                    .cast::<XAnimDeltaPartQuatDataFramesRaw>()
                    .xfile_into(xfile, (numframes, size))?
                    .map(|d| *d)
                    .unwrap(),
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

impl<'a> XFileInto<XAnimDeltaPartQuatDataFrames, (u16, u16)>
    for XAnimDeltaPartQuatDataFramesRaw<'a>
{
    fn xfile_into(
        &self,
        mut xfile: impl Read + Seek,
        data: (u16, u16),
    ) -> Result<XAnimDeltaPartQuatDataFrames> {
        let indices = self.indices.xfile_into(&mut xfile, data)?;
        let frames = if data.1 != 0 {
            self.frames.to_array(data.1 as usize + 1).to_vec(xfile)?
        } else {
            Vec::new()
        };

        Ok(XAnimDeltaPartQuatDataFrames { frames, indices })
    }
}
