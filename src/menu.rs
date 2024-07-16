use std::mem::transmute;

use common::Vec4;
use num::FromPrimitive;

use crate::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MenuListRaw<'a> {
    pub name: XString<'a>,
    pub menus: FatPointerCountFirstU32<'a, MenuDefRaw<'a>>,
}
assert_size!(MenuListRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MenuList {
    pub name: String,
    pub menus: Vec<MenuDef>,
}

impl<'a> XFileInto<MenuList, ()> for MenuListRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<MenuList> {
        let name = self.name.xfile_into(&mut xfile, ())?;
        let menus = self.menus.xfile_into(&mut xfile, ())?;

        Ok(MenuList { name, menus })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MenuDefRaw<'a> {
    pub window: WindowDefRaw<'a>,
    pub font: XString<'a>,
    pub full_screen: i32,
    pub ui_3d_window_id: i32,
    pub item_count: i32,
    pub font_index: i32,
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
    pub show_bits: u64,
    pub hide_bits: u64,
    pub allowed_binding: XString<'a>,
    pub sound_name: XString<'a>,
    pub image_track: i32,
    pub control: i32,
    pub focus_color: [f32; 4],
    pub disable_color: [f32; 4],
    pub rect_x_exp: ExpressionStatementRaw<'a>,
    pub rect_y_exp: ExpressionStatementRaw<'a>,
    pub items: Ptr32<'a, Ptr32<'a, ItemDefRaw<'a>>>,
    pub pad: [u8; 4],
}
assert_size!(MenuDefRaw, 400);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MenuDef {
    pub window: WindowDef,
    pub font: String,
    pub full_screen: bool,
    pub ui_3d_window_id: i32,
    pub font_index: i32,
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
    pub items: Vec<Box<ItemDef>>,
}

impl<'a> XFileInto<MenuDef, ()> for MenuDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<MenuDef> {
        let window = self.window.xfile_into(&mut xfile, ())?;
        let font = self.font.xfile_into(&mut xfile, ())?;
        let full_screen = self.full_screen != 0;
        let intial_rect_info = self.intial_rect_info.into();
        let on_event = self.on_event.xfile_into(&mut xfile, ())?;
        let on_key = self.on_key.xfile_into(&mut xfile, ())?;
        let visible_exp = self.visible_exp.xfile_into(&mut xfile, ())?;
        let allowed_binding = self.allowed_binding.xfile_into(&mut xfile, ())?;
        let sound_name = self.sound_name.xfile_into(&mut xfile, ())?;
        let focus_color = self.focus_color.into();
        let disable_color = self.disable_color.into();
        let rect_x_exp = self.rect_x_exp.xfile_into(&mut xfile, ())?;
        let rect_y_exp = self.rect_y_exp.xfile_into(&mut xfile, ())?;
        let items = self
            .items
            .to_array(self.item_count as _)
            .xfile_into(&mut xfile, ())?
            .into_iter()
            .flatten()
            .collect();

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
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct WindowDefRaw<'a> {
    pub name: XString<'a>,
    pub rect: RectDefRaw,
    pub rect_client: RectDefRaw,
    pub group: XString<'a>,
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
    pub dynamic_flags: [i32; MAX_LOCAL_CLIENTS],
    pub next_time: i32,
    pub fore_color: [f32; 4],
    pub back_color: [f32; 4],
    pub border_color: [f32; 4],
    pub outline_color: [f32; 4],
    pub rotation: f32,
    pub background: Ptr32<'a, techset::MaterialRaw<'a>>,
}
assert_size!(WindowDefRaw, 164);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct WindowDef {
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
    pub dynamic_flags: [i32; MAX_LOCAL_CLIENTS],
    pub next_time: i32,
    pub fore_color: Vec4,
    pub back_color: Vec4,
    pub border_color: Vec4,
    pub outline_color: Vec4,
    pub rotation: f32,
    pub background: Option<Box<techset::Material>>,
}

impl<'a> XFileInto<WindowDef, ()> for WindowDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<WindowDef> {
        let name = self.name.xfile_into(&mut xfile, ())?;
        let rect = self.rect.into();
        let rect_client = self.rect_client.into();
        let group = self.group.xfile_into(&mut xfile, ())?;
        let fore_color = self.fore_color.into();
        let back_color = self.back_color.into();
        let border_color = self.border_color.into();
        let outline_color = self.outline_color.into();
        let background = self.background.xfile_into(xfile, ())?;

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
    pub name: XString<'a>,
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

