use crate::prelude::*;
use crate::{image_filters, ImageFilter};
use skia_bindings::SkImageFilter;

impl RCHandle<SkImageFilter> {
    pub fn compose(outer: impl Into<ImageFilter>, inner: impl Into<ImageFilter>) -> Option<Self> {
        image_filters::compose(outer, inner)
    }
}
