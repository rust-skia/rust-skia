use crate::prelude::*;
use crate::skia::{PathEffect, Matrix, scalar, Path};
use skia_bindings::{C_SkLine2DPathEffect_Make, C_SkPath2DPathEffect_Make};

pub enum Line2DPathEffect {}

impl Line2DPathEffect {

    pub fn new(width: scalar, matrix: &Matrix) -> Option<PathEffect> {
        PathEffect::from_ptr(unsafe {
            C_SkLine2DPathEffect_Make(width, matrix.native())
        })
    }
}

pub enum Path2DPathEffect {}

impl Path2DPathEffect {

    pub fn new(matrix: &Matrix, path: &Path) -> PathEffect {
        PathEffect::from_ptr(unsafe {
            C_SkPath2DPathEffect_Make(matrix.native(), path.native())
        }).unwrap()
    }
}
