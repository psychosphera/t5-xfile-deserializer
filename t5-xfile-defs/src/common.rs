#![allow(clippy::clone_on_copy)]
#![allow(clippy::unit_arg)]

#[cfg(feature = "serde")]
use core::mem::transmute;

use crate::{assert_size, size_of};

#[cfg(feature = "serde")]
use serde::de::Visitor;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(all(feature = "serde", feature = "cgmath"))]
#[cfg(feature = "cgmath")]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Vec2(pub cgmath::Vector2<f32>);
#[cfg(feature = "cgmath")]
impl Default for Vec2 {
    fn default() -> Self {
        Self(cgmath::Vector2 { x: 0.0, y: 0.0 })
    }
}
#[cfg(feature = "cgmath")]
impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        Vec2(cgmath::Vector2 {
            x: value[0],
            y: value[1],
        })
    }
}

#[cfg(all(feature = "cgmath", feature = "serde"))]
impl Serialize for Vec2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        [self.0.x, self.0.y].serialize(serializer)
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        let arr = deserializer.deserialize_seq(ArrayVisitor::<f32, 2>::new())?;
        Ok(Self(cgmath::Vector2 {
            x: arr[0],
            y: arr[1],
        }))
    }
}

#[cfg(not(feature = "cgmath"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct Vec2(pub [f32; 2]);
#[cfg(not(feature = "cgmath"))]
impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        Vec2(value)
    }
}

#[cfg(feature = "cgmath")]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Vec3(cgmath::Vector3<f32>);
#[cfg(feature = "cgmath")]
impl Default for Vec3 {
    fn default() -> Self {
        Self(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    }
}
#[cfg(feature = "cgmath")]
impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Vec3(cgmath::Vector3 {
            x: value[0],
            y: value[1],
            z: value[2],
        })
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl Serialize for Vec3 {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        [self.0.x, self.0.y, self.0.z].serialize(serializer)
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        let arr = deserializer.deserialize_seq(ArrayVisitor::<f32, 3>::new())?;
        Ok(Self(cgmath::Vector3 {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }))
    }
}
#[cfg(not(feature = "cgmath"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct Vec3(pub [f32; 3]);
#[cfg(not(feature = "cgmath"))]
impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Vec3(value)
    }
}

