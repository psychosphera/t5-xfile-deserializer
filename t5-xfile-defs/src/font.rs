use alloc::{boxed::Box, vec::Vec};

use crate::{
    FatPointer, Ptr32, Result, T5XFileDeserialize, T5XFileSerialize, XFileDeserializeInto,
    XFileSerialize, XString, XStringRaw, assert_size,
    techset::{Material, MaterialRaw},
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct FontRaw<'a> {
    pub font_name: XStringRaw<'a>,
    pub pixel_height: i32,
    pub glyph_count: i32,
    pub material: Ptr32<'a, MaterialRaw<'a>>,
    pub glow_material: Ptr32<'a, MaterialRaw<'a>>,
    pub glyphs: Ptr32<'a, Glyph>,
}
assert_size!(FontRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Font {
    pub font_name: XString,
    pub pixel_height: i32,
    pub material: Option<Box<Material>>,
    pub glow_material: Option<Box<Material>>,
    pub glyphs: Vec<Glyph>,
}

impl<'a> XFileDeserializeInto<Font, ()> for FontRaw<'a> {
    fn xfile_deserialize_into(&self, de: &mut impl T5XFileDeserialize, _data: ()) -> Result<Font> {
        Ok(Font {
            font_name: self.font_name.xfile_deserialize_into(de, ())?,
            pixel_height: self.pixel_height,
            material: self.material.xfile_deserialize_into(de, ())?,
            glow_material: self.glow_material.xfile_deserialize_into(de, ())?,
            glyphs: self.glyphs.to_array(self.glyph_count as _).to_vec(de)?,
        })
    }
}

impl<'a> XFileSerialize<()> for Font {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let font_name = XStringRaw::from_str(self.font_name.get());
        let glyph_count = self.glyphs.len() as _;
        let material = Ptr32::from_box::<MaterialRaw>(&self.material);
        let glow_material = Ptr32::from_box::<MaterialRaw>(&self.glow_material);
        let glyphs = Ptr32::from_slice::<Glyph>(&self.glyphs);

        let font = FontRaw {
            font_name,
            pixel_height: self.pixel_height,
            glyph_count,
            material,
            glow_material,
            glyphs,
        };

        ser.store_into_xfile(font)?;
        self.font_name.xfile_serialize(ser, ())?;
        self.material.xfile_serialize(ser, ())?;
        self.glow_material.xfile_serialize(ser, ())?;
        self.glyphs.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Glyph {
    pub letter: u16,
    pub x0: i8,
    pub y0: i8,
    pub dx: u8,
    pub pixel_width: u8,
    pub pixel_height: u8,
    pad: [u8; 1],
    pub s0: f32,
    pub to: f32,
    pub s1: f32,
    pub t1: f32,
}
assert_size!(Glyph, 24);

impl<T: Copy> XFileSerialize<T> for Glyph {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: T) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}
