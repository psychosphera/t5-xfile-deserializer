#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::panic;
use serde::{
    de::{DeserializeOwned, Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{
    ffi::{CString, OsString},
    fmt::{self, Debug},
    fs::File,
    io::{BufReader, Read, Seek, Write},
    marker::PhantomData,
    mem::{size_of, transmute},
    path::Path,
    str::FromStr,
};

// FastFiles (internally known as XFiles) are structured as follows:
//
// ----------------------------------------------------------------------------
// | Offset    | Size | Field       | Description                             |
// ----------------------------------------------------------------------------
// | 0x0000000 | 4    | Magic       | Magic value to identify the file as an  |
// |           |      |             | XFile. Will always be ASCII "IWff".     |
// ----------------------------------------------------------------------------
// | 0x0000004 | 1    | Compression | Magic values to identify the            |
// |           |      |             | compression method used. Will always be |
// |           |      |             | ASCII 'u' or '0' for T5 on PC.          |
// ----------------------------------------------------------------------------
// | 0x0000005 | 3    | Unknown     | Exact meaning unknown. Maybe it was     |
// |           |      |             | supposed to represent some version info |
// |           |      |             | info? Will always be ASCII "100".       |
// ----------------------------------------------------------------------------
// | 0x0000008 | 4    | Version     | The real version. For reasons explained |
// |           |      |             | below, XFiles are neither backward- nor |
// |           |      |             | forward-compatible for deserialization  |
// |           |      |             | purposes. It is **imperative** that the |
// |           |      |             | XFile version match the version the     |
// |           |      |             | deserializer is expecting. For the      |
// |           |      |             | latest version of T5, the value is      |
// |           |      |             | 0x000001D9                              |
// ----------------------------------------------------------------------------
// | 0x000000C | *    | Blob        | The rest of the file is a DEFLATE-      |
// |           |      |             | compressed blob. To get the "real"      |
// |           |      |             | contents of the file, it must be        |
// |           |      |             | inflated.                               |
// ----------------------------------------------------------------------------
//
// The inflated blob is structured as follows:
//
// ----------------------------------------------------------------------------
// | Offset    | Size | Field       | Description                             |
// ----------------------------------------------------------------------------
// | 0x0000000 | 36   | XFile       | See the [`XFile`] struct below.         |
// ----------------------------------------------------------------------------
// | 0x0000024 | 16   | XAssetList  | See the [`XAssetList`] struct below.    |
// ----------------------------------------------------------------------------
// | 0x0000034 | *    | XAssets     | The XAssets.                            |
// ----------------------------------------------------------------------------
//
// The XAssetList essentially contains two fat pointers: first, to a string
// array, then an asset array. And herein comes the first major annoyance
// with XFiles - the assets are essentially just the structs used by the engine
// serialzed into a file. Any pointers in said structs become offsets in the
// file. Occasionally the offsets are NULL or a "real" value, but most of the
// time they're 0xFFFFFFFF, which indicates that, instead of being at a
// specific offset, they come immediately after the current struct. This means
// basically nothing in the file is relocatable, and removing or adding a
// single byte will corrupt everything after.
//
// In addition, if the structures' sizes or alignments don't match exactly what
// the serializer used, or if new structures are added, the file is basically
// un-parseable (this is why, as mentioned above, the versions must match
// exactly). Pulling out only assets of a specific type is also impossible,
// because you can't know where they're at in the file until you pull out
// everything before it too. For this reason, you're more or less forced into
// deserializng everything at once and then grabbing the assets you need
// afterwards.

trait BigArray<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

macro_rules! big_array {
    ($($len:expr,)+) => {
        $(
            impl<'de, T> BigArray<'de> for [T; $len]
                where T: Default + Copy + Deserialize<'de>
            {
                fn deserialize<D>(deserializer: D) -> Result<[T; $len], D::Error>
                    where D: Deserializer<'de>
                {
                    struct ArrayVisitor<T> {
                        element: PhantomData<T>,
                    }

                    impl<'de, T> Visitor<'de> for ArrayVisitor<T>
                        where T: Default + Copy + Deserialize<'de>
                    {
                        type Value = [T; $len];

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str(concat!("an array of length ", $len))
                        }

                        fn visit_seq<A>(self, mut seq: A) -> Result<[T; $len], A::Error>
                            where A: SeqAccess<'de>
                        {
                            let mut arr = [T::default(); $len];
                            for i in 0..$len {
                                arr[i] = seq.next_element()?
                                    .ok_or_else(|| Error::invalid_length(i, &self))?;
                            }
                            Ok(arr)
                        }
                    }

                    let visitor = ArrayVisitor { element: PhantomData };
                    deserializer.deserialize_tuple($len, visitor)
                }
            }
        )+
    }
}

big_array! {
    130,
}

macro_rules! assert_size {
    ($t:ty, $n:literal) => {
        static_assertions::assert_eq_size!($t, [u8; $n]);
    }
}

macro_rules! impl_xfile_into_primitive {
    ($($t:ty),+) => {
        $(
            impl XFileInto<$t> for $t {
                fn xfile_into(&self, _xfile: impl Read + Seek) -> $t {
                    *self
                }
            }
        )+
    };
}

impl_xfile_into_primitive!(
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64, bool
);

fn load_from_xfile<T: DeserializeOwned>(xfile: impl Read + Seek) -> T {
    bincode::deserialize_from::<_, T>(xfile).unwrap()
}

trait XFileInto<T> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> T;
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(transparent)]
struct Ptr32<'a, T>(u32, PhantomData<&'a mut T>);

