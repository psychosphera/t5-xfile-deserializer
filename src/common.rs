#[cfg(feature = "cgmath")]
pub type Vec2 = cgmath::Vector2<f32>;
#[cfg(not(feature = "cgmath"))]
pub type Vec2 = [f32; 2];

#[cfg(feature = "cgmath")]
pub type Vec3 = cgmath::Vector3<f32>;
#[cfg(not(feature = "cgmath"))]
pub type Vec3 = [f32; 3];

#[cfg(feature = "cgmath")]
pub type Vec4 = cgmath::Vector4<f32>;
#[cfg(not(feature = "cgmath"))]
pub type Vec4 = [f32; 4];

#[cfg(feature = "cgmath")]
pub type Mat2 = cgmath::Matrix2<f32>;
#[cfg(not(feature = "cgmath"))]
pub type Mat2 = [Vec2; 2];

#[cfg(feature = "cgmath")]
pub type Mat3 = cgmath::Matrix3<f32>;
#[cfg(not(feature = "cgmath"))]
pub type Mat3 = [Vec3; 3];

#[cfg(feature = "cgmath")]
pub type Mat4 = cgmath::Matrix4<f32>;
#[cfg(not(feature = "cgmath"))]
pub type Mat4 = [Vec4; 4];
