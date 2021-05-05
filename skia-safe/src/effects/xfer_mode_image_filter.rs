use crate::prelude::*;
use crate::{image_filters, BlendMode, IRect, ImageFilter};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    #[allow(deprecated)]
    pub fn xfer_mode<'a>(
        blend_mode: BlendMode,
        background: impl Into<Option<ImageFilter>>,
        foreground: impl Into<Option<ImageFilter>>,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<Self> {
        image_filters::xfermode(
            blend_mode,
            background,
            foreground,
            crop_rect.into().map(|r| r.into()),
        )
    }
}
