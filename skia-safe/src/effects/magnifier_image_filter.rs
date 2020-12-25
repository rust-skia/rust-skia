use crate::prelude::*;
use crate::{image_filters, scalar, IRect, Rect};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn magnifier<'a>(
        self,
        crop_rect: impl Into<Option<&'a IRect>>,
        src_rect: impl AsRef<Rect>,
        inset: scalar,
    ) -> Option<Self> {
        image_filters::magnifier(src_rect, inset, self, crop_rect.into().map(|r| r.into()))
    }
}
