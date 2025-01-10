use crate::{
    FatPointer, FatPointerCountFirstU32, Ptr32, Result, T5XFileDeserialize, XFileDeserializeInto,
    XString, assert_size,
    techset::{GfxImage, GfxImageRaw, Material, MaterialRaw},
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemSetRaw<'a> {
    pub color_count: i32,
    pub layers: FatPointerCountFirstU32<'a, EmblemLayer>,
    pub categories: FatPointerCountFirstU32<'a, EmblemCategoryRaw<'a>>,
    pub icons: FatPointerCountFirstU32<'a, EmblemIconRaw<'a>>,
    pub backgrounds: FatPointerCountFirstU32<'a, EmblemBackgroundRaw<'a>>,
    pub background_lookup: FatPointerCountFirstU32<'a, u16>,
}
assert_size!(EmblemSetRaw, 44);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EmblemSet {
    pub color_count: i32,
    pub layers: Vec<EmblemLayer>,
    pub categories: Vec<EmblemCategory>,
    pub icons: Vec<EmblemIcon>,
    pub backgrounds: Vec<EmblemBackground>,
    pub background_lookup: Vec<u16>,
}

impl<'a> XFileDeserializeInto<EmblemSet, ()> for EmblemSetRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<EmblemSet> {
        let layers = self.layers.to_vec(de)?;
        let categories = self.categories.xfile_deserialize_into(de, ())?;
        let icons = self.icons.xfile_deserialize_into(de, ())?;
        let backgrounds = self.backgrounds.xfile_deserialize_into(de, ())?;
        let background_lookup = self.background_lookup.to_vec(de)?;

        Ok(EmblemSet {
            color_count: self.color_count,
            layers,
            categories,
            icons,
            backgrounds,
            background_lookup,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct EmblemLayer {
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
}
assert_size!(EmblemLayer, 12);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemCategoryRaw<'a> {
    pub name: XString<'a>,
    pub description: XString<'a>,
}
assert_size!(EmblemCategoryRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EmblemCategory {
    pub name: String,
    pub description: String,
}

impl<'a> XFileDeserializeInto<EmblemCategory, ()> for EmblemCategoryRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<EmblemCategory> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let description = self.description.xfile_deserialize_into(de, ())?;

        Ok(EmblemCategory { name, description })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemIconRaw<'a> {
    pub image: Ptr32<'a, GfxImageRaw<'a>>,
    pub description: XString<'a>,
    pub outline_size: f32,
    pub default_color: i32,
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
    pub unclassify_at: i32,
    pub sort_key: i32,
    pub category: u32,
}
assert_size!(EmblemIconRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EmblemIcon {
    pub image: Option<Box<GfxImage>>,
    pub description: String,
    pub outline_size: f32,
    pub default_color: i32,
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
    pub unclassify_at: i32,
    pub sort_key: i32,
    pub category: u32,
}

impl<'a> XFileDeserializeInto<EmblemIcon, ()> for EmblemIconRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<EmblemIcon> {
        let image = self.image.xfile_deserialize_into(de, ())?;
        let description = self.description.xfile_deserialize_into(de, ())?;

        Ok(EmblemIcon {
            image,
            description,
            outline_size: self.outline_size,
            default_color: self.default_color,
            cost: self.cost,
            unlock_level: self.unlock_level,
            unlock_plevel: self.unlock_plevel,
            unclassify_at: self.unclassify_at,
            sort_key: self.sort_key,
            category: self.category,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemBackgroundRaw<'a> {
    pub material: Ptr32<'a, MaterialRaw<'a>>,
    pub description: XString<'a>,
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
    pub unclassify_at: i32,
}
assert_size!(EmblemBackgroundRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EmblemBackground {
    pub material: Option<Box<Material>>,
    pub description: String,
    pub cost: i32,
    pub unlock_level: i32,
    pub unlock_plevel: i32,
    pub unclassify_at: i32,
}

impl<'a> XFileDeserializeInto<EmblemBackground, ()> for EmblemBackgroundRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<EmblemBackground> {
        let material = self.material.xfile_deserialize_into(de, ())?;
        let description = self.description.xfile_deserialize_into(de, ())?;

        Ok(EmblemBackground {
            material,
            description,
            cost: self.cost,
            unlock_level: self.unlock_level,
            unlock_plevel: self.unlock_plevel,
            unclassify_at: self.unclassify_at,
        })
    }
}
