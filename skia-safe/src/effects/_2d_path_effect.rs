use crate::prelude::*;
use crate::{scalar, Matrix, Path, PathEffect};
use skia_bindings::SkPathEffect;

impl RCHandle<SkPathEffect> {
    pub fn line_2d(width: scalar, matrix: &Matrix) -> Option<PathEffect> {
        line_2d_path_effect::new(width, matrix)
    }

    pub fn path_2d(matrix: &Matrix, path: &Path) -> PathEffect {
        path_2d_path_effect::new(matrix, path)
    }
}

pub mod line_2d_path_effect {
    use crate::prelude::*;
    use crate::{scalar, Matrix, PathEffect};
    use skia_bindings::C_SkLine2DPathEffect_Make;

    pub fn new(width: scalar, matrix: &Matrix) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe { C_SkLine2DPathEffect_Make(width, matrix.native()) })
    }
}

pub mod path_2d_path_effect {
    use crate::prelude::*;
    use crate::{Matrix, Path, PathEffect};
    use skia_bindings::C_SkPath2DPathEffect_Make;

    pub fn new(matrix: &Matrix, path: &Path) -> PathEffect {
        PathEffect::from_ptr(unsafe { C_SkPath2DPathEffect_Make(matrix.native(), path.native()) })
            .unwrap()
    }
}
