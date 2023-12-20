use crate::*;

#[derive(Clone, Debug, Deserialize)]
pub struct GfxLightDefRaw<'a> {
    pub name: XString<'a>,
    pub attenuation: GfxLightImageRaw<'a>,
    pub lmap_lookup_start: i32,
}

pub struct GfxLightDef {
    pub name: String,
    pub attenuation: GfxLightImage,
    pub lmap_lookup_start: i32,
}

impl<'a> XFileInto<GfxLightDef, ()> for GfxLightDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> GfxLightDef {
        GfxLightDef {
            name: self.name.xfile_into(&mut xfile, ()),
            attenuation: self.attenuation.xfile_into(xfile, ()),
            lmap_lookup_start: self.lmap_lookup_start,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GfxLightImageRaw<'a> {
    pub image: Ptr32<'a, techset::GfxImageRaw<'a>>,
    pub sampler_state: u8,
}

pub struct GfxLightImage {
    pub image: Option<Box<techset::GfxImage>>,
    pub sampler_state: u8,
}

impl<'a> XFileInto<GfxLightImage, ()> for GfxLightImageRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> GfxLightImage {
        GfxLightImage {
            image: self.image.xfile_into(xfile, ()),
            sampler_state: self.sampler_state,
        }
    }
}
