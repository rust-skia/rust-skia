use crate::prelude::*;
use crate::{image_filters, ImageFilter};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn compose(
        outer: impl AsOwned<ImageFilter>,
        inner: impl AsOwned<ImageFilter>,
    ) -> Option<Self> {
        image_filters::compose(outer, inner)
    }
}

#[deprecated(since = "0.19.0", note = "use image_filters::compose")]
pub fn new(
    outer: impl AsOwned<ImageFilter>,
    inner: impl AsOwned<ImageFilter>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkComposeImageFilter_Make(outer.as_owned().into_ptr(), inner.as_owned().into_ptr())
    })
}
