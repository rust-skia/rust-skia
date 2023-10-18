use crate::prelude::*;
use crate::{Matrix, Paint, Path, Rect};
use skia_bindings as sb;

/// Returns the filled equivalent of the stroked path.
///
/// * `src` - [`Path`] read to create a filled version
/// * `paint` - [`Paint`], from which attributes such as stroke cap, width, miter, and join,
///                  as well as `path_effect` will be used.
/// * `dst` - resulting [`Path`]
/// * `cull_rect` - optional limit passed to [`crate::PathEffect`]
/// * `matrix` - if scale > 1, increase precision, else if (0 < scale < 1) reduce precision
///                  to favor speed and size
/// Returns: `true` if the dst path was updated, `false` if it was not (e.g. if the path
///                  represents hairline and cannot be filled).
pub fn fill_path_with_paint<'a>(
    src: &Path,
    paint: &Paint,
    dst: &mut Path,
    cull_rect: impl Into<Option<&'a Rect>>,
    matrix: impl Into<Option<Matrix>>,
) -> bool {
    let cull_rect: Option<&'a Rect> = cull_rect.into();
    let matrix = matrix.into().unwrap_or(Matrix::scale((1.0, 1.0)));

    unsafe {
        sb::C_PathUtils_FillPathWithPaint(
            src.native(),
            paint.native(),
            dst.native_mut(),
            cull_rect.native_ptr_or_null(),
            matrix.native(),
        )
    }
}
