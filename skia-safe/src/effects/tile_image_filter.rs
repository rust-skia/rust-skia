use crate::prelude::*;
use crate::{ImageFilter, Rect};
use skia_bindings::{C_SkTileImageFilter_Make, SkImageFilter};

impl RCHandle<SkImageFilter> {
    pub fn tile(&self, src: impl AsRef<Rect>, dst: impl AsRef<Rect>) -> Option<Self> {
        new(src, dst, self)
    }
}

pub fn new(
    src: impl AsRef<Rect>,
    dst: impl AsRef<Rect>,
    input: &ImageFilter,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        C_SkTileImageFilter_Make(
            src.as_ref().native(),
            dst.as_ref().native(),
            input.shared_native(),
        )
    })
}
