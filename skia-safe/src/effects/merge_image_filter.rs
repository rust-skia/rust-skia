use crate::prelude::*;
use crate::{image_filter::CropRect, ImageFilter};
use skia_bindings::{C_SkMergeImageFilter_Make, SkImageFilter};
use std::convert::TryInto;

impl RCHandle<SkImageFilter> {
    pub fn merge<'a>(
        filters: &[&Self],
        crop_rect: impl Into<Option<&'a CropRect>>,
    ) -> Option<Self> {
        new(filters, crop_rect)
    }
}

#[allow(clippy::new_ret_no_self)]
pub fn new<'a>(
    filters: &[&ImageFilter],
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    let shared_filters: Vec<*const SkImageFilter> = filters
        .iter()
        .map(|f| f.shared_native() as *const _)
        .collect();
    ImageFilter::from_ptr(unsafe {
        C_SkMergeImageFilter_Make(
            shared_filters.as_ptr(),
            shared_filters.len().try_into().unwrap(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
