//! `SkTiledImageUtils`' [`self::draw_image()`]/[`self::draw_image_rect()`] methods are intended
//! to be direct replacements for their [`Canvas`] equivalents. The `SkTiledImageUtils` calls will
//! break [`crate::Bitmap`]-backed [`Image`]s into smaller tiles and draw them if the original
//! image is too large to be uploaded to the GPU. If the original image doesn't need tiling or is
//! already gpu-backed the [`self::draw_image()`]/[`self::draw_image_rect()`] calls will fall
//! through to the matching [`Canvas`] call.

use skia_bindings::{self as sb, C_SkTiledImageUtils_DrawImageRect};

use crate::{Canvas, Image, Paint, Point, Rect, SamplingOptions, canvas, prelude::*, scalar};

pub fn draw_image_rect(
    canvas: &Canvas,
    image: &Image,
    src: impl AsRef<Rect>,
    dst: impl AsRef<Rect>,
    sampling: Option<SamplingOptions>,
    paint: Option<&Paint>,
    constraint: impl Into<Option<canvas::SrcRectConstraint>>,
) {
    let sampling = sampling.unwrap_or_default();
    let constraint = constraint.into().unwrap_or(canvas::SrcRectConstraint::Fast);
    unsafe {
        C_SkTiledImageUtils_DrawImageRect(
            canvas.native_ptr(),
            image.native(),
            src.as_ref().native(),
            dst.as_ref().native(),
            sampling.native(),
            paint.native_ptr_or_null(),
            constraint,
        )
    }
}

pub fn draw_image(
    canvas: &Canvas,
    image: &Image,
    xy: impl Into<Point>,
    sampling: Option<SamplingOptions>,
    paint: Option<&Paint>,
    constraint: impl Into<Option<canvas::SrcRectConstraint>>,
) {
    let p = xy.into();
    let src = Rect::from_iwh(image.width(), image.height());
    let dst = Rect::from_xywh(p.x, p.y, image.width() as scalar, image.height() as scalar);

    draw_image_rect(canvas, image, src, dst, sampling, paint, constraint)
}

pub const NUM_IMAGE_KEY_VALUES: usize = 6;

/// Retrieves a set of values that can be used as part of a cache key for the provided image.
///
/// Unfortunately, [`Image::unique_id`] isn't sufficient as an [`Image`] cache key. In
/// particular, [`crate::Bitmap`]-backed [`Image`]s can share a single [`crate::Bitmap`] and refer
/// to different subsets of it. In this situation the optimal key is based on the
/// [`crate::Bitmap`]'s generation ID and the subset rectangle.
/// For [`crate::Picture`]-backed images this method will attempt to generate a concise
/// internally-based key (i.e., containing picture ID, matrix translation, width and height,
/// etc.). For complicated [`crate::Picture`]-backed images (i.e., those w/ a paint or a full
/// matrix) it will fall back to using `image`'s unique key.
///
/// - `image` The image for which key values are desired
///
/// Returns the resulting key values.
pub fn get_image_key_values(image: &Image) -> [u32; NUM_IMAGE_KEY_VALUES] {
    let mut key_values = [0u32; NUM_IMAGE_KEY_VALUES];
    unsafe { sb::C_SkTiledImageUtils_GetImageKeyValues(image.native(), key_values.as_mut_ptr()) }
    key_values
}
