use skia_bindings::{
    SkMatrix_TypeMask
};

bitflags! {
    pub struct MatrixTypeMask: u32 {
        const IDENTITY = SkMatrix_TypeMask::kIdentity_Mask as u32;
        const TRANSLATE = SkMatrix_TypeMask::kTranslate_Mask as u32;
        const SCALE = SkMatrix_TypeMask::kScale_Mask as u32;
        const AFFINE = SkMatrix_TypeMask::kAffine_Mask as u32;
        const PERSPECTIVE = SkMatrix_TypeMask::kPerspective_Mask as u32;
    }
}
