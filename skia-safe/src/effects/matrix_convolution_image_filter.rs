use crate::prelude::*;
use crate::{scalar, IPoint, ISize, ImageFilter, ImageFilterCropRect};
use skia_bindings::{
    C_SkMatrixConvolutionImageFilter_Make, SkImageFilter, SkMatrixConvolutionImageFilter_TileMode,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum MatrixConvolutionImageFilterTileMode {
    Clamp = SkMatrixConvolutionImageFilter_TileMode::kClamp_TileMode as _,
    Repeat = SkMatrixConvolutionImageFilter_TileMode::kRepeat_TileMode as _,
    ClampToBlack = SkMatrixConvolutionImageFilter_TileMode::kClampToBlack_TileMode as _,
}

impl NativeTransmutable<SkMatrixConvolutionImageFilter_TileMode>
    for MatrixConvolutionImageFilterTileMode
{
}
#[test]
fn test_tile_mode_layout() {
    MatrixConvolutionImageFilterTileMode::test_layout();
}

pub enum MatrixConvolutionImageFilter {}

impl MatrixConvolutionImageFilter {
    #[allow(clippy::new_ret_no_self)]
    #[allow(clippy::too_many_arguments)]
    pub fn new<'a, IS: Into<ISize>, IP: Into<IPoint>, CR: Into<Option<&'a ImageFilterCropRect>>>(
        kernel_size: IS,
        kernel: &[scalar],
        gain: scalar,
        bias: scalar,
        kernel_offset: IP,
        tile_mode: MatrixConvolutionImageFilterTileMode,
        convolve_alpha: bool,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        let kernel_size = kernel_size.into();
        assert_eq!(
            (kernel_size.width * kernel_size.height) as usize,
            kernel.len()
        );
        ImageFilter::from_ptr(unsafe {
            C_SkMatrixConvolutionImageFilter_Make(
                kernel_size.native(),
                kernel.as_ptr(),
                gain,
                bias,
                kernel_offset.into().native(),
                tile_mode.into_native(),
                convolve_alpha,
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    #[allow(clippy::too_many_arguments)]
    pub fn matrix_convolution<
        'a,
        IS: Into<ISize>,
        IP: Into<IPoint>,
        CR: Into<Option<&'a ImageFilterCropRect>>,
    >(
        &self,
        crop_rect: CR,
        kernel_size: IS,
        kernel: &[scalar],
        gain: scalar,
        bias: scalar,
        kernel_offset: IP,
        tile_mode: MatrixConvolutionImageFilterTileMode,
        convolve_alpha: bool,
    ) -> Option<Self> {
        MatrixConvolutionImageFilter::new(
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
