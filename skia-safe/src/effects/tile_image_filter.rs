use crate::prelude::*;
use crate::{ImageFilter, Rect};
use skia_bindings::C_SkTileImageFilter_Make;

pub enum TileImageFilter {}

impl TileImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<SR: AsRef<Rect>, DR: AsRef<Rect>>(
        src: SR,
        dst: DR,
        input: &ImageFilter,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkTileImageFilter_Make(
                src.as_ref().native(),
                dst.as_ref().native(),
                input.shared_native(),
            )
        })
    }
}
