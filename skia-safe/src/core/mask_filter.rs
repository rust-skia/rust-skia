use crate::prelude::*;
use crate::{scalar, BlurStyle, CoverageMode, Matrix, NativeFlattenable};
use skia_bindings::{
    C_SkMaskFilter_Combine, C_SkMaskFilter_Compose, C_SkMaskFilter_Deserialize,
    C_SkMaskFilter_MakeBlur, C_SkMaskFilter_makeWithMatrix, SkFlattenable, SkMaskFilter,
    SkRefCntBase,
};

pub type MaskFilter = RCHandle<SkMaskFilter>;

impl NativeRefCountedBase for SkMaskFilter {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl NativeFlattenable for SkMaskFilter {
    fn native_flattenable(&self) -> &SkFlattenable {
        &self._base
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { C_SkMaskFilter_Deserialize(data.as_ptr() as _, data.len()) }
    }
}

impl RCHandle<SkMaskFilter> {
    pub fn blur(style: BlurStyle, sigma: scalar, respect_ctm: impl Into<Option<bool>>) -> Self {
        Self::from_ptr(unsafe {
            C_SkMaskFilter_MakeBlur(
                style.into_native(),
                sigma,
                respect_ctm.into().unwrap_or(true),
            )
        })
        .unwrap()
    }

    pub fn compose(outer: Self, inner: Self) -> Option<Self> {
        Self::from_ptr(unsafe { C_SkMaskFilter_Compose(outer.into_ptr(), inner.into_ptr()) })
    }

    pub fn combine(filter_a: Self, filter_b: Self, mode: CoverageMode) -> Option<Self> {
        Self::from_ptr(unsafe {
            C_SkMaskFilter_Combine(filter_a.into_ptr(), filter_b.into_ptr(), mode.into_native())
        })
    }

    pub fn with_matrix(&self, matrix: &Matrix) -> Self {
        Self::from_ptr(unsafe { C_SkMaskFilter_makeWithMatrix(self.native(), matrix.native()) })
            .unwrap()
    }
}