#[cfg(feature = "cgmath")]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Vec4(cgmath::Vector4<f32>);
#[cfg(feature = "cgmath")]
impl Default for Vec4 {
    fn default() -> Self {
        Self(cgmath::Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        })
    }
}
#[cfg(feature = "cgmath")]
impl From<[f32; 4]> for Vec4 {
    fn from(value: [f32; 4]) -> Self {
        Vec4(cgmath::Vector4 {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
        })
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl Serialize for Vec4 {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        [self.0.x, self.0.y, self.0.z, self.0.w].serialize(serializer)
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl<'de> Deserialize<'de> for Vec4 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        let arr = deserializer.deserialize_seq(ArrayVisitor::<f32, 4>::new())?;
        Ok(Self(cgmath::Vector4 {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: arr[3],
        }))
    }
}
#[cfg(not(feature = "cgmath"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct Vec4(pub [f32; 4]);
#[cfg(not(feature = "cgmath"))]
impl From<[f32; 4]> for Vec4 {
    fn from(value: [f32; 4]) -> Self {
        Vec4(value)
    }
}

#[cfg(feature = "cgmath")]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Mat2(pub cgmath::Matrix2<f32>);
#[cfg(feature = "cgmath")]
impl Default for Mat2 {
    fn default() -> Self {
        Self(cgmath::Matrix2 {
            x: cgmath::Vector2::<f32> { x: 0.0, y: 0.0 },
            y: cgmath::Vector2::<f32> { x: 0.0, y: 0.0 },
        })
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl Serialize for Mat2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        [self.0.x.x, self.0.x.y, self.0.y.x, self.0.y.y].serialize(serializer)
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl<'de> Deserialize<'de> for Mat2 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        let arr = deserializer.deserialize_seq(ArrayVisitor::<f32, 4>::new())?;
        Ok(Self(cgmath::Matrix2 {
            x: cgmath::Vector2 {
                x: arr[0],
                y: arr[1],
            },
            y: cgmath::Vector2 {
                x: arr[2],
                y: arr[3],
            },
        }))
    }
}
#[cfg(not(feature = "cgmath"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct Mat2(pub [Vec2; 2]);

#[cfg(feature = "cgmath")]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Mat3(pub cgmath::Matrix3<f32>);
#[cfg(feature = "cgmath")]
impl Default for Mat3 {
    fn default() -> Self {
        Self(cgmath::Matrix3 {
            x: cgmath::Vector3::<f32> {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            y: cgmath::Vector3::<f32> {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            z: cgmath::Vector3::<f32> {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        })
    }
}
#[cfg(feature = "cgmath")]
impl From<[[f32; 3]; 3]> for Mat3 {
    fn from(value: [[f32; 3]; 3]) -> Self {
        Self(cgmath::Matrix3 {
            x: cgmath::Vector3::<f32> {
                x: value[0][0],
                y: value[0][1],
                z: value[0][2],
            },
            y: cgmath::Vector3::<f32> {
                x: value[1][0],
                y: value[1][1],
                z: value[1][2],
            },
            z: cgmath::Vector3::<f32> {
                x: value[2][0],
                y: value[2][1],
                z: value[2][2],
            },
        })
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl Serialize for Mat3 {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        [
            self.0.x.x, self.0.x.y, self.0.x.z, self.0.y.x, self.0.y.y, self.0.y.z, self.0.z.x,
            self.0.z.y, self.0.z.z,
        ]
        .serialize(serializer)
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl<'de> Deserialize<'de> for Mat3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        let arr = deserializer.deserialize_seq(ArrayVisitor::<f32, 9>::new())?;
        Ok(Self(cgmath::Matrix3 {
            x: cgmath::Vector3 {
                x: arr[0],
                y: arr[1],
                z: arr[2],
            },
            y: cgmath::Vector3 {
                x: arr[3],
                y: arr[4],
                z: arr[5],
            },
            z: cgmath::Vector3 {
                x: arr[6],
                y: arr[7],
                z: arr[8],
            },
        }))
    }
}
#[cfg(not(feature = "cgmath"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct Mat3(pub [Vec3; 3]);
#[cfg(not(feature = "cgmath"))]
impl From<[[f32; 3]; 3]> for Mat3 {
    fn from(value: [[f32; 3]; 3]) -> Self {
        Self([value[0].into(), value[1].into(), value[2].into()])
    }
}

#[cfg(feature = "cgmath")]
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Mat4(pub cgmath::Matrix4<f32>);
#[cfg(feature = "cgmath")]
impl Default for Mat4 {
    fn default() -> Self {
        Self(cgmath::Matrix4 {
            x: cgmath::Vector4::<f32> {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            y: cgmath::Vector4::<f32> {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            z: cgmath::Vector4::<f32> {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            w: cgmath::Vector4::<f32> {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
        })
    }
}
#[cfg(feature = "cgmath")]
impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(value: [[f32; 4]; 4]) -> Self {
        Self(cgmath::Matrix4 {
            x: value[0].into(),
            y: value[1].into(),
            z: value[2].into(),
            w: value[3].into(),
        })
    }
}

#[cfg(all(feature = "cgmath", feature = "serde"))]
impl Serialize for Mat4 {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        [
            self.0.x.x, self.0.x.y, self.0.x.z, self.0.x.w, self.0.y.x, self.0.y.y, self.0.y.z,
            self.0.y.w, self.0.z.x, self.0.z.y, self.0.z.z, self.0.z.w, self.0.w.x, self.0.w.y,
            self.0.w.z, self.0.w.w,
        ]
        .serialize(serializer)
    }
}
#[cfg(all(feature = "cgmath", feature = "serde"))]
impl<'de> Deserialize<'de> for Mat4 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        let arr = deserializer.deserialize_seq(ArrayVisitor::<f32, 16>::new())?;
        Ok(Self(cgmath::Matrix4 {
            x: cgmath::Vector4 {
                x: arr[0],
                y: arr[1],
                z: arr[2],
                w: arr[3],
            },
            y: cgmath::Vector4 {
                x: arr[4],
                y: arr[5],
                z: arr[7],
                w: arr[7],
            },
            z: cgmath::Vector4 {
                x: arr[8],
                y: arr[9],
                z: arr[10],
                w: arr[11],
            },
            w: cgmath::Vector4 {
                x: arr[12],
                y: arr[13],
                z: arr[14],
                w: arr[15],
            },
        }))
    }
}
#[cfg(not(feature = "cgmath"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct Mat4(pub [Vec4; 4]);
#[cfg(not(feature = "cgmath"))]
impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(value: [[f32; 4]; 4]) -> Self {
        Self([
            value[0].into(),
            value[1].into(),
            value[2].into(),
            value[3].into(),
        ])
    }
}

#[cfg(feature = "serde")]
struct D3D9Visitor {}

#[cfg(all(feature = "serde", feature = "d3d9"))]
#[allow(dead_code)]
impl D3D9Visitor {
    const LEN: usize = 8;
}

#[cfg(all(feature = "serde", not(feature = "d3d9")))]
#[allow(dead_code)]
impl D3D9Visitor {
    const LEN: usize = 0;
}

#[cfg(feature = "serde")]
assert_size!(D3D9VS, D3D9Visitor::LEN);
#[cfg(feature = "serde")]
assert_size!(D3D9PS, D3D9Visitor::LEN);
#[cfg(feature = "serde")]
assert_size!(D3D9Tex, D3D9Visitor::LEN);
#[cfg(feature = "serde")]
assert_size!(D3D9VolTex, D3D9Visitor::LEN);
#[cfg(feature = "serde")]
assert_size!(D3D9CubeTex, D3D9Visitor::LEN);
#[cfg(feature = "serde")]
assert_size!(D3D9VB, D3D9Visitor::LEN);
#[cfg(feature = "serde")]
assert_size!(D3D9IB, D3D9Visitor::LEN);

#[cfg(all(feature = "serde", feature = "d3d9"))]
impl<'de> Visitor<'de> for D3D9Visitor {
    type Value = [u8; Self::LEN];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("an array of length {}", Self::LEN))
    }

    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> core::result::Result<Self::Value, E> {
        Ok(v[..Self::LEN].try_into().unwrap())
    }
}

#[cfg(all(feature = "serde", not(feature = "d3d9")))]
impl<'de> Visitor<'de> for D3D9Visitor {
    type Value = ();

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("()")
    }

