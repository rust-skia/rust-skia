use crate::prelude::*;
use crate::{image_filters, IRect};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn merge<'a>(
        filters: impl IntoIterator<Item = Option<Self>>,
        crop_rect: impl Into<Option<&'a IRect>>,
    ) -> Option<Self> {
        image_filters::merge(filters, crop_rect.into().map(|r| r.into()))
    }
}
