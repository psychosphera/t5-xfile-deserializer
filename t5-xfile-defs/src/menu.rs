use core::mem::transmute;

use alloc::{boxed::Box, format, string::String, vec::Vec};

use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::prelude::*;

use crate::{
    Error, ErrorKind, FatPointerCountFirstU32, FatPointerCountLastU32, Ptr32, Result,
    T5XFileDeserialize, XFileDeserializeInto, XStringRaw, assert_size,
    common::Vec4,
    file_line_col,
    techset::{Material, MaterialRaw},
};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MenuListRaw<'a, const MAX_LOCAL_CLIENTS: usize> {
    pub name: XStringRaw<'a>,
    pub menus: FatPointerCountFirstU32<'a, Ptr32<'a, MenuDefRaw<'a, MAX_LOCAL_CLIENTS>>>,
}
assert_size!(MenuListRaw<1>, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MenuList<const MAX_LOCAL_CLIENTS: usize> {
    pub name: String,
    pub menus: Vec<Box<MenuDef<MAX_LOCAL_CLIENTS>>>,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<MenuList<MAX_LOCAL_CLIENTS>, ()>
    for MenuListRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MenuList<MAX_LOCAL_CLIENTS>> {
        //dbg!(self);
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        //dbg!(de.stream_pos()?);
        let menus = self
            .menus
            .xfile_deserialize_into(de, ())?
            .into_iter()
            .flatten()
            .collect();
        //dbg!(&menus);
        //dbg!(de.stream_pos()?);

        Ok(MenuList { name, menus })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct MenuDefRaw<'a, const MAX_LOCAL_CLIENTS: usize> {
    pub window: WindowDefRaw<'a, MAX_LOCAL_CLIENTS>,
    pub font: XStringRaw<'a>,
    pub full_screen: i32,
    pub ui_3d_window_id: i32,
    pub item_count: i32,
    pub font_index: i32,
    #[serde(with = "serde_arrays")]
    pub cursor_item: [i32; MAX_LOCAL_CLIENTS],
    pub fade_cycle: i32,
    pub priority: i32,
    pub fade_clamp: f32,
    pub fade_amount: f32,
    pub fade_in_amount: f32,
    pub blur_radius: f32,
    pub open_slide_speed: i32,
    pub close_slide_speed: i32,
    pub open_slide_direction: i32,
    pub close_slide_direction: i32,
    pub intial_rect_info: RectDefRaw,
    pub open_fading_time: i32,
    pub close_fading_time: i32,
    pub fade_time_counter: i32,
    pub slide_time_counter: i32,
    pub on_event: Ptr32<'a, GenericEventHandlerRaw<'a>>,
    pub on_key: Ptr32<'a, ItemKeyHandlerRaw<'a>>,
    pub visible_exp: ExpressionStatementRaw<'a>,
    #[allow(dead_code)]
    pad: [u8; 4],
    pub show_bits: u64,
    pub hide_bits: u64,
    pub allowed_binding: XStringRaw<'a>,
    pub sound_name: XStringRaw<'a>,
    pub image_track: i32,
    pub control: i32,
    pub focus_color: [f32; 4],
    pub disable_color: [f32; 4],
    pub rect_x_exp: ExpressionStatementRaw<'a>,
    pub rect_y_exp: ExpressionStatementRaw<'a>,
    pub items: Ptr32<'a, Ptr32<'a, ItemDefRaw<'a, MAX_LOCAL_CLIENTS>>>,
    #[allow(dead_code)]
    pad2: [u8; 4],
}
assert_size!(MenuDefRaw<1>, 400);

impl<'a, const MAX_LOCAL_CLIENTS: usize> Default for MenuDefRaw<'a, MAX_LOCAL_CLIENTS> {
    fn default() -> Self {
        Self {
            window: WindowDefRaw::default(),
            font: XStringRaw::default(),
            full_screen: i32::default(),
            ui_3d_window_id: i32::default(),
            item_count: i32::default(),
            font_index: i32::default(),
            cursor_item: [i32::default(); MAX_LOCAL_CLIENTS],
            fade_cycle: i32::default(),
            priority: i32::default(),
            fade_clamp: f32::default(),
            fade_amount: f32::default(),
            fade_in_amount: f32::default(),
            blur_radius: f32::default(),
            open_slide_speed: i32::default(),
            close_slide_speed: i32::default(),
            open_slide_direction: i32::default(),
            close_slide_direction: i32::default(),
            intial_rect_info: RectDefRaw::default(),
            open_fading_time: i32::default(),
            close_fading_time: i32::default(),
            fade_time_counter: i32::default(),
            slide_time_counter: i32::default(),
            on_event: Ptr32::default(),
            on_key: Ptr32::default(),
            visible_exp: ExpressionStatementRaw::default(),
            pad: [u8::default(); 4],
            show_bits: u64::default(),
            hide_bits: u64::default(),
            allowed_binding: XStringRaw::default(),
            sound_name: XStringRaw::default(),
            image_track: i32::default(),
            control: i32::default(),
            focus_color: [f32::default(); 4],
            disable_color: [f32::default(); 4],
            rect_x_exp: ExpressionStatementRaw::default(),
            rect_y_exp: ExpressionStatementRaw::default(),
            items: Ptr32::default(),
            pad2: [u8::default(); 4],
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MenuDef<const MAX_LOCAL_CLIENTS: usize> {
    pub window: WindowDef<MAX_LOCAL_CLIENTS>,
    pub font: String,
    pub full_screen: bool,
    pub ui_3d_window_id: i32,
    pub font_index: i32,
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub cursor_item: [i32; MAX_LOCAL_CLIENTS],
    pub fade_cycle: i32,
    pub priority: i32,
    pub fade_clamp: f32,
    pub fade_amount: f32,
    pub fade_in_amount: f32,
    pub blur_radius: f32,
    pub open_slide_speed: i32,
    pub close_slide_speed: i32,
    pub open_slide_direction: i32,
    pub close_slide_direction: i32,
    pub intial_rect_info: RectDef,
    pub open_fading_time: i32,
    pub close_fading_time: i32,
    pub fade_time_counter: i32,
    pub slide_time_counter: i32,
    pub on_event: Option<Box<GenericEventHandler>>,
    pub on_key: Option<Box<ItemKeyHandler>>,
    pub visible_exp: ExpressionStatement,
    pub show_bits: u64,
    pub hide_bits: u64,
    pub allowed_binding: String,
    pub sound_name: String,
    pub image_track: i32,
    pub control: i32,
    pub focus_color: Vec4,
    pub disable_color: Vec4,
    pub rect_x_exp: ExpressionStatement,
    pub rect_y_exp: ExpressionStatement,
    pub items: Vec<Box<ItemDef<MAX_LOCAL_CLIENTS>>>,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<MenuDef<MAX_LOCAL_CLIENTS>, ()>
    for MenuDefRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MenuDef<MAX_LOCAL_CLIENTS>> {
        //dbg!(de.stream_pos()?);
        //dbg!(self);
        let window = self.window.xfile_deserialize_into(de, ())?;
        //dbg!(&window);
        //dbg!(de.stream_pos()?);
        let font = self.font.xfile_deserialize_into(de, ())?;
        //dbg!(&font);
        //dbg!(de.stream_pos()?);
        let on_event = self.on_event.xfile_deserialize_into(de, ())?;
        //dbg!(&on_event);
        //dbg!(de.stream_pos()?);
        let on_key = self.on_key.xfile_deserialize_into(de, ())?;
        //dbg!(&on_key);
        //dbg!(de.stream_pos()?);
        let visible_exp = self.visible_exp.xfile_deserialize_into(de, ())?;
        //dbg!(&visible_exp);
        //dbg!(de.stream_pos()?);
        let allowed_binding = self.allowed_binding.xfile_deserialize_into(de, ())?;
        //dbg!(&allowed_binding);
        //dbg!(de.stream_pos()?);
        let sound_name = self.sound_name.xfile_deserialize_into(de, ())?;
        //dbg!(&sound_name);
        //dbg!(de.stream_pos()?);
        let rect_x_exp = self.rect_x_exp.xfile_deserialize_into(de, ())?;
        //dbg!(&rect_x_exp);
        //dbg!(de.stream_pos()?);
        let rect_y_exp = self.rect_y_exp.xfile_deserialize_into(de, ())?;
        //dbg!(&rect_y_exp);
        //dbg!(de.stream_pos()?);
        let items = self
            .items
            .to_array(self.item_count as _)
            .xfile_deserialize_into(de, ())?
            .into_iter()
            .flatten()
            .collect();
        //dbg!(&items);
        //dbg!(de.stream_pos()?);

        let focus_color = self.focus_color.into();
        let disable_color = self.disable_color.into();
        let full_screen = self.full_screen != 0;
        let intial_rect_info = self.intial_rect_info.into();

        Ok(MenuDef {
            window,
            font,
            full_screen,
            ui_3d_window_id: self.ui_3d_window_id,
            font_index: self.font_index,
            cursor_item: self.cursor_item,
            fade_cycle: self.fade_cycle,
            priority: self.priority,
            fade_clamp: self.fade_clamp,
            fade_amount: self.fade_amount,
            fade_in_amount: self.fade_in_amount,
            blur_radius: self.blur_radius,
            open_slide_speed: self.open_slide_speed,
            close_slide_speed: self.close_slide_speed,
            open_slide_direction: self.open_slide_direction,
            close_slide_direction: self.close_slide_direction,
            intial_rect_info,
            open_fading_time: self.open_fading_time,
            close_fading_time: self.close_fading_time,
            fade_time_counter: self.fade_time_counter,
            slide_time_counter: self.slide_time_counter,
            on_event,
            on_key,
            visible_exp,
            show_bits: self.show_bits,
            hide_bits: self.hide_bits,
            allowed_binding,
            sound_name,
            image_track: self.image_track,
            control: self.control,
            focus_color,
            disable_color,
            rect_x_exp,
            rect_y_exp,
            items,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct WindowDefRaw<'a, const MAX_LOCAL_CLIENTS: usize> {
    pub name: XStringRaw<'a>,
    pub rect: RectDefRaw,
    pub rect_client: RectDefRaw,
    pub group: XStringRaw<'a>,
    pub style: u8,
    pub border: u8,
    pub modal: u8,
    pub frame_sides: u8,
    pub frame_tex_size: f32,
    pub frame_size: f32,
    pub owner_draw: i32,
    pub owner_draw_flags: i32,
    pub border_size: f32,
    pub static_flags: i32,
    #[serde(with = "serde_arrays")]
    pub dynamic_flags: [i32; MAX_LOCAL_CLIENTS],
    pub next_time: i32,
    pub fore_color: [f32; 4],
    pub back_color: [f32; 4],
    pub border_color: [f32; 4],
    pub outline_color: [f32; 4],
    pub rotation: f32,
    pub background: Ptr32<'a, MaterialRaw<'a>>,
}
assert_size!(WindowDefRaw<1>, 164);

impl<'a, const MAX_LOCAL_CLIENTS: usize> Default for WindowDefRaw<'a, MAX_LOCAL_CLIENTS> {
    fn default() -> Self {
        Self {
            name: XStringRaw::default(),
            rect: RectDefRaw::default(),
            rect_client: RectDefRaw::default(),
            group: XStringRaw::default(),
            style: u8::default(),
            border: u8::default(),
            modal: u8::default(),
            frame_sides: u8::default(),
            frame_tex_size: f32::default(),
            frame_size: f32::default(),
            owner_draw: i32::default(),
            owner_draw_flags: i32::default(),
            border_size: f32::default(),
            static_flags: i32::default(),
            dynamic_flags: [i32::default(); MAX_LOCAL_CLIENTS],
            next_time: i32::default(),
            fore_color: [f32::default(); 4],
            back_color: [f32::default(); 4],
            border_color: [f32::default(); 4],
            outline_color: [f32::default(); 4],
            rotation: f32::default(),
            background: Ptr32::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct WindowDef<const MAX_LOCAL_CLIENTS: usize> {
    pub name: String,
    pub rect: RectDef,
    pub rect_client: RectDef,
    pub group: String,
    pub style: u8,
    pub border: u8,
    pub modal: u8,
    pub frame_sides: u8,
    pub frame_tex_size: f32,
    pub frame_size: f32,
    pub owner_draw: i32,
    pub owner_draw_flags: i32,
    pub border_size: f32,
    pub static_flags: i32,
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub dynamic_flags: [i32; MAX_LOCAL_CLIENTS],
    pub next_time: i32,
    pub fore_color: Vec4,
    pub back_color: Vec4,
    pub border_color: Vec4,
    pub outline_color: Vec4,
    pub rotation: f32,
    pub background: Option<Box<Material>>,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<WindowDef<MAX_LOCAL_CLIENTS>, ()>
    for WindowDefRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<WindowDef<MAX_LOCAL_CLIENTS>> {
        //dbg!(de.stream_pos()?);
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        //dbg!(de.stream_pos()?);
        let group = self.group.xfile_deserialize_into(de, ())?;
        //dbg!(&group);
        //dbg!(de.stream_pos()?);
        let background = self.background.xfile_deserialize_into(de, ())?;
        //dbg!(&background);
        //dbg!(de.stream_pos()?);

        let rect = self.rect.into();
        let rect_client = self.rect_client.into();
        let fore_color = self.fore_color.into();
        let back_color = self.back_color.into();
        let border_color = self.border_color.into();
        let outline_color = self.outline_color.into();

        Ok(WindowDef {
            name,
            rect,
            rect_client,
            group,
            style: self.style,
            border: self.border,
            modal: self.modal,
            frame_sides: self.frame_sides,
            frame_tex_size: self.frame_tex_size,
            frame_size: self.frame_size,
            owner_draw: self.owner_draw,
            owner_draw_flags: self.owner_draw_flags,
            border_size: self.border_size,
            static_flags: self.static_flags,
            dynamic_flags: self.dynamic_flags,
            next_time: self.next_time,
            fore_color,
            back_color,
            border_color,
            outline_color,
            rotation: self.rotation,
            background,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct RectDefRaw {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub horz_align: i32,
    pub vert_align: i32,
}
assert_size!(RectDefRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RectDef {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub horz_align: i32,
    pub vert_align: i32,
}

impl Into<RectDef> for RectDefRaw {
    fn into(self) -> RectDef {
        RectDef {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            horz_align: self.horz_align,
            vert_align: self.vert_align,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GenericEventHandlerRaw<'a> {
    pub name: XStringRaw<'a>,
    pub event_script: Ptr32<'a, GenericEventScriptRaw<'a>>,
    pub next: Ptr32<'a, GenericEventHandlerRaw<'a>>,
}
assert_size!(GenericEventHandlerRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GenericEventHandler {
    pub name: String,
    pub event_script: Option<Box<GenericEventScript>>,
    pub next: Option<Box<Self>>, // TODO
}

impl<'a> XFileDeserializeInto<GenericEventHandler, ()> for GenericEventHandlerRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GenericEventHandler> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let event_script = self.event_script.xfile_deserialize_into(de, ())?;
        let next = if self.next.is_null() {
            None
        } else {
            self.next.xfile_deserialize_into(de, ())?
        };

        Ok(GenericEventHandler {
            name,
            event_script,
            next,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GenericEventScriptRaw<'a> {
    pub prerequisites: Ptr32<'a, ScriptConditionRaw<'a>>,
    pub condition: ExpressionStatementRaw<'a>,
    pub type_: i32,
    pub fire_on_true: bool,
    pad: [u8; 3],
    pub action: XStringRaw<'a>,
    pub block_id: i32,
    pub construct_id: i32,
    pub next: Ptr32<'a, GenericEventScriptRaw<'a>>,
}
assert_size!(GenericEventScriptRaw, 44);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GenericEventScript {
    pub prerequisites: Option<Box<ScriptCondition>>,
    pub condition: ExpressionStatement,
    pub type_: i32,
    pub fire_on_true: bool,
    pub action: String,
    pub block_id: i32,
    pub construct_id: i32,
    pub next: Option<Box<Self>>, // TODO,
}

impl<'a> XFileDeserializeInto<GenericEventScript, ()> for GenericEventScriptRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GenericEventScript> {
        let prerequisites = self.prerequisites.xfile_deserialize_into(de, ())?;
        let condition = self.condition.xfile_deserialize_into(de, ())?;
        let action = self.action.xfile_deserialize_into(de, ())?;
        let next = if self.next.is_null() {
            None
        } else {
            self.next.xfile_deserialize_into(de, ())?
        };

        Ok(GenericEventScript {
            prerequisites,
            condition,
            type_: self.type_,
            fire_on_true: self.fire_on_true,
            action,
            block_id: self.block_id,
            construct_id: self.construct_id,
            next,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ScriptConditionRaw<'a> {
    pub fire_on_true: bool,
    pad: [u8; 3],
    pub block_id: i32,
    pub construct_id: i32,
    pub next: Ptr32<'a, ScriptConditionRaw<'a>>,
}
assert_size!(ScriptConditionRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ScriptCondition {
    pub fire_on_true: bool,
    pub block_id: i32,
    pub construct_id: i32,
    pub next: Option<Box<Self>>, // TODO
}

impl<'a> XFileDeserializeInto<ScriptCondition, ()> for ScriptConditionRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ScriptCondition> {
        let next = if self.next.is_null() {
            None
        } else {
            self.next.xfile_deserialize_into(de, ())?
        };

        Ok(ScriptCondition {
            fire_on_true: self.fire_on_true,
            block_id: self.block_id,
            construct_id: self.construct_id,
            next,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ExpressionStatementRaw<'a> {
    pub filename: XStringRaw<'a>,
    pub line: i32,
    pub rpn: FatPointerCountFirstU32<'a, ExpressionRpnRaw>,
}
assert_size!(ExpressionStatementRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ExpressionStatement {
    pub filename: String,
    pub line: i32,
    pub rpn: Vec<ExpressionRpn>,
}

impl<'a> XFileDeserializeInto<ExpressionStatement, ()> for ExpressionStatementRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ExpressionStatement> {
        let filename = self.filename.xfile_deserialize_into(de, ())?;
        let rpn = self.rpn.xfile_deserialize_into(de, ())?;

        Ok(ExpressionStatement {
            filename,
            line: self.line,
            rpn,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ExpressionRpnRaw {
    pub type_: i32,
    pub data: ExpressionRpnDataUnionRaw,
}
assert_size!(ExpressionRpnRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ExpressionRpn {
    pub data: Option<ExpressionRpnDataUnion>,
}

impl XFileDeserializeInto<ExpressionRpn, ()> for ExpressionRpnRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ExpressionRpn> {
        let data = self.data.xfile_deserialize_into(de, self.type_)?;
        Ok(ExpressionRpn { data })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ExpressionRpnDataUnionRaw([u8; 8]);
assert_size!(ExpressionRpnDataUnionRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum ExpressionRpnDataUnion {
    Constant(Operand),
    CmdIdx(i32),
}

impl XFileDeserializeInto<Option<ExpressionRpnDataUnion>, i32> for ExpressionRpnDataUnionRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        type_: i32,
    ) -> Result<Option<ExpressionRpnDataUnion>> {
        if type_ == 0 {
            Ok(Some(ExpressionRpnDataUnion::Constant(
                unsafe { transmute::<_, OperandRaw>(self.0) }.xfile_deserialize_into(de, ())?,
            )))
        } else {
            Ok(None)
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct OperandRaw {
    pub data_type: i32,
    pub internals: OperandInternalDataUnionRaw,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, FromPrimitive)]
#[repr(i32)]
pub(crate) enum ExpDataType {
    #[default]
    INT = 0,
    FLOAT = 1,
    STRING = 2,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Operand {
    pub internals: OperandInternalDataUnion,
}

impl XFileDeserializeInto<Operand, ()> for OperandRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<Operand> {
        let data_type = FromPrimitive::from_i32(self.data_type).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.data_type as _),
        ))?;
        let internals = self.internals.xfile_deserialize_into(de, data_type)?;
        Ok(Operand { internals })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct OperandInternalDataUnionRaw(u32);
assert_size!(OperandInternalDataUnionRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum OperandInternalDataUnion {
    Int(i32),
    Float(f32),
    String(String),
}

impl XFileDeserializeInto<OperandInternalDataUnion, ExpDataType> for OperandInternalDataUnionRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        data_type: ExpDataType,
    ) -> Result<OperandInternalDataUnion> {
        Ok(match data_type {
            ExpDataType::INT => OperandInternalDataUnion::Int(self.0 as _),
            ExpDataType::FLOAT => OperandInternalDataUnion::Float(f32::from_bits(self.0)),
            ExpDataType::STRING => OperandInternalDataUnion::String(
                XStringRaw::from_u32(self.0).xfile_deserialize_into(de, ())?,
            ),
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ItemKeyHandlerRaw<'a> {
    pub key: i32,
    pub key_script: Ptr32<'a, GenericEventScriptRaw<'a>>,
    pub next: Ptr32<'a, ItemKeyHandlerRaw<'a>>,
}
assert_size!(ItemKeyHandlerRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ItemKeyHandler {
    pub key: i32,
    pub key_script: Option<Box<GenericEventScript>>,
    pub next: Option<Box<ItemKeyHandler>>, // TODO
}

impl<'a> XFileDeserializeInto<ItemKeyHandler, ()> for ItemKeyHandlerRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ItemKeyHandler> {
        let key_script = self.key_script.xfile_deserialize_into(de, ())?;
        let next = if self.next.is_null() {
            None
        } else {
            self.next.xfile_deserialize_into(de, ())?
        };
        Ok(ItemKeyHandler {
            key: self.key,
            key_script,
            next,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ItemDefRaw<'a, const MAX_LOCAL_CLIENTS: usize> {
    pub window: WindowDefRaw<'a, MAX_LOCAL_CLIENTS>,
    pub type_: i32,
    pub data_type: i32,
    pub image_track: i32,
    pub dvar: XStringRaw<'a>,
    pub dvar_text: XStringRaw<'a>,
    pub enable_dvar: XStringRaw<'a>,
    pub dvar_flags: i32,
    pub type_data: ItemDefDataRaw<'a, MAX_LOCAL_CLIENTS>,
    pub parent: Ptr32<'a, MenuDefRaw<'a, MAX_LOCAL_CLIENTS>>,
    pub rect_exp_data: Ptr32<'a, RectDataRaw<'a>>,
    pub visible_exp: ExpressionStatementRaw<'a>,
    pad: [u8; 4],
    pub show_bits: u64,
    pub hide_bits: u64,
    pub forecolor_a_exp: ExpressionStatementRaw<'a>,
    pub ui_3d_window_id: i32,
    pub on_event: Ptr32<'a, GenericEventHandlerRaw<'a>>,
    pub anim_info: Ptr32<'a, UIAnimInfoRaw<'a>>,
    #[allow(dead_code)]
    pad2: [u8; 4],
}
assert_size!(ItemDefRaw<1>, 272);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ItemDef<const MAX_LOCAL_CLIENTS: usize> {
    pub window: WindowDef<MAX_LOCAL_CLIENTS>,
    pub type_: i32,
    pub data_type: i32,
    pub image_track: i32,
    pub dvar: String,
    pub dvar_text: String,
    pub enable_dvar: String,
    pub dvar_flags: i32,
    pub type_data: Option<ItemDefData<MAX_LOCAL_CLIENTS>>,
    pub parent: Option<Box<MenuDef<MAX_LOCAL_CLIENTS>>>, // TODO
    pub rect_exp_data: Option<Box<RectData>>,
    pub visible_exp: ExpressionStatement,
    pub show_bits: u64,
    pub hide_bits: u64,
    pub forecolor_a_exp: ExpressionStatement,
    pub ui_3d_window_id: i32,
    pub on_event: Option<Box<GenericEventHandler>>,
    pub anim_info: Option<Box<UIAnimInfo>>,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<ItemDef<MAX_LOCAL_CLIENTS>, ()>
    for ItemDefRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ItemDef<MAX_LOCAL_CLIENTS>> {
        //dbg!(de.stream_pos().unwrap());
        //dbg!(self);
        let window = self.window.xfile_deserialize_into(de, ())?;
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&window);
        let dvar = self.dvar.xfile_deserialize_into(de, ())?;
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&dvar);
        let dvar_text = self.dvar_text.xfile_deserialize_into(de, ())?;
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&dvar_text);
        let enable_dvar = self.enable_dvar.xfile_deserialize_into(de, ())?;
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&enable_dvar);
        let type_data = self.type_data.xfile_deserialize_into(de, self.type_)?;
        //dbg!(de.stream_pos().unwrap());
        ////dbg!(&type_data);
        let parent = if self.parent.is_null() || self.parent.is_real() {
            None
        } else {
            return Err(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::Todo("ItemDef: fix recursion.".to_string()),
            ));
        };
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&parent);
        let rect_exp_data = self.rect_exp_data.xfile_deserialize_into(de, ())?;
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&rect_exp_data);
        let visible_exp = self.visible_exp.xfile_deserialize_into(de, ())?;
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&visible_exp);
        let forecolor_a_exp = self.forecolor_a_exp.xfile_deserialize_into(de, ())?;
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&forecolor_a_exp);
        let on_event = self.on_event.xfile_deserialize_into(de, ())?;
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&on_event);
        let anim_info = self.anim_info.xfile_deserialize_into(de, ())?;
        //dbg!(de.stream_pos().unwrap());
        //dbg!(&anim_info);

        Ok(ItemDef {
            window,
            type_: self.type_,
            data_type: self.data_type,
            image_track: self.image_track,
            dvar,
            dvar_text,
            enable_dvar,
            dvar_flags: self.dvar_flags,
            type_data,
            parent,
            rect_exp_data,
            visible_exp,
            show_bits: self.show_bits,
            hide_bits: self.hide_bits,
            forecolor_a_exp,
            ui_3d_window_id: self.ui_3d_window_id,
            on_event,
            anim_info,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ItemDefDataRaw<'a, const MAX_LOCAL_CLIENTS: usize>(Ptr32<'a, ()>);
assert_size!(ItemDefDataRaw<1>, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum ItemDefData<const MAX_LOCAL_CLIENTS: usize> {
    TextDef(Option<Box<TextDef<MAX_LOCAL_CLIENTS>>>),
    ImageDef(Option<Box<ImageDef>>),
    BlankButtonDef(Option<Box<FocusItemDef<MAX_LOCAL_CLIENTS>>>),
    OwnerDrawDef(Option<Box<OwnerDrawDef>>),
}

impl<'a, const MAX_LOCAL_CLIENTS: usize>
    XFileDeserializeInto<Option<ItemDefData<MAX_LOCAL_CLIENTS>>, i32>
    for ItemDefDataRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        type_: i32,
    ) -> Result<Option<ItemDefData<MAX_LOCAL_CLIENTS>>> {
        //dbg!(self);
        if self.0.is_null() {
            Ok(None)
        } else if type_ == 2 {
            Ok(Some(ItemDefData::ImageDef(
                self.0
                    .cast::<ImageDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            )))
        } else if type_ == 21 || type_ == 19 {
            Ok(Some(ItemDefData::BlankButtonDef(
                self.0
                    .cast::<FocusItemDefRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, type_)?,
            )))
        } else if type_ == 6 {
            Ok(Some(ItemDefData::OwnerDrawDef(
                self.0
                    .cast::<OwnerDrawDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            )))
        } else if type_ == 17 || type_ > 22 {
            Err(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BrokenInvariant(format!("ItemDefData: type ({type_}) > 22",)),
            ))
        } else {
            Ok(Some(ItemDefData::TextDef(
                self.0
                    .cast::<TextDefRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, type_)?,
            )))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct TextDefRaw<'a, const MAX_LOCAL_CLIENTS: usize> {
    #[serde(with = "serde_arrays")]
    pub text_rect: [RectDefRaw; MAX_LOCAL_CLIENTS],
    pub alignment: i32,
    pub font_enum: i32,
    pub item_flags: i32,
    pub text_align_mode: i32,
    pub textalignx: f32,
    pub textaligny: f32,
    pub textscale: f32,
    pub text_style: i32,
    pub text: XStringRaw<'a>,
    pub text_exp_data: Ptr32<'a, TextExpRaw<'a>>,
    pub text_type_data: TextDefDataRaw<'a, MAX_LOCAL_CLIENTS>,
}
assert_size!(TextDefRaw<1>, 68);

impl<'a, const MAX_LOCAL_CLIENTS: usize> Default for TextDefRaw<'a, MAX_LOCAL_CLIENTS> {
    fn default() -> Self {
        Self {
            text_rect: [RectDefRaw::default(); MAX_LOCAL_CLIENTS],
            alignment: i32::default(),
            font_enum: i32::default(),
            item_flags: i32::default(),
            text_align_mode: i32::default(),
            textalignx: f32::default(),
            textaligny: f32::default(),
            textscale: f32::default(),
            text_style: i32::default(),
            text: XStringRaw::default(),
            text_exp_data: Ptr32::default(),
            text_type_data: TextDefDataRaw::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct TextDef<const MAX_LOCAL_CLIENTS: usize> {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub text_rect: [RectDef; MAX_LOCAL_CLIENTS],
    pub alignment: i32,
    pub font_enum: i32,
    pub item_flags: i32,
    pub text_align_mode: i32,
    pub textalignx: f32,
    pub textaligny: f32,
    pub textscale: f32,
    pub text_style: i32,
    pub text: String,
    pub text_exp_data: Option<Box<TextExp>>,
    pub text_type_data: Option<TextDefData<MAX_LOCAL_CLIENTS>>,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<TextDef<MAX_LOCAL_CLIENTS>, i32>
    for TextDefRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        type_: i32,
    ) -> Result<TextDef<MAX_LOCAL_CLIENTS>> {
        //dbg!(self);
        let text_rect = self.text_rect.map(Into::into);
        let text = self.text.xfile_deserialize_into(de, ())?;
        let text_exp_data = self.text_exp_data.xfile_deserialize_into(de, ())?;
        let text_type_data = self.text_type_data.xfile_deserialize_into(de, type_)?;

        Ok(TextDef {
            text_rect,
            alignment: self.alignment,
            font_enum: self.font_enum,
            item_flags: self.item_flags,
            text_align_mode: self.text_align_mode,
            textalignx: self.textalignx,
            textaligny: self.textaligny,
            textscale: self.textscale,
            text_style: self.text_style,
            text,
            text_exp_data,
            text_type_data,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct TextExpRaw<'a> {
    pub text_exp: ExpressionStatementRaw<'a>,
}
assert_size!(TextExpRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct TextExp {
    pub text_exp: ExpressionStatement,
}

impl<'a> XFileDeserializeInto<TextExp, ()> for TextExpRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<TextExp> {
        let text_exp = self.text_exp.xfile_deserialize_into(de, ())?;

        Ok(TextExp { text_exp })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct TextDefDataRaw<'a, const MAX_LOCAL_CLIENTS: usize>(Ptr32<'a, ()>);
assert_size!(TextDefDataRaw<1>, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum TextDefData<const MAX_LOCAL_CLIENTS: usize> {
    FocusItemDef(Option<Box<FocusItemDef<MAX_LOCAL_CLIENTS>>>),
    GameMsgDef(Option<Box<GameMsgDef>>),
}

impl<'a, const MAX_LOCAL_CLIENTS: usize>
    XFileDeserializeInto<Option<TextDefData<MAX_LOCAL_CLIENTS>>, i32>
    for TextDefDataRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        type_: i32,
    ) -> Result<Option<TextDefData<MAX_LOCAL_CLIENTS>>> {
        //dbg!(self);
        if self.0.is_null() {
            Ok(None)
        } else if type_ == 15 {
            Ok(Some(TextDefData::GameMsgDef(
                self.0.cast::<GameMsgDef>().xfile_get(de)?.map(Box::new),
            )))
        } else if type_ < 3
            || type_ == 6
            || type_ == 7
            || type_ == 17
            || type_ == 18
            || type_ == 19
            || type_ > 23
        {
            Err(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BrokenInvariant(format!("TextDefData: type ({type_}) invalid.",)),
            ))
        } else {
            Ok(Some(TextDefData::FocusItemDef(
                self.0
                    .cast::<FocusItemDefRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, type_)?,
            )))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct FocusItemDefRaw<'a, const MAX_LOCAL_CLIENTS: usize> {
    pub mouse_enter_text: XStringRaw<'a>,
    pub mouse_exit_text: XStringRaw<'a>,
    pub mouse_enter: XStringRaw<'a>,
    pub mouse_exit: XStringRaw<'a>,
    pub on_key: Ptr32<'a, ItemKeyHandlerRaw<'a>>,
    pub focus_type_data: FocusDefDataRaw<'a, MAX_LOCAL_CLIENTS>,
}
assert_size!(FocusItemDefRaw<1>, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FocusItemDef<const MAX_LOCAL_CLIENTS: usize> {
    pub mouse_enter_text: String,
    pub mouse_exit_text: String,
    pub mouse_enter: String,
    pub mouse_exit: String,
    pub on_key: Option<Box<ItemKeyHandler>>,
    pub focus_type_data: Option<FocusDefData<MAX_LOCAL_CLIENTS>>,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<FocusItemDef<MAX_LOCAL_CLIENTS>, i32>
    for FocusItemDefRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        type_: i32,
    ) -> Result<FocusItemDef<MAX_LOCAL_CLIENTS>> {
        //dbg!(self);
        let mouse_enter_text = self.mouse_enter_text.xfile_deserialize_into(de, ())?;
        let mouse_exit_text = self.mouse_exit_text.xfile_deserialize_into(de, ())?;
        let mouse_enter = self.mouse_enter.xfile_deserialize_into(de, ())?;
        let mouse_exit = self.mouse_exit.xfile_deserialize_into(de, ())?;
        let on_key = self.on_key.xfile_deserialize_into(de, ())?;
        let focus_type_data = self.focus_type_data.xfile_deserialize_into(de, type_)?;

        Ok(FocusItemDef {
            mouse_enter_text,
            mouse_exit_text,
            mouse_enter,
            mouse_exit,
            on_key,
            focus_type_data,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct FocusDefDataRaw<'a, const MAX_LOCAL_CLIENTS: usize>(Ptr32<'a, ()>);
assert_size!(FocusDefDataRaw<1>, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum FocusDefData<const MAX_LOCAL_CLIENTS: usize> {
    ListBox(Option<Box<ListBoxDef<MAX_LOCAL_CLIENTS>>>),
    Multi(Option<Box<MultiDef>>),
    EditField(Option<Box<EditFieldDef<MAX_LOCAL_CLIENTS>>>),
    EnumDvar(Option<Box<EnumDvarDef>>),
}

impl<'a, const MAX_LOCAL_CLIENTS: usize>
    XFileDeserializeInto<Option<FocusDefData<MAX_LOCAL_CLIENTS>>, i32>
    for FocusDefDataRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        type_: i32,
    ) -> Result<Option<FocusDefData<MAX_LOCAL_CLIENTS>>> {
        //dbg!(self);
        if self.0.is_null() {
            Ok(None)
        } else if type_ == 4 {
            Ok(Some(FocusDefData::ListBox(
                self.0
                    .cast::<ListBoxDefRaw<MAX_LOCAL_CLIENTS>>()
                    .xfile_deserialize_into(de, ())?,
            )))
        } else if type_ == 10 {
            Ok(Some(FocusDefData::Multi(
                self.0
                    .cast::<MultiDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            )))
        } else if type_ == 11 {
            Ok(Some(FocusDefData::EnumDvar(
                self.0
                    .cast::<EnumDvarDefRaw>()
                    .xfile_deserialize_into(de, ())?,
            )))
        } else if type_ == 5
            || type_ == 7
            || type_ == 8
            || type_ == 9
            || type_ == 12
            || type_ == 13
            || type_ == 14
            || type_ == 16
            || type_ == 22
            || type_ == 30
        {
            Ok(Some(FocusDefData::EditField(
                self.0
                    .cast::<EditFieldDef<MAX_LOCAL_CLIENTS>>()
                    .xfile_get(de)?
                    .map(Box::new),
            )))
        } else {
            Err(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BrokenInvariant(format!("FocusDefData: type ({type_}) invalid.",)),
            ))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct ListBoxDefRaw<'a, const MAX_LOCAL_CLIENTS: usize> {
    pub mouse_pos: i32,
    #[serde(with = "serde_arrays")]
    pub cursor_pos: [i32; MAX_LOCAL_CLIENTS],
    #[serde(with = "serde_arrays")]
    pub start_pos: [i32; MAX_LOCAL_CLIENTS],
    #[serde(with = "serde_arrays")]
    pub end_pos: [i32; MAX_LOCAL_CLIENTS],
    pub draw_padding: i32,
    pub element_width: f32,
    pub element_height: f32,
    pub num_columns: i32,
    pub special: f32,
    pub column_info: [ColumnInfoRaw; 16],
    pub not_selectable: i32,
    pub no_scroll_bars: i32,
    pub use_paging: i32,
    pub select_border: [f32; 4],
    pub disable_color: [f32; 4],
    pub focus_color: [f32; 4],
    pub element_highlight_color: [f32; 4],
    pub element_background_color: [f32; 4],
    pub select_icon: Ptr32<'a, MaterialRaw<'a>>,
    pub background_item_listbox: Ptr32<'a, MaterialRaw<'a>>,
    pub highlight_texture: Ptr32<'a, MaterialRaw<'a>>,
    pub no_blinking_highlight: i32,
    pub rows: FatPointerCountLastU32<'a, MenuRowRaw<'a>>,
    #[allow(dead_code)]
    pub row_count: i32,
}
assert_size!(ListBoxDefRaw<1>, 668);

impl<'a, const MAX_LOCAL_CLIENTS: usize> Default for ListBoxDefRaw<'a, MAX_LOCAL_CLIENTS> {
    fn default() -> Self {
        Self {
            mouse_pos: i32::default(),
            cursor_pos: [i32::default(); MAX_LOCAL_CLIENTS],
            start_pos: [i32::default(); MAX_LOCAL_CLIENTS],
            end_pos: [i32::default(); MAX_LOCAL_CLIENTS],
            draw_padding: i32::default(),
            element_width: f32::default(),
            element_height: f32::default(),
            num_columns: i32::default(),
            special: f32::default(),
            column_info: [ColumnInfoRaw::default(); 16],
            not_selectable: i32::default(),
            no_scroll_bars: i32::default(),
            use_paging: i32::default(),
            select_border: [f32::default(); 4],
            disable_color: [f32::default(); 4],
            focus_color: [f32::default(); 4],
            element_highlight_color: [f32::default(); 4],
            element_background_color: [f32::default(); 4],
            select_icon: Ptr32::default(),
            background_item_listbox: Ptr32::default(),
            highlight_texture: Ptr32::default(),
            no_blinking_highlight: i32::default(),
            rows: FatPointerCountLastU32::default(),
            row_count: i32::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ListBoxDef<const MAX_LOCAL_CLIENTS: usize> {
    pub mouse_pos: i32,
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub cursor_pos: [i32; MAX_LOCAL_CLIENTS],
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub start_pos: [i32; MAX_LOCAL_CLIENTS],
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub end_pos: [i32; MAX_LOCAL_CLIENTS],
    pub draw_padding: bool,
    pub element_width: f32,
    pub element_height: f32,
    pub num_columns: i32,
    pub special: f32,
    pub column_info: [ColumnInfo; 16],
    pub not_selectable: bool,
    pub no_scroll_bars: bool,
    pub use_paging: bool,
    pub select_border: Vec4,
    pub disable_color: Vec4,
    pub focus_color: Vec4,
    pub element_highlight_color: Vec4,
    pub element_background_color: Vec4,
    pub select_icon: Option<Box<Material>>,
    pub background_item_listbox: Option<Box<Material>>,
    pub highlight_texture: Option<Box<Material>>,
    pub no_blinking_highlight: bool,
    pub rows: Vec<MenuRow>,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<ListBoxDef<MAX_LOCAL_CLIENTS>, ()>
    for ListBoxDefRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ListBoxDef<MAX_LOCAL_CLIENTS>> {
        let draw_padding = self.draw_padding != 0;
        let column_info = self.column_info.map(Into::into);
        let not_selectable = self.not_selectable != 0;
        let no_scroll_bars = self.no_scroll_bars != 0;
        let use_paging = self.use_paging != 0;
        let select_border = self.select_border.into();
        let disable_color = self.disable_color.into();
        let focus_color = self.focus_color.into();
        let element_highlight_color = self.element_highlight_color.into();
        let element_background_color = self.element_background_color.into();
        let select_icon = self.select_icon.xfile_deserialize_into(de, ())?;
        let background_item_listbox = self
            .background_item_listbox
            .xfile_deserialize_into(de, ())?;
        let highlight_texture = self.highlight_texture.xfile_deserialize_into(de, ())?;
        let no_blinking_highlight = self.no_blinking_highlight != 0;
        let rows = self.rows.xfile_deserialize_into(de, self.num_columns)?;

        Ok(ListBoxDef {
            mouse_pos: self.mouse_pos,
            cursor_pos: self.cursor_pos,
            start_pos: self.start_pos,
            end_pos: self.end_pos,
            draw_padding,
            element_width: self.element_width,
            element_height: self.element_height,
            num_columns: self.num_columns,
            special: self.special,
            column_info,
            not_selectable,
            no_scroll_bars,
            use_paging,
            select_border,
            disable_color,
            focus_color,
            element_highlight_color,
            element_background_color,
            select_icon,
            background_item_listbox,
            highlight_texture,
            no_blinking_highlight,
            rows,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ColumnInfoRaw {
    pub element_style: i32,
    pub max_chars: i32,
    pub rect: RectDefRaw,
}
assert_size!(ColumnInfoRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ColumnInfo {
    pub element_style: i32,
    pub max_chars: i32,
    pub rect: RectDef,
}

impl Into<ColumnInfo> for ColumnInfoRaw {
    fn into(self) -> ColumnInfo {
        let rect = self.rect.into();

        ColumnInfo {
            element_style: self.element_style,
            max_chars: self.max_chars,
            rect,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MenuRowRaw<'a> {
    pub cells: Ptr32<'a, MenuCellRaw<'a>>,
    pub event_name: XStringRaw<'a>,
    pub on_focus_event_name: XStringRaw<'a>,
    pub disable_arg: bool,
    pad: [u8; 3],
    pub status: i32,
    pub name: i32,
}
assert_size!(MenuRowRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MenuRow {
    pub cells: Vec<MenuCell>,
    pub event_name: String,
    pub on_focus_event_name: String,
    pub disable_arg: bool,
    pub status: i32,
    pub name: i32,
}

impl<'a> XFileDeserializeInto<MenuRow, i32> for MenuRowRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        num_columns: i32,
    ) -> Result<MenuRow> {
        let cells = self
            .cells
            .to_array(num_columns as _)
            .xfile_deserialize_into(de, ())?;
        let event_name = self.event_name.xfile_deserialize_into(de, ())?;
        let on_focus_event_name = self.on_focus_event_name.xfile_deserialize_into(de, ())?;

        Ok(MenuRow {
            cells,
            event_name,
            on_focus_event_name,
            disable_arg: self.disable_arg,
            status: self.status,
            name: self.name,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MenuCellRaw<'a> {
    pub type_: i32,
    pub max_chars: i32,
    pub string_value: XStringRaw<'a>,
}
assert_size!(MenuCellRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MenuCell {
    pub type_: i32,
    pub max_chars: i32,
    pub string_value: String,
}

impl<'a> XFileDeserializeInto<MenuCell, ()> for MenuCellRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MenuCell> {
        let string_value = self.string_value.xfile_deserialize_into(de, ())?;

        Ok(MenuCell {
            type_: self.type_,
            max_chars: self.max_chars,
            string_value,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MultiDefRaw<'a> {
    pub dvar_list: [XStringRaw<'a>; 32],
    pub dvar_str: [XStringRaw<'a>; 32],
    pub dvar_value: [f32; 32],
    pub count: i32,
    pub action_on_press_enter_only: i32,
    pub str_def: i32,
}
assert_size!(MultiDefRaw, 396);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MultiDef {
    pub dvar_list: [String; 32],
    pub dvar_str: [String; 32],
    pub dvar_value: [f32; 32],
    pub count: i32,
    pub action_on_press_enter_only: bool,
    pub str_def: i32,
}

impl<'a> XFileDeserializeInto<MultiDef, ()> for MultiDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MultiDef> {
        let dvar_list = self
            .dvar_list
            .into_iter()
            .map(|d| d.xfile_deserialize_into(de, ()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .unwrap();
        let dvar_str = self
            .dvar_str
            .into_iter()
            .map(|d| d.xfile_deserialize_into(de, ()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .unwrap();
        let action_on_press_enter_only = self.action_on_press_enter_only != 0;

        Ok(MultiDef {
            dvar_list,
            dvar_str,
            dvar_value: self.dvar_value,
            count: self.count,
            action_on_press_enter_only,
            str_def: self.str_def,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct EditFieldDef<const MAX_LOCAL_CLIENTS: usize> {
    #[serde(with = "serde_arrays")]
    pub cursor_pos: [i32; MAX_LOCAL_CLIENTS],
    pub min_val: f32,
    pub max_val: f32,
    pub def_val: f32,
    pub range: f32,
    pub max_chars: i32,
    pub max_chars_goto_next: i32,
    pub max_paint_chars: i32,
    pub paint_offset: i32,
}
assert_size!(EditFieldDef<1>, 36);

impl<const MAX_LOCAL_CLIENTS: usize> Default for EditFieldDef<MAX_LOCAL_CLIENTS> {
    fn default() -> Self {
        Self {
            cursor_pos: [i32::default(); MAX_LOCAL_CLIENTS],
            min_val: f32::default(),
            max_val: f32::default(),
            def_val: f32::default(),
            range: f32::default(),
            max_chars: i32::default(),
            max_chars_goto_next: i32::default(),
            max_paint_chars: i32::default(),
            paint_offset: i32::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EnumDvarDefRaw<'a> {
    pub enum_dvar_name: XStringRaw<'a>,
}
assert_size!(EnumDvarDefRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EnumDvarDef {
    pub enum_dvar_name: String,
}

impl<'a> XFileDeserializeInto<EnumDvarDef, ()> for EnumDvarDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<EnumDvarDef> {
        let enum_dvar_name = self.enum_dvar_name.xfile_deserialize_into(de, ())?;

        Ok(EnumDvarDef { enum_dvar_name })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct GameMsgDef {
    pub game_msg_window_index: i32,
    pub game_msg_window_mode: i32,
}
assert_size!(GameMsgDef, 8);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ImageDefRaw<'a> {
    pub material_exp: ExpressionStatementRaw<'a>,
}
assert_size!(ImageDefRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ImageDef {
    pub material_exp: ExpressionStatement,
}

impl<'a> XFileDeserializeInto<ImageDef, ()> for ImageDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<ImageDef> {
        let material_exp = self.material_exp.xfile_deserialize_into(de, ())?;

        Ok(ImageDef { material_exp })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct OwnerDrawDefRaw<'a> {
    pub data_exp: ExpressionStatementRaw<'a>,
}
assert_size!(OwnerDrawDefRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct OwnerDrawDef {
    pub data_exp: ExpressionStatement,
}

impl<'a> XFileDeserializeInto<OwnerDrawDef, ()> for OwnerDrawDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<OwnerDrawDef> {
        let data_exp = self.data_exp.xfile_deserialize_into(de, ())?;

        Ok(OwnerDrawDef { data_exp })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct RectDataRaw<'a> {
    pub rect_x_exp: ExpressionStatementRaw<'a>,
    pub rect_y_exp: ExpressionStatementRaw<'a>,
    pub rect_w_exp: ExpressionStatementRaw<'a>,
    pub rect_h_exp: ExpressionStatementRaw<'a>,
}
assert_size!(RectDataRaw, 64);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RectData {
    pub rect_x_exp: ExpressionStatement,
    pub rect_y_exp: ExpressionStatement,
    pub rect_w_exp: ExpressionStatement,
    pub rect_h_exp: ExpressionStatement,
}

impl<'a> XFileDeserializeInto<RectData, ()> for RectDataRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<RectData> {
        let rect_x_exp = self.rect_x_exp.xfile_deserialize_into(de, ())?;
        let rect_y_exp = self.rect_y_exp.xfile_deserialize_into(de, ())?;
        let rect_w_exp = self.rect_w_exp.xfile_deserialize_into(de, ())?;
        let rect_h_exp = self.rect_h_exp.xfile_deserialize_into(de, ())?;

        Ok(RectData {
            rect_x_exp,
            rect_y_exp,
            rect_w_exp,
            rect_h_exp,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct UIAnimInfoRaw<'a> {
    pub anim_states: FatPointerCountFirstU32<'a, Ptr32<'a, AnimParamsDefRaw<'a>>>,
    pub current_anim_state: AnimParamsDefRaw<'a>,
    pub next_anim_state: AnimParamsDefRaw<'a>,
    pub animating: i32,
    pub anim_start_time: i32,
    pub anim_duration: i32,
}
assert_size!(UIAnimInfoRaw, 236);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct UIAnimInfo {
    pub anim_states: Vec<Box<AnimParamsDef>>,
    pub current_anim_state: AnimParamsDef,
    pub next_anim_state: AnimParamsDef,
    pub animating: bool,
    pub anim_start_time: i32,
    pub anim_duration: i32,
}

impl<'a> XFileDeserializeInto<UIAnimInfo, ()> for UIAnimInfoRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<UIAnimInfo> {
        let anim_states = self
            .anim_states
            .xfile_deserialize_into(de, ())?
            .into_iter()
            .flatten()
            .collect();
        let current_anim_state = self.current_anim_state.xfile_deserialize_into(de, ())?;
        let next_anim_state = self.next_anim_state.xfile_deserialize_into(de, ())?;
        let animating = self.animating != 0;

        Ok(UIAnimInfo {
            anim_states,
            current_anim_state,
            next_anim_state,
            animating,
            anim_start_time: self.anim_start_time,
            anim_duration: self.anim_duration,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct AnimParamsDefRaw<'a> {
    pub name: XStringRaw<'a>,
    pub rect_client: RectDefRaw,
    pub border_size: f32,
    pub fore_color: [f32; 4],
    pub back_color: [f32; 4],
    pub border_color: [f32; 4],
    pub outline_color: [f32; 4],
    pub text_scale: f32,
    pub rotation: f32,
    pub on_event: Ptr32<'a, GenericEventHandlerRaw<'a>>,
}
assert_size!(AnimParamsDefRaw, 108);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AnimParamsDef {
    pub name: String,
    pub rect_client: RectDef,
    pub border_size: f32,
    pub fore_color: Vec4,
    pub back_color: Vec4,
    pub border_color: Vec4,
    pub outline_color: Vec4,
    pub text_scale: f32,
    pub rotation: f32,
    pub on_event: Option<Box<GenericEventHandler>>,
}

impl<'a> XFileDeserializeInto<AnimParamsDef, ()> for AnimParamsDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<AnimParamsDef> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let rect_client = self.rect_client.into();
        let fore_color = self.fore_color.into();
        let back_color = self.back_color.into();
        let border_color = self.border_color.into();
        let outline_color = self.outline_color.into();
        let on_event = self.on_event.xfile_deserialize_into(de, ())?;

        Ok(AnimParamsDef {
            name,
            rect_client,
            border_size: self.border_size,
            fore_color,
            back_color,
            border_color,
            outline_color,
            text_scale: self.text_scale,
            rotation: self.rotation,
            on_event,
        })
    }
}
