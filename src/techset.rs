use crate::*;
use num_derive::FromPrimitive;

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialTechniqueSetRaw<'a> {
    pub name: XString<'a>,
    pub world_vert_format: u8,
    unused: u8,
    pub techset_flags: u16,
    #[serde(with = "BigArray")]
    pub techniques: [Ptr32<'a, MaterialTechniqueRaw<'a>>; 130],
}
assert_size!(MaterialTechniqueSetRaw, 528);

impl<'a> Default for MaterialTechniqueSetRaw<'a> {
    fn default() -> Self {
        MaterialTechniqueSetRaw {
            name: XString::default(),
            world_vert_format: u8::default(),
            unused: u8::default(),
            techset_flags: u16::default(),
            techniques: [Ptr32::default(); 130],
        }
    }
}

#[derive(Clone, Debug)]
pub struct MaterialTechniqueSet {
    pub name: String,
    pub world_vert_format: u8,
    pub techset_flags: u16,
    pub techniques: Vec<Box<MaterialTechnique>>,
}

impl<'a> XFileInto<MaterialTechniqueSet> for MaterialTechniqueSetRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialTechniqueSet {
        //dbg!(*self);

        let name = self.name;
        let name = name.xfile_into(&mut xfile);
        //dbg!(&name);

        let techniques = self.techniques;
        let techniques = techniques
            .iter()
            .flat_map(|p| p.xfile_into(&mut xfile))
            .map(|p| Box::new(p))
            .collect::<Vec<_>>();

        assert!(techniques.len() <= 130);
        //dbg!(techniques);

        MaterialTechniqueSet {
            name,
            world_vert_format: self.world_vert_format,
            techset_flags: self.techset_flags,
            techniques,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialTechniqueRaw<'a> {
    pub name: XString<'a>,
    pub flags: u16,
    pub passes: FlexibleArrayU16<MaterialPassRaw<'a>>,
}
assert_size!(MaterialTechniqueRaw, 8);

#[derive(Clone, Debug)]
pub struct MaterialTechnique {
    pub name: String,
    pub flags: u16,
    pub passes: Vec<MaterialPass>,
}

