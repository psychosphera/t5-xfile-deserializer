use core::mem::transmute;

use alloc::{boxed::Box, format, vec::Vec};

#[allow(unused_imports)]
use crate::prelude::*;

use crate::{
    Error, ErrorKind, FatPointer, FatPointerCountLastU32, FlexibleArray, FlexibleArrayU16,
    FlexibleArrayU32, Ptr32, Result, T5XFileDeserialize, T5XFileSerialize, XFileDeserializeInto,
    XFileSerialize, XString, XStringRaw, assert_size,
    common::{GfxCubeTexture, GfxPixelShader, GfxVertexShader, GfxVolumeTexture, Vec2, Vec4},
    file_line_col,
};

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

const MAX_TECHNIQUES: usize = 130;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct MaterialTechniqueSetRaw<'a> {
    pub name: XStringRaw<'a>,
    pub world_vert_format: u8,
    #[allow(dead_code)]
    unused: u8,
    pub techset_flags: u16,
    #[serde(with = "serde_arrays")]
    pub techniques: [Ptr32<'a, MaterialTechniqueRaw<'a>>; MAX_TECHNIQUES],
}
assert_size!(MaterialTechniqueSetRaw, 528);

impl<'a> Default for MaterialTechniqueSetRaw<'a> {
    fn default() -> Self {
        MaterialTechniqueSetRaw {
            name: XStringRaw::default(),
            world_vert_format: u8::default(),
            unused: u8::default(),
            techset_flags: u16::default(),
            techniques: [Ptr32::default(); MAX_TECHNIQUES],
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialTechniqueSet {
    pub name: XString,
    pub world_vert_format: u8,
    pub techset_flags: u16,
    pub techniques: Vec<Box<MaterialTechnique>>,
}

impl<'a> XFileDeserializeInto<MaterialTechniqueSet, ()> for MaterialTechniqueSetRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialTechniqueSet> {
        //dbg!(self.name);

        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);

        let techniques = self.techniques;
        let techniques = techniques
            .iter()
            .flat_map(|p| p.xfile_deserialize_into(de, ()))
            .flatten()
            .collect::<Vec<_>>();

        //dbg!(techniques);

        Ok(MaterialTechniqueSet {
            name,
            world_vert_format: self.world_vert_format,
            techset_flags: self.techset_flags,
            techniques,
        })
    }
}

impl XFileSerialize<()> for MaterialTechniqueSet {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let mut techniques = self
            .techniques
            .iter()
            .map(|_| Ptr32::unreal())
            .collect::<Vec<_>>();
        assert!(techniques.len() <= MAX_TECHNIQUES);
        techniques.resize(MAX_TECHNIQUES, Ptr32::null());
        let techset = MaterialTechniqueSetRaw {
            name,
            world_vert_format: self.world_vert_format,
            unused: 0u8,
            techset_flags: self.techset_flags,
            techniques: techniques.try_into().unwrap(),
        };
        ser.store_into_xfile(techset)?;
        self.name.xfile_serialize(ser, ())?;
        self.techniques.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialTechniqueRaw<'a> {
    pub name: XStringRaw<'a>,
    pub flags: u16,
    pub passes: FlexibleArrayU16<MaterialPassRaw<'a>>,
}
assert_size!(MaterialTechniqueRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialTechnique {
    pub name: XString,
    pub flags: u16,
    pub passes: Vec<MaterialPass>,
}

impl<'a> XFileDeserializeInto<MaterialTechnique, ()> for MaterialTechniqueRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialTechnique> {
        //dbg!(self);

        // passes must be deserialized first since its a flexible array (part of the MaterialTechnique), not a pointer.
        let passes = self
            .passes
            .to_vec(de)?
            .iter()
            .map(|t| t.xfile_deserialize_into(de, ()))
            .collect::<Result<Vec<_>>>()?;
        //dbg!(&passes);

        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);

        Ok(MaterialTechnique {
            name,
            flags: self.flags,
            passes,
        })
    }
}

