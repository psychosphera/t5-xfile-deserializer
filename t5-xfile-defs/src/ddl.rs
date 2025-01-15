use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{
    FatPointerCountFirstU32, FatPointerCountLastU32, Ptr32, Result, T5XFileDeserialize,
    XFileDeserializeInto, XStringRaw, assert_size,
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct DdlRootRaw<'a> {
    pub name: XStringRaw<'a>,
    pub ddl_def: Ptr32<'a, DdlDefRaw<'a>>,
}
assert_size!(DdlRootRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DdlRoot {
    pub name: String,
    pub ddl_defs: Vec<Box<DdlDef>>,
}

impl<'a> XFileDeserializeInto<DdlRoot, ()> for DdlRootRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<DdlRoot> {
        let name = self.name.xfile_deserialize_into(de, ())?;

        let mut ddl_defs = Vec::new();
        let mut ddl_def_raw = self.ddl_def;

        loop {
            if ddl_def_raw.is_null() {
                break;
            }

            let ddl_def = ddl_def_raw.xfile_get(de)?.unwrap();
            ddl_def_raw = ddl_def.next;
            ddl_defs.push(Box::new(ddl_def.xfile_deserialize_into(de, ())?));
        }

        Ok(DdlRoot { name, ddl_defs })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct DdlDefRaw<'a> {
    pub version: i32,
    pub size: i32,
    pub struct_list: FatPointerCountLastU32<'a, DdlStructDefRaw<'a>>,
    pub enum_list: FatPointerCountLastU32<'a, DdlEnumDefRaw<'a>>,
    pub next: Ptr32<'a, Self>,
}
assert_size!(DdlDefRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DdlDef {
    pub version: i32,
    pub size: i32,
    pub struct_list: Vec<DdlStructDef>,
    pub enum_list: Vec<DdlEnumDef>,
}

impl<'a> XFileDeserializeInto<DdlDef, ()> for DdlDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<DdlDef> {
        let struct_list = self.struct_list.xfile_deserialize_into(de, ())?;
        let enum_list = self.enum_list.xfile_deserialize_into(de, ())?;

        Ok(DdlDef {
            version: self.version,
            size: self.size,
            struct_list,
            enum_list,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct DdlStructDefRaw<'a> {
    pub name: XStringRaw<'a>,
    pub size: i32,
    pub members: FatPointerCountFirstU32<'a, DdlMemberDefRaw<'a>>,
}
assert_size!(DdlStructDefRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DdlStructDef {
    pub name: String,
    pub size: i32,
    pub members: Vec<DdlMemberDef>,
}

impl<'a> XFileDeserializeInto<DdlStructDef, ()> for DdlStructDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<DdlStructDef> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let members = self.members.xfile_deserialize_into(de, ())?;

        Ok(DdlStructDef {
            name,
            size: self.size,
            members,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct DdlMemberDefRaw<'a> {
    pub name: XStringRaw<'a>,
    pub size: i32,
    pub offset: i32,
    pub type_: i32,
    pub external_index: i32,
    pub min: u32,
    pub max: u32,
    pub server_delta: u32,
    pub client_delta: u32,
    pub array_size: i32,
    pub enum_index: i32,
    pub permission: i32,
}
assert_size!(DdlMemberDefRaw, 48);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DdlMemberDef {
    pub name: String,
    pub size: i32,
    pub offset: i32,
    pub type_: i32,
    pub external_index: i32,
    pub min: u32,
    pub max: u32,
    pub server_delta: u32,
    pub client_delta: u32,
    pub array_size: i32,
    pub enum_index: i32,
    pub permission: i32,
}

impl<'a> XFileDeserializeInto<DdlMemberDef, ()> for DdlMemberDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<DdlMemberDef> {
        let name = self.name.xfile_deserialize_into(de, ())?;

        Ok(DdlMemberDef {
            name,
            size: self.size,
            offset: self.offset,
            type_: self.type_,
            external_index: self.external_index,
            min: self.min,
            max: self.max,
            server_delta: self.server_delta,
            client_delta: self.client_delta,
            array_size: self.array_size,
            enum_index: self.enum_index,
            permission: self.permission,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct DdlEnumDefRaw<'a> {
    pub name: XStringRaw<'a>,
    pub members: FatPointerCountFirstU32<'a, XStringRaw<'a>>,
}
assert_size!(DdlEnumDefRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DdlEnumDef {
    pub name: String,
    pub members: Vec<String>,
}

impl<'a> XFileDeserializeInto<DdlEnumDef, ()> for DdlEnumDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<DdlEnumDef> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let members = self.members.xfile_deserialize_into(de, ())?;

        Ok(DdlEnumDef { name, members })
    }
}
