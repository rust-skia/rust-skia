use crate::prelude::*;
use crate::{image_filters, ImageFilter, Rect};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn tile(self, src: impl AsRef<Rect>, dst: impl AsRef<Rect>) -> Option<Self> {
        image_filters::tile(src, dst, self)
    }
}

#[deprecated(since = "0.19.0", note = "use image_filters::tile")]
pub fn new(
    src: impl AsRef<Rect>,
    dst: impl AsRef<Rect>,
    input: impl Into<ImageFilter>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkTileImageFilter_Make(
            src.as_ref().native(),
            dst.as_ref().native(),
            input.into().into_ptr(),
        )
    })
}
