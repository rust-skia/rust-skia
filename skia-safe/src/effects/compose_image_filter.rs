use crate::prelude::*;
use crate::ImageFilter;
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn compose(outer: ImageFilter, inner: ImageFilter) -> Option<Self> {
        new(outer, inner)
    }
}

#[deprecated(since = "m78", note = "use image_filters::compose")]
pub fn new(outer: ImageFilter, inner: ImageFilter) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkComposeImageFilter_Make(outer.into_ptr(), inner.into_ptr())
    })
}
