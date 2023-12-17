use crate::*;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct FontRaw<'a> {
    pub font_name: XString<'a>,
    pub pixel_height: i32,
    pub glyph_count: i32,
    pub material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub glow_material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub glyphs: Ptr32<'a, Glyph>,
}
assert_size!(FontRaw, 24);

pub struct Font {
    pub font_name: String,
    pub pixel_height: i32,
    pub material: Option<Box<techset::Material>>,
    pub glow_material: Option<Box<techset::Material>>,
    pub glyphs: Vec<Glyph>,
}

impl<'a> XFileInto<Font> for FontRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Font {
        Font {
            font_name: self.font_name.xfile_into(&mut xfile),
            pixel_height: self.pixel_height,
            material: self.material.xfile_into(&mut xfile),
            glow_material: self.glow_material.xfile_into(&mut xfile),
            glyphs: self.glyphs.to_array(self.glyph_count as _).to_vec(xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Glyph {
    pub letter: u16,
    pub x0: i8,
    pub y0: i8,
    pub dx: u8,
    pub pixel_width: u8,
    pub pixel_height: u8,
    pub s0: f32,
    pub to: f32,
    pub s1: f32,
    pub t1: f32,
}
assert_size!(Glyph, 24);
