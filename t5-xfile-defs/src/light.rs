use alloc::{boxed::Box, string::String};
use serde::{Deserialize, Serialize};

use crate::{
    Ptr32, Result, T5XFileDeserialize, XFileDeserializeInto, XString, assert_size,
    techset::{GfxImage, GfxImageRaw},
};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightDefRaw<'a> {
    pub name: XString<'a>,
    pub attenuation: GfxLightImageRaw<'a>,
    pub lmap_lookup_start: i32,
}
assert_size!(GfxLightDefRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightDef {
    pub name: String,
    pub attenuation: GfxLightImage,
    pub lmap_lookup_start: i32,
}

impl<'a> XFileDeserializeInto<GfxLightDef, ()> for GfxLightDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxLightDef> {
        //dbg!(self);
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        //dbg!(de.stream_pos()?);
        let attenuation = self.attenuation.xfile_deserialize_into(de, ())?;
        //dbg!(&attenuation);
        //dbg!(de.stream_pos()?);
        Ok(GfxLightDef {
            name,
            attenuation,
            lmap_lookup_start: self.lmap_lookup_start,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightImageRaw<'a> {
    pub image: Ptr32<'a, GfxImageRaw<'a>>,
    pub sampler_state: u8,
    pad: [u8; 3],
}
assert_size!(GfxLightImageRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightImage {
    pub image: Option<Box<GfxImage>>,
    pub sampler_state: u8,
}

impl<'a> XFileDeserializeInto<GfxLightImage, ()> for GfxLightImageRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxLightImage> {
        //dbg!(self);
        //dbg!(de.stream_pos()?);
        let image = self.image.xfile_deserialize_into(de, ())?;
        //dbg!(&image);
        //dbg!(de.stream_pos()?);
        Ok(GfxLightImage {
            image,
            sampler_state: self.sampler_state,
        })
    }
}
