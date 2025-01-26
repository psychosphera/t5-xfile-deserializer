use core::fmt::Display;

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use bitflags::bitflags;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::{
    Error, ErrorKind, FatPointer, FatPointerCountFirstU32, FatPointerCountLastU32, Ptr32, Result,
    T5XFileDeserialize, T5XFileSerialize, XFileDeserializeInto, XFileSerialize, XString,
    XStringRaw, assert_size, common::Vec2, file_line_col, prelude::*,
};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SndBankRaw<'a> {
    pub name: XStringRaw<'a>,
    pub aliases: FatPointerCountFirstU32<'a, SndAliasListRaw<'a>>,
    pub alias_index: Ptr32<'a, SndIndexEntry>,
    pub pack_hash: u32,
    pub pack_location: u32,
    pub radverbs: FatPointerCountFirstU32<'a, SndRadverbRaw>,
    pub snapshots: FatPointerCountLastU32<'a, SndSnapshotRaw>,
}
assert_size!(SndBankRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndBank {
    pub name: XString,
    pub aliases: Vec<SndAliasList>,
    pub alias_index: Vec<SndIndexEntry>,
    pub pack_hash: u32,
    pub pack_location: u32,
    pub radverbs: Vec<SndRadverb>,
    pub snapshots: Vec<SndSnapshot>,
}

impl<'a> XFileDeserializeInto<SndBank, ()> for SndBankRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<SndBank> {
        //dbg!(self);
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        let aliases = self.aliases.xfile_deserialize_into(de, ())?;
        let alias_index = self
            .alias_index
            .to_array(self.aliases.size() as _)
            .to_vec(de)?;
        let radverbs = self.radverbs.to_vec_into(de)?;
        let snapshots = self.snapshots.to_vec_into(de)?;

        Ok(SndBank {
            name,
            aliases,
            alias_index,
            pack_hash: self.pack_hash,
            pack_location: self.pack_location,
            radverbs,
            snapshots,
        })
    }
}

impl XFileSerialize<()> for SndBank {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let aliases = FatPointerCountFirstU32::from_slice(&self.aliases);
        let alias_index = Ptr32::from_slice(&self.alias_index);
        let radverbs = FatPointerCountFirstU32::from_slice(&self.radverbs);
        let snapshots = FatPointerCountLastU32::from_slice(&self.snapshots);

        let snd_bank = SndBankRaw {
            name,
            aliases,
            alias_index,
            pack_hash: self.pack_hash,
            pack_location: self.pack_location,
            radverbs,
            snapshots,
        };

        ser.store_into_xfile(snd_bank)?;
        self.name.xfile_serialize(ser, ())?;
        self.aliases.xfile_serialize(ser, ())?;
        self.alias_index.xfile_serialize(ser, ())?;
        self.radverbs.xfile_serialize(ser, ())?;
        self.snapshots.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SndAliasListRaw<'a> {
    pub name: XStringRaw<'a>,
    pub id: u32,
    pub aliases: FatPointerCountLastU32<'a, SndAliasRaw<'a>>,
    pub sequence: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndAliasList {
    pub name: XString,
    pub id: u32,
    pub aliases: Vec<SndAlias>,
    pub sequence: i32,
}

impl<'a> XFileDeserializeInto<SndAliasList, ()> for SndAliasListRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<SndAliasList> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let aliases = self.aliases.xfile_deserialize_into(de, ())?;

        Ok(SndAliasList {
            name,
            id: self.id,
            aliases,
            sequence: self.sequence,
        })
    }
}

impl XFileSerialize<()> for SndAliasList {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let aliases = FatPointerCountLastU32::from_slice(&self.aliases);

        let snd_alias_list = SndAliasListRaw {
            name,
            id: self.id,
            aliases,
            sequence: self.sequence,
        };