impl XFileSerialize<()> for MaterialTechnique {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let passes = FlexibleArrayU16::new(self.passes.len());
        let technique = MaterialTechniqueRaw {
            name,
            flags: self.flags,
            passes,
        };
        ser.store_into_xfile(technique)?;
        self.passes.xfile_serialize(ser, ())?;
        self.name.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialPassRaw<'a> {
    pub vertex_decl: Ptr32<'a, MaterialVertexDeclaration>,
    pub vertex_shader: Ptr32<'a, MaterialVertexShaderRaw<'a>>,
    pub pixel_shader: Ptr32<'a, MaterialPixelShaderRaw<'a>>,
    pub per_prim_arg_count: u8,
    pub per_obj_arg_count: u8,
    pub stable_arg_count: u8,
    pub custom_sampler_flags: u8,
    pub args: u32,
}
assert_size!(MaterialPassRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialPass {
    pub vertex_decl: Option<Box<MaterialVertexDeclaration>>,
    pub vertex_shader: Option<Box<MaterialVertexShader>>,
    pub pixel_shader: Option<Box<MaterialPixelShader>>,
    pub per_prim_arg_count: u8,
    pub per_obj_arg_count: u8,
    pub stable_arg_count: u8,
    pub custom_sampler_flags: u8,
    pub args: Vec<MaterialShaderArgument>,
}

impl<'a> XFileDeserializeInto<MaterialPass, ()> for MaterialPassRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialPass> {
        //dbg!(self);
        //let pos = xfile.stream_position()?;
        //dbg!(pos);

        let vertex_decl = self.vertex_decl.xfile_get(de)?.map(Box::new);
        //dbg!(&vertex_decl);
        let vertex_shader = self.vertex_shader;
        let vertex_shader = vertex_shader.xfile_deserialize_into(de, ())?;
        //dbg!(&vertex_shader);
        let pixel_shader = self.pixel_shader;
        let pixel_shader = pixel_shader.xfile_deserialize_into(de, ())?;
        //dbg!(&pixel_shader);

        let argc = self.per_prim_arg_count as u16
            + self.per_obj_arg_count as u16
            + self.stable_arg_count as u16;

        let mut args = Vec::with_capacity(argc as _);

        if self.args != 0 {
            for _ in 0..argc {
                //let pos = xfile.stream_position()?;
                //dbg!(pos);
                let arg_raw = de.load_from_xfile::<MaterialShaderArgumentRaw>()?;
                //let pos = xfile.stream_position()?;
                //dbg!(pos);
                let arg = arg_raw.xfile_deserialize_into(de, ())?;
                args.push(arg);
            }
        }

        //dbg!(&args);

        Ok(MaterialPass {
            vertex_decl,
            vertex_shader,
            pixel_shader,
            per_prim_arg_count: self.per_prim_arg_count,
            per_obj_arg_count: self.per_obj_arg_count,
            stable_arg_count: self.stable_arg_count,
            custom_sampler_flags: self.custom_sampler_flags,
            args,
        })
    }
}

impl XFileSerialize<()> for MaterialPass {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let vertex_decl = Ptr32::from_box(&self.vertex_decl);
        let vertex_shader = Ptr32::from_box(&self.vertex_shader);
        let pixel_shader = Ptr32::from_box(&self.pixel_shader);

        let pass = MaterialPassRaw {
            vertex_decl,
            vertex_shader,
            pixel_shader,
            per_prim_arg_count: self.per_prim_arg_count,
            per_obj_arg_count: self.per_obj_arg_count,
            stable_arg_count: self.stable_arg_count,
            custom_sampler_flags: self.custom_sampler_flags,
            args: self.args.len() as _,
        };

        ser.store_into_xfile(pass)?;
        self.vertex_decl.xfile_serialize(ser, ())?;
        self.vertex_shader.xfile_serialize(ser, ())?;
        self.pixel_shader.xfile_serialize(ser, ())?;
        for arg in self.args.iter() {
            arg.xfile_serialize(ser, ())?;
        }

        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct MaterialVertexDeclaration {
    pub stream_count: u8,
    pub has_optional_source: bool,
    pub is_loaded: bool,
    #[allow(dead_code)]
    unused: u8,
    pub routing: MaterialVertexStreamRouting,
}
assert_size!(MaterialVertexDeclaration, 108);

impl XFileSerialize<()> for MaterialVertexDeclaration {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct MaterialVertexStreamRouting {
    pub data: [MaterialStreamRouting; 16],
    pub decl: [u32; 18],
}
assert_size!(MaterialVertexStreamRouting, 104);

impl XFileSerialize<()> for MaterialVertexStreamRouting {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct MaterialStreamRouting {
    pub source: u8,
    pub data: u8,
}
assert_size!(MaterialStreamRouting, 2);

impl XFileSerialize<()> for MaterialStreamRouting {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialVertexShaderRaw<'a> {
    pub name: XStringRaw<'a>,
    pub prog: MaterialVertexShaderProgramRaw<'a>,
}
assert_size!(MaterialVertexShaderRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialVertexShader {
    pub name: XString,
    pub prog: MaterialVertexShaderProgram,
}

impl<'a> XFileDeserializeInto<MaterialVertexShader, ()> for MaterialVertexShaderRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialVertexShader> {
        //dbg!(self);
        //let pos = xfile.stream_position()?;
        //dbg!(pos);

        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);

        Ok(MaterialVertexShader {
            name,
            prog: self.prog.xfile_deserialize_into(de, ())?,
        })
    }
}

impl XFileSerialize<()> for MaterialVertexShader {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());

        let vs = Ptr32::null();
        let program = FatPointerCountLastU32::from_slice(&self.prog.load_def.program);
        let load_def = GfxVertexShaderLoadDefRaw { program };
        let prog = MaterialVertexShaderProgramRaw { vs, load_def };

        let vertex_shader = MaterialVertexShaderRaw { name, prog };

