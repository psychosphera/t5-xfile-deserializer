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
#[cfg(not(feature = "cgmath"))]
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
#[cfg(not(feature = "cgmath"))]
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
#[cfg(not(feature = "cgmath"))]
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
#[cfg(not(feature = "cgmath"))]
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
#[cfg(not(feature = "cgmath"))]
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
#[cfg(not(feature = "cgmath"))]
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct Mat4(pub [Vec3; 4]);
