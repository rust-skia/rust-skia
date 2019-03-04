use rust_skia::{
    SkMatrix_TypeMask
};

bitflags! {
    pub struct MatrixTypeMask: u32 {
        const Identity = SkMatrix_TypeMask::kIdentity_Mask as u32;
        const Translate = SkMatrix_TypeMask::kTranslate_Mask as u32;
        const Scale = SkMatrix_TypeMask::kScale_Mask as u32;
        const Affine = SkMatrix_TypeMask::kAffine_Mask as u32;
        const Perspective = SkMatrix_TypeMask::kPerspective_Mask as u32;
    }
}