impl<'a, T> Default for Ptr32<'a, T> {
    fn default() -> Self {
        Self(0, PhantomData::default())
    }
}

impl<'a, T: DeserializeOwned + Clone + Copy + Debug + XFileInto<U>, U> XFileInto<Option<U>>
    for Ptr32<'a, T>
{
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> Option<U> {
        if self.0 == 0x00000000 {
            return None;
        }

        let pos = xfile.stream_position().unwrap();

        if self.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(self.0 as _)).unwrap();
        }

        let t = bincode::deserialize_from::<_, T>(&mut xfile).unwrap();

        if self.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(pos as _)).unwrap();
        }

        Some(t.xfile_into(&mut xfile))
    }
}

impl<'a, T: DeserializeOwned + Debug> Ptr32<'a, T> {
    fn xfile_get(self, mut xfile: impl Read + Seek) -> Option<T> {
        if self.0 == 0x00000000 {
            return None;
        }

        let pos = xfile.stream_position().unwrap();

        if self.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(self.0 as _)).unwrap();
        }

        let t = bincode::deserialize_from::<_, T>(&mut xfile).unwrap();

        if self.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(pos as _)).unwrap();
        }

        let t = dbg!(t);

        Some(t)
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(transparent)]
struct FlexibleArrayU16<T: DeserializeOwned> {
    count: u16,
    _p: PhantomData<T>,
}

impl<T: DeserializeOwned + Copy> FlexibleArrayU16<T> {
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        let mut v = vec![0u8; self.count as usize * size_of::<T>()];

        xfile.read_exact(&mut v).unwrap();

        let mut vt = Vec::new();

        for i in 0..self.count as usize {
            let s = &v[i * size_of::<T>()..(i + 1) * size_of::<T>()];
            vt.push(bincode::deserialize(s).unwrap());
        }

        vt
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(transparent)]
struct FlexibleArrayU32<T: DeserializeOwned> {
    count: u32,
    _p: PhantomData<T>,
}

impl<T: DeserializeOwned> FlexibleArrayU32<T> {
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        let mut v = vec![0u8; self.count as usize * size_of::<T>()];

        xfile.read_exact(&mut v).unwrap();

        let mut vt = Vec::new();

        for i in 0..self.count as usize {
            let s = &v[i * size_of::<T>()..(i + 1) * size_of::<T>()];
            vt.push(bincode::deserialize(s).unwrap());
        }

        vt
    }
}

#[derive(Clone, Debug, Deserialize)]
struct FatPointerCountFirstU16<'a, T: Debug + Clone> {
    size: u16,
    p: Ptr32<'a, T>,
}

