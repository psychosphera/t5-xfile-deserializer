use alloc::{boxed::Box, vec::Vec};

use crate::{
    FatPointer, FatPointerCountFirstU32, FatPointerCountLastU32, Ptr32, Result, T5XFileDeserialize,
    T5XFileSerialize, XFileDeserializeInto, XFileSerialize, XString, XStringRaw, assert_size,
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
    pub name: XString,
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

impl XFileSerialize<()> for DdlRoot {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let ddl_defs = Ptr32::from_slice(&self.ddl_defs);

        let ddl_root = DdlRootRaw {
            name,
            ddl_def: ddl_defs,
        };

        ser.store_into_xfile(ddl_root)?;

        for ddl_def in &self.ddl_defs[..self.ddl_defs.len() - 2] {
            ddl_def.xfile_serialize(ser, false)?;
        }

        self.ddl_defs.last().unwrap().xfile_serialize(ser, true)
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

impl XFileSerialize<bool> for DdlDef {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, is_last: bool) -> Result<()> {
        let struct_list = FatPointerCountLastU32::from_slice(&self.struct_list);
        let enum_list = FatPointerCountLastU32::from_slice(&self.enum_list);
        let next = if is_last {
            Ptr32::null()
        } else {
            Ptr32::unreal()
        };
        let ddl_def_raw = DdlDefRaw {
            version: self.version,
            size: self.size,
            struct_list,
            enum_list,
            next,
        };

        ser.store_into_xfile(ddl_def_raw)?;
        self.struct_list.xfile_serialize(ser, ())?;
        self.enum_list.xfile_serialize(ser, ())
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
    pub name: XString,
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

impl XFileSerialize<()> for DdlStructDef {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let members = FatPointerCountFirstU32::from_slice(&self.members);

        let struct_def = DdlStructDefRaw {
            name,
            size: self.size,
            members,
        };

        ser.store_into_xfile(struct_def)?;
        self.name.xfile_serialize(ser, ())?;
        self.members.xfile_serialize(ser, ())
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
    pub name: XString,
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

impl XFileSerialize<()> for DdlMemberDef {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());

        let member_def = DdlMemberDefRaw {
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
        };

        ser.store_into_xfile(member_def)?;

        self.name.xfile_serialize(ser, ())
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
    pub name: XString,
    pub members: Vec<XString>,
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

impl XFileSerialize<()> for DdlEnumDef {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let members = FatPointerCountFirstU32::from_slice(&self.members);

        let enum_def = DdlEnumDefRaw { name, members };

        ser.store_into_xfile(enum_def)?;

        self.name.xfile_serialize(ser, ())?;
        self.members.xfile_serialize(ser, ())
    }
}