        ser.store_into_xfile(snd_alias_list)?;
        self.name.xfile_serialize(ser, ())?;
        self.aliases.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SndAliasRaw<'a> {
    pub name: XStringRaw<'a>,
    pub id: u32,
    pub subtitle: XStringRaw<'a>,
    pub secondaryname: XStringRaw<'a>,
    pub sound_file: Ptr32<'a, SoundFileRaw<'a>>,
    pub flags: u32,
    pub duck: u32,
    pub context_type: u32,
    pub context_value: u32,
    pub flux_time: u16,
    pub start_delay: u16,
    pub radverb_send: u16,
    pub center_send: u16,
    pub vol_min: u16,
    pub vol_max: u16,
    pub team_vol_mod: u16,
    pub pitch_min: u16,
    pub pitch_max: u16,
    pub team_pitch_mod: u16,
    pub dist_min: u16,
    pub dist_max: u16,
    pub dist_radverb_max: u16,
    pub envelop_min: u16,
    pub envelop_max: u16,
    pub envelop_perecentage: u16,
    pub min_priority_threshold: u8,
    pub max_priority_threshold: u8,
    pub probability: u8,
    pub occlusion_level: u8,
    pub occlusion_wet_dry: u8,
    pub min_priority: u8,
    pub max_priority: u8,
    pub pan: u8,
    pub dry_curve: u8,
    pub wet_curve: u8,
    pub dry_min_curve: u8,
    pub wet_min_curve: u8,
    pub limit_count: u8,
    pub entity_limit_count: u8,
    pub snapshot_group: u8,
    pad: [u8; 1],
}
assert_size!(SndAliasRaw, 84);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndAlias {
    pub name: XString,
    pub id: u32,
    pub subtitle: XString,
    pub secondaryname: XString,
    pub sound_file: Option<Box<SoundFile>>,
    pub flags: u32,
    pub duck: u32,
    pub context_type: u32,
    pub context_value: u32,
    pub flux_time: u16,
    pub start_delay: u16,
    pub radverb_send: u16,
    pub center_send: u16,
    pub vol_min: u16,
    pub vol_max: u16,
    pub team_vol_mod: u16,
    pub pitch_min: u16,
    pub pitch_max: u16,
    pub team_pitch_mod: u16,
    pub dist_min: u16,
    pub dist_max: u16,
    pub dist_radverb_max: u16,
    pub envelop_min: u16,
    pub envelop_max: u16,
    pub envelop_perecentage: u16,
    pub min_priority_threshold: u8,
    pub max_priority_threshold: u8,
    pub probability: u8,
    pub occlusion_level: u8,
    pub occlusion_wet_dry: u8,
    pub min_priority: u8,
    pub max_priority: u8,
    pub pan: u8,
    pub dry_curve: u8,
    pub wet_curve: u8,
    pub dry_min_curve: u8,
    pub wet_min_curve: u8,
    pub limit_count: u8,
    pub entity_limit_count: u8,
    pub snapshot_group: u8,
}

impl<'a> XFileDeserializeInto<SndAlias, ()> for SndAliasRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<SndAlias> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        dbg!(&name);
        let subtitle = self.subtitle.xfile_deserialize_into(de, ())?;
        //dbg!(&subtitle);
        let secondaryname = self.secondaryname.xfile_deserialize_into(de, ())?;
        //dbg!(&secondaryname);
        let sound_file = self.sound_file.xfile_deserialize_into(de, ())?;

        Ok(SndAlias {
            name,
            id: self.id,
            subtitle,
            secondaryname,
            sound_file,
            flags: self.flags,
            duck: self.duck,
            context_type: self.context_type,
            context_value: self.context_value,
            flux_time: self.flux_time,
            start_delay: self.start_delay,
            radverb_send: self.radverb_send,
            center_send: self.center_send,
            vol_min: self.vol_min,
            vol_max: self.vol_max,
            team_vol_mod: self.team_vol_mod,
            pitch_min: self.pitch_min,
            pitch_max: self.pitch_max,
            team_pitch_mod: self.team_pitch_mod,
            dist_min: self.dist_min,
            dist_max: self.dist_max,
            dist_radverb_max: self.dist_radverb_max,
            envelop_min: self.envelop_min,
            envelop_max: self.envelop_max,
            envelop_perecentage: self.envelop_perecentage,
            min_priority_threshold: self.min_priority_threshold,
            max_priority_threshold: self.max_priority_threshold,
            probability: self.probability,
            occlusion_level: self.occlusion_level,
            occlusion_wet_dry: self.occlusion_wet_dry,
            min_priority: self.min_priority,
            max_priority: self.max_priority,
            pan: self.pan,
            dry_curve: self.dry_curve,
            wet_curve: self.wet_curve,
            dry_min_curve: self.dry_min_curve,
            wet_min_curve: self.wet_min_curve,
            limit_count: self.limit_count,
            entity_limit_count: self.entity_limit_count,
            snapshot_group: self.snapshot_group,
        })
    }
}

impl XFileSerialize<()> for SndAlias {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let subtitle = XStringRaw::from_str(self.subtitle.get());
        let secondaryname = XStringRaw::from_str(self.secondaryname.get());
        let sound_file = Ptr32::from_box(&self.sound_file);

        let snd_alias = SndAliasRaw {
            name,
            id: self.id,
            subtitle,
            secondaryname,
            sound_file,
            flags: self.flags,
            duck: self.duck,
            context_type: self.context_type,
            context_value: self.context_value,
            flux_time: self.flux_time,
            start_delay: self.start_delay,
            radverb_send: self.radverb_send,
            center_send: self.center_send,
            vol_min: self.vol_min,
            vol_max: self.vol_max,
            team_vol_mod: self.team_vol_mod,
            pitch_min: self.pitch_min,
            pitch_max: self.pitch_max,
            team_pitch_mod: self.team_pitch_mod,
            dist_min: self.dist_min,
            dist_max: self.dist_max,
            dist_radverb_max: self.dist_radverb_max,
            envelop_min: self.envelop_min,
            envelop_max: self.envelop_max,
            envelop_perecentage: self.envelop_perecentage,
            min_priority_threshold: self.min_priority_threshold,
            max_priority_threshold: self.max_priority_threshold,
            probability: self.probability,
            occlusion_level: self.occlusion_level,
            occlusion_wet_dry: self.occlusion_wet_dry,
            min_priority: self.min_priority,
            max_priority: self.max_priority,
            pan: self.pan,
            dry_curve: self.dry_curve,
            wet_curve: self.wet_curve,
            dry_min_curve: self.dry_min_curve,
            wet_min_curve: self.wet_min_curve,
            limit_count: self.limit_count,
            entity_limit_count: self.entity_limit_count,
            snapshot_group: self.snapshot_group,
            pad: [0u8; 1],
        };

