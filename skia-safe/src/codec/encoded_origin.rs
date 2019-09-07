use crate::prelude::NativeTransmutable;
use crate::{ISize, Matrix};
use skia_bindings as sb;
use skia_bindings::SkEncodedOrigin;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EncodedOrigin {
    TopLeft = SkEncodedOrigin::kTopLeft_SkEncodedOrigin as _,
    TopRight = SkEncodedOrigin::kTopRight_SkEncodedOrigin as _,
    BottomRight = SkEncodedOrigin::kBottomRight_SkEncodedOrigin as _,
    BottomLeft = SkEncodedOrigin::kBottomLeft_SkEncodedOrigin as _,
    LeftTop = SkEncodedOrigin::kLeftTop_SkEncodedOrigin as _,
    RightTop = SkEncodedOrigin::kRightTop_SkEncodedOrigin as _,
    RightBottom = SkEncodedOrigin::kRightBottom_SkEncodedOrigin as _,
    LeftBottom = SkEncodedOrigin::kLeftBottom_SkEncodedOrigin as _,
}

impl Default for EncodedOrigin {
    fn default() -> Self {
        EncodedOrigin::TopLeft
    }
}

impl NativeTransmutable<SkEncodedOrigin> for EncodedOrigin {}
#[test]
fn test_encoded_origin_layout() {
    EncodedOrigin::test_layout()
}

impl EncodedOrigin {
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