        ser.store_into_xfile(vertex_shader)?;
        self.name.xfile_serialize(ser, ())?;
        self.prog.load_def.program.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialVertexShaderProgramRaw<'a> {
    #[cfg_attr(not(feature = "d3d9"), allow(dead_code))]
    pub vs: Ptr32<'a, ()>,
    pub load_def: GfxVertexShaderLoadDefRaw<'a>,
}
assert_size!(MaterialVertexShaderProgramRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialVertexShaderProgram {
    pub vs: Option<Box<GfxVertexShader>>,
    pub load_def: GfxVertexShaderLoadDef,
}

impl<'a> XFileDeserializeInto<MaterialVertexShaderProgram, ()>
    for MaterialVertexShaderProgramRaw<'a>
{
    #[cfg(feature = "d3d9")]
    fn xfile_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialVertexShaderProgram> {
        //dbg!(self);

        let load_def = self.load_def.xfile_deserialize_into(de, ())?;

        let vs = if de.create_d3d9() {
            let vs = unsafe {
                de.d3d9_state()
                    .unwrap()
                    .device
                    .CreateVertexShader(load_def.program.as_ptr())
            }?;
            Some(Box::new(GfxVertexShader(vs)))
        } else {
            None
        };

        Ok(MaterialVertexShaderProgram { vs, load_def })
    }

    #[cfg(not(feature = "d3d9"))]
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialVertexShaderProgram> {
        //dbg!(self);

        let load_def = self.load_def.xfile_deserialize_into(de, ())?;

        Ok(MaterialVertexShaderProgram { vs: None, load_def })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GfxVertexShaderLoadDefRaw<'a> {
    pub program: FatPointerCountLastU32<'a, u32>,
}
assert_size!(GfxVertexShaderLoadDefRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxVertexShaderLoadDef {
    pub program: Vec<u32>,
}

const DXBC_MAGIC: u32 = 0xFFFE0300;

impl<'a> XFileDeserializeInto<GfxVertexShaderLoadDef, ()> for GfxVertexShaderLoadDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxVertexShaderLoadDef> {
        //dbg!(*self);

        let program = self.program.to_vec(de)?;
        //dbg!(&program[0]);
        if !de.silent() && !program.is_empty() && program[0] != DXBC_MAGIC {
            println!(
                "Warning: incorrect magic value for shader program. Shader is probably invalid or corrupt."
            );
        }

        Ok(GfxVertexShaderLoadDef { program })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialPixelShaderRaw<'a> {
    pub name: XStringRaw<'a>,
    pub prog: MaterialPixelShaderProgramRaw<'a>,
}
assert_size!(MaterialPixelShaderRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialPixelShader {
    pub name: XString,
    pub prog: MaterialPixelShaderProgram,
}

impl<'a> XFileDeserializeInto<MaterialPixelShader, ()> for MaterialPixelShaderRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialPixelShader> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let prog = self.prog.xfile_deserialize_into(de, ())?;

        Ok(MaterialPixelShader { name, prog })
    }
}

impl XFileSerialize<()> for MaterialPixelShader {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let ps = Ptr32::null();
        let program = FatPointerCountLastU32::from_slice(&self.prog.load_def.program);
        let load_def = GfxPixelShaderLoadDefRaw { program };
        let prog = MaterialPixelShaderProgramRaw { ps, load_def };
        let pixel_shader = MaterialPixelShaderRaw { name, prog };
        ser.store_into_xfile(pixel_shader)?;
        self.name.xfile_serialize(ser, ())?;
        self.prog.load_def.program.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialPixelShaderProgramRaw<'a> {
    #[cfg_attr(not(feature = "d3d9"), allow(dead_code))]
    pub ps: Ptr32<'a, ()>,
    pub load_def: GfxPixelShaderLoadDefRaw<'a>,
}
assert_size!(MaterialPixelShaderProgramRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialPixelShaderProgram {
    pub ps: Option<Box<GfxPixelShader>>,
    pub load_def: GfxPixelShaderLoadDef,
}

impl<'a> XFileDeserializeInto<MaterialPixelShaderProgram, ()>
    for MaterialPixelShaderProgramRaw<'a>
{
    #[cfg(feature = "d3d9")]
    fn xfile_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialPixelShaderProgram> {
        //dbg!(*self);

        let load_def = self.load_def.xfile_deserialize_into(de, ())?;

        let ps = if de.create_d3d9() {
            let ps = unsafe {
                de.d3d9_state()
                    .unwrap()
                    .device
                    .CreatePixelShader(load_def.program.as_ptr())
            }?;
            Some(Box::new(GfxPixelShader(ps)))
        } else {
            None
        };

        Ok(MaterialPixelShaderProgram { ps, load_def })
    }

    #[cfg(not(feature = "d3d9"))]
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialPixelShaderProgram> {
        //dbg!(*self);

        let load_def = self.load_def.xfile_deserialize_into(de, ())?;

        Ok(MaterialPixelShaderProgram { ps: None, load_def })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GfxPixelShaderLoadDefRaw<'a> {
    pub program: FatPointerCountLastU32<'a, u32>,
}
assert_size!(GfxPixelShaderLoadDefRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxPixelShaderLoadDef {
    pub program: Vec<u32>,
}

impl<'a> XFileDeserializeInto<GfxPixelShaderLoadDef, ()> for GfxPixelShaderLoadDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxPixelShaderLoadDef> {
        //dbg!(*self);
        //let pos = xfile.stream_position()?;
        //dbg!(pos);

        let program = self.program.to_vec(de)?;

        Ok(GfxPixelShaderLoadDef { program })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub enum MaterialArgumentDefRaw {
    LiteralConst([f32; 4]),
    CodeConst(MaterialArgumentCodeConst),
    CodeSampler(u32),
    NameHash(u32),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub enum MaterialArgumentDef {
    LiteralConst(Vec4),
    CodeConst(MaterialArgumentCodeConst),
    CodeSampler(u32),
    NameHash(u32),
}

impl Into<MaterialArgumentDef> for MaterialArgumentDefRaw {
    fn into(self) -> MaterialArgumentDef {
        match self {
            Self::LiteralConst(v) => MaterialArgumentDef::LiteralConst(v.into()),
            Self::CodeConst(c) => MaterialArgumentDef::CodeConst(c),
            Self::CodeSampler(s) => MaterialArgumentDef::CodeSampler(s),
            Self::NameHash(h) => MaterialArgumentDef::NameHash(h),
        }
    }
}

impl Into<MaterialArgumentDefRaw> for MaterialArgumentDef {
    fn into(self) -> MaterialArgumentDefRaw {
        match self {
            Self::LiteralConst(v) => MaterialArgumentDefRaw::LiteralConst(v.get()),
            Self::CodeConst(c) => MaterialArgumentDefRaw::CodeConst(c),
            Self::CodeSampler(s) => MaterialArgumentDefRaw::CodeSampler(s),
            Self::NameHash(h) => MaterialArgumentDefRaw::NameHash(h),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialShaderArgumentRaw {
    pub arg_type: u16,
    pub dest: u16,
    pub u: u32,
}
assert_size!(MaterialShaderArgumentRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub struct MaterialShaderArgument {
    pub arg_type: MtlArg,
    pub dest: u16,
    pub u: MaterialArgumentDef,
}

impl XFileDeserializeInto<MaterialShaderArgument, ()> for MaterialShaderArgumentRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialShaderArgument> {
        //let pos = xfile.stream_position()?;
        //dbg!(pos);

        //dbg!(*self);

        if self.arg_type > 7 {
            return Err(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BrokenInvariant(format!(
                    "MaterialShaderArgument: arg_type ({}) > 7",
                    self.arg_type
                )),
            ));
        }

        let u = match self.arg_type {
            MTL_ARG_LITERAL_PIXEL_CONST | MTL_ARG_LITERAL_VERTEX_CONST => {
                MaterialArgumentDefRaw::LiteralConst(de.load_from_xfile()?)
            }
            MTL_ARG_CODE_PIXEL_CONST | MTL_ARG_CODE_VERTEX_CONST => {
                MaterialArgumentDefRaw::CodeConst(MaterialArgumentCodeConst::from_u32(self.u))
            }
            MTL_ARG_CODE_PIXEL_SAMPLER => MaterialArgumentDefRaw::CodeSampler(self.u),
            MTL_ARG_MATERIAL_VERTEX_CONST
            | MTL_ARG_MATERIAL_PIXEL_SAMPLER
            | MTL_ARG_MATERIAL_PRIM_END => MaterialArgumentDefRaw::NameHash(self.u),
            _ => unreachable!(), // safe because of the check above
        };

        Ok(MaterialShaderArgument {
            arg_type: unsafe { transmute(self.arg_type) },
            dest: self.dest,
            u: u.into(),
        })
    }
}

impl XFileSerialize<()> for MaterialShaderArgument {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let u = match self.u {
            MaterialArgumentDef::LiteralConst(_) => Ptr32::<()>::unreal().as_u32(),
            MaterialArgumentDef::CodeConst(c) => c.as_u32(),
            MaterialArgumentDef::CodeSampler(s) => s,
            MaterialArgumentDef::NameHash(h) => h,
        };

        let arg = MaterialShaderArgumentRaw {
            arg_type: self.arg_type as _,
            dest: self.dest,
            u,
        };

        ser.store_into_xfile(arg)?;
        match self.u {
            MaterialArgumentDef::LiteralConst(v) => {
                ser.store_into_xfile(v.get())?;
            }
            _ => {}
        };
        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct MaterialArgumentCodeConst {
    pub index: u16,
    pub first_row: u8,
    pub row_count: u8,
}
assert_size!(MaterialArgumentCodeConst, 4);

impl MaterialArgumentCodeConst {
    pub fn from_u32(u: u32) -> Self {
        unsafe { transmute(u) }
    }

    pub fn as_u32(self) -> u32 {
        unsafe { transmute(self) }
    }
}

const MTL_ARG_MATERIAL_VERTEX_CONST: u16 = 0;
const MTL_ARG_LITERAL_VERTEX_CONST: u16 = 1;
const MTL_ARG_MATERIAL_PIXEL_SAMPLER: u16 = 2;
const MTL_ARG_CODE_VERTEX_CONST: u16 = 3;
const MTL_ARG_CODE_PIXEL_SAMPLER: u16 = 4;
const MTL_ARG_MATERIAL_PRIM_END: u16 = 6;
const MTL_ARG_CODE_PIXEL_CONST: u16 = 5;
const MTL_ARG_LITERAL_PIXEL_CONST: u16 = 7;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
#[repr(u16)]
pub enum MtlArg {
    #[default]
    MATERIAL_VERTEX_CONST = 0,
    LITERAL_VERTEX_CONST = 1,
    MATERIAL_PIXEL_SAMPLER = 2,
    CODE_VERTEX_CONST = 3,
    CODE_PIXEL_SAMPLER = 4,
    CODE_PIXEL_CONST = 5,
    MATERIAL_PRIM_END = 6,
    LITERAL_PIXEL_CONST = 7,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct MaterialRaw<'a> {
    pub info: MaterialInfoRaw<'a>,
    #[serde(with = "serde_arrays")]
    pub state_bits_entry: [u8; 130],
    pub texture_count: u8,
    pub constant_count: u8,
    pub state_bits_count: u8,
    pub state_flags: u8,
    pub camera_region: u8,
    pub max_streamed_mips: u8,
    pub technique_set: Ptr32<'a, MaterialTechniqueSetRaw<'a>>,
    pub texture_table: Ptr32<'a, MaterialTextureDefRaw<'a>>,
    pub constant_table: Ptr32<'a, MaterialConstantDef>,
    pub state_bits_table: Ptr32<'a, GfxStateBits>,
}
assert_size!(MaterialRaw, 192);

impl<'a> Default for MaterialRaw<'a> {
    fn default() -> Self {
        Self {
            info: MaterialInfoRaw::default(),
            state_bits_entry: [0; 130],
            texture_count: 0,
            constant_count: 0,
            state_bits_count: 0,
            state_flags: 0,
            camera_region: 0,
            max_streamed_mips: 0,
            technique_set: Ptr32::default(),
            texture_table: Ptr32::default(),
            constant_table: Ptr32::default(),
            state_bits_table: Ptr32::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Material {
    pub info: MaterialInfo,
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub state_bits_entry: [u8; MAX_TECHNIQUES],
    pub textures: Vec<MaterialTextureDef>,
    pub constants: Vec<MaterialConstantDef>,
    pub state_bits: Vec<GfxStateBits>,
    pub state_flags: u8,
    pub camera_region: u8,
    pub max_streamed_mips: u8,
    pub technique_set: Option<Box<MaterialTechniqueSet>>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            info: MaterialInfo::default(),
            state_bits_entry: [0; MAX_TECHNIQUES],
            textures: Vec::default(),
            constants: Vec::default(),
            state_bits: Vec::default(),
            state_flags: 0,
            camera_region: 0,
            max_streamed_mips: 0,
            technique_set: None,
        }
    }
}

impl<'a> XFileDeserializeInto<Material, ()> for MaterialRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<Material> {
        //dbg!(self);
        //dbg!(de.stream_pos()?);

        let info = self.info.xfile_deserialize_into(de, ())?;
        let technique_set = self.technique_set.xfile_deserialize_into(de, ())?;
        let textures = self
            .texture_table
            .to_array(self.texture_count as _)
            .xfile_deserialize_into(de, ())?;
        let constants = self
            .constant_table
            .to_array(self.constant_count as _)
            .to_vec(de)?;
        let state_bits = self
            .state_bits_table
            .to_array(self.state_bits_count as _)
            .to_vec(de)?;

        Ok(Material {
            info,
            state_bits_entry: self.state_bits_entry,
            textures,
            constants,
            state_bits,
            state_flags: self.state_flags,
            camera_region: self.camera_region,
            max_streamed_mips: self.max_streamed_mips,
            technique_set,
        })
    }
}

impl XFileSerialize<()> for Material {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.info.name.get());
        let info = MaterialInfoRaw {
            name,
            game_flags: self.info.game_flags,
            pad: 0u8,
            sort_key: self.info.sort_key,
            texture_atlas_row_count: self.info.texture_atlas_row_count,
            texture_atlas_column_count: self.info.texture_atlas_column_count,
            pad2: [0u8; 4],
            draw_surf: self.info.draw_surf,
            surface_type_bits: self.info.surface_type_bits,
            layered_surface_types: self.info.layered_surface_types,
            hash_index: self.info.hash_index as _,
            unused: [0u8; 6],
        };

        let technique_set = Ptr32::from_box(&self.technique_set);
        let texture_table = Ptr32::from_slice(&self.textures);
        let constant_table = Ptr32::from_slice(&self.constants);
        let state_bits_table = Ptr32::from_slice(&self.state_bits);

        let material = MaterialRaw {
            info,
            state_bits_entry: self.state_bits_entry,
            texture_count: self.textures.len() as _,
            constant_count: self.constants.len() as _,
            state_bits_count: self.state_bits.len() as _,
            state_flags: self.state_flags,
            camera_region: self.camera_region,
            max_streamed_mips: self.max_streamed_mips,
            technique_set,
            texture_table,
            constant_table,
            state_bits_table,
        };

        ser.store_into_xfile(material)?;
        self.info.name.xfile_serialize(ser, ())?;
        self.technique_set.xfile_serialize(ser, ())?;
        self.textures.xfile_serialize(ser, ())?;
        self.constants.xfile_serialize(ser, ())?;
        self.state_bits.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialInfoRaw<'a> {
    pub name: XStringRaw<'a>,
    pub game_flags: u32,
    #[allow(dead_code)]
    pad: u8,
    pub sort_key: u8,
    pub texture_atlas_row_count: u8,
    pub texture_atlas_column_count: u8,
    pad2: [u8; 4],
    pub draw_surf: GfxDrawSurf,
    pub surface_type_bits: u32,
    pub layered_surface_types: u32,
    pub hash_index: u16,
    #[allow(dead_code)]
    unused: [u8; 6],
}
assert_size!(MaterialInfoRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct MaterialInfo {
    pub name: XString,
    pub game_flags: u32,
    pub sort_key: u8,
    pub texture_atlas_row_count: u8,
    pub texture_atlas_column_count: u8,
    pub draw_surf: GfxDrawSurf,
    pub surface_type_bits: u32,
    pub layered_surface_types: u32,
    pub hash_index: usize,
}

impl<'a> XFileDeserializeInto<MaterialInfo, ()> for MaterialInfoRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialInfo> {
        //dbg!(self);
        //dbg!(de.stream_pos()?);
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);
        //dbg!(de.stream_pos()?);

        Ok(MaterialInfo {
            name,
            game_flags: self.game_flags,
            sort_key: self.sort_key,
            texture_atlas_row_count: self.texture_atlas_row_count,
            texture_atlas_column_count: self.texture_atlas_column_count,
            draw_surf: self.draw_surf,
            surface_type_bits: self.surface_type_bits,
            layered_surface_types: self.layered_surface_types,
            hash_index: self.hash_index as _,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub struct GfxDrawSurf {
    pub fields: u64,
}
assert_size!(GfxDrawSurf, 8);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialTextureDefRaw<'a> {
    pub name_hash: u32,
    pub name_start: i8,
    pub name_end: i8,
    pub sampler_state: u8,
    pub semantic: u8,
    pub is_mature_content: bool,
    #[allow(dead_code)]
    pad: [u8; 3],
    pub u: MaterialTextureDefInfoRaw<'a>,
}
assert_size!(MaterialTextureDefRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct MaterialTextureDef {
    pub name_hash: u32,
    pub name_start: char,
    pub name_end: char,
    pub sampler_state: u8,
    pub semantic: Semantic,
    pub is_mature_content: bool,
    pub u: MaterialTextureDefInfo,
}

impl<'a> XFileDeserializeInto<MaterialTextureDef, ()> for MaterialTextureDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialTextureDef> {
        let semantic = num::FromPrimitive::from_u8(self.semantic).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.semantic as _),
        ))?;
        let info = if semantic == Semantic::WATER_MAP {
            let p = self.u.p.cast::<WaterRaw>();
            let w = p.xfile_deserialize_into(de, ())?;
            MaterialTextureDefInfo::Water(w)
        } else {
            let p = self.u.p.cast::<GfxImageRaw>();
            let i = p.xfile_deserialize_into(de, ())?;
            MaterialTextureDefInfo::Image(i)
        };

        Ok(MaterialTextureDef {
            name_hash: self.name_hash,
            name_start: core::char::from_u32(self.name_start as _).ok_or(
                Error::new_with_offset(
                    file_line_col!(),
                    de.stream_pos()? as _,
                    ErrorKind::BadChar(self.name_start as _),
                ),
            )?,
            name_end: core::char::from_u32(self.name_end as _).ok_or(Error::new_with_offset(
                file_line_col!(),
                de.stream_pos()? as _,
                ErrorKind::BadChar(self.name_end as _),
            ))?,
            sampler_state: self.sampler_state,
            semantic,
            is_mature_content: self.is_mature_content,
            u: info,
        })
    }
}

impl XFileSerialize<()> for MaterialTextureDef {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let p = match &self.u {
            MaterialTextureDefInfo::Image(i) => Ptr32::from_box(i),
            MaterialTextureDefInfo::Water(w) => Ptr32::from_box(w),
        };
        let u = MaterialTextureDefInfoRaw { p };

        let texture_def = MaterialTextureDefRaw {
            name_hash: self.name_hash,
            name_start: self.name_start as _,
            name_end: self.name_end as _,
            sampler_state: self.sampler_state,
            semantic: self.semantic as _,
            is_mature_content: self.is_mature_content,
            pad: [0u8; 3],
            u,
        };

        ser.store_into_xfile(texture_def)?;
        match &self.u {
            MaterialTextureDefInfo::Image(i) => i.xfile_serialize(ser, ()),
            MaterialTextureDefInfo::Water(w) => w.xfile_serialize(ser, ()),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Semantic {
    #[default]
    IDLE = 0x00,
    FUNCTION = 0x01,
    COLOR_MAP = 0x02,
    NORMAL_MAP = 0x05,
    SPECULAR_MAP = 0x08,
    WATER_MAP = 0x0B,
    UNKNOWN_0C = 0x0C,
    UNKNOWN_0D = 0x0D,
    UNKNOWN_0E = 0x0E,
    UNKNOWN_10 = 0x10,
    UNKNOWN_11 = 0x11,
    COLOR_7 = 0x13,
    COLOR_15 = 0x1B,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct MaterialTextureDefInfoRaw<'a> {
    p: Ptr32<'a, ()>,
}
assert_size!(MaterialTextureDefInfoRaw, 4);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum MaterialTextureDefInfo {
    Image(Option<Box<GfxImage>>),
    Water(Option<Box<Water>>),
}

impl Default for MaterialTextureDefInfo {
    fn default() -> Self {
        Self::Image(None)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct WaterRaw<'a> {
    pub writable: WaterWrtitable,
    pub h0: Ptr32<'a, Complex>,
    pub w_term: Ptr32<'a, f32>,
    pub m: i32,
    pub n: i32,
    pub lx: f32,
    pub ly: f32,
    pub gravity: f32,
    pub windvel: f32,
    pub winddir: [f32; 2],
    pub amplitude: f32,
    pub code_constant: [f32; 4],
    pub image: Ptr32<'a, GfxImageRaw<'a>>,
}
assert_size!(WaterRaw, 68);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct Water {
    pub writable: WaterWrtitable,
    pub h0: Vec<Complex>,
    pub w_term: Vec<f32>,
    pub m: i32,
    pub n: i32,
    pub lx: f32,
    pub ly: f32,
    pub gravity: f32,
    pub windvel: f32,
    pub winddir: Vec2,
    pub amplitude: f32,
    pub code_constant: Vec4,
    pub image: Option<Box<GfxImage>>,
}

impl<'a> XFileDeserializeInto<Water, ()> for WaterRaw<'a> {
    fn xfile_deserialize_into(&self, de: &mut impl T5XFileDeserialize, _data: ()) -> Result<Water> {
        let h0 = if !self.h0.is_null() {
            let mut h0 = Vec::new();
            for _ in 0..self.m * self.n {
                h0.push(de.load_from_xfile()?);
            }
            h0
        } else {
            Vec::new()
        };

        let w_term = if !self.w_term.is_null() {
            let mut w_term = Vec::new();
            for _ in 0..self.m * self.n {
                w_term.push(de.load_from_xfile()?);
            }
            w_term
        } else {
            Vec::new()
        };

        Ok(Water {
            writable: self.writable,
            h0,
            w_term,
            m: self.m,
            n: self.n,
            lx: self.lx,
            ly: self.ly,
            gravity: self.gravity,
            windvel: self.windvel,
            winddir: self.winddir.into(),
            amplitude: self.amplitude,
            code_constant: self.code_constant.into(),
            image: self.image.xfile_deserialize_into(de, ())?,
        })
    }
}

impl XFileSerialize<()> for Water {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let h0 = Ptr32::from_slice(&self.h0);
        let w_term = Ptr32::from_slice(&self.w_term);
        let image = Ptr32::from_box(&self.image);
        let water = WaterRaw {
            writable: self.writable,
            h0,
            w_term,
            m: self.m,
            n: self.n,
            lx: self.lx,
            ly: self.ly,
            gravity: self.gravity,
            windvel: self.windvel,
            winddir: self.winddir.get(),
            amplitude: self.amplitude,
            code_constant: self.code_constant.get(),
            image,
        };

        ser.store_into_xfile(water)?;
        self.h0.xfile_serialize(ser, ())?;
        self.w_term.xfile_serialize(ser, ())?;
        self.image.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct WaterWrtitable {
    pub float_time: f32,
}
assert_size!(WaterWrtitable, 4);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct Complex {
    pub real: f32,
    pub imag: f32,
}
assert_size!(Complex, 8);

impl XFileSerialize<()> for Complex {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GfxImageRaw<'a> {
    pub texture: GfxTextureRaw<'a>,
    pub map_type: u8,
    pub semantic: u8,
    pub category: u8,
    pub delay_load_pixels: bool,
    pub picmip: Picmip,
    pub no_picmip: bool,
    pub track: u8,
    pub card_memory: CardMemory,
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub level_count: u8,
    pub streaming: bool,
    pub base_size: u32,
    pub pixels: Ptr32<'a, u8>,
    pub loaded_size: u32,
    pub skipped_mip_levels: u8,
    pad: [u8; 3],
    pub name: XStringRaw<'a>,
    pub hash: u32,
}
assert_size!(GfxImageRaw, 52);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct GfxImage {
    pub texture: GfxTexture,
    pub map_type: MapType,
    pub semantic: Semantic,
    pub category: ImgCategory,
    pub delay_load_pixels: bool,
    pub picmip: Option<Picmip>,
    pub track: u8,
    pub card_memory: CardMemory,
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub level_count: u8,
    pub streaming: bool,
    pub base_size: u32,
    pub pixels: Vec<u8>,
    pub loaded_size: u32,
    pub skipped_mip_levels: u8,
    pub name: XString,
    pub hash: u32,
}

impl<'a> XFileDeserializeInto<GfxImage, ()> for GfxImageRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxImage> {
        //dbg!(self);
        //let _ = de.load_from_xfile::<[u8; 3]>();
        //dbg!(de.stream_pos()?);
        let name = self.name.xfile_deserialize_into(de, ())?;
        //dbg!(&name);

        let texture = self.texture.xfile_deserialize_into(de, ())?;

        let map_type = num::FromPrimitive::from_u8(self.map_type).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.map_type as _),
        ))?;
        let semantic = num::FromPrimitive::from_u8(self.semantic).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.semantic as _),
        ))?;
        let category = num::FromPrimitive::from_u8(self.category).ok_or(Error::new_with_offset(
            file_line_col!(),
            de.stream_pos()? as _,
            ErrorKind::BadFromPrimitive(self.category as _),
        ))?;

        let picmip = if self.no_picmip {
            None
        } else {
            Some(self.picmip)
        };

        Ok(GfxImage {
            texture,
            map_type,
            semantic,
            category,
            delay_load_pixels: self.delay_load_pixels,
            picmip,
            track: self.track,
            card_memory: self.card_memory,
            width: self.width,
            height: self.height,
            depth: self.depth,
            level_count: self.level_count,
            streaming: self.streaming,
            base_size: self.base_size,
            loaded_size: self.loaded_size,
            pixels: Vec::new(),
            skipped_mip_levels: self.skipped_mip_levels,
            name,
            hash: self.hash,
        })
    }
}

