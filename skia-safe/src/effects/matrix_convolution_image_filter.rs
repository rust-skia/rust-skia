use crate::prelude::*;
use crate::{image_filter::CropRect, image_filters, scalar, IPoint, IRect, ISize, ImageFilter};
use skia_bindings as sb;
use skia_bindings::{SkImageFilter, SkMatrixConvolutionImageFilter_TileMode};

impl RCHandle<SkImageFilter> {
    #[allow(clippy::too_many_arguments)]
    pub fn matrix_convolution<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        kernel_size: impl Into<ISize>,
        kernel: &[scalar],
        gain: scalar,
        bias: scalar,
        kernel_offset: impl Into<IPoint>,
        tile_mode: crate::TileMode,
        convolve_alpha: bool,
    ) -> Option<Self> {
        image_filters::matrix_convolution(
            kernel_size,
            kernel,
            gain,
            bias,
            kernel_offset,
            tile_mode,
            convolve_alpha,
            self,
            crop_rect,
        )
    }
}

#[deprecated(since = "m78", note = "use skia_safe::TileMode")]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum TileMode {
    Clamp = SkMatrixConvolutionImageFilter_TileMode::kClamp_TileMode as _,
    Repeat = SkMatrixConvolutionImageFilter_TileMode::kRepeat_TileMode as _,
    ClampToBlack = SkMatrixConvolutionImageFilter_TileMode::kClampToBlack_TileMode as _,
}

#[allow(deprecated)]
impl NativeTransmutable<SkMatrixConvolutionImageFilter_TileMode> for TileMode {}
#[test]
fn test_tile_mode_layout() {
    TileMode::test_layout();
}

#[deprecated(since = "m78", note = "use color_filters::matrix_convolution")]
#[allow(deprecated)]
#[allow(clippy::too_many_arguments)]
pub fn new<'a>(
    kernel_size: impl Into<ISize>,
    kernel: &[scalar],
    gain: scalar,
    bias: scalar,
    kernel_offset: impl Into<IPoint>,
    tile_mode: TileMode,
    convolve_alpha: bool,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    let kernel_size = kernel_size.into();
    assert_eq!(
        (kernel_size.width * kernel_size.height) as usize,
        kernel.len()
    );
    ImageFilter::from_ptr(unsafe {
        sb::C_SkMatrixConvolutionImageFilter_Make(
            kernel_size.native(),
            kernel.as_ptr(),
            gain,
            bias,
            kernel_offset.into().native(),
            tile_mode.into_native(),
            convolve_alpha,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
