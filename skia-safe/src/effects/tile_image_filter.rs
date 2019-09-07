use crate::prelude::*;
use crate::{ImageFilter, Rect};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn tile(self, src: impl AsRef<Rect>, dst: impl AsRef<Rect>) -> Option<Self> {
        new(src, dst, self)
    }
}

#[deprecated(since = "m78", note = "use color_filters::tile")]
pub fn new(
    src: impl AsRef<Rect>,
    dst: impl AsRef<Rect>,
    input: ImageFilter,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkTileImageFilter_Make(
            src.as_ref().native(),
            dst.as_ref().native(),
            input.into_ptr(),
        )
    })
}