        ser.store_into_xfile(snd_alias)?;
        self.name.xfile_serialize(ser, ())?;
        self.subtitle.xfile_serialize(ser, ())?;
        self.secondaryname.xfile_serialize(ser, ())?;
        self.sound_file.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SoundFileRaw<'a> {
    pub u: SoundFileRefRaw<'a>,
    pub type_: u8,
    pub exists: u8,
    pad: [u8; 2],
}
assert_size!(SoundFileRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SoundFile {
    pub u: SoundFileRef,
    pub exists: bool,
}

impl<'a> XFileDeserializeInto<SoundFile, ()> for SoundFileRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<SoundFile> {
        let u = self.u.xfile_deserialize_into(de, self.type_)?;
        let exists = self.exists != 0;

        Ok(SoundFile { u, exists })
    }
}

impl XFileSerialize<()> for SoundFile {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let (u, type_) = match &self.u {
            SoundFileRef::Loaded(l) => (SoundFileRefRaw(Ptr32::from_box(&l)), 1u8),
            SoundFileRef::Streamed(s) => (SoundFileRefRaw(Ptr32::from_box(&s)), 0u8),
        };

        let sound_file = SoundFileRaw {
            u,
            type_,
            exists: self.exists as _,
            pad: [0u8; 2],
        };

        ser.store_into_xfile(sound_file)?;

        match &self.u {
            SoundFileRef::Loaded(l) => l.xfile_serialize(ser, ()),
            SoundFileRef::Streamed(s) => s.xfile_serialize(ser, ()),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SoundFileRefRaw<'a>(Ptr32<'a, ()>);
assert_size!(SoundFileRefRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum SoundFileRef {
    Loaded(Option<Box<LoadedSound>>),
    Streamed(Option<Box<StreamedSound>>),
}

impl<'a> XFileDeserializeInto<SoundFileRef, u8> for SoundFileRefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        type_: u8,
    ) -> Result<SoundFileRef> {
        if type_ == 1 {
            Ok(SoundFileRef::Loaded(
                self.0
                    .cast::<LoadedSoundRaw>()
                    .xfile_deserialize_into(de, ())?,
            ))
        } else {
            Ok(SoundFileRef::Streamed(
                self.0
                    .cast::<StreamedSoundRaw>()
                    .xfile_deserialize_into(de, ())?,
            ))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct LoadedSoundRaw<'a> {
    pub name: XStringRaw<'a>,
    pub sound: SndAssetRaw<'a>,
}
assert_size!(LoadedSoundRaw, 60);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct LoadedSound {
    pub name: XString,
    pub sound: SndAsset,
}

impl<'a> XFileDeserializeInto<LoadedSound, ()> for LoadedSoundRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<LoadedSound> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        let sound = self.sound.xfile_deserialize_into(de, ())?;

        Ok(LoadedSound { name, sound })
    }
}

