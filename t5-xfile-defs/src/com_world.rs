use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{
    FatPointer, FatPointerCountFirstU32, Ptr32ArrayConst, Result, T5XFileDeserialize,
    XFileDeserializeInto, XString, assert_size,
    common::{Vec3, Vec4},
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Default, Debug, Deserialize)]
pub(crate) struct ComWorldRaw<'a> {
    pub name: XString<'a>,
    pub is_in_use: i32,
    pub primary_lights: FatPointerCountFirstU32<'a, ComPrimaryLightRaw<'a>>,
    pub water_header: ComWaterHeader,
    pub water_cells: FatPointerCountFirstU32<'a, ComWaterCell>,
    pub burnable_header: ComBurnableHeader,
    pub burnable_cells: FatPointerCountFirstU32<'a, ComBurnableCellRaw<'a>>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct ComWorld {
    pub name: String,
    pub is_in_use: bool,
    pub primary_lights: Vec<ComPrimaryLight>,
    pub water_header: ComWaterHeader,
    pub water_cells: Vec<ComWaterCell>,
    pub burnable_header: ComBurnableHeader,
    pub burnable_cells: Vec<ComBurnableCell>,
}

impl<'a> XFileDeserializeInto<ComWorld, ()> for ComWorldRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ComWorld> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let primary_lights = self.primary_lights.xfile_deserialize_into(de, ())?;
        let water_cells = self.water_cells.to_vec(de)?;
        let burnable_cells = self.burnable_cells.xfile_deserialize_into(de, ())?;

        Ok(ComWorld {
            name,
            is_in_use: self.is_in_use != 0,
            primary_lights,
            water_header: self.water_header.clone(),
            water_cells,
            burnable_header: self.burnable_header.clone(),
            burnable_cells,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Default, Debug, Deserialize)]
pub(crate) struct ComPrimaryLightRaw<'a> {
    pub type_: u8,
    pub can_use_shadow_map: u8,
    pub exponent: u8,
    pub priority: u8,
    pub cull_dist: i16,
    #[allow(dead_code)]
    pad: [u8; 2],
    pub color: [f32; 3],
    pub dir: [f32; 3],
    pub origin: [f32; 3],
    pub radius: f32,
    pub cos_half_fov_outer: f32,
    pub cos_half_fov_inner: f32,
    pub cos_half_fov_expanded: f32,
    pub rotation_limit: f32,
    pub translation_limit: f32,
    pub mip_distance: f32,
    pub diffuse_color: [f32; 4],
    pub specular_color: [f32; 4],
    pub attenuation: [f32; 4],
    pub falloff: [f32; 4],
    pub angle: [f32; 4],
    pub aabb: [f32; 4],
    pub cookie_control_0: [f32; 4],
    pub cookie_control_1: [f32; 4],
    pub cookie_control_2: [f32; 4],
    pub def_name: XString<'a>,
}
assert_size!(ComPrimaryLightRaw, 220);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct ComPrimaryLight {
    pub type_: u8,
    pub can_use_shadow_map: bool,
    pub exponent: u8,
    pub priority: u8,
    pub cull_dist: i16,
    pub color: Vec3,
    pub dir: Vec3,
    pub origin: Vec3,
    pub radius: f32,
    pub cos_half_fov_outer: f32,
    pub cos_half_fov_inner: f32,
    pub cos_half_fov_expanded: f32,
    pub rotation_limit: f32,
    pub translation_limit: f32,
    pub mip_distance: f32,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
    pub attenuation: Vec4,
    pub falloff: Vec4,
    pub angle: Vec4,
    pub aabb: Vec4,
    pub cookie_control_0: Vec4,
    pub cookie_control_1: Vec4,
    pub cookie_control_2: Vec4,
    pub def_name: String,
}

impl<'a> XFileDeserializeInto<ComPrimaryLight, ()> for ComPrimaryLightRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ComPrimaryLight> {
        let color = self.color.into();
        let dir = self.dir.into();
        let origin = self.origin.into();
        let diffuse_color = self.diffuse_color.into();
        let specular_color = self.specular_color.into();
        let attenuation = self.attenuation.into();
        let falloff = self.falloff.into();
        let angle = self.angle.into();
        let aabb = self.aabb.into();
        let cookie_control_0 = self.cookie_control_0.into();
        let cookie_control_1 = self.cookie_control_1.into();
        let cookie_control_2 = self.cookie_control_2.into();
        let def_name = self.def_name.xfile_deserialize_into(de, ())?;

        Ok(ComPrimaryLight {
            type_: self.type_,
            can_use_shadow_map: self.can_use_shadow_map != 0,
            exponent: self.exponent,
            priority: self.priority,
            cull_dist: self.cull_dist,
            color,
            dir,
            origin,
            radius: self.radius,
            cos_half_fov_outer: self.cos_half_fov_outer,
            cos_half_fov_inner: self.cos_half_fov_inner,
            cos_half_fov_expanded: self.cos_half_fov_expanded,
            rotation_limit: self.rotation_limit,
            translation_limit: self.translation_limit,
            mip_distance: self.mip_distance,
            diffuse_color,
            specular_color,
            attenuation,
            falloff,
            angle,
            aabb,
            cookie_control_0,
            cookie_control_1,
            cookie_control_2,
            def_name,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Default, Debug, Deserialize)]
pub struct ComWaterHeader {
    pub minx: i32,
    pub miny: i32,
    pub maxx: i32,
    pub maxy: i32,
}
assert_size!(ComWaterHeader, 16);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Default, Debug, Deserialize)]
pub struct ComWaterCell {
    pub waterheight: i16,
    pub flooroffset: u8,
    pub shoredist: u8,
    pub color: [u8; 4],
}
assert_size!(ComWaterCell, 8);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Default, Debug, Deserialize)]
pub struct ComBurnableHeader {
    pub minx: i32,
    pub miny: i32,
    pub maxx: i32,
    pub maxy: i32,
}
assert_size!(ComWaterHeader, 16);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Default, Debug, Deserialize)]
pub(crate) struct ComBurnableCellRaw<'a> {
    pub x: i32,
    pub y: i32,
    pub data: Ptr32ArrayConst<'a, ComBurnableSample, 32>,
}
assert_size!(ComWaterHeader, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct ComBurnableCell {
    pub x: i32,
    pub y: i32,
    pub data: Option<Box<[ComBurnableSample; 32]>>,
}

impl<'a> XFileDeserializeInto<ComBurnableCell, ()> for ComBurnableCellRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ComBurnableCell> {
        let data = if self.data.is_null() {
            None
        } else {
            Some(Box::new(self.data.to_vec(de)?.try_into().unwrap()))
        };
        Ok(ComBurnableCell {
            x: self.x,
            y: self.y,
            data,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Default, Debug, Deserialize)]
pub struct ComBurnableSample {
    pub state: u8,
}
assert_size!(ComBurnableSample, 1);
