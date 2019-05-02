use crate::prelude::*;
use crate::{scalar, Color, ImageFilter, ImageFilterCropRect, Vector};
use skia_bindings::{
    C_SkDropShadowImageFilter_Make, SkDropShadowImageFilter_ShadowMode, SkImageFilter,
};

pub enum DropShadowImageFilter {}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum DropShadowImageFilterShadowMode {
    DrawShadowAndForeground =
        SkDropShadowImageFilter_ShadowMode::kDrawShadowAndForeground_ShadowMode as _,
    DrawShadowOnly = SkDropShadowImageFilter_ShadowMode::kDrawShadowOnly_ShadowMode as _,
}

impl NativeTransmutable<SkDropShadowImageFilter_ShadowMode> for DropShadowImageFilterShadowMode {}
#[test]
fn test_shadow_mode_layout() {
    DropShadowImageFilterShadowMode::test_layout();
}

impl DropShadowImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, IV: Into<Vector>, IC: Into<Color>, CR: Into<Option<&'a ImageFilterCropRect>>>(
        delta: IV,
        (sigma_x, sigma_y): (scalar, scalar),
        color: IC,
        shadow_mode: DropShadowImageFilterShadowMode,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        let delta = delta.into();
        let color = color.into();
        ImageFilter::from_ptr(unsafe {
            C_SkDropShadowImageFilter_Make(
                delta.x,
                delta.y,
                sigma_x,
                sigma_y,
                color.into_native(),
                shadow_mode.into_native(),
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn drop_shadow<'a, IV: Into<Vector>, IC: Into<Color>, CR: Into<Option<&'a ImageFilterCropRect>>>(
        &self,
        crop_rect: CR,
        delta: IV,
        sigma: (scalar, scalar),
        color: IC,
        shadow_mode: DropShadowImageFilterShadowMode,
    ) -> Option<Self> {
        DropShadowImageFilter::new(delta, sigma, color, shadow_mode, self, crop_rect)
    }
}