    fn visit_unit<E: serde::de::Error>(self) -> core::result::Result<Self::Value, E> {
        Ok(())
    }
}

#[cfg(feature = "d3d9")]
type D3D9VS = windows::Win32::Graphics::Direct3D9::IDirect3DVertexShader9;
#[cfg(not(feature = "d3d9"))]
type D3D9VS = ();

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GfxVertexShader(pub D3D9VS);
#[cfg(feature = "serde")]
impl Serialize for GfxVertexShader {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        unsafe { transmute::<_, [u8; size_of!(D3D9VS)]>(self.0.clone()) }.serialize(serializer)
    }
}
#[cfg(all(feature = "serde", feature = "d3d9"))]
impl<'de> Deserialize<'de> for GfxVertexShader {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(unsafe { transmute::<_, Self>(deserializer.deserialize_bytes(D3D9Visitor {})?) })
    }
}
#[cfg(all(feature = "serde", not(feature = "d3d9")))]
impl<'de> Deserialize<'de> for GfxVertexShader {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(Self(deserializer.deserialize_unit(D3D9Visitor {})?))
    }
}

#[cfg(feature = "d3d9")]
type D3D9PS = windows::Win32::Graphics::Direct3D9::IDirect3DPixelShader9;
#[cfg(not(feature = "d3d9"))]
type D3D9PS = ();

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GfxPixelShader(pub D3D9PS);
#[cfg(feature = "serde")]
impl Serialize for GfxPixelShader {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        unsafe { transmute::<_, [u8; size_of!(D3D9PS)]>(self.0.clone()) }.serialize(serializer)
    }
}
#[cfg(all(feature = "serde", feature = "d3d9"))]
impl<'de> Deserialize<'de> for GfxPixelShader {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(unsafe { transmute::<_, Self>(deserializer.deserialize_bytes(D3D9Visitor {})?) })
    }
}
#[cfg(all(feature = "serde", not(feature = "d3d9")))]
impl<'de> Deserialize<'de> for GfxPixelShader {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(Self(deserializer.deserialize_unit(D3D9Visitor {})?))
    }
}

#[cfg(feature = "d3d9")]
pub type IDirect3DBaseTexture9 = windows::Win32::Graphics::Direct3D9::IDirect3DBaseTexture9;
#[cfg(not(feature = "d3d9"))]
pub type IDirect3DBaseTexture9 = ();

#[cfg(feature = "d3d9")]
type D3D9Tex = windows::Win32::Graphics::Direct3D9::IDirect3DTexture9;
#[cfg(not(feature = "d3d9"))]
type D3D9Tex = ();

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GfxTexture(D3D9Tex);
#[cfg(feature = "serde")]
impl Serialize for GfxTexture {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        unsafe { transmute::<_, [u8; size_of!(D3D9Tex)]>(self.0.clone()) }.serialize(serializer)
    }
}
#[cfg(all(feature = "serde", feature = "d3d9"))]
impl<'de> Deserialize<'de> for GfxTexture {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(unsafe { transmute::<_, Self>(deserializer.deserialize_bytes(D3D9Visitor {})?) })
    }
}
#[cfg(all(feature = "serde", not(feature = "d3d9")))]
impl<'de> Deserialize<'de> for GfxTexture {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(Self(deserializer.deserialize_unit(D3D9Visitor {})?))
    }
}