impl<'a, T: DeserializeOwned + Debug + Clone> FatPointerCountFirstU16<'a, T> {
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        let pos = xfile.stream_position().unwrap();

        if self.p.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(self.p.0 as _)).unwrap();
        }

        let mut vt = Vec::new();

        for _ in 0..self.size {
            vt.push(bincode::deserialize_from(&mut xfile).unwrap());
        }

        if self.p.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(pos as _)).unwrap();
        }

        vt
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
struct FatPointerCountFirstU32<'a, T> {
    size: u32,
    p: Ptr32<'a, T>,
}

impl<'a, T: DeserializeOwned + Debug + Copy> FatPointerCountFirstU32<'a, T> {
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        let pos = xfile.stream_position().unwrap();

        if self.p.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(self.p.0 as _)).unwrap();
        }

        let mut vt = Vec::new();

        for _ in 0..self.size {
            vt.push(bincode::deserialize_from(&mut xfile).unwrap());
        }

        if self.p.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(pos as _)).unwrap();
        }

        vt
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
struct FatPointerCountLastU16<'a, T> {
    p: Ptr32<'a, T>,
    size: u16,
}

impl<'a, T: DeserializeOwned + Debug + Copy> FatPointerCountLastU16<'a, T> {
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        let pos = xfile.stream_position().unwrap();

        if self.p.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(self.p.0 as _)).unwrap();
        }

        let mut vt = Vec::new();

        for _ in 0..self.size {
            vt.push(bincode::deserialize_from(&mut xfile).unwrap());
        }

        if self.p.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(pos as _)).unwrap();
        }

        vt
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
struct FatPointerCountLastU32<'a, T> {
    p: Ptr32<'a, T>,
    size: u32,
}

impl<'a, T: DeserializeOwned + Debug + Copy> FatPointerCountLastU32<'a, T> {
    fn to_vec(self, mut xfile: impl Read + Seek) -> Vec<T> {
        if self.p.0 == 0x00000000 {
            return Vec::new();
        }

        let pos = xfile.stream_position().unwrap();

        if self.p.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(self.p.0 as _)).unwrap();
        }

        let mut vt = Vec::new();

        for _ in 0..self.size {
            vt.push(bincode::deserialize_from(&mut xfile).unwrap());
        }

        if self.p.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(pos as _)).unwrap();
        }

        vt
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct XFileHeader {
    magic: [u8; 8],
    version: u32,
}

assert_size!(XFileHeader, 12);

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct XFile {
    size: u32,
    external_size: u32,
    block_size: [u32; 7],
}

assert_size!(XFile, 36);

#[derive(Deserialize)]
#[repr(C, packed)]
struct XAssetList {
    string_count: u32,
    strings: u32,
    asset_count: u32,
    assets: u32,
}

assert_size!(XAssetList, 16);

fn xfile_header_magic_is_valid(header: &XFileHeader) -> bool {
    header.magic[0] == b'I'
        && header.magic[1] == b'W'
        && header.magic[2] == b'f'
        && header.magic[3] == b'f'
        && (header.magic[4] == b'u' || header.magic[4] == b'0')
        && header.magic[5] == b'1'
        && header.magic[6] == b'0'
        && header.magic[7] == b'0'
}

const XFILE_VERSION: u32 = 0x000001D9u32;

fn xfile_is_correct_version(header: &XFileHeader) -> bool {
    header.version == XFILE_VERSION
}

