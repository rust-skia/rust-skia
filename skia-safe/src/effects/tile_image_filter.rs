use crate::prelude::*;
use crate::{image_filters, Rect};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn tile(self, src: impl AsRef<Rect>, dst: impl AsRef<Rect>) -> Option<Self> {
        image_filters::tile(src, dst, self)
    }
}
