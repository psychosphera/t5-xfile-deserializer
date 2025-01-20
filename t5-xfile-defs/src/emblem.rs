use crate::{
    FatPointer, FatPointerCountFirstU32, Ptr32, Result, T5XFileDeserialize, T5XFileSerialize,
    XFileDeserializeInto, XFileSerialize, XString, XStringRaw, assert_size,
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

impl XFileSerialize<()> for EmblemSet {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let color_count = self.color_count;
        let layers = FatPointerCountFirstU32::from_slice(&self.layers);
        let categories = FatPointerCountFirstU32::from_slice(&self.categories);
        let icons = FatPointerCountFirstU32::from_slice(&self.icons);
        let backgrounds = FatPointerCountFirstU32::from_slice(&self.backgrounds);
        let background_lookup = FatPointerCountFirstU32::from_slice(&self.background_lookup);

        let emblem_set = EmblemSetRaw {
            color_count,
            layers,
            categories,
            icons,
            backgrounds,
            background_lookup,
        };

        ser.store_into_xfile(emblem_set)?;
        self.layers.xfile_serialize(ser, ())?;
        self.categories.xfile_serialize(ser, ())?;
        self.icons.xfile_serialize(ser, ())?;
        self.backgrounds.xfile_serialize(ser, ())?;
        self.background_lookup.xfile_serialize(ser, ())
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

impl XFileSerialize<()> for EmblemLayer {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemCategoryRaw<'a> {
    pub name: XStringRaw<'a>,
    pub description: XStringRaw<'a>,
}
assert_size!(EmblemCategoryRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EmblemCategory {
    pub name: XString,
    pub description: XString,
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

impl XFileSerialize<()> for EmblemCategory {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let description = XStringRaw::from_str(self.name.get());

        let emblem_category = EmblemCategoryRaw { name, description };

        ser.store_into_xfile(emblem_category)?;
        self.name.xfile_serialize(ser, ())?;
        self.description.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemIconRaw<'a> {
    pub image: Ptr32<'a, GfxImageRaw<'a>>,
    pub description: XStringRaw<'a>,
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
    pub description: XString,
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

impl XFileSerialize<()> for EmblemIcon {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let image = Ptr32::from_box(&self.image);
        let description = XStringRaw::from_str(self.description.get());

        let emblem_icon = EmblemIconRaw {
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
        };

        ser.store_into_xfile(emblem_icon)?;
        self.image.xfile_serialize(ser, ())?;
        self.description.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EmblemBackgroundRaw<'a> {
    pub material: Ptr32<'a, MaterialRaw<'a>>,
    pub description: XStringRaw<'a>,
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
    pub description: XString,
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

impl XFileSerialize<()> for EmblemBackground {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let material = Ptr32::from_box(&self.material);
        let description = XStringRaw::from_str(self.description.get());

        let emblem_background = EmblemBackgroundRaw {
            material,
            description,
            cost: self.cost,
            unlock_level: self.unlock_level,
            unlock_plevel: self.unlock_plevel,
            unclassify_at: self.unclassify_at,
        };

        ser.store_into_xfile(emblem_background)?;
        self.material.xfile_serialize(ser, ())?;
        self.description.xfile_serialize(ser, ())
    }
}
