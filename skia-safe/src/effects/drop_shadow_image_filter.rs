use crate::prelude::*;
use crate::{scalar, Color, ImageFilter, ImageFilterCropRect};
use skia_bindings::{C_SkDropShadowImageFilter_Make, SkDropShadowImageFilter_ShadowMode};

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
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        (dx, dy): (scalar, scalar),
        (sigma_x, sigma_y): (scalar, scalar),
        color: Color,
        shadow_mode: DropShadowImageFilterShadowMode,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkDropShadowImageFilter_Make(
                dx,
                dy,
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
