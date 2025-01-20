use alloc::{ffi::CString, string::ToString, vec::Vec};

#[allow(unused_imports)]
use crate::prelude::*;

use crate::{
    FatPointer, FatPointerCountLastU32, Ptr32, Result, T5XFileDeserialize, T5XFileSerialize,
    XFileDeserializeInto, XFileSerialize, XString, XStringRaw, assert_size, common::Vec4,
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct RawFileRaw<'a> {
    pub name: XStringRaw<'a>,
    pub len: i32,
    pub buffer: Ptr32<'a, u8>,
}
assert_size!(RawFileRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RawFile {
    pub name: XString,
    pub buffer: Vec<u8>,
}

impl<'a> XFileDeserializeInto<RawFile, ()> for RawFileRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<RawFile> {
        //dbg!(&self);
        let name = self.name.xfile_deserialize_into(de, ())?;
        let buffer = self.buffer.to_array(self.len as usize + 1).to_vec(de)?;
        Ok(RawFile { name, buffer })
    }
}

impl XFileSerialize<()> for RawFile {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_u32(0xFFFFFFFF);
        let len = self.buffer.len() as _;
        let buffer = Ptr32::unreal();
        let raw_file = RawFileRaw { name, len, buffer };

        ser.store_into_xfile(raw_file)?;
        self.buffer.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct StringTableRaw<'a> {
    pub name: XStringRaw<'a>,
    pub column_count: i32,
    pub row_count: i32,
    pub values: Ptr32<'a, StringTableCellRaw<'a>>,
    pub cell_index: Ptr32<'a, i16>,
}
assert_size!(StringTableRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct StringTable {
    pub name: XString,
    pub column_count: usize,
    pub row_count: usize,
    pub values: Vec<StringTableCell>,
    pub cell_index: Vec<i16>,
}

impl<'a> XFileDeserializeInto<StringTable, ()> for StringTableRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<StringTable> {
        let size = self.column_count as usize * self.row_count as usize;

        Ok(StringTable {
            name: self.name.xfile_deserialize_into(de, ())?,
            column_count: self.column_count as _,
            row_count: self.row_count as _,
            values: self.values.to_array(size).xfile_deserialize_into(de, ())?,
            cell_index: self.cell_index.to_array(size).to_vec(de)?,
        })
    }
}

impl XFileSerialize<()> for StringTable {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let column_count = self.column_count as _;
        let row_count = self.row_count as _;
        let values = Ptr32::from_slice::<StringTableCellRaw>(&self.values);
        let cell_index = Ptr32::from_slice(&self.cell_index);

        let string_table = StringTableRaw {
            name,
            column_count,
            row_count,
            values,
            cell_index,
        };

        ser.store_into_xfile(string_table)?;
        self.name.xfile_serialize(ser, ())?;
        self.values.xfile_serialize(ser, ())?;
        self.cell_index.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct StringTableCellRaw<'a> {
    pub name: XStringRaw<'a>,
    pub hash: i32,
}
assert_size!(StringTableCellRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct StringTableCell {
    pub name: XString,
    pub hash: i32,
}

impl<'a> XFileDeserializeInto<StringTableCell, ()> for StringTableCellRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<StringTableCell> {
        Ok(StringTableCell {
            name: self.name.xfile_deserialize_into(de, ())?,
            hash: self.hash,
        })
    }
}

impl XFileSerialize<()> for StringTableCell {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());

        let cell = StringTableCellRaw {
            name,
            hash: self.hash,
        };

        ser.store_into_xfile(cell)?;
        self.name.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexRaw<'a> {
    pub name: XStringRaw<'a>,
    pub header: PackIndexHeaderRaw,
    pub entries: Ptr32<'a, PackIndexEntryRaw>,
}
assert_size!(PackIndexRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PackIndex {
    pub name: XString,
    pub header: PackIndexHeader,
    pub entries: Vec<PackIndexEntry>,
}

impl<'a> XFileDeserializeInto<PackIndex, ()> for PackIndexRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<PackIndex> {
        Ok(PackIndex {
            name: self.name.xfile_deserialize_into(de, ())?,
            header: self.header.into(),
            entries: self
                .entries
                .to_array(self.header.count as _)
                .to_vec_into(de)?,
        })
    }
}

impl XFileSerialize<()> for PackIndex {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let header = self.header.into();
        let entries = Ptr32::from_slice(&self.entries);

        let pack_index = PackIndexRaw {
            name,
            header,
            entries,
        };