impl XFileSerialize<()> for GfxImage {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let texture = GfxTextureRaw { p: Ptr32::unreal() };

        let pixels = Ptr32::from_slice(&self.pixels);
        let name = XStringRaw::from_str(self.name.get());

        let image = GfxImageRaw {
            texture,
            map_type: self.map_type as _,
            semantic: self.semantic as _,
            category: self.category as _,
            delay_load_pixels: self.delay_load_pixels,
            picmip: self.picmip.unwrap_or_default(),
            no_picmip: self.picmip.is_some(),
            track: self.track,
            card_memory: self.card_memory,
            width: self.width,
            height: self.height,
            depth: self.depth,
            level_count: self.level_count,
            streaming: self.streaming,
            base_size: self.base_size,
            pixels,
            loaded_size: self.loaded_size,
            skipped_mip_levels: self.skipped_mip_levels,
            pad: [0u8; 3],
            name,
            hash: self.hash,
        };

        ser.store_into_xfile(image)?;
        self.name.xfile_serialize(ser, ())?;
        self.texture.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GfxTextureRaw<'a> {
    p: Ptr32<'a, ()>,
}
assert_size!(GfxTextureRaw, 4);

// 2D -> Map
// 3D -> Volmap
// Cube -> Cubemap
// LoadDef -> Used to load one of the above
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum GfxTexture {
    Map(Option<Box<crate::common::GfxTexture>>),
    Volmap(Option<Box<GfxVolumeTexture>>),
    Cubemap(Option<Box<GfxCubeTexture>>),
    LoadDef(Option<Box<GfxImageLoadDef>>),
}

