use common::Vec4;
use serde::Deserialize;

use crate::*;

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct RawFileRaw<'a> {
    name: XString<'a>,
    buffer: FatPointerCountFirstU32<'a, u8>,
}
assert_size!(RawFileRaw, 12);

#[derive(Clone, Debug)]
pub struct RawFile {
    pub name: String,
    pub buffer: Vec<u8>,
}

impl<'a> XFileInto<RawFile, ()> for RawFileRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> RawFile {
        RawFile {
            name: self.name.xfile_into(&mut xfile, ()),
            buffer: self.buffer.to_vec(xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct StringTableRaw<'a> {
    name: XString<'a>,
    column_count: i32,
    row_count: i32,
    values: Ptr32<'a, StringTableCellRaw<'a>>,
    cell_index: Ptr32<'a, i16>,
}
assert_size!(StringTableRaw, 20);

#[derive(Clone, Debug)]
pub struct StringTable {
    pub name: String,
    pub column_count: usize,
    pub row_count: usize,
    pub values: Vec<StringTableCell>,
    pub cell_index: Vec<i16>,
}

impl<'a> XFileInto<StringTable, ()> for StringTableRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> StringTable {
        let size = self.column_count as usize * self.row_count as usize;

        StringTable {
            name: self.name.xfile_into(&mut xfile, ()),
            column_count: self.column_count as _,
            row_count: self.row_count as _,
            values: self.values.to_array(size).xfile_into(&mut xfile, ()),
            cell_index: self.cell_index.to_array(size).to_vec(xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct StringTableCellRaw<'a> {
    name: XString<'a>,
    hash: i32,
}
assert_size!(StringTableCellRaw, 8);

#[derive(Clone, Debug)]
pub struct StringTableCell {
    pub name: String,
    pub hash: i32,
}

impl<'a> XFileInto<StringTableCell, ()> for StringTableCellRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> StringTableCell {
        StringTableCell {
            name: self.name.xfile_into(xfile, ()),
            hash: self.hash,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexRaw<'a> {
    name: XString<'a>,
    header: PackIndexHeaderRaw,
    entries: Ptr32<'a, PackIndexEntryRaw>,
}
assert_size!(PackIndexRaw, 28);

#[derive(Clone, Debug)]
pub struct PackIndex {
    pub name: String,
    pub header: PackIndexHeader,
    pub entries: Vec<PackIndexEntry>,
}

impl<'a> XFileInto<PackIndex, ()> for PackIndexRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> PackIndex {
        PackIndex {
            name: self.name.xfile_into(&mut xfile, ()),
            header: self.header.into(),
            entries: self
                .entries
                .to_array(self.header.count as _)
                .to_vec(xfile)
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexHeaderRaw {
    magic: u32,
    timestamp: u32,
    count: u32,
    alignment: u32,
    data_start: u32,
}
assert_size!(PackIndexHeaderRaw, 20);

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

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct PackIndexEntryRaw {
    hash: u32,
    offset: u32,
    size: u32,
}
assert_size!(PackIndexEntryRaw, 12);

#[derive(Clone, Debug)]
pub struct PackIndexEntry {
    pub hash: u32,
    pub offset: usize,
    pub size: usize,
}

impl Into<PackIndexEntry> for PackIndexEntryRaw {
    fn into(self) -> PackIndexEntry {
        PackIndexEntry {
            hash: self.hash,
            offset: self.offset as _,
            size: self.size as _,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct MapEntsRaw<'a> {
    name: XString<'a>,
    entity_string: FatPointerCountLastU32<'a, u8>,
}
assert_size!(MapEntsRaw, 12);

#[derive(Clone, Debug)]
pub struct MapEnts {
    pub name: String,
    pub entity_string: String,
}

impl<'a> XFileInto<MapEnts, ()> for MapEntsRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> MapEnts {
        let name = self.name.xfile_into(&mut xfile, ());

        let mut chars = self.entity_string.to_vec(xfile);
        if chars.bytes().last().unwrap().unwrap() != b'\0' {
            chars.push(b'\0');
        }

        MapEnts {
            name,
            entity_string: CString::from_vec_with_nul(chars)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct LocalizeEntryRaw<'a> {
    value: XString<'a>,
    name: XString<'a>,
}
assert_size!(LocalizeEntryRaw, 8);

#[derive(Clone, Debug)]
pub struct LocalizeEntry {
    pub value: String,
    pub name: String,
}

impl<'a> XFileInto<LocalizeEntry, ()> for LocalizeEntryRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> LocalizeEntry {
        LocalizeEntry {
            value: self.value.xfile_into(&mut xfile, ()),
            name: self.name.xfile_into(xfile, ()),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct XGlobalsRaw<'a> {
    name: XString<'a>,
    xanim_stream_buffer_size: i32,
    cinematic_max_width: i32,
    cinematic_max_height: i32,
    extracam_resolution: i32,
    gump_reserve: i32,
    screen_clear_color: [f32; 4],
}

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

impl<'a> XFileInto<XGlobals, ()> for XGlobalsRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> XGlobals {
        XGlobals {
            name: self.name.xfile_into(xfile, ()),
            xanim_stream_buffer_size: self.xanim_stream_buffer_size,
            cinematic_max_width: self.cinematic_max_width,
            cinematic_max_height: self.cinematic_max_height,
            extracam_resolution: self.extracam_resolution,
            gump_reserve: self.gump_reserve,
            screen_clear_color: self.screen_clear_color.into(),
        }
    }
}
