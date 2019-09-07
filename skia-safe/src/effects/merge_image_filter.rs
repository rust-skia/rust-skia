use crate::prelude::*;
use crate::{image_filter::CropRect, ImageFilter};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;
use std::convert::TryInto;

impl RCHandle<SkImageFilter> {
    pub fn merge<'a>(
        filters: impl IntoIterator<Item = Self>,
        crop_rect: impl Into<Option<&'a CropRect>>,
    ) -> Option<Self> {
        new(filters, crop_rect)
    }
}

#[deprecated(since = "m78", note = "use color_filters::merge")]
#[allow(clippy::new_ret_no_self)]
pub fn new<'a>(
    filters: impl IntoIterator<Item = ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    let filter_ptrs: Vec<*mut SkImageFilter> = filters.into_iter().map(|f| f.into_ptr()).collect();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkMergeImageFilter_Make(
            filter_ptrs.as_ptr(),
            filter_ptrs.len().try_into().unwrap(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
