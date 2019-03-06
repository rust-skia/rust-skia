use rust_skia::SkVector4;
use crate::prelude::NativeTransmutable;

// TODO: complete the implementation

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl NativeTransmutable<SkVector4> for Vector4 {}

#[test]
fn test_vector4_layout() {
    Vector4::test_layout()
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Vector4 { x, y, z, w }
    }
}

impl From<(f32, f32, f32)> for Vector4 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Vector4::from((x, y, z, 1.0))
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from(v4: [f32; 4]) -> Self {
        Vector4::from((v4[0], v4[1], v4[2], v4[3]))
    }
}

impl From<[f32; 3]> for Vector4 {
    fn from(v3: [f32; 3]) -> Self {
        Vector4::from((v3[0], v3[1], v3[2]))
    }
}

impl Default for Vector4 {
    fn default() -> Self {
        (0.0, 0.0, 0.0).into()
    }
}
