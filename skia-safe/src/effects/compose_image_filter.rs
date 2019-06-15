use crate::prelude::*;
use crate::ImageFilter;
use skia_bindings::{C_SkComposeImageFilter_Make, SkImageFilter};

impl RCHandle<SkImageFilter> {
    pub fn compose(outer: &ImageFilter, inner: &ImageFilter) -> Option<Self> {
        new(outer, inner)
    }
}

pub fn new(outer: &ImageFilter, inner: &ImageFilter) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        C_SkComposeImageFilter_Make(outer.shared_native(), inner.shared_native())
    })
}
