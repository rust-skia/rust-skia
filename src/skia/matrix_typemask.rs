use rust_skia::*;

bitflags! {
    pub struct MatrixTypeMask: u32 {
        const Identity = SkMatrix_TypeMask::kIdentity_Mask as u32;
        const Translate = SkMatrix_TypeMask::kTranslate_Mask as u32;
        const Scale = SkMatrix_TypeMask::kScale_Mask as u32;
        const Affine = SkMatrix_TypeMask::kAffine_Mask as u32;
        const Perspective = SkMatrix_TypeMask::kPerspective_Mask as u32;
    }
}

impl From<SkMatrix_TypeMask> for MatrixTypeMask {
    fn from(mask: SkMatrix_TypeMask) -> MatrixTypeMask {
        MatrixTypeMask::from_bits(mask as u32).unwrap()
    }
}

impl From<SkMatrix44_TypeMask> for MatrixTypeMask {
    fn from(mask: SkMatrix44_TypeMask) -> MatrixTypeMask {
        MatrixTypeMask::from_bits(mask as u32).unwrap()
    }
}
