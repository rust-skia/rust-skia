use crate::prelude::*;
use crate::{image_filters, scalar, IRect};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn dilate<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        radii: (scalar, scalar),
    ) -> Option<Self> {
        image_filters::dilate(radii, self, crop_rect)
    }

    pub fn erode<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        radii: (scalar, scalar),
    ) -> Option<Self> {
        image_filters::erode(radii, self, crop_rect)
    }
}

pub mod dilate_image_filter {
    use crate::image_filter::CropRect;
    use crate::prelude::*;
    use crate::ImageFilter;
    use skia_bindings as sb;

    #[deprecated(since = "0.19.0", note = "use image_filters::dilate")]
    pub fn new<'a>(
        (radius_x, radius_y): (i32, i32),
        input: impl Into<ImageFilter>,
        crop_rect: impl Into<Option<&'a CropRect>>,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            sb::C_SkDilateImageFilter_Make(
                radius_x,
                radius_y,
                input.into().into_ptr(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

pub mod erode_image_filter {
    use crate::image_filter::CropRect;
    use crate::prelude::*;
    use crate::ImageFilter;
    use skia_bindings as sb;

    #[deprecated(since = "0.19.0", note = "use image_filters::erode")]
    pub fn new<'a>(
        (radius_x, radius_y): (i32, i32),
        input: impl Into<ImageFilter>,
        crop_rect: impl Into<Option<&'a CropRect>>,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            sb::C_SkErodeImageFilter_Make(
                radius_x,
                radius_y,
                input.into().into_ptr(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}