        ser.store_into_xfile(pack_index)?;
        self.name.xfile_serialize(ser, ())?;
        self.header.xfile_serialize(ser, ())?;
        self.entries.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexHeaderRaw {
    pub magic: u32,
    pub timestamp: u32,
    pub count: u32,
    pub alignment: u32,
    pub data_start: u32,
}
assert_size!(PackIndexHeaderRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub struct PackIndexHeader {
    pub magic: u32,
    pub timestamp: u32,
    pub count: usize,
    pub alignment: usize,
    pub data_start: usize,
}

impl From<PackIndexHeaderRaw> for PackIndexHeader {
    fn from(value: PackIndexHeaderRaw) -> Self {
        Self {
            magic: value.magic,
            timestamp: value.timestamp,
            count: value.count as _,
            alignment: value.alignment as _,
            data_start: value.data_start as _,
        }
    }
}

impl From<PackIndexHeader> for PackIndexHeaderRaw {
    fn from(value: PackIndexHeader) -> Self {
        Self {
            magic: value.magic,
            timestamp: value.timestamp,
            count: value.count as _,
            alignment: value.alignment as _,
            data_start: value.data_start as _,
        }
    }
}

impl XFileSerialize<()> for PackIndexHeader {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let pack_index_header = PackIndexHeaderRaw {
            magic: self.magic,
            timestamp: self.timestamp,
            count: self.count as _,
            alignment: self.alignment as _,
            data_start: self.data_start as _,
        };

        ser.store_into_xfile(pack_index_header)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexEntryRaw {
    pub hash: u32,
    pub offset: u32,
    pub size: u32,
}
assert_size!(PackIndexEntryRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PackIndexEntry {
    pub hash: u32,
    pub offset: usize,
    pub size: usize,
}

impl From<PackIndexEntryRaw> for PackIndexEntry {
    fn from(value: PackIndexEntryRaw) -> Self {
        Self {
            hash: value.hash,
            offset: value.offset as _,
            size: value.size as _,
        }
    }
}

impl XFileSerialize<()> for PackIndexEntry {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let pack_index_entry = PackIndexEntry {
            hash: self.hash,
            offset: self.offset as _,
            size: self.size as _,
        };

        ser.store_into_xfile(pack_index_entry)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct MapEntsRaw<'a> {
    pub name: XStringRaw<'a>,
    pub entity_string: FatPointerCountLastU32<'a, u8>,
}
assert_size!(MapEntsRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MapEnts {
    pub name: XString,
    pub entity_string: XString,
}

impl<'a> XFileDeserializeInto<MapEnts, ()> for MapEntsRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MapEnts> {
        let name = self.name.xfile_deserialize_into(de, ())?;

        let mut chars = self.entity_string.to_vec(de)?;
        if chars.is_empty() {
            return Ok(MapEnts {
                name,
                entity_string: XString::new(),
            });
        }

        if *chars.last().unwrap() != b'\0' {
            chars.push(b'\0');
        }

        let entity_string = XString(
            CString::from_vec_with_nul(chars)
                .unwrap()
                .to_string_lossy()
                .to_string(),
        );

        Ok(MapEnts {
            name,
            entity_string,
        })
    }
}

impl XFileSerialize<()> for MapEnts {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let bytes = self
            .entity_string
            .get()
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<_>>();
        let entity_string = FatPointerCountLastU32::from_slice(&bytes);

        let map_ents = MapEntsRaw {
            name,
            entity_string,
        };

        ser.store_into_xfile(map_ents)?;
        self.name.xfile_serialize(ser, ())?;
        self.entity_string.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct LocalizeEntryRaw<'a> {
    pub value: XStringRaw<'a>,
    pub name: XStringRaw<'a>,
}
assert_size!(LocalizeEntryRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct LocalizeEntry {
    pub value: XString,
    pub name: XString,
}

impl<'a> XFileDeserializeInto<LocalizeEntry, ()> for LocalizeEntryRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<LocalizeEntry> {
        let value = self.value.xfile_deserialize_into(de, ())?;
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&value, &name);
        Ok(LocalizeEntry { value, name })
    }
}

impl XFileSerialize<()> for LocalizeEntry {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let value = XStringRaw::from_str(self.value.get());
        let name = XStringRaw::from_str(self.name.get());

        let localize_entry = LocalizeEntryRaw { value, name };

        ser.store_into_xfile(localize_entry)?;
        self.value.xfile_serialize(ser, ())?;
        self.name.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct XGlobalsRaw<'a> {
    pub name: XStringRaw<'a>,
    pub xanim_stream_buffer_size: i32,
    pub cinematic_max_width: i32,
    pub cinematic_max_height: i32,
    pub extracam_resolution: i32,
    pub gump_reserve: i32,
    pub screen_clear_color: [f32; 4],
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct XGlobals {
    pub name: XString,
    pub xanim_stream_buffer_size: i32,
    pub cinematic_max_width: i32,
    pub cinematic_max_height: i32,
    pub extracam_resolution: i32,
    pub gump_reserve: i32,
    pub screen_clear_color: Vec4,
}

impl<'a> XFileDeserializeInto<XGlobals, ()> for XGlobalsRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<XGlobals> {
        Ok(XGlobals {
            name: self.name.xfile_deserialize_into(de, ())?,
            xanim_stream_buffer_size: self.xanim_stream_buffer_size,
            cinematic_max_width: self.cinematic_max_width,
            cinematic_max_height: self.cinematic_max_height,
            extracam_resolution: self.extracam_resolution,
            gump_reserve: self.gump_reserve,
            screen_clear_color: self.screen_clear_color.into(),
        })
    }
}

impl XFileSerialize<()> for XGlobals {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());

        let xglobals = XGlobalsRaw {
            name,
            xanim_stream_buffer_size: self.xanim_stream_buffer_size,
            cinematic_max_height: self.cinematic_max_height,
            cinematic_max_width: self.cinematic_max_width,
            extracam_resolution: self.extracam_resolution,
            gump_reserve: self.gump_reserve,
            screen_clear_color: self.screen_clear_color.get(),
        };

        ser.store_into_xfile(xglobals)?;
        self.name.xfile_serialize(ser, ())
    }
}
