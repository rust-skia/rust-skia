use crate::prelude::NativeTransmutable;
use crate::{ISize, Matrix};
use skia_bindings as sb;
use skia_bindings::SkEncodedOrigin;

// Even though possible, we are not using the rewritten SkEncodedOrigin enum,
// because of the to_matrix() implementation below, which needs passed an ISize and so
// can not be implemented in the skia-bindings crate.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EncodedOrigin {
    TopLeft = SkEncodedOrigin::TopLeft as _,
    TopRight = SkEncodedOrigin::TopRight as _,
    BottomRight = SkEncodedOrigin::BottomRight as _,
    BottomLeft = SkEncodedOrigin::BottomLeft as _,
    LeftTop = SkEncodedOrigin::LeftTop as _,
    RightTop = SkEncodedOrigin::RightTop as _,
    RightBottom = SkEncodedOrigin::RightBottom as _,
    LeftBottom = SkEncodedOrigin::LeftBottom as _,
}

impl NativeTransmutable<SkEncodedOrigin> for EncodedOrigin {}

#[test]
fn test_encoded_origin_layout() {
    EncodedOrigin::test_layout();
}

impl Default for EncodedOrigin {
    fn default() -> Self {
        EncodedOrigin::TopLeft
    }
}

impl EncodedOrigin {
    pub const LAST: Self = EncodedOrigin::LeftBottom;
    pub const DEFAULT: Self = EncodedOrigin::TopLeft;

    pub fn to_matrix(self, size: impl Into<ISize>) -> Matrix {
        let size = size.into();
        let mut m = Matrix::default();
        unsafe {
            sb::C_SkEncodedOriginToMatrix(
                self.into_native(),
                size.width,
                size.height,
                m.native_mut(),
            )
        };
        m
    }
}