impl XFileSerialize<()> for LoadedSound {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());

        let seek_table = FatPointerCountFirstU32::from_slice(&self.sound.seek_table);
        let data = FatPointerCountFirstU32::from_slice(&self.sound.data);
        let sound = SndAssetRaw {
            version: self.sound.version,
            frame_count: self.sound.frame_count,
            frame_rate: self.sound.frame_rate,
            channel_count: self.sound.channel_count,
            header_size: self.sound.header_size,
            block_size: self.sound.block_size,
            buffer_size: self.sound.buffer_size,
            format: self.sound.format as _,
            channel_flags: self.sound.channel_flags.bits(),
            flags: self.sound.flags.bits(),
            seek_table,
            data,
        };

        let loaded_sound = LoadedSoundRaw { name, sound };

        ser.store_into_xfile(loaded_sound)?;
        self.name.xfile_serialize(ser, ())?;
        self.sound.seek_table.xfile_serialize(ser, ())?;
        self.sound.data.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SndAssetRaw<'a> {
    pub version: u32,
    pub frame_count: u32,
    pub frame_rate: u32,
    pub channel_count: u32,
    pub header_size: u32,
    pub block_size: u32,
    pub buffer_size: u32,
    pub format: u32,
    pub channel_flags: u32,
    pub flags: u32,
    pub seek_table: FatPointerCountFirstU32<'a, u32>,
    pub data: FatPointerCountFirstU32<'a, u8>,
}
assert_size!(SndAssetRaw, 56);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum SndAssetFormat {
    #[default]
    PCMS16 = 0,
    PCMS24 = 1,
    PCMS32 = 2,
    IEEE = 3,
    XMA4 = 4,
    MP3 = 5,
    MSADPCM = 6,
    WMA = 7,
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[derive(Copy, Clone, Debug, Deserialize)]
    pub struct SndAssetFlags: u32 {
        const LOOPING         = 0x1;
        const PAD_LOOP_BUFFER = 0x2;
    }
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[derive(Copy, Clone, Debug, Deserialize)]
    pub struct SndAssetChannel: u32 {
        const L   = 0x01;
        const R   = 0x02;
        const C   = 0x04;
        const LFE = 0x08;
        const LS  = 0x10;
        const RS  = 0x20;
        const LB  = 0x40;
        const RB  = 0x80;
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndAsset {
    pub version: u32,
    pub frame_count: u32,
    pub frame_rate: u32,
    pub channel_count: u32,
    pub header_size: u32,
    pub block_size: u32,
    pub buffer_size: u32,
    pub format: SndAssetFormat,
    pub channel_flags: SndAssetChannel,
    pub flags: SndAssetFlags,
    pub seek_table: Vec<u32>,
    pub data: Vec<u8>,
}

impl<'a> XFileDeserializeInto<SndAsset, ()> for SndAssetRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<SndAsset> {
        let format = num::FromPrimitive::from_u32(self.format).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.format as _),
        ))?;
        let channel_flags =
            SndAssetChannel::from_bits(self.channel_flags).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadFromPrimitive(self.channel_flags as _),
            ))?;
        let flags = SndAssetFlags::from_bits(self.flags).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.flags as _),
        ))?;
        let seek_table = self.seek_table.to_vec(de)?;
        let data = self.data.to_vec(de)?;

        Ok(SndAsset {
            version: self.version,
            frame_count: self.frame_count,
            frame_rate: self.frame_rate,
            channel_count: self.channel_count,
            header_size: self.header_size,
            block_size: self.block_size,
            buffer_size: self.buffer_size,
            format,
            channel_flags,
            flags,
            seek_table,
            data,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct StreamedSoundRaw<'a> {
    pub filename: XStringRaw<'a>,
    pub prime_snd: Ptr32<'a, PrimedSndRaw<'a>>,
}
assert_size!(StreamedSoundRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct StreamedSound {
    pub filename: XString,
    pub prime_snd: Option<Box<PrimedSnd>>,
}

impl<'a> XFileDeserializeInto<StreamedSound, ()> for StreamedSoundRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<StreamedSound> {
        let filename = self.filename.xfile_deserialize_into(de, ())?;
        //dbg!(&filename);
        let prime_snd = self.prime_snd.xfile_deserialize_into(de, ())?;

        Ok(StreamedSound {
            filename,
            prime_snd,
        })
    }
}

impl XFileSerialize<()> for StreamedSound {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let filename = XStringRaw::from_str(self.filename.get());
        let prime_snd = Ptr32::from_box(&self.prime_snd);

        let streamed_sound = StreamedSoundRaw {
            filename,
            prime_snd,
        };

        ser.store_into_xfile(streamed_sound)?;
        self.filename.xfile_serialize(ser, ())?;
        self.prime_snd.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PrimedSndRaw<'a> {
    pub name: XStringRaw<'a>,
    pub buffer: FatPointerCountLastU32<'a, u8>,
}
assert_size!(PrimedSndRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PrimedSnd {
    pub name: XString,
    pub buffer: Vec<u8>,
}

impl<'a> XFileDeserializeInto<PrimedSnd, ()> for PrimedSndRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<PrimedSnd> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        let buffer = self.buffer.to_vec(de)?;

        Ok(PrimedSnd { name, buffer })
    }
}

impl XFileSerialize<()> for PrimedSnd {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let buffer = FatPointerCountLastU32::from_slice(&self.buffer);

        let primed_snd = PrimedSndRaw { name, buffer };

        ser.store_into_xfile(primed_snd)?;
        self.name.xfile_serialize(ser, ())?;
        self.buffer.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct SndIndexEntry {
    pub value: u16,
    pub next: u16,
}
assert_size!(SndIndexEntry, 4);

impl XFileSerialize<()> for SndIndexEntry {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct SndName([u8; 32]);

impl Display for SndName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let len = self
            .0
            .into_iter()
            .enumerate()
            .find(|(_, c)| *c == 0)
            .map(|(i, _)| i)
            .unwrap_or(32);
        let s = self.0[..len].iter().map(|c| *c as char).collect::<String>();
        write!(f, "{}", s)
    }
}