#[cfg(feature = "d3d9")]
type D3D9VolTex = windows::Win32::Graphics::Direct3D9::IDirect3DVolumeTexture9;
#[cfg(not(feature = "d3d9"))]
type D3D9VolTex = ();

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GfxVolumeTexture(D3D9VolTex);
#[cfg(feature = "serde")]
impl Serialize for GfxVolumeTexture {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        unsafe { transmute::<_, [u8; size_of!(D3D9VolTex)]>(self.0.clone()) }.serialize(serializer)
    }
}
#[cfg(all(feature = "serde", feature = "d3d9"))]
impl<'de> Deserialize<'de> for GfxVolumeTexture {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(unsafe { transmute::<_, Self>(deserializer.deserialize_bytes(D3D9Visitor {})?) })
    }
}
#[cfg(all(feature = "serde", not(feature = "d3d9")))]
impl<'de> Deserialize<'de> for GfxVolumeTexture {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(Self(deserializer.deserialize_unit(D3D9Visitor {})?))
    }
}

#[cfg(feature = "d3d9")]
type D3D9CubeTex = windows::Win32::Graphics::Direct3D9::IDirect3DCubeTexture9;
#[cfg(not(feature = "d3d9"))]
#[allow(dead_code)]
type D3D9CubeTex = ();

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GfxCubeTexture(D3D9Tex);
#[cfg(feature = "serde")]
impl Serialize for GfxCubeTexture {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        unsafe { transmute::<_, [u8; size_of!(D3D9CubeTex)]>(self.0.clone()) }.serialize(serializer)
    }
}
#[cfg(all(feature = "serde", feature = "d3d9"))]
impl<'de> Deserialize<'de> for GfxCubeTexture {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(unsafe { transmute::<_, Self>(deserializer.deserialize_bytes(D3D9Visitor {})?) })
    }
}
#[cfg(all(feature = "serde", not(feature = "d3d9")))]
impl<'de> Deserialize<'de> for GfxCubeTexture {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(Self(deserializer.deserialize_unit(D3D9Visitor {})?))
    }
}

#[cfg(feature = "d3d9")]
type D3D9VB = windows::Win32::Graphics::Direct3D9::IDirect3DVertexBuffer9;
#[cfg(not(feature = "d3d9"))]
type D3D9VB = ();

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GfxVertexBuffer(pub D3D9VB);
#[cfg(feature = "serde")]
impl Serialize for GfxVertexBuffer {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        unsafe { transmute::<_, [u8; size_of!(D3D9VB)]>(self.0.clone()) }.serialize(serializer)
    }
}
#[cfg(all(feature = "serde", feature = "d3d9"))]
impl<'de> Deserialize<'de> for GfxVertexBuffer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(unsafe { transmute::<_, Self>(deserializer.deserialize_bytes(D3D9Visitor {})?) })
    }
}
#[cfg(all(feature = "serde", not(feature = "d3d9")))]
impl<'de> Deserialize<'de> for GfxVertexBuffer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(Self(deserializer.deserialize_unit(D3D9Visitor {})?))
    }
}

#[cfg(feature = "d3d9")]
type D3D9IB = windows::Win32::Graphics::Direct3D9::IDirect3DIndexBuffer9;
#[cfg(not(feature = "d3d9"))]
type D3D9IB = ();

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GfxIndexBuffer(D3D9IB);
#[cfg(feature = "serde")]
impl Serialize for GfxIndexBuffer {
    fn serialize<S: Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
        unsafe { transmute::<_, [u8; size_of!(D3D9IB)]>(self.0.clone()) }.serialize(serializer)
    }
}
#[cfg(all(feature = "serde", feature = "d3d9"))]
impl<'de> Deserialize<'de> for GfxIndexBuffer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(unsafe { transmute::<_, Self>(deserializer.deserialize_bytes(D3D9Visitor {})?) })
    }
}
#[cfg(all(feature = "serde", not(feature = "d3d9")))]
impl<'de> Deserialize<'de> for GfxIndexBuffer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        Ok(Self(deserializer.deserialize_unit(D3D9Visitor {})?))
    }
}
