use crate::prelude::*;
use crate::{
    BlurStyle,
    scalar,
    Matrix,
    CoverageMode
};
use skia_bindings::{
    C_SkMaskFilter_Combine,
    C_SkMaskFilter_Compose,
    C_SkMaskFilter_MakeBlur,
    SkRefCntBase,
    SkMaskFilter,
    C_SkMaskFilter_makeWithMatrix
};

pub type MaskFilter = RCHandle<SkMaskFilter>;

impl NativeRefCountedBase for SkMaskFilter {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl RCHandle<SkMaskFilter> {

    pub fn blur(style: BlurStyle, sigma: scalar, respect_ctm: impl Into<Option<bool>>) -> Self {
        Self::from_ptr(unsafe {
            C_SkMaskFilter_MakeBlur(style.into_native(), sigma, respect_ctm.into().unwrap_or(true))
        }).unwrap()
    }

    pub fn compose(outer: &Self, inner: &Self) -> Option<Self> {
        Self::from_ptr(unsafe {
            C_SkMaskFilter_Compose(outer.shared_native(), inner.shared_native())
        })
    }

    pub fn combine(filter_a: &Self, filter_b: &Self, mode: CoverageMode) -> Option<Self> {
        Self::from_ptr(unsafe {
            C_SkMaskFilter_Combine(filter_a.shared_native(), filter_b.shared_native(), mode.into_native())
        })
    }

    pub fn with_matrix(&self, matrix: &Matrix) -> Self {
        Self::from_ptr(unsafe {
            C_SkMaskFilter_makeWithMatrix(self.native(), matrix.native())
        }).unwrap()
    }

    // TODO: implement Flattenable
}
