use crate::prelude::*;
use crate::{image_filter::CropRect, image_filters, scalar, Color, IRect, ImageFilter, Vector};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn drop_shadow<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        delta: impl Into<Vector>,
        sigma: (scalar, scalar),
        color: impl Into<Color>,
        shadow_mode: ShadowMode,
    ) -> Option<Self> {
        match shadow_mode {
            ShadowMode::DrawShadowAndForeground => {
                image_filters::drop_shadow(delta, sigma, color, self, crop_rect)
            }
            ShadowMode::DrawShadowOnly => {
                image_filters::drop_shadow_only(delta, sigma, color, self, crop_rect)
            }
        }
    }
}

pub use skia_bindings::SkDropShadowImageFilter_ShadowMode as ShadowMode;
#[test]
fn test_shadow_mode_naming() {
    let _ = ShadowMode::DrawShadowOnly;
}

#[deprecated(
    since = "0.19.0",
    note = "use color_filters::drop_shadow & color_filters::drop_shadow_only"
)]
pub fn new<'a>(
    delta: impl Into<Vector>,
    (sigma_x, sigma_y): (scalar, scalar),
    color: impl Into<Color>,
    shadow_mode: ShadowMode,
    input: impl Into<ImageFilter>,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    let delta = delta.into();
    let color = color.into();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkDropShadowImageFilter_Make(
            delta.x,
            delta.y,
            sigma_x,
            sigma_y,
            color.into_native(),
            shadow_mode,
            input.into().into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
