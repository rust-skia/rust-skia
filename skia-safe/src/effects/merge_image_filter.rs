use crate::prelude::*;
use crate::{ImageFilter, ImageFilterCropRect};
use skia_bindings::{C_SkMergeImageFilter_Make, SkImageFilter};
use std::convert::TryInto;

pub enum MergeImageFilter {}

impl MergeImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        filters: &[&ImageFilter],
        crop_rect: CR,
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
}
