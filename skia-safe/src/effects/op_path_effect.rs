pub mod merge_path_effect {
    use crate::prelude::*;
    use crate::{PathEffect, PathOp};
    use skia_bindings as sb;
    use skia_bindings::SkPathEffect;

    impl RCHandle<SkPathEffect> {
        pub fn merge(
            one: impl Into<PathEffect>,
            two: impl Into<PathEffect>,
            op: PathOp,
        ) -> PathEffect {
            new(one, two, op)
        }
    }

    pub fn new(one: impl Into<PathEffect>, two: impl Into<PathEffect>, op: PathOp) -> PathEffect {
        PathEffect::from_ptr(unsafe {
            sb::C_SkMergePathEffect_Make(one.into().into_ptr(), two.into().into_ptr(), op)
        })
        .unwrap()
    }
}

pub mod matrix_path_effect {
    use crate::prelude::*;
    use crate::{Matrix, PathEffect, Vector};
    use skia_bindings as sb;
    use skia_bindings::SkPathEffect;

    impl RCHandle<SkPathEffect> {
        pub fn matrix_translate(d: impl Into<Vector>) -> Option<PathEffect> {
            new_translate(d)
        }

        pub fn matrix(matrix: &Matrix) -> Option<PathEffect> {
            new(matrix)
        }
    }

    pub fn new_translate(d: impl Into<Vector>) -> Option<PathEffect> {
        let d = d.into();
        PathEffect::from_ptr(unsafe { sb::C_SkMatrixPathEffect_MakeTranslate(d.x, d.y) })
    }

    pub fn new(matrix: &Matrix) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe { sb::C_SkMatrixPathEffect_Make(matrix.native()) })
    }
}

pub mod stroke_path_effect {
    use crate::prelude::*;
    use crate::{paint, scalar, PathEffect};
    use skia_bindings as sb;
    use skia_bindings::SkPathEffect;

    impl RCHandle<SkPathEffect> {
        pub fn stroke(
            width: scalar,
            join: paint::Join,
            cap: paint::Cap,
            miter: impl Into<Option<scalar>>,
        ) -> Option<PathEffect> {
            new(width, join, cap, miter)
        }
    }

    pub fn new(
        width: scalar,
        join: paint::Join,
        cap: paint::Cap,
        miter: impl Into<Option<scalar>>,
    ) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            sb::C_SkStrokePathEffect_Make(width, join, cap, miter.into().unwrap_or(4.0))
        })
    }
}
