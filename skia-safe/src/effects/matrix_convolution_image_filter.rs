use crate::prelude::*;
use crate::{image_filters, scalar, IPoint, IRect, ISize};
use skia_bindings::SkImageFilter;

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
            crop_rect.into().map(|r| r.into()),
        )
    }
}