fn decompress_xfile(filename: impl AsRef<Path>) -> BufReader<File> {
    let mut file = File::open(&filename).unwrap();

    println!(
        "Found file '{}', reading header...",
        filename.as_ref().display()
    );

    let mut header_bytes = [0u8; 12];
    file.read_exact(&mut header_bytes).unwrap();

    println!("Header read, verifying...");

    let header = bincode::deserialize::<XFileHeader>(&header_bytes).unwrap();
    assert!(
        xfile_header_magic_is_valid(&header),
        "Fastfile header magic invalid: valid values are IWffu100 and IWff0100"
    );
    assert!(
        xfile_is_correct_version(&header),
        "Fastfile is wrong version (version: 0x{:08x}, correct version: {}",
        { let header = header.version; header },
        XFILE_VERSION
    );

    println!("Header verified, reading payload...");

    let mut compressed_payload = Vec::new();
    let bytes_read = file.read_to_end(&mut compressed_payload).unwrap();
    assert!(bytes_read as u64 == file.metadata().unwrap().len() - 12);

    println!("Payload read, inflating... (this may take a while)");

    let decompressed_payload = inflate::inflate_bytes_zlib(&compressed_payload).unwrap();

    println!(
        "Payload inflated, compressed size: {} bytes, decompressed size, {} bytes",
        bytes_read,
        decompressed_payload.len()
    );
    println!("Caching decompressed payload to disk...");

    let mut file_out = File::create(filename.as_ref().with_extension("cache")).unwrap();
    file_out.write_all(&decompressed_payload).unwrap();
    file_out.seek(std::io::SeekFrom::Start(0)).unwrap();

    println!("Decompressed payload cached.");
    BufReader::new(file_out)
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct XAsset<'a> {
    asset_type: u32,
    asset_data: Ptr32<'a, ()>,
}

assert_size!(XAsset, 8);

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, Deserialize)]
struct XString(u32);

assert_size!(XString, 4);

impl XFileInto<String> for XString {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> String {
        if self.0 == 0x00000000 {
            return String::new();
        }

        dbg!(*self);

        let pos = xfile.stream_position().unwrap();

        if self.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(self.0 as _)).unwrap();
        }

        let s = file_read_string(&mut xfile);

        if self.0 != 0xFFFFFFFF {
            xfile.seek(std::io::SeekFrom::Start(pos as _)).unwrap();
        }

        s
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialTechniqueSetRaw<'a> {
    name: XString,
    world_vert_format: u8,
    unused: u8,
    techset_flags: u16,
    #[serde(with = "BigArray")]
    techniques: [Ptr32<'a, MaterialTechniqueRaw<'a>>; 130],
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
struct MaterialTechniqueSet {
    name: String,
    world_vert_format: u8,
    unused: u8,
    techset_flags: u16,
    techniques: [Box<MaterialTechnique>; 130],
}

impl<'a> XFileInto<MaterialTechniqueSet> for MaterialTechniqueSetRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialTechniqueSet {
        dbg!(*self);

        let name = self.name;
        let name = name.xfile_into(&mut xfile);
        dbg!(&name);

        let techniques = self.techniques;
        let techniques = techniques
            .iter()
            .flat_map(|p| p.xfile_into(&mut xfile))
            .map(|p| Box::new(p))
            .collect::<Vec<_>>();

        dbg!(&techniques);

        MaterialTechniqueSet {
            name,
            world_vert_format: self.world_vert_format,
            unused: self.unused,
            techset_flags: self.techset_flags,
            techniques: techniques.try_into().unwrap(),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialTechniqueRaw<'a> {
    name: XString,
    flags: u16,
    passes: FlexibleArrayU16<MaterialPassRaw<'a>>,
}

assert_size!(MaterialTechniqueRaw, 8);

#[derive(Clone, Debug)]
struct MaterialTechnique {
    name: String,
    flags: u16,
    passes: Vec<MaterialPass>,
}

impl<'a> XFileInto<MaterialTechnique> for MaterialTechniqueRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialTechnique {
        dbg!(*self);

        dbg!(self.passes);

        // passes must be deserialized first since its a flexible array (part of the MaterialTechnique), not a pointer.
        let passes = self
            .passes
            .to_vec(&mut xfile)
            .iter()
            .map(|t| t.xfile_into(&mut xfile))
            .collect();
        // dbg!(&passes);

        let name = self.name;
        let name = name.xfile_into(&mut xfile);
        dbg!(&name);

        MaterialTechnique {
            name,
            flags: self.flags,
            passes,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialPassRaw<'a> {
    vertex_decl: Ptr32<'a, MaterialVertexDeclaration>,
    vertex_shader: Ptr32<'a, MaterialVertexShaderRaw<'a>>,
    pixel_shader: Ptr32<'a, MaterialPixelShaderRaw<'a>>,
    per_prim_arg_count: u8,
    per_obj_arg_count: u8,
    stable_arg_count: u8,
    custom_sampler_flags: u8,
    args: u32,
}