impl<'a> XFileInto<MaterialTechnique> for MaterialTechniqueRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialTechnique {
        //dbg!(*self);

        //dbg!(self.passes);

        // passes must be deserialized first since its a flexible array (part of the MaterialTechnique), not a pointer.
        let passes = self
            .passes
            .to_vec(&mut xfile)
            .iter()
            .map(|t| t.xfile_into(&mut xfile))
            .collect();
        //dbg!(&passes);

        let name = self.name;
        let name = name.xfile_into(&mut xfile);
        //dbg!(&name);

        MaterialTechnique {
            name,
            flags: self.flags,
            passes,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialPassRaw<'a> {
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

impl<'a> XFileInto<MaterialPass> for MaterialPassRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialPass {
        //dbg!(*self);
        //let pos = xfile.stream_position().unwrap();
        //dbg!(pos);

        let vertex_decl = self.vertex_decl.xfile_get(&mut xfile).map(Box::new);
        //dbg!(&vertex_decl);
        let vertex_shader = self.vertex_shader;
        let vertex_shader = vertex_shader.xfile_into(&mut xfile).map(Box::new);
        //dbg!(&vertex_shader);
        let pixel_shader = self.pixel_shader;
        let pixel_shader = pixel_shader.xfile_into(&mut xfile).map(Box::new);
        //dbg!(&pixel_shader);

        let argc = self.per_prim_arg_count as u16 + self.per_obj_arg_count as u16 + self.stable_arg_count as u16;

        let mut args = Vec::with_capacity(argc as _);

        if self.args != 0 {
            for _ in 0..argc {
                //let pos = xfile.stream_position().unwrap();
                //dbg!(pos);
                let arg_raw = load_from_xfile::<MaterialShaderArgumentRaw>(&mut xfile);
                //let pos = xfile.stream_position().unwrap();
                //dbg!(pos);
                let arg = arg_raw.xfile_into(&mut xfile);
                args.push(arg);
            }
        }

        //dbg!(&args);

        MaterialPass {
            vertex_decl,
            vertex_shader,
            pixel_shader,
            per_prim_arg_count: self.per_prim_arg_count,
            per_obj_arg_count: self.per_obj_arg_count,
            stable_arg_count: self.stable_arg_count,
            custom_sampler_flags: self.custom_sampler_flags,
            args,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialVertexDeclaration {
    pub stream_count: u8,
    pub has_optional_source: bool,
    pub is_loaded: bool,
    unused: u8,
    pub routing: MaterialVertexStreamRouting,
}
assert_size!(MaterialVertexDeclaration, 108);

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialVertexStreamRouting {
    pub data: [MaterialStreamRouting; 16],
    pub decl: [u32; 18],
}
assert_size!(MaterialVertexStreamRouting, 104);

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialStreamRouting {
    pub source: u8,
    pub data: u8,
}
assert_size!(MaterialStreamRouting, 2);

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialVertexShaderRaw<'a> {
    pub name: XString<'a>,
    pub prog: MaterialVertexShaderProgramRaw<'a>,
}
assert_size!(MaterialVertexShaderRaw, 16);

#[derive(Clone, Debug)]
pub struct MaterialVertexShader {
    pub name: String,
    pub prog: MaterialVertexShaderProgram,
}

impl<'a> XFileInto<MaterialVertexShader> for MaterialVertexShaderRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialVertexShader {
        //dbg!(*self);
        //let pos = xfile.stream_position().unwrap();
        //dbg!(pos);

        let name = self.name;
        let name = name.xfile_into(&mut xfile);
        //dbg!(&name);

        MaterialVertexShader {
            name,
            prog: self.prog.xfile_into(&mut xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialVertexShaderProgramRaw<'a> {
    pub vs: Ptr32<'a, ()>,
    pub load_def: GfxVertexShaderLoadDefRaw<'a>,
}
assert_size!(MaterialVertexShaderProgramRaw, 12);

#[derive(Clone, Debug)]
pub struct MaterialVertexShaderProgram {
    pub vs: Option<*mut ()>,
    pub load_def: GfxVertexShaderLoadDef,
}

impl<'a> XFileInto<MaterialVertexShaderProgram> for MaterialVertexShaderProgramRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialVertexShaderProgram {
        //dbg!(*self);

        MaterialVertexShaderProgram {
            vs: None,
            load_def: self.load_def.xfile_into(&mut xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct GfxVertexShaderLoadDefRaw<'a> {
    pub program: FatPointerCountLastU32<'a, u32>,
}
assert_size!(GfxVertexShaderLoadDefRaw, 8);

#[derive(Clone, Debug)]
pub struct GfxVertexShaderLoadDef {
    pub program: Vec<u32>,
}

impl<'a> XFileInto<GfxVertexShaderLoadDef> for GfxVertexShaderLoadDefRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> GfxVertexShaderLoadDef {
        //dbg!(*self);

        let program = self.program.to_vec(xfile);
        //dbg!(&program[0]);
        assert!(program[0] == 0xFFFE0300, "program[0] != 0xFFFE0300 ({})", program[0]);

        GfxVertexShaderLoadDef { program }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialPixelShaderRaw<'a> {
    pub name: XString<'a>,
    pub prog: MaterialPixelShaderProgramRaw<'a>,
}
assert_size!(MaterialPixelShaderRaw, 16);

#[derive(Clone, Debug)]
pub struct MaterialPixelShader {
    pub name: String,
    pub prog: MaterialPixelShaderProgram,
}

impl<'a> XFileInto<MaterialPixelShader> for MaterialPixelShaderRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialPixelShader {
        //dbg!(*self);

        let name = self.name;
        let name = name.xfile_into(&mut xfile);
        //dbg!(&name);

        MaterialPixelShader {
            name,
            prog: self.prog.xfile_into(&mut xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialPixelShaderProgramRaw<'a> {
    pub ps: Ptr32<'a, ()>,
    pub load_def: GfxPixelShaderLoadDefRaw<'a>,
}
assert_size!(MaterialPixelShaderProgramRaw, 12);

#[derive(Clone, Debug)]
pub struct MaterialPixelShaderProgram {
    pub ps: Option<*mut ()>,
    pub load_def: GfxPixelShaderLoadDef,
}

impl<'a> XFileInto<MaterialPixelShaderProgram> for MaterialPixelShaderProgramRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialPixelShaderProgram {
        //dbg!(*self);

        MaterialPixelShaderProgram {
            ps: None,
            load_def: self.load_def.xfile_into(&mut xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct GfxPixelShaderLoadDefRaw<'a> {
    pub program: FatPointerCountLastU32<'a, u32>,
}
assert_size!(GfxPixelShaderLoadDefRaw, 8);

#[derive(Clone, Debug)]
pub struct GfxPixelShaderLoadDef {
    pub program: Vec<u32>,
}

impl<'a> XFileInto<GfxPixelShaderLoadDef> for GfxPixelShaderLoadDefRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> GfxPixelShaderLoadDef {
        //dbg!(*self);
        //let pos = xfile.stream_position().unwrap();
        //dbg!(pos);

        let program = self.program.to_vec(xfile);

        GfxPixelShaderLoadDef { program }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MaterialArgumentDef {
    LiteralConst([f32; 4]),
    CodeConst(MaterialArgumentCodeConst),
    CodeSampler(u32),
    NameHash(u32),
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
pub struct MaterialShaderArgumentRaw {
    pub arg_type: u16,
    pub dest: u16,
    pub u: u32,
}
assert_size!(MaterialShaderArgumentRaw, 8);

#[derive(Copy, Clone, Debug)]
pub struct MaterialShaderArgument {
    pub arg_type: MtlArg,
    pub dest: u16,
    pub u: MaterialArgumentDef,
}

impl XFileInto<MaterialShaderArgument> for MaterialShaderArgumentRaw {
    fn xfile_into(&self, xfile: impl Read + Seek) -> MaterialShaderArgument {
        //let pos = xfile.stream_position().unwrap();
        //dbg!(pos);

        //dbg!(*self);

        assert!(self.arg_type <= 7);

        let u = match self.arg_type {
            MTL_ARG_LITERAL_PIXEL_CONST | MTL_ARG_LITERAL_VERTEX_CONST => 
                MaterialArgumentDef::LiteralConst(load_from_xfile(xfile)),
            MTL_ARG_CODE_PIXEL_CONST | MTL_ARG_CODE_VERTEX_CONST =>
                MaterialArgumentDef::CodeConst(MaterialArgumentCodeConst::from_u32(self.u)),
            MTL_ARG_CODE_PIXEL_SAMPLER =>
                MaterialArgumentDef::CodeSampler(self.u),
            MTL_ARG_MATERIAL_VERTEX_CONST | MTL_ARG_MATERIAL_PIXEL_SAMPLER | MTL_ARG_MATERIAL_PRIM_END => 
                MaterialArgumentDef::NameHash(self.u),
            _ => unreachable!(),
        };

        MaterialShaderArgument {
            arg_type: unsafe { transmute(self.arg_type) },
            dest: self.dest,
            u,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
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
}

const MTL_ARG_MATERIAL_VERTEX_CONST: u16 = 0;
const MTL_ARG_LITERAL_VERTEX_CONST: u16 = 1;
const MTL_ARG_MATERIAL_PIXEL_SAMPLER: u16 = 2;
const MTL_ARG_CODE_VERTEX_CONST: u16 = 3;
const MTL_ARG_CODE_PIXEL_SAMPLER: u16 = 4;
const MTL_ARG_MATERIAL_PRIM_END: u16 = 6;
const MTL_ARG_CODE_PIXEL_CONST: u16 = 5;
const MTL_ARG_LITERAL_PIXEL_CONST: u16 = 7;

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(u16)]
pub enum MtlArg {
    MATERIAL_VERTEX_CONST = 0,
    LITERAL_VERTEX_CONST = 1,
    MATERIAL_PIXEL_SAMPLER = 2,
    CODE_VERTEX_CONST = 3,
    CODE_PIXEL_SAMPLER = 4,
    CODE_PIXEL_CONST = 5,
    MATERIAL_PRIM_END = 6,
    LITERAL_PIXEL_CONST = 7,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MaterialRaw<'a> {
    pub info: MaterialInfoRaw<'a>,
    #[serde(with = "BigArray")]
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
    pub state_bits_table: Ptr32<'a, GfxStateBits>
}
assert_size!(MaterialRaw, 192);

pub struct Material {
    pub info: MaterialInfo,
    pub state_bits_entry: [u8; 130],
    pub textures: Vec<MaterialTextureDef>,
    pub constants: Vec<MaterialConstantDef>,
    pub state_bits: Vec<GfxStateBits>,
    pub state_flags: u8,
    pub camera_region: u8,
    pub max_streamed_mips: u8,
    pub technique_set: Box<MaterialTechniqueSet>,
}

impl<'a> XFileInto<Material> for MaterialRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Material {
        let info = self.info.xfile_into(&mut xfile);
        let techset = self.technique_set.xfile_into(&mut xfile).unwrap();
        let mut textures = Vec::new();
        for _ in 0..self.texture_count {
            textures.push(load_from_xfile::<MaterialTextureDefRaw>(&mut xfile).xfile_into(&mut xfile));
        }
        let mut constants = Vec::new();
        for _ in 0..self.constant_count {
            constants.push(load_from_xfile::<MaterialConstantDef>(&mut xfile));
        }
        let mut state_bits = Vec::new();
        for _ in 0..self.constant_count {
            state_bits.push(load_from_xfile::<GfxStateBits>(&mut xfile));
        }

        Material { 
            info, 
            state_bits_entry: self.state_bits_entry, 
            textures, 
            constants, 
            state_bits, 
            state_flags: self.state_flags, 
            camera_region: self.camera_region, 
            max_streamed_mips: self.max_streamed_mips,
            technique_set: Box::new(techset)
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MaterialInfoRaw<'a> {
    pub name: XString<'a>,
    pub game_flags: u32,
    pad: u8,
    pub sort_key: u8,
    pub texture_atlas_row_count: u8,
    pub texture_atlas_column_count: u8,
    pub draw_surf: GfxDrawSurf,
    pub surface_type_bits: u32,
    pub layered_surface_types: u32,
    pub hash_index: u16,
    unused: [u8; 6],
}
assert_size!(MaterialInfoRaw, 40);

pub struct MaterialInfo {
    pub name: String,
    pub game_flags: u32,
    pub sort_key: u8,
    pub texture_atlas_row_count: u8,
    pub texture_atlas_column_count: u8,
    pub draw_surf: GfxDrawSurf,
    pub surface_type_bits: u32,
    pub layered_surface_types: u32,
    pub hash_index: u16,
}

impl<'a> XFileInto<MaterialInfo> for MaterialInfoRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialInfo {
        MaterialInfo { 
            name: self.name.xfile_into(&mut xfile), 
            game_flags: self.game_flags, 
            sort_key: self.sort_key, 
            texture_atlas_row_count: self.texture_atlas_row_count, 
            texture_atlas_column_count: self.texture_atlas_column_count, 
            draw_surf: self.draw_surf, 
            surface_type_bits: self.surface_type_bits, 
            layered_surface_types: self.layered_surface_types, 
            hash_index: self.hash_index, 
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxDrawSurf {
    fields: u64,
}
assert_size!(GfxDrawSurf, 8);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MaterialTextureDefRaw<'a> {
    pub name_hash: u32,
    pub name_start: i8,
    pub name_end: i8,
    pub sampler_state: u8,
    pub semantic: u8,
    pub is_mature_content: bool,
    pad: [u8; 3],
    pub u: MaterialTextureDefInfoRaw<'a>,
}
assert_size!(MaterialTextureDefRaw, 16);

pub struct MaterialTextureDef {
    pub name_hash: u32,
    pub name_start: char,
    pub name_end: char,
    pub sampler_state: u8,
    pub semantic: Semantic,
    pub is_mature_content: bool,
    pub u: MaterialTextureDefInfo,
}

impl<'a> XFileInto<MaterialTextureDef> for MaterialTextureDefRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> MaterialTextureDef {
        let semantic = num::FromPrimitive::from_u8(self.semantic).unwrap();
        let info = if semantic == Semantic::WaterMap {
            let p = unsafe { transmute::<_, Ptr32<'a, WaterRaw>>(self.u.p) };
            let w = p.xfile_into(xfile).unwrap();
            MaterialTextureDefInfo::Water(Box::new(w))
        } else {
            let p = unsafe { transmute::<_, Ptr32<'a, GfxImageRaw>>(self.u.p) };
            let i = p.xfile_into(xfile).unwrap();
            MaterialTextureDefInfo::Image(Box::new(i))
        };

        MaterialTextureDef { 
            name_hash: self.name_hash, 
            name_start: core::char::from_u32(self.name_start as _).unwrap(), 
            name_end: core::char::from_u32(self.name_end as _).unwrap(), 
            sampler_state: self.sampler_state, 
            semantic, 
            is_mature_content: self.is_mature_content, 
            u: info,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Semantic {
    Idle = 0x00,
    Function = 0x01,
    ColorMap = 0x02,
    NormalMap = 0x05,
    SpecularMap = 0x08,
    WaterMap = 0x0B,
    Color7 = 0x13,
    Color15 = 0x1B,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MaterialTextureDefInfoRaw<'a> {
    p: Ptr32<'a, ()>,
}
assert_size!(MaterialTextureDefInfoRaw, 4);

pub enum MaterialTextureDefInfo {
    Image(Box<GfxImage>),
    Water(Box<Water>),
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct WaterRaw<'a> {
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

pub struct Water {
    pub writable: WaterWrtitable,
    pub h0: Vec<Complex>,
    pub w_term: Vec<f32>,
    pub m: i32,
    pub n: i32,
    pub lx: f32,
    pub ly: f32,
    pub gravity: f32,
    pub winddir: [f32; 2],
    pub amplitude: f32,
    pub code_constant: [f32; 4],
    pub image: Box<GfxImage>,
}

impl<'a> XFileInto<Water> for WaterRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Water {
        let h0 = if self.h0.0 != 0 {
            let mut h0 = Vec::new();
            for _ in 0..self.m * self.n {
                h0.push(load_from_xfile(&mut xfile));
            }
            h0
        } else {
            Vec::new()
        };

        let w_term = if self.w_term.0 != 0 {
            let mut w_term = Vec::new();
            for _ in 0..self.m * self.n {
                w_term.push(load_from_xfile(&mut xfile));
            }
            w_term
        } else {
            Vec::new()
        };

        Water { 
            writable: self.writable, 
            h0, 
            w_term, 
            m: self.m, 
            n: self.n, 
            lx: self.lx, 
            ly: self.ly, 
            gravity: self.gravity, 
            winddir: self.winddir, 
            amplitude: self.amplitude, 
            code_constant: self.code_constant, 
            image: Box::new(self.image.xfile_into(xfile).unwrap()),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct WaterWrtitable {
    pub float_time: f32,
}
assert_size!(WaterWrtitable, 4);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Complex {
    pub real: f32,
    pub imag: f32,
}
assert_size!(Complex, 8);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxImageRaw<'a> {
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
    pub name: XString<'a>,
    pub hash: u32,
}
assert_size!(GfxImageRaw, 52);

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
    pub name: String,
    pub hash: u32,
}

impl<'a> XFileInto<GfxImage> for GfxImageRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> GfxImage {
        let map_type = num::FromPrimitive::from_u8(self.map_type).unwrap();
        let semantic = num::FromPrimitive::from_u8(self.semantic).unwrap();
        let category = num::FromPrimitive::from_u8(self.category).unwrap();

        let picmip = if self.no_picmip {
            None
        } else {
            Some(self.picmip)
        };

        let texture = self.texture.p.cast::<GfxImageLoadDefRaw>().xfile_into(&mut xfile).unwrap();

        GfxImage { 
            texture: GfxTexture::LoadDef(Box::new(texture)), 
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
            pixels: Vec::new(), 
            loaded_size: self.loaded_size, 
            skipped_mip_levels: self.skipped_mip_levels, 
            name: self.name.xfile_into(xfile), 
            hash: self.hash, 
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxTextureRaw<'a> {
    p: Ptr32<'a, ()>,
}
assert_size!(GfxTextureRaw, 4);

type IDirect3DBaseTexture9 = ();
type IDirect3DTexture9 = ();
type IDirect3DVolumeTexture9 = ();
type IDirect3DCubeTexture9 = ();

// 2D -> Map
// 3D -> Volmap
// Cube -> Cubemap
pub enum GfxTexture {
    Map(Box<IDirect3DTexture9>),
    Volmap(Box<IDirect3DVolumeTexture9>),
    Cubemap(Box<IDirect3DCubeTexture9>),
    LoadDef(Box<GfxImageLoadDef>),
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum MapType {
    TwoD = 0x03,
    ThreeD = 0x04,
    Cube = 0x05,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum ImgCategory {
    Unknown = 0x00,
    LoadFromFile = 0x03,
    Water = 0x05,
    RenderTarget = 0x06,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Picmip {
    pub platform: [u8; 2],
}
assert_size!(Picmip, 2);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct CardMemory {
    pub platform: [u32; 2],
}
assert_size!(CardMemory, 8);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MaterialConstantDef {
    pub name_hash: u32,
    pub name: [u8; 12],
    pub literal: [f32; 4],
}
assert_size!(MaterialConstantDef, 32);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxStateBits {
    pub load_bits: [u32; 2],
}
assert_size!(GfxStateBits, 8);

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxImageLoadDefRaw {
    level_count: u8,
    flags: u8,
    format: D3DFORMAT,
    resource: FlexibleArrayU32<u8>,
}
assert_size!(GfxImageLoadDefRaw, 12);

pub struct GfxImageLoadDef {
    level_count: u8,
    flags: u8,
    format: D3DFORMAT,
    resource: Vec<u8>,
}

type D3DFORMAT = i32;

impl XFileInto<GfxImageLoadDef> for GfxImageLoadDefRaw {
    fn xfile_into(&self, xfile: impl Read + Seek) -> GfxImageLoadDef {
        GfxImageLoadDef { 
            level_count: self.level_count, 
            flags: self.flags, 
            format: self.format, 
            resource: self.resource.to_vec(xfile), 
        }
    }
}