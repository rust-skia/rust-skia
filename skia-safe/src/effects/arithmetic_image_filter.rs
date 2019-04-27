use crate::prelude::*;

use skia_bindings::C_SkArithmeticImageFilter_Make;
use crate::{ImageFilter, ImageFilterCropRect};

pub enum ArithmeticImageFilter {}

impl ArithmeticImageFilter {

    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(k1: f32, k2: f32, k3: f32, k4: f32, enforce_pm_color: bool, background: &ImageFilter, foreground: &ImageFilter, crop_rect: CR) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkArithmeticImageFilter_Make(k1, k2, k3, k4, enforce_pm_color, background.shared_native(), foreground.shared_native(), crop_rect.into().native_ptr_or_null())
        })




    }
}