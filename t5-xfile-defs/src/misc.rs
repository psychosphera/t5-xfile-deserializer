use alloc::{
    ffi::CString,
    string::{String, ToString},
    vec::Vec,
};

#[allow(unused_imports)]
use crate::prelude::*;

use crate::{
    FatPointer, FatPointerCountLastU32, Ptr32, Result, T5XFileDeserialize, XFileDeserializeInto,
    XString, assert_size, common::Vec4,
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct RawFileRaw<'a> {
    pub name: XString<'a>,
    pub len: i32,
    pub buffer: Ptr32<'a, u8>,
}
assert_size!(RawFileRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RawFile {
    pub name: String,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct StringTableRaw<'a> {
    pub name: XString<'a>,
    pub column_count: i32,
    pub row_count: i32,
    pub values: Ptr32<'a, StringTableCellRaw<'a>>,
    pub cell_index: Ptr32<'a, i16>,
}
assert_size!(StringTableRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct StringTable {
    pub name: String,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct StringTableCellRaw<'a> {
    pub name: XString<'a>,
    pub hash: i32,
}
assert_size!(StringTableCellRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct StringTableCell {
    pub name: String,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexRaw<'a> {
    pub name: XString<'a>,
    pub header: PackIndexHeaderRaw,
    pub entries: Ptr32<'a, PackIndexEntryRaw>,
}
assert_size!(PackIndexRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PackIndex {
    pub name: String,
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
#[derive(Clone, Debug)]
pub struct PackIndexHeader {
    pub magic: u32,
    pub timestamp: u32,
    pub count: usize,
    pub alignment: usize,
    pub data_start: usize,
}

impl Into<PackIndexHeader> for PackIndexHeaderRaw {
    fn into(self) -> PackIndexHeader {
        PackIndexHeader {
            magic: self.magic,
            timestamp: self.timestamp,
            count: self.count as _,
            alignment: self.alignment as _,
            data_start: self.data_start as _,
        }
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct MapEntsRaw<'a> {
    pub name: XString<'a>,
    pub entity_string: FatPointerCountLastU32<'a, u8>,
}
assert_size!(MapEntsRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MapEnts {
    pub name: String,
    pub entity_string: String,
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
                entity_string: String::new(),
            });
        }

        if *chars.last().unwrap() != b'\0' {
            chars.push(b'\0');
        }

        let entity_string = CString::from_vec_with_nul(chars)
            .unwrap()
            .to_string_lossy()
            .to_string();

        Ok(MapEnts {
            name,
            entity_string,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct LocalizeEntryRaw<'a> {
    pub value: XString<'a>,
    pub name: XString<'a>,
}
assert_size!(LocalizeEntryRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct LocalizeEntry {
    pub value: String,
    pub name: String,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct XGlobalsRaw<'a> {
    pub name: XString<'a>,
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
    pub name: String,
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