impl Default for GfxTexture {
    fn default() -> Self {
        Self::LoadDef(None)
    }
}

impl<'a> XFileDeserializeInto<GfxTexture, ()> for GfxTextureRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxTexture> {
        //dbg!(de.stream_pos()?);
        let load_def = self
            .p
            .cast::<GfxImageLoadDefRaw>()
            .xfile_deserialize_into(de, ())?;

        Ok(GfxTexture::LoadDef(load_def))
    }
}

impl XFileSerialize<()> for GfxTexture {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        match self {
            Self::LoadDef(d) => {
                let Some(d) = d else {
                    return Ok(());
                };

                let resource = FlexibleArrayU32::new(d.resource.len() as _);
                let load_def = GfxImageLoadDefRaw {
                    level_count: d.level_count,
                    flags: d.flags,
                    pad: [0u8; 2],
                    format: d.format,
                    resource,
                };

                ser.store_into_xfile(load_def)?;
                d.resource.xfile_serialize(ser, ())
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum MapType {
    #[default]
    UNKNOWN = 0x00,
    TWO_DIMENSIONAL = 0x03,
    THREE_DIMENSIONAL = 0x04,
    CUBE = 0x05,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum ImgCategory {
    #[default]
    UNKNOWN = 0x00,
    ONE = 0x01,
    TWO = 0x02,
    LOAD_FROM_FILE = 0x03,
    WATER = 0x05,
    RENDER_TARGET = 0x06,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct Picmip {
    pub platform: [u8; 2],
}
assert_size!(Picmip, 2);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct CardMemory {
    pub platform: [u32; 2],
}
assert_size!(CardMemory, 8);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct MaterialConstantDef {
    pub name_hash: u32,
    pub name: [u8; 12],
    pub literal: [f32; 4],
}
assert_size!(MaterialConstantDef, 32);

impl XFileSerialize<()> for MaterialConstantDef {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct GfxStateBits {
    pub load_bits: [u32; 2],
}
assert_size!(GfxStateBits, 8);

impl XFileSerialize<()> for GfxStateBits {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        ser.store_into_xfile(*self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub(crate) struct GfxImageLoadDefRaw {
    pub level_count: u8,
    pub flags: u8,
    pad: [u8; 2],
    pub format: D3DFORMAT,
    pub resource: FlexibleArrayU32<u8>,
}
assert_size!(GfxImageLoadDefRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct GfxImageLoadDef {
    pub level_count: u8,
    pub flags: u8,
    pub format: D3DFORMAT,
    pub resource: Vec<u8>,
}

type D3DFORMAT = i32;

impl XFileDeserializeInto<GfxImageLoadDef, ()> for GfxImageLoadDefRaw {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxImageLoadDef> {
        //dbg!(self);
        //dbg!(de.stream_pos()?);

        Ok(GfxImageLoadDef {
            level_count: self.level_count,
            flags: self.flags,
            format: self.format,
            resource: self.resource.to_vec(de)?,
        })
    }
}
