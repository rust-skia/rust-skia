use crate::prelude::*;
use crate::{image_filter::CropRect, image_filters, scalar, IRect, ImageFilter};
use skia_bindings as sb;
use skia_bindings::{SkBlurImageFilter_TileMode, SkImageFilter};

impl RCHandle<SkImageFilter> {
    pub fn blur<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        sigma: (scalar, scalar),
        tile_mode: impl Into<Option<crate::TileMode>>,
    ) -> Option<Self> {
        image_filters::blur(sigma, tile_mode, self, crop_rect)
    }
}

#[deprecated(since = "0.19.0", note = "use skia_safe::TileMode")]
pub use sb::SkBlurImageFilter_TileMode as TileMode;

#[test]
fn test_tile_mode_naming() {
    let _ = TileMode::Clamp;
}

#[allow(deprecated)]
impl NativeTransmutable<SkBlurImageFilter_TileMode> for TileMode {}
#[allow(deprecated)]
#[test]
fn test_tile_mode_layout() {
    TileMode::test_layout();
}

#[allow(deprecated)]
#[deprecated(since = "0.19.0", note = "use image_filters::blur")]
pub fn new<'a>(
    (sigma_x, sigma_y): (scalar, scalar),
    input: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
    tile_mode: impl Into<Option<TileMode>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkBlurImageFilter_Make(
            sigma_x,
            sigma_y,
            input.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
            tile_mode
                .into()
                .unwrap_or(TileMode::ClampToBlack)
                .into_native(),
        )
    })
}
