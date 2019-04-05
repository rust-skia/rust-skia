use skia_bindings::SkVector4;
use crate::prelude::NativeTransmutable;
use crate::core::scalar;

// TODO: complete the implementation

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector4 {
    x: scalar,
    y: scalar,
    z: scalar,
    w: scalar
}

impl NativeTransmutable<SkVector4> for Vector4 {}

#[test]
fn test_vector4_layout() {
    Vector4::test_layout()
}

impl Default for Vector4 {
    fn default() -> Self {
        (0.0, 0.0, 0.0).into()
    }
}

impl From<(scalar, scalar, scalar, scalar)> for Vector4 {
    fn from((x, y, z, w): (scalar, scalar, scalar, scalar)) -> Self {
        Vector4 { x, y, z, w }
    }
}

impl From<(scalar, scalar, scalar)> for Vector4 {
    fn from((x, y, z): (scalar, scalar, scalar)) -> Self {
        Vector4::from((x, y, z, 1.0))
    }
}

impl From<[scalar; 4]> for Vector4 {
    fn from(v4: [scalar; 4]) -> Self {
        Vector4::from((v4[0], v4[1], v4[2], v4[3]))
    }
}

impl From<[scalar; 3]> for Vector4 {
    fn from(v3: [scalar; 3]) -> Self {
        Vector4::from((v3[0], v3[1], v3[2]))
    }
}

