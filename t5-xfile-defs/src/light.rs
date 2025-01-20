use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::prelude::*;

use crate::{
    Ptr32, Result, T5XFileDeserialize, T5XFileSerialize, XFileDeserializeInto, XFileSerialize,
    XString, XStringRaw, assert_size,
    techset::{GfxImage, GfxImageRaw},
};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightDefRaw<'a> {
    pub name: XStringRaw<'a>,
    pub attenuation: GfxLightImageRaw<'a>,
    pub lmap_lookup_start: i32,
}
assert_size!(GfxLightDefRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightDef {
    pub name: XString,
    pub attenuation: GfxLightImage,
    pub lmap_lookup_start: i32,
}

impl XFileSerialize<()> for GfxLightDef {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let attenuation = GfxLightImageRaw {
            image: Ptr32::from_box(&self.attenuation.image),
            sampler_state: self.attenuation.sampler_state,
            pad: [0u8; 3],
        };

        let light_def = GfxLightDefRaw {
            name,
            attenuation,
            lmap_lookup_start: self.lmap_lookup_start,
        };

        ser.store_into_xfile(light_def)?;
        self.name.xfile_serialize(ser, ())?;
        self.attenuation.xfile_serialize(ser, ())
    }
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

impl XFileSerialize<()> for GfxLightImage {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let image = Ptr32::from_box(&self.image);

        let light_image = GfxLightImageRaw {
            image,
            sampler_state: self.sampler_state,
            pad: [0u8; 3],
        };

        ser.store_into_xfile(light_image)?;
        self.image.xfile_serialize(ser, ())
    }
}
