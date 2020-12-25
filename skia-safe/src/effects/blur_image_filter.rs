use crate::prelude::*;
use crate::{image_filters, scalar, IRect};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn blur<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        sigma: (scalar, scalar),
        tile_mode: impl Into<Option<crate::TileMode>>,
    ) -> Option<Self> {
        image_filters::blur(sigma, tile_mode, self, crop_rect.into().map(|r| r.into()))
    }
}