assert_size!(MaterialPassRaw, 20);

#[derive(Clone, Debug)]
struct MaterialPass {
    vertex_decl: Option<Box<MaterialVertexDeclaration>>,
    vertex_shader: Option<Box<MaterialVertexShader>>,
    pixel_shader: Option<Box<MaterialPixelShader>>,
    per_prim_arg_count: u8,
    per_obj_arg_count: u8,
    stable_arg_count: u8,
    custom_sampler_flags: u8,
    args: Vec<MaterialShaderArgument>,
}

impl<'a> XFileInto<MaterialPass> for MaterialPassRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialPass {
        dbg!(*self);

        let vertex_decl = self.vertex_decl.xfile_get(&mut xfile).map(Box::new);
        let vertex_shader = self.vertex_shader;
        let vertex_shader = vertex_shader.xfile_into(&mut xfile).map(Box::new);
        let pixel_shader = self.pixel_shader;
        let pixel_shader = pixel_shader.xfile_into(&mut xfile).map(Box::new);

        let argc = self.per_obj_arg_count + self.per_obj_arg_count + self.stable_arg_count;

        let mut args = Vec::with_capacity(argc as _);
        for _ in 0..argc {
            let pos = xfile.stream_position().unwrap();
            dbg!(pos);
            let arg_raw = load_from_xfile::<MaterialShaderArgumentRaw>(&mut xfile);
            let pos = xfile.stream_position().unwrap();
            dbg!(pos);
            let arg = arg_raw.xfile_into(&mut xfile);
            args.push(arg);
        }

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
struct MaterialVertexDeclaration {
    stream_count: u8,
    has_optional_source: bool,
    is_loaded: bool,
    unused: u8,
    routing: MaterialVertexStreamRouting,
}

assert_size!(MaterialVertexDeclaration, 108);

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialVertexStreamRouting {
    data: [MaterialStreamRouting; 16],
    decl: [u32; 18],
}

assert_size!(MaterialVertexStreamRouting, 104);

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialStreamRouting {
    source: u8,
    data: u8,
}

assert_size!(MaterialStreamRouting, 2);

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialVertexShaderRaw<'a> {
    name: XString,
    prog: MaterialVertexShaderProgramRaw<'a>,
}

assert_size!(MaterialVertexShaderRaw, 16);

#[derive(Clone, Debug)]
struct MaterialVertexShader {
    name: String,
    prog: MaterialVertexShaderProgram,
}

