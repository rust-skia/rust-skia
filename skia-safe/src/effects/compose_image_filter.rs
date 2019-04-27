use crate::prelude::*;
use crate::ImageFilter;
use skia_bindings::{C_SkComposeImageFilter_Make, SkImageFilter};

pub enum ComposeImageFilter {}

impl ComposeImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(outer: &ImageFilter, inner: &ImageFilter) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkComposeImageFilter_Make(outer.shared_native(), inner.shared_native())
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn compose(outer: &ImageFilter, inner: &ImageFilter) -> Option<Self> {
        ComposeImageFilter::new(outer, inner)
    }
}
