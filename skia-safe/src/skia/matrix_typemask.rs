use skia_bindings::{
    SkMatrix_TypeMask_kIdentity_Mask,
    SkMatrix_TypeMask_kAffine_Mask,
    SkMatrix_TypeMask_kTranslate_Mask,
    SkMatrix_TypeMask_kScale_Mask,
    SkMatrix_TypeMask_kPerspective_Mask
};

bitflags! {
    pub struct MatrixTypeMask: u32 {
        const IDENTITY = SkMatrix_TypeMask_kIdentity_Mask as u32;
        const TRANSLATE = SkMatrix_TypeMask_kTranslate_Mask as u32;
        const SCALE = SkMatrix_TypeMask_kScale_Mask as u32;
        const AFFINE = SkMatrix_TypeMask_kAffine_Mask as u32;
        const PERSPECTIVE = SkMatrix_TypeMask_kPerspective_Mask as u32;
    }
}