impl<'a> XFileInto<GenericEventHandler, ()> for GenericEventHandlerRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<GenericEventHandler> {
        let name = self.name.xfile_into(&mut xfile, ())?;
        let event_script = self.event_script.xfile_into(&mut xfile, ())?;
        let next = if self.next.is_null() {
            None
        } else {
            self.next.xfile_into(xfile, ())?
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
    pub action: XString<'a>,
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

impl<'a> XFileInto<GenericEventScript, ()> for GenericEventScriptRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<GenericEventScript> {
        let prerequisites = self.prerequisites.xfile_into(&mut xfile, ())?;
        let condition = self.condition.xfile_into(&mut xfile, ())?;
        let action = self.action.xfile_into(&mut xfile, ())?;
        let next = if self.next.is_null() {
            None
        } else {
            self.next.xfile_into(xfile, ())?
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

impl<'a> XFileInto<ScriptCondition, ()> for ScriptConditionRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> Result<ScriptCondition> {
        let next = if self.next.is_null() {
            None
        } else {
            self.next.xfile_into(xfile, ())?
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
    pub filename: XString<'a>,
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

impl<'a> XFileInto<ExpressionStatement, ()> for ExpressionStatementRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<ExpressionStatement> {
        let filename = self.filename.xfile_into(&mut xfile, ())?;
        let rpn = self.rpn.xfile_into(&mut xfile, ())?;

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
    pub data: ExpressionRpnDataUnion,
}

impl XFileInto<ExpressionRpn, ()> for ExpressionRpnRaw {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> Result<ExpressionRpn> {
        let data = self.data.xfile_into(xfile, self.type_)?;
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

impl XFileInto<ExpressionRpnDataUnion, i32> for ExpressionRpnDataUnionRaw {
    fn xfile_into(&self, xfile: impl Read + Seek, type_: i32) -> Result<ExpressionRpnDataUnion> {
        if type_ == 0 {
            Ok(ExpressionRpnDataUnion::Constant(
                unsafe { transmute::<_, OperandRaw>(self.0) }.xfile_into(xfile, ())?,
            ))
        } else {
            Err(Error::BrokenInvariant(format!(
                "{}: ExpressionRpnDataUnion: type ({type_}) != 0",
                file_line_col!()
            )))
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

impl XFileInto<Operand, ()> for OperandRaw {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> Result<Operand> {
        let data_type = FromPrimitive::from_i32(self.data_type)
            .ok_or(Error::BadFromPrimitive(self.data_type as _))?;
        let internals = self.internals.xfile_into(xfile, data_type)?;
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

impl XFileInto<OperandInternalDataUnion, ExpDataType> for OperandInternalDataUnionRaw {
    fn xfile_into(
        &self,
        xfile: impl Read + Seek,
        data_type: ExpDataType,
    ) -> Result<OperandInternalDataUnion> {
        Ok(match data_type {
            ExpDataType::INT => OperandInternalDataUnion::Int(self.0 as _),
            ExpDataType::FLOAT => OperandInternalDataUnion::Float(f32::from_bits(self.0)),
            ExpDataType::STRING => {
                OperandInternalDataUnion::String(XString::from_u32(self.0).xfile_into(xfile, ())?)
            }
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

impl<'a> XFileInto<ItemKeyHandler, ()> for ItemKeyHandlerRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<ItemKeyHandler> {
        let key_script = self.key_script.xfile_into(&mut xfile, ())?;
        let next = if self.next.is_null() {
            None
        } else {
            self.next.xfile_into(xfile, ())?
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
pub(crate) struct ItemDefRaw<'a> {
    pub window: WindowDefRaw<'a>,
    pub type_: i32,
    pub data_type: i32,
    pub image_track: i32,
    pub dvar: XString<'a>,
    pub dvar_text: XString<'a>,
    pub enable_dvar: XString<'a>,
    pub dvar_flags: i32,
    pub type_data: ItemDefDataRaw<'a>,
    pub parent: Ptr32<'a, MenuDefRaw<'a>>,
    pub rect_exp_data: Ptr32<'a, RectDataRaw<'a>>,
    pub visible_exp: ExpressionStatementRaw<'a>,
    pub show_bits: u64,
    pub hide_bits: u64,
    pub forecolor_a_exp: ExpressionStatementRaw<'a>,
    pub ui_3d_window_id: i32,
    pub on_event: Ptr32<'a, GenericEventHandlerRaw<'a>>,
    pub anim_info: Ptr32<'a, UIAnimInfoRaw<'a>>,
    pad: [u8; 8],
}
assert_size!(ItemDefRaw, 272);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ItemDef {
    pub window: WindowDef,
    pub type_: i32,
    pub data_type: i32,
    pub image_track: i32,
    pub dvar: String,
    pub dvar_text: String,
    pub enable_dvar: String,
    pub dvar_flags: i32,
    pub type_data: ItemDefData,
    pub parent: Option<Box<MenuDef>>, // TODO
    pub rect_exp_data: Option<Box<RectData>>,
    pub visible_exp: ExpressionStatement,
    pub show_bits: u64,
    pub hide_bits: u64,
    pub forecolor_a_exp: ExpressionStatement,
    pub ui_3d_window_id: i32,
    pub on_event: Option<Box<GenericEventHandler>>,
    pub anim_info: Option<Box<UIAnimInfo>>,
}

impl<'a> XFileInto<ItemDef, ()> for ItemDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<ItemDef> {
        let window = self.window.xfile_into(&mut xfile, ())?;
        let dvar = self.dvar.xfile_into(&mut xfile, ())?;
        let dvar_text = self.dvar_text.xfile_into(&mut xfile, ())?;
        let enable_dvar = self.enable_dvar.xfile_into(&mut xfile, ())?;
        let type_data = self.type_data.xfile_into(&mut xfile, self.type_)?;
        let parent = if self.parent.is_null() || self.parent.as_u32() != 0xFFFFFFFF {
            None
        } else {
            return Err(Error::Todo(format!(
                "{}: ItemDef: fix recursion.",
                file_line_col!()
            )));
        };
        let rect_exp_data = self.rect_exp_data.xfile_into(&mut xfile, ())?;
        let visible_exp = self.visible_exp.xfile_into(&mut xfile, ())?;
        let forecolor_a_exp = self.forecolor_a_exp.xfile_into(&mut xfile, ())?;
        let on_event = self.on_event.xfile_into(&mut xfile, ())?;
        let anim_info = self.anim_info.xfile_into(xfile, ())?;

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
pub(crate) struct ItemDefDataRaw<'a>(Ptr32<'a, ()>);
assert_size!(ItemDefDataRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum ItemDefData {
    TextDef(Option<Box<TextDef>>),
    ImageDef(Option<Box<ImageDef>>),
    BlankButtonDef(Option<Box<FocusItemDef>>),
    OwnerDrawDef(Option<Box<OwnerDrawDef>>),
}

impl<'a> XFileInto<ItemDefData, i32> for ItemDefDataRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, type_: i32) -> Result<ItemDefData> {
        if type_ == 2 {
            Ok(ItemDefData::ImageDef(
                self.0.cast::<ImageDefRaw>().xfile_into(xfile, ())?,
            ))
        } else if type_ == 21 || type_ == 19 {
            Ok(ItemDefData::BlankButtonDef(
                self.0.cast::<FocusItemDefRaw>().xfile_into(xfile, type_)?,
            ))
        } else if type_ == 6 {
            Ok(ItemDefData::OwnerDrawDef(
                self.0.cast::<OwnerDrawDefRaw>().xfile_into(xfile, ())?,
            ))
        } else if type_ > 22 {
            Err(Error::BrokenInvariant(format!(
                "{}: ItemDefData: type ({type_}) > 22",
                file_line_col!()
            )))
        } else {
            Ok(ItemDefData::TextDef(
                self.0.cast::<TextDefRaw>().xfile_into(xfile, type_)?,
            ))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct TextDefRaw<'a> {
    pub text_rect: [RectDefRaw; MAX_LOCAL_CLIENTS],
    pub alignment: i32,
    pub font_enum: i32,
    pub item_flags: i32,
    pub text_align_mode: i32,
    pub textalignx: f32,
    pub textaligny: f32,
    pub textscale: f32,
    pub text_style: i32,
    pub text: XString<'a>,
    pub text_exp_data: Ptr32<'a, TextExpRaw<'a>>,
    pub text_type_data: TextDefDataRaw<'a>,
}
assert_size!(TextDefRaw, 68);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct TextDef {
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
    pub text_type_data: TextDefData,
}

impl<'a> XFileInto<TextDef, i32> for TextDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, type_: i32) -> Result<TextDef> {
        let text_rect = self.text_rect.map(Into::into);
        let text = self.text.xfile_into(&mut xfile, ())?;
        let text_exp_data = self.text_exp_data.xfile_into(&mut xfile, ())?;
        let text_type_data = self.text_type_data.xfile_into(xfile, type_)?;

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

impl<'a> XFileInto<TextExp, ()> for TextExpRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> Result<TextExp> {
        let text_exp = self.text_exp.xfile_into(xfile, ())?;

        Ok(TextExp { text_exp })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct TextDefDataRaw<'a>(Ptr32<'a, ()>);
assert_size!(TextDefDataRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum TextDefData {
    FocusItemDef(Option<Box<FocusItemDef>>),
    GameMsgDef(Option<Box<GameMsgDef>>),
}

impl<'a> XFileInto<TextDefData, i32> for TextDefDataRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, type_: i32) -> Result<TextDefData> {
        if type_ == 15 {
            Ok(TextDefData::GameMsgDef(
                self.0.cast::<GameMsgDef>().xfile_get(xfile)?.map(Box::new),
            ))
        } else if type_ < 3
            || type_ == 6
            || type_ == 7
            || type_ == 17
            || type_ == 18
            || type_ == 19
            || type_ > 23
        {
            Err(Error::BrokenInvariant(format!(
                "{}: TextDefData: type ({type_}) invalid.",
                file_line_col!()
            )))
        } else {
            Ok(TextDefData::FocusItemDef(
                self.0.cast::<FocusItemDefRaw>().xfile_into(xfile, type_)?,
            ))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct FocusItemDefRaw<'a> {
    pub mouse_enter_text: XString<'a>,
    pub mouse_exit_text: XString<'a>,
    pub mouse_enter: XString<'a>,
    pub mouse_exit: XString<'a>,
    pub on_key: Ptr32<'a, ItemKeyHandlerRaw<'a>>,
    pub focus_type_data: FocusDefDataRaw<'a>,
}
assert_size!(FocusItemDefRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct FocusItemDef {
    pub mouse_enter_text: String,
    pub mouse_exit_text: String,
    pub mouse_enter: String,
    pub mouse_exit: String,
    pub on_key: Option<Box<ItemKeyHandler>>,
    pub focus_type_data: FocusDefData,
}

impl<'a> XFileInto<FocusItemDef, i32> for FocusItemDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, type_: i32) -> Result<FocusItemDef> {
        let mouse_enter_text = self.mouse_enter_text.xfile_into(&mut xfile, ())?;
        let mouse_exit_text = self.mouse_exit_text.xfile_into(&mut xfile, ())?;
        let mouse_enter = self.mouse_enter.xfile_into(&mut xfile, ())?;
        let mouse_exit = self.mouse_exit.xfile_into(&mut xfile, ())?;
        let on_key = self.on_key.xfile_into(&mut xfile, ())?;
        let focus_type_data = self.focus_type_data.xfile_into(xfile, type_)?;

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
pub(crate) struct FocusDefDataRaw<'a>(Ptr32<'a, ()>);
assert_size!(FocusDefDataRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum FocusDefData {
    ListBox(Option<Box<ListBoxDef>>),
    Multi(Option<Box<MultiDef>>),
    EditField(Option<Box<EditFieldDef>>),
    EnumDvar(Option<Box<EnumDvarDef>>),
}

impl<'a> XFileInto<FocusDefData, i32> for FocusDefDataRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, type_: i32) -> Result<FocusDefData> {
        if type_ == 4 {
            Ok(FocusDefData::ListBox(
                self.0.cast::<ListBoxDefRaw>().xfile_into(xfile, ())?,
            ))
        } else if type_ == 10 {
            Ok(FocusDefData::Multi(
                self.0.cast::<MultiDefRaw>().xfile_into(xfile, ())?,
            ))
        } else if type_ == 11 {
            Ok(FocusDefData::EnumDvar(
                self.0.cast::<EnumDvarDefRaw>().xfile_into(xfile, ())?,
            ))
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
            Ok(FocusDefData::EditField(
                self.0
                    .cast::<EditFieldDef>()
                    .xfile_get(xfile)?
                    .map(Box::new),
            ))
        } else {
            Err(Error::BrokenInvariant(format!(
                "{}: FocusDefData: type ({type_}) invalid.",
                file_line_col!()
            )))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct ListBoxDefRaw<'a> {
    pub mouse_pos: i32,
    pub cursor_pos: [i32; MAX_LOCAL_CLIENTS],
    pub start_pos: [i32; MAX_LOCAL_CLIENTS],
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
    pub select_icon: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub background_item_listbox: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub highlight_texture: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub no_blinking_highlight: i32,
    pub rows: FatPointerCountLastU32<'a, MenuRowRaw<'a>>,
    pub row_count: i32,
}
assert_size!(ListBoxDefRaw, 668);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ListBoxDef {
    pub mouse_pos: i32,
    pub cursor_pos: [i32; MAX_LOCAL_CLIENTS],
    pub start_pos: [i32; MAX_LOCAL_CLIENTS],
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
    pub select_icon: Option<Box<techset::Material>>,
    pub background_item_listbox: Option<Box<techset::Material>>,
    pub highlight_texture: Option<Box<techset::Material>>,
    pub no_blinking_highlight: bool,
    pub rows: Vec<MenuRow>,
}

impl<'a> XFileInto<ListBoxDef, ()> for ListBoxDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<ListBoxDef> {
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
        let select_icon = self.select_icon.xfile_into(&mut xfile, ())?;
        let background_item_listbox = self.background_item_listbox.xfile_into(&mut xfile, ())?;
        let highlight_texture = self.highlight_texture.xfile_into(&mut xfile, ())?;
        let no_blinking_highlight = self.no_blinking_highlight != 0;
        let rows = self.rows.xfile_into(xfile, self.num_columns)?;

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
    pub event_name: XString<'a>,
    pub on_focus_event_name: XString<'a>,
    pub disable_arg: bool,
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

impl<'a> XFileInto<MenuRow, i32> for MenuRowRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, num_columns: i32) -> Result<MenuRow> {
        let cells = self
            .cells
            .to_array(num_columns as _)
            .xfile_into(&mut xfile, ())?;
        let event_name = self.event_name.xfile_into(&mut xfile, ())?;
        let on_focus_event_name = self.on_focus_event_name.xfile_into(xfile, ())?;

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
    pub string_value: XString<'a>,
}
assert_size!(MenuCellRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MenuCell {
    pub type_: i32,
    pub max_chars: i32,
    pub string_value: String,
}

impl<'a> XFileInto<MenuCell, ()> for MenuCellRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> Result<MenuCell> {
        let string_value = self.string_value.xfile_into(xfile, ())?;

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
    pub dvar_list: [XString<'a>; 32],
    pub dvar_str: [XString<'a>; 32],
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

impl<'a> XFileInto<MultiDef, ()> for MultiDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<MultiDef> {
        let dvar_list = self
            .dvar_list
            .into_iter()
            .map(|d| d.xfile_into(&mut xfile, ()))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .unwrap();
        let dvar_str = self
            .dvar_str
            .into_iter()
            .map(|d| d.xfile_into(&mut xfile, ()))
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
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct EditFieldDef {
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
assert_size!(EditFieldDef, 36);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct EnumDvarDefRaw<'a> {
    pub enum_dvar_name: XString<'a>,
}
assert_size!(EnumDvarDefRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EnumDvarDef {
    pub enum_dvar_name: String,
}

impl<'a> XFileInto<EnumDvarDef, ()> for EnumDvarDefRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> Result<EnumDvarDef> {
        let enum_dvar_name = self.enum_dvar_name.xfile_into(xfile, ())?;

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

impl<'a> XFileInto<ImageDef, ()> for ImageDefRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> Result<ImageDef> {
        let material_exp = self.material_exp.xfile_into(xfile, ())?;

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

impl<'a> XFileInto<OwnerDrawDef, ()> for OwnerDrawDefRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek, _data: ()) -> Result<OwnerDrawDef> {
        let data_exp = self.data_exp.xfile_into(xfile, ())?;

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

impl<'a> XFileInto<RectData, ()> for RectDataRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<RectData> {
        let rect_x_exp = self.rect_x_exp.xfile_into(&mut xfile, ())?;
        let rect_y_exp = self.rect_y_exp.xfile_into(&mut xfile, ())?;
        let rect_w_exp = self.rect_w_exp.xfile_into(&mut xfile, ())?;
        let rect_h_exp = self.rect_h_exp.xfile_into(&mut xfile, ())?;

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

impl<'a> XFileInto<UIAnimInfo, ()> for UIAnimInfoRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<UIAnimInfo> {
        let anim_states = self
            .anim_states
            .xfile_into(&mut xfile, ())?
            .into_iter()
            .flatten()
            .collect();
        let current_anim_state = self.current_anim_state.xfile_into(&mut xfile, ())?;
        let next_anim_state = self.next_anim_state.xfile_into(xfile, ())?;
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
    pub name: XString<'a>,
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

impl<'a> XFileInto<AnimParamsDef, ()> for AnimParamsDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek, _data: ()) -> Result<AnimParamsDef> {
        let name = self.name.xfile_into(&mut xfile, ())?;
        let rect_client = self.rect_client.into();
        let fore_color = self.fore_color.into();
        let back_color = self.back_color.into();
        let border_color = self.border_color.into();
        let outline_color = self.outline_color.into();
        let on_event = self.on_event.xfile_into(xfile, ())?;

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
