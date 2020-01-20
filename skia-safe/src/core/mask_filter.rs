use crate::prelude::*;
use crate::{scalar, BlurStyle, CoverageMode, Matrix, NativeFlattenable};
use skia_bindings as sb;
use skia_bindings::{SkFlattenable, SkMaskFilter, SkRefCntBase};

pub type MaskFilter = RCHandle<SkMaskFilter>;

impl NativeBase<SkRefCntBase> for SkMaskFilter {}
impl NativeBase<SkFlattenable> for SkMaskFilter {}

impl NativeRefCountedBase for SkMaskFilter {
    type Base = SkRefCntBase;
}

impl NativeFlattenable for SkMaskFilter {
    fn native_flattenable(&self) -> &SkFlattenable {
        self.base()
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { sb::C_SkMaskFilter_Deserialize(data.as_ptr() as _, data.len()) }
    }
}

impl RCHandle<SkMaskFilter> {
    pub fn blur(style: BlurStyle, sigma: scalar, respect_ctm: impl Into<Option<bool>>) -> Self {
        Self::from_ptr(unsafe {
            sb::C_SkMaskFilter_MakeBlur(style, sigma, respect_ctm.into().unwrap_or(true))
        })
        .unwrap()
    }

    pub fn compose(outer: Self, inner: Self) -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_SkMaskFilter_Compose(outer.into_ptr(), inner.into_ptr()) })
    }

    pub fn combine(filter_a: Self, filter_b: Self, mode: CoverageMode) -> Option<Self> {
        Self::from_ptr(unsafe {
            sb::C_SkMaskFilter_Combine(filter_a.into_ptr(), filter_b.into_ptr(), mode)
        })
    }

    pub fn with_matrix(&self, matrix: &Matrix) -> Self {
        Self::from_ptr(unsafe { sb::C_SkMaskFilter_makeWithMatrix(self.native(), matrix.native()) })
            .unwrap()
    }
}