impl SndName {
    fn from_str(string: &str) -> Result<Self> {
        if string.len() > 32 {
            return Err(Error::new(
                file_line_col!(),
                ErrorKind::BrokenInvariant(format!(
                    "SndName: string \"{string}\" too long (must be <=32 bytes)."
                )),
            ));
        }

        let mut bytes = string.chars().map(|c| c as u8).collect::<Vec<_>>();
        bytes.resize(32, 0);
        Ok(Self(bytes.try_into().unwrap()))
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SndRadverbRaw {
    pub name: SndName,
    pub id: u32,
    pub smoothing: f32,
    pub early_time: f32,
    pub late_time: f32,
    pub early_gain: f32,
    pub late_gain: f32,
    pub return_gain: f32,
    pub early_lpf: f32,
    pub late_lpf: f32,
    pub input_lpf: f32,
    pub damp_lpf: f32,
    pub wall_reflect: f32,
    pub dry_gain: f32,
    pub early_size: f32,
    pub late_size: f32,
    pub diffusion: f32,
}
assert_size!(SndRadverbRaw, 96);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndRadverb {
    pub name: XString,
    pub id: u32,
    pub smoothing: f32,
    pub early_time: f32,
    pub late_time: f32,
    pub early_gain: f32,
    pub late_gain: f32,
    pub return_gain: f32,
    pub early_lpf: f32,
    pub late_lpf: f32,
    pub input_lpf: f32,
    pub damp_lpf: f32,
    pub wall_reflect: f32,
    pub dry_gain: f32,
    pub early_size: f32,
    pub late_size: f32,
    pub diffusion: f32,
}

impl From<SndRadverbRaw> for SndRadverb {
    fn from(value: SndRadverbRaw) -> Self {
        let name = XString(value.name.to_string());
        //dbg!(&name);
        Self {
            name,
            id: value.id,
            smoothing: value.smoothing,
            early_time: value.early_time,
            late_time: value.late_time,
            early_gain: value.early_gain,
            late_gain: value.late_gain,
            return_gain: value.return_gain,
            early_lpf: value.early_lpf,
            late_lpf: value.late_lpf,
            input_lpf: value.input_lpf,
            damp_lpf: value.damp_lpf,
            wall_reflect: value.wall_reflect,
            dry_gain: value.dry_gain,
            early_size: value.early_size,
            late_size: value.late_size,
            diffusion: value.diffusion,
        }
    }
}

impl XFileSerialize<()> for SndRadverb {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = SndName::from_str(self.name.get())?;

        let snd_radverb = SndRadverbRaw {
            name,
            id: self.id,
            smoothing: self.smoothing,
            early_time: self.early_time,
            late_time: self.late_time,
            early_gain: self.early_gain,
            late_gain: self.late_gain,
            return_gain: self.return_gain,
            early_lpf: self.early_lpf,
            late_lpf: self.late_lpf,
            input_lpf: self.input_lpf,
            damp_lpf: self.damp_lpf,
            wall_reflect: self.wall_reflect,
            dry_gain: self.dry_gain,
            early_size: self.early_size,
            late_size: self.late_size,
            diffusion: self.diffusion,
        };

        ser.store_into_xfile(snd_radverb)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SndSnapshotRaw {
    pub name: SndName,
    pub id: u32,
    pub occlusion_name: SndName,
    pub occlusion_id: u32,
    pub fade_in: f32,
    pub fade_out: f32,
    pub distance: f32,
    pub fade_in_curve: u32,
    pub fade_out_curve: u32,
    #[serde(with = "serde_arrays")]
    pub attenuation: [f32; 64],
}
assert_size!(SndSnapshotRaw, 348);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndSnapshot {
    pub name: XString,
    pub id: u32,
    pub occlusion_name: XString,
    pub occlusion_id: u32,
    pub fade_in: f32,
    pub fade_out: f32,
    pub distance: f32,
    pub fade_in_curve: u32,
    pub fade_out_curve: u32,
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub attenuation: [f32; 64],
}

impl From<SndSnapshotRaw> for SndSnapshot {
    fn from(value: SndSnapshotRaw) -> Self {
        let name = XString(value.name.to_string());
        //dbg!(&name);
        let occlusion_name = XString(value.occlusion_name.to_string());
        //dbg!(&occlusion_name);

        Self {
            name,
            id: value.id,
            occlusion_name,
            occlusion_id: value.occlusion_id,
            fade_in: value.fade_in,
            fade_out: value.fade_out,
            distance: value.distance,
            fade_in_curve: value.fade_in_curve,
            fade_out_curve: value.fade_out_curve,
            attenuation: value.attenuation,
        }
    }
}

impl XFileSerialize<()> for SndSnapshot {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = SndName::from_str(self.name.get())?;
        let occlusion_name = SndName::from_str(self.occlusion_name.get())?;

        let snd_snapshot = SndSnapshotRaw {
            name,
            id: self.id,
            occlusion_name,
            occlusion_id: self.occlusion_id,
            fade_in: self.fade_in,
            fade_out: self.fade_out,
            distance: self.distance,
            fade_in_curve: self.fade_in_curve,
            fade_out_curve: self.fade_out_curve,
            attenuation: self.attenuation,
        };

        ser.store_into_xfile(snd_snapshot)?;
        self.name.xfile_serialize(ser, ())?;
        self.occlusion_name.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SndPatchRaw<'a> {
    pub name: XStringRaw<'a>,
    pub elements: FatPointerCountFirstU32<'a, u32>,
    pub files: FatPointerCountFirstU32<'a, SoundFileRaw<'a>>,
}
assert_size!(SndPatchRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndPatch {
    pub name: XString,
    pub elements: Vec<u32>,
    pub files: Vec<SoundFile>,
}

impl<'a> XFileDeserializeInto<SndPatch, ()> for SndPatchRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<SndPatch> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        let elements = self.elements.to_vec(de)?;
        let files = self.files.xfile_deserialize_into(de, ())?;

        Ok(SndPatch {
            name,
            elements,
            files,
        })
    }
}

impl XFileSerialize<()> for SndPatch {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let elements = FatPointerCountFirstU32::from_slice(&self.elements);
        let files = FatPointerCountFirstU32::from_slice(&self.files);

        let snd_patch = SndPatchRaw {
            name,
            elements,
            files,
        };

        ser.store_into_xfile(snd_patch)?;
        self.name.xfile_serialize(ser, ())?;
        self.elements.xfile_serialize(ser, ())?;
        self.files.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct SndDriverGlobalsRaw<'a> {
    pub name: XStringRaw<'a>,
    pub groups: FatPointerCountFirstU32<'a, SndGroupRaw>,
    pub curves: FatPointerCountFirstU32<'a, SndCurveRaw>,
    pub pans: FatPointerCountFirstU32<'a, SndPanRaw>,
    pub snapshot_groups: FatPointerCountFirstU32<'a, SndSnapshotGroupRaw>,
    pub contexts: FatPointerCountFirstU32<'a, SndContext>,
    pub masters: FatPointerCountFirstU32<'a, SndMasterRaw>,
}
assert_size!(SndDriverGlobalsRaw, 52);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndDriverGlobals {
    pub name: XString,
    pub groups: Vec<SndGroup>,
    pub curves: Vec<SndCurve>,
    pub pans: Vec<SndPan>,
    pub snapshot_groups: Vec<SndSnapshotGroup>,
    pub contexts: Vec<SndContext>,
    pub masters: Vec<SndMaster>,
}

impl<'a> XFileDeserializeInto<SndDriverGlobals, ()> for SndDriverGlobalsRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<SndDriverGlobals> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        let groups = self
            .groups
            .to_vec(de)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        let curves = self.curves.to_vec_into(de)?;
        let pans = self.pans.to_vec_into(de)?;
        let snapshot_groups = self.snapshot_groups.to_vec_into(de)?;
        let contexts = self.contexts.to_vec_into(de)?;
        let masters = self.masters.to_vec_into(de)?;

        Ok(SndDriverGlobals {
            name,
            groups,
            curves,
            pans,
            snapshot_groups,
            contexts,
            masters,
        })
    }
}

