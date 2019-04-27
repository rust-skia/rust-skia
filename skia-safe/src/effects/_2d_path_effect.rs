use crate::prelude::*;
use crate::{scalar, Matrix, Path, PathEffect};
use skia_bindings::{C_SkLine2DPathEffect_Make, C_SkPath2DPathEffect_Make, SkPathEffect};

pub enum Line2DPathEffect {}

impl Line2DPathEffect {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(width: scalar, matrix: &Matrix) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe { C_SkLine2DPathEffect_Make(width, matrix.native()) })
    }
}

pub enum Path2DPathEffect {}

impl Path2DPathEffect {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(matrix: &Matrix, path: &Path) -> PathEffect {
        PathEffect::from_ptr(unsafe { C_SkPath2DPathEffect_Make(matrix.native(), path.native()) })
            .unwrap()
    }
}

impl RCHandle<SkPathEffect> {
    pub fn line_2d(width: scalar, matrix: &Matrix) -> Option<PathEffect> {
        Line2DPathEffect::new(width, matrix)
    }

    pub fn path_2d(matrix: &Matrix, path: &Path) -> PathEffect {
        Path2DPathEffect::new(matrix, path)
    }
}
