use crate::prelude::*;
use crate::{scalar, ImageFilter, ImageFilterCropRect};
use skia_bindings::{C_SkBlurImageFilter_Make, SkBlurImageFilter_TileMode, SkImageFilter};

pub enum BlurImageFilter {}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum BlurImageFilterTileMode {
    Clamp = SkBlurImageFilter_TileMode::kClamp_TileMode as _,
    Repeat = SkBlurImageFilter_TileMode::kRepeat_TileMode as _,
    ClampToBlack = SkBlurImageFilter_TileMode::kClampToBlack_TileMode as _,
}

impl NativeTransmutable<SkBlurImageFilter_TileMode> for BlurImageFilterTileMode {}
#[test]
fn test_tile_mode_layout() {
    BlurImageFilterTileMode::test_layout();
}

impl BlurImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<
        'a,
        CR: Into<Option<&'a ImageFilterCropRect>>,
        TM: Into<Option<BlurImageFilterTileMode>>,
    >(
        (sigma_x, sigma_y): (scalar, scalar),
        input: &ImageFilter,
        crop_rect: CR,
        tile_mode: TM,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkBlurImageFilter_Make(
                sigma_x,
                sigma_y,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
                tile_mode
                    .into()
                    .unwrap_or(BlurImageFilterTileMode::ClampToBlack)
                    .into_native(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn blur<
        'a,
        CR: Into<Option<&'a ImageFilterCropRect>>,
        TM: Into<Option<BlurImageFilterTileMode>>,
    >(
        &self,
        crop_rect: CR,
        sigma: (scalar, scalar),
        tile_mode: TM,
    ) -> Option<Self> {
        BlurImageFilter::new(sigma, self, crop_rect, tile_mode)
    }
}
