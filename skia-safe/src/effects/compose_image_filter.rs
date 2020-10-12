use crate::prelude::*;
use crate::{image_filters, ImageFilter};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn compose(outer: impl Into<ImageFilter>, inner: impl Into<ImageFilter>) -> Option<Self> {
        image_filters::compose(outer, inner)
    }
}

#[deprecated(since = "0.19.0", note = "use image_filters::compose")]
pub fn new(outer: impl Into<ImageFilter>, inner: impl Into<ImageFilter>) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkComposeImageFilter_Make(outer.into().into_ptr(), inner.into().into_ptr())
    })
}