impl<'a> XFileInto<MaterialVertexShader> for MaterialVertexShaderRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialVertexShader {
        dbg!(*self);

        let name = self.name;
        let name = name.xfile_into(&mut xfile);
        dbg!(&name);

        MaterialVertexShader {
            name,
            prog: self.prog.xfile_into(&mut xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialVertexShaderProgramRaw<'a> {
    vs: Ptr32<'a, ()>,
    load_def: GfxVertexShaderLoadDefRaw<'a>,
}

assert_size!(MaterialVertexShaderProgramRaw, 12);

#[derive(Clone, Debug)]
struct MaterialVertexShaderProgram {
    vs: Option<*mut ()>,
    load_def: GfxVertexShaderLoadDef,
}

impl<'a> XFileInto<MaterialVertexShaderProgram> for MaterialVertexShaderProgramRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialVertexShaderProgram {
        dbg!(*self);

        MaterialVertexShaderProgram {
            vs: None,
            load_def: self.load_def.xfile_into(&mut xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct GfxVertexShaderLoadDefRaw<'a> {
    program: FatPointerCountLastU16<'a, u32>,
}

assert_size!(GfxVertexShaderLoadDefRaw, 8);

#[derive(Clone, Debug)]
struct GfxVertexShaderLoadDef {
    program: Vec<u32>,
}

impl<'a> XFileInto<GfxVertexShaderLoadDef> for GfxVertexShaderLoadDefRaw<'a> {
    fn xfile_into(&self, xfile: impl Read + Seek) -> GfxVertexShaderLoadDef {
        dbg!(*self);

        let program = self.program.to_vec(xfile);

        GfxVertexShaderLoadDef { program }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialPixelShaderRaw<'a> {
    name: XString,
    prog: MaterialPixelShaderProgramRaw<'a>,
}

assert_size!(MaterialPixelShaderRaw, 16);

#[derive(Clone, Debug)]
struct MaterialPixelShader {
    name: String,
    prog: MaterialPixelShaderProgram,
}

impl<'a> XFileInto<MaterialPixelShader> for MaterialPixelShaderRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialPixelShader {
        dbg!(*self);

        let name = self.name;
        let name = name.xfile_into(&mut xfile);
        dbg!(&name);

        MaterialPixelShader {
            name,
            prog: self.prog.xfile_into(&mut xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialPixelShaderProgramRaw<'a> {
    ps: Ptr32<'a, ()>,
    load_def: GfxPixelShaderLoadDefRaw<'a>,
}

assert_size!(MaterialPixelShaderProgramRaw, 12);

#[derive(Clone, Debug)]
struct MaterialPixelShaderProgram {
    ps: Option<*mut ()>,
    load_def: GfxPixelShaderLoadDef,
}

impl<'a> XFileInto<MaterialPixelShaderProgram> for MaterialPixelShaderProgramRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialPixelShaderProgram {
        dbg!(*self);

        MaterialPixelShaderProgram {
            ps: None,
            load_def: self.load_def.xfile_into(&mut xfile),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct GfxPixelShaderLoadDefRaw<'a> {
    program: FatPointerCountLastU16<'a, u8>,
}

assert_size!(GfxPixelShaderLoadDefRaw, 8);

#[derive(Clone, Debug)]
struct GfxPixelShaderLoadDef {
    program: Vec<u8>,
}

impl<'a> XFileInto<GfxPixelShaderLoadDef> for GfxPixelShaderLoadDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> GfxPixelShaderLoadDef {
        dbg!(*self);
        let pos = xfile.stream_position().unwrap();
        dbg!(pos);

        let program = self.program.to_vec(xfile);

        GfxPixelShaderLoadDef { program }
    }
}

#[derive(Copy, Clone, Debug)]
enum MaterialArgumentDef {
    LiteralConst([f32; 4]),
    CodeConst(MaterialArgumentCodeConst),
    CodeSampler(u32),
    NameHash(u32),
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C, packed)]
struct MaterialShaderArgumentRaw {
    arg_type: u16,
    dest: u16,
    u: u32,
}

assert_size!(MaterialShaderArgumentRaw, 8);

#[derive(Copy, Clone, Debug)]
struct MaterialShaderArgument {
    arg_type: MtlArg,
    dest: u16,
    u: MaterialArgumentDef,
}

impl XFileInto<MaterialShaderArgument> for MaterialShaderArgumentRaw {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> MaterialShaderArgument {
        let pos = xfile.stream_position().unwrap();
        dbg!(pos);

        dbg!(*self);

        assert!(self.arg_type <= 7 && self.arg_type != 6);

        let u = match self.arg_type {
            MTL_ARG_LITERAL_PIXEL_CONST | MTL_ARG_LITERAL_VERTEX_CONST => {
                MaterialArgumentDef::LiteralConst(load_from_xfile(xfile))
            }
            MTL_ARG_CODE_PIXEL_CONST | MTL_ARG_CODE_VERTEX_CONST => {
                MaterialArgumentDef::CodeConst(unsafe { transmute(self.u) })
            }
            MTL_ARG_CODE_PIXEL_SAMPLER | MTL_ARG_MATERIAL_PIXEL_SAMPLER => {
                MaterialArgumentDef::CodeSampler(self.u)
            }
            _ => unimplemented!(),
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
struct MaterialArgumentCodeConst {
    index: u16,
    first_row: u8,
    row_count: u8,
}

assert_size!(MaterialArgumentCodeConst, 4);

const MTL_ARG_MATERIAL_VERTEX_CONST: u16 = 0;
const MTL_ARG_LITERAL_VERTEX_CONST: u16 = 1;
const MTL_ARG_MATERIAL_PIXEL_SAMPLER: u16 = 2;
const MTL_ARG_CODE_VERTEX_CONST: u16 = 3;
const MTL_ARG_CODE_PIXEL_SAMPLER: u16 = 4;
const MTL_ARG_CODE_PIXEL_CONST: u16 = 5;
const MTL_ARG_LITERAL_PIXEL_CONST: u16 = 7;

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(u16)]
enum MtlArg {
    MATERIAL_VERTEX_CONST = 0,
    LITERAL_VERTEX_CONST = 1,
    MATERIAL_PIXEL_SAMPLER = 2,
    CODE_VERTEX_CONST = 3,
    CODE_PIXEL_SAMPLER = 4,
    CODE_PIXEL_CONST = 5,
    LITERAL_PIXEL_CONST = 7,
}

fn file_read_string(mut xfile: impl Read + Seek) -> String {
    let mut string_buf = Vec::new();
    let mut c_buf = [0u8; 1];

    loop {
        xfile.read_exact(&mut c_buf).unwrap();
        let c = c_buf[0];
        string_buf.push(c);
        if c == b'\0' {
            break;
        }
    }

    dbg!(xfile.stream_position().unwrap());
    CString::from_vec_with_nul(string_buf)
        .unwrap()
        .to_string_lossy()
        .to_string()
}

fn main() {
    let filename = std::env::args_os()
        .nth(1)
        .unwrap_or(OsString::from_str("cuba.ff").unwrap());
    let cached_filename = Path::new(&filename).with_extension("cache");

    let mut file = if !Path::new(&filename).with_extension("cache").exists() {
        decompress_xfile(filename)
    } else {
        println!("Found inflated cache file, reading...");

        let mut file = std::fs::File::open(cached_filename).unwrap();
        let mut bytes = Vec::new();
        let bytes_read = file.read_to_end(&mut bytes).unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        assert!(bytes_read as u64 == file.metadata().unwrap().len());

        println!("Cache read, size: {} bytes", bytes_read);

        BufReader::with_capacity(0x8000, file)
    };

    let _xfile = bincode::deserialize_from::<_, XFile>(&mut file).unwrap();

    dbg!(file.stream_position().unwrap());

    let mut xasset_list_buf = [0u8; size_of::<XAssetList>()];
    file.read_exact(&mut xasset_list_buf).unwrap();
    let xasset_list = bincode::deserialize::<XAssetList>(&xasset_list_buf).unwrap();

    dbg!(file.stream_position().unwrap());
    println!("fastfile contains {} assets.", { let c = xasset_list.asset_count; c });

    let mut string_offsets = Vec::new();

    for _ in 0..xasset_list.string_count {
        string_offsets.push(bincode::deserialize_from::<_, u32>(&mut file).unwrap());
    }

    let mut strings = Vec::new();

    dbg!(file.stream_position().unwrap());

    for string_offset in string_offsets {
        if string_offset == 0 {
            continue;
        }

        if string_offset == 0xFFFFFFFF {
            strings.push(file_read_string(&mut file));
        } else {
            panic!("offsets unimplemented!");
        }
    }

    dbg!(strings);

    let mut assets = Vec::new();

    dbg!(file.stream_position().unwrap());

    for _ in 0..xasset_list.asset_count {
        assets.push(bincode::deserialize_from::<_, XAsset>(&mut file).unwrap());
    }

    dbg!(file.stream_position().unwrap());

    for asset in assets {
        assert!(asset.asset_type == 7);

        dbg!(asset);

        let p = Ptr32::<MaterialTechniqueSetRaw>(asset.asset_data.0, PhantomData::default());

        dbg!(p);

        let a = p.xfile_into(&mut file);
        dbg!(a);
    }
}