impl XFileSerialize<()> for SndDriverGlobals {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let groups = FatPointerCountFirstU32::from_slice(&self.groups);
        let curves = FatPointerCountFirstU32::from_slice(&self.curves);
        let pans = FatPointerCountFirstU32::from_slice(&self.pans);
        let snapshot_groups = FatPointerCountFirstU32::from_slice(&self.snapshot_groups);
        let contexts = FatPointerCountFirstU32::from_slice(&self.contexts);
        let masters = FatPointerCountFirstU32::from_slice(&self.masters);

        let snddriver_globals = SndDriverGlobalsRaw {
            name,
            groups,
            curves,
            pans,
            snapshot_groups,
            contexts,
            masters,
        };

        ser.store_into_xfile(snddriver_globals)?;
        self.name.xfile_serialize(ser, ())?;
        self.groups.xfile_serialize(ser, ())?;
        self.curves.xfile_serialize(ser, ())?;
        self.pans.xfile_serialize(ser, ())?;
        self.snapshot_groups.xfile_serialize(ser, ())?;
        self.contexts.xfile_serialize(ser, ())?;
        self.masters.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct SndGroupRaw {
    pub name: SndName,
    pub parent_name: SndName,
    pub id: u32,
    pub parent_index: i32,
    pub category: u32,
    pub attenuation_sp: u16,
    pub attenuation_mp: u16,
}
assert_size!(SndGroupRaw, 80);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum SndCategory {
    #[default]
    SFX = 0,
    MUSIC = 1,
    VOICE = 2,
    UI = 3,
    COUNT = 4,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndGroup {
    pub name: XString,
    pub parent_name: XString,
    pub id: u32,
    pub parent_index: i32,
    pub category: SndCategory,
    pub attenuation_sp: u16,
    pub attenuation_mp: u16,
}

impl TryInto<SndGroup> for SndGroupRaw {
    type Error = Error;
    fn try_into(self) -> core::result::Result<SndGroup, Self::Error> {
        let name = XString(self.name.to_string());
        //dbg!(&name);
        let parent_name = XString(self.parent_name.to_string());
        //dbg!(&parent_name);
        let category = FromPrimitive::from_u32(self.category).ok_or(Error::new_with_offset(
            file_line_col!(),
            0,
            ErrorKind::BadFromPrimitive(self.category as _),
        ))?;

        Ok(SndGroup {
            name,
            parent_name,
            id: self.id,
            parent_index: self.parent_index,
            category,
            attenuation_sp: self.attenuation_sp,
            attenuation_mp: self.attenuation_mp,
        })
    }
}

impl XFileSerialize<()> for SndGroup {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = SndName::from_str(self.name.get())?;
        let parent_name = SndName::from_str(self.parent_name.get())?;

        let snd_group = SndGroupRaw {
            name,
            parent_name,
            id: self.id,
            parent_index: self.parent_index,
            category: self.category as _,
            attenuation_sp: self.attenuation_sp,
            attenuation_mp: self.attenuation_mp,
        };

        ser.store_into_xfile(snd_group)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct SndCurveRaw {
    pub name: SndName,
    pub id: u32,
    pub points: [[f32; 2]; 8],
}
assert_size!(SndCurveRaw, 100);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndCurve {
    pub name: XString,
    pub id: u32,
    pub points: [Vec2; 8],
}

impl From<SndCurveRaw> for SndCurve {
    fn from(value: SndCurveRaw) -> Self {
        let name = XString(value.name.to_string());
        let points = value.points.map(|p| Vec2::from(p));
        //dbg!(&name);

        SndCurve {
            name,
            id: value.id,
            points,
        }
    }
}

impl XFileSerialize<()> for SndCurve {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = SndName::from_str(self.name.get())?;
        let points = self.points.map(|p| p.get());

        let snd_curve = SndCurveRaw {
            name,
            id: self.id,
            points,
        };

        ser.store_into_xfile(snd_curve)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct SndPanRaw {
    pub name: SndName,
    pub id: u32,
    pub front: f32,
    pub back: f32,
    pub center: f32,
    pub lfe: f32,
    pub left: f32,
    pub right: f32,
}
assert_size!(SndPanRaw, 60);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndPan {
    pub name: XString,
    pub id: u32,
    pub front: f32,
    pub back: f32,
    pub center: f32,
    pub lfe: f32,
    pub left: f32,
    pub right: f32,
}

impl From<SndPanRaw> for SndPan {
    fn from(value: SndPanRaw) -> Self {
        let name = XString(value.name.to_string());
        //dbg!(&name);

        Self {
            name,
            id: value.id,
            front: value.front,
            back: value.back,
            center: value.center,
            lfe: value.lfe,
            left: value.left,
            right: value.right,
        }
    }
}

impl XFileSerialize<()> for SndPan {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = SndName::from_str(self.name.get())?;

        let snd_pan = SndPanRaw {
            name,
            id: self.id,
            front: self.front,
            back: self.back,
            center: self.center,
            lfe: self.lfe,
            left: self.left,
            right: self.right,
        };

        ser.store_into_xfile(snd_pan)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct SndSnapshotGroupRaw {
    pub name: SndName,
}
assert_size!(SndSnapshotGroupRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndSnapshotGroup {
    pub name: XString,
}

impl From<SndSnapshotGroupRaw> for SndSnapshotGroup {
    fn from(value: SndSnapshotGroupRaw) -> Self {
        let name = XString(value.name.to_string());
        //dbg!(&name);
        Self { name }
    }
}

impl XFileSerialize<()> for SndSnapshotGroup {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = SndName::from_str(self.name.get())?;

        let snd_snapshot_group = SndSnapshotGroupRaw { name };

        ser.store_into_xfile(snd_snapshot_group)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct SndContext {
    pub type_: u32,
    pub value_count: u32,
    pub values: [u32; 8],
}
assert_size!(SndContext, 40);

impl XFileSerialize<()> for SndContext {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct SndMasterRaw {
    pub name: SndName,
    pub id: u32,
    pub notch_e: f32,
    pub notch_g: f32,
    pub notch_f: f32,
    pub notch_q: f32,
    pub low_e: f32,
    pub low_g: f32,
    pub low_f: f32,
    pub low_q: f32,
    pub peak_1_e: f32,
    pub peak_1_g: f32,
    pub peak_1_f: f32,
    pub peak_1_q: f32,
    pub peak_2_e: f32,
    pub peak_2_g: f32,
    pub peak_2_f: f32,
    pub peak_2_q: f32,
    pub hi_e: f32,
    pub hi_g: f32,
    pub hi_f: f32,
    pub hi_q: f32,
    pub eq_g: f32,
    pub comp_e: f32,
    pub comp_pg: f32,
    pub comp_mg: f32,
    pub comp_t: f32,
    pub comp_r: f32,
    pub comp_ta: f32,
    pub comp_tr: f32,
    pub limit_e: f32,
    pub limit_pg: f32,
    pub limit_mg: f32,
    pub limit_t: f32,
    pub limit_r: f32,
    pub limit_ta: f32,
    pub limit_tr: f32,
}
assert_size!(SndMasterRaw, 176);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SndMaster {
    pub name: XString,
    pub id: u32,
    pub notch_e: f32,
    pub notch_g: f32,
    pub notch_f: f32,
    pub notch_q: f32,
    pub low_e: f32,
    pub low_g: f32,
    pub low_f: f32,
    pub low_q: f32,
    pub peak_1_e: f32,
    pub peak_1_g: f32,
    pub peak_1_f: f32,
    pub peak_1_q: f32,
    pub peak_2_e: f32,
    pub peak_2_g: f32,
    pub peak_2_f: f32,
    pub peak_2_q: f32,
    pub hi_e: f32,
    pub hi_g: f32,
    pub hi_f: f32,
    pub hi_q: f32,
    pub eq_g: f32,
    pub comp_e: f32,
    pub comp_pg: f32,
    pub comp_mg: f32,
    pub comp_t: f32,
    pub comp_r: f32,
    pub comp_ta: f32,
    pub comp_tr: f32,
    pub limit_e: f32,
    pub limit_pg: f32,
    pub limit_mg: f32,
    pub limit_t: f32,
    pub limit_r: f32,
    pub limit_ta: f32,
    pub limit_tr: f32,
}

impl From<SndMasterRaw> for SndMaster {
    fn from(value: SndMasterRaw) -> Self {
        let name = XString(value.name.to_string());
        //dbg!(&name);

        SndMaster {
            name,
            id: value.id,
            notch_e: value.notch_e,
            notch_g: value.notch_g,
            notch_f: value.notch_f,
            notch_q: value.notch_q,
            low_e: value.low_e,
            low_g: value.low_g,
            low_f: value.low_f,
            low_q: value.low_q,
            peak_1_e: value.peak_1_e,
            peak_1_g: value.peak_1_g,
            peak_1_f: value.peak_1_f,
            peak_1_q: value.peak_1_q,
            peak_2_e: value.peak_2_e,
            peak_2_g: value.peak_2_g,
            peak_2_f: value.peak_2_f,
            peak_2_q: value.peak_2_q,
            hi_e: value.hi_e,
            hi_g: value.hi_g,
            hi_f: value.hi_f,
            hi_q: value.hi_q,
            eq_g: value.eq_g,
            comp_e: value.comp_e,
            comp_pg: value.comp_pg,
            comp_mg: value.comp_mg,
            comp_t: value.comp_t,
            comp_r: value.comp_r,
            comp_ta: value.comp_ta,
            comp_tr: value.comp_tr,
            limit_e: value.limit_e,
            limit_pg: value.limit_pg,
            limit_mg: value.limit_mg,
            limit_t: value.limit_t,
            limit_r: value.limit_r,
            limit_ta: value.limit_ta,
            limit_tr: value.limit_tr,
        }
    }
}

impl XFileSerialize<()> for SndMaster {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = SndName::from_str(self.name.get())?;

        let snd_master = SndMasterRaw {
            name,
            id: self.id,
            notch_e: self.notch_e,
            notch_g: self.notch_g,
            notch_f: self.notch_f,
            notch_q: self.notch_q,
            low_e: self.low_e,
            low_g: self.low_g,
            low_f: self.low_f,
            low_q: self.low_q,
            peak_1_e: self.peak_1_e,
            peak_1_g: self.peak_1_g,
            peak_1_f: self.peak_1_f,
            peak_1_q: self.peak_1_q,
            peak_2_e: self.peak_2_e,
            peak_2_g: self.peak_2_g,
            peak_2_f: self.peak_2_f,
            peak_2_q: self.peak_2_q,
            hi_e: self.hi_e,
            hi_g: self.hi_g,
            hi_f: self.hi_f,
            hi_q: self.hi_q,
            eq_g: self.eq_g,
            comp_e: self.comp_e,
            comp_pg: self.comp_pg,
            comp_mg: self.comp_mg,
            comp_t: self.comp_t,
            comp_r: self.comp_r,
            comp_ta: self.comp_ta,
            comp_tr: self.comp_tr,
            limit_e: self.limit_e,
            limit_pg: self.limit_pg,
            limit_mg: self.limit_mg,
            limit_t: self.limit_t,
            limit_r: self.limit_r,
            limit_ta: self.limit_ta,
            limit_tr: self.limit_tr,
        };

        ser.store_into_xfile(snd_master)
    }
}
