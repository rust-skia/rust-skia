//! Wrapper for pathops/SkPathOps.h

use crate::prelude::*;
use crate::{Path, Rect};
use skia_bindings::{
    C_SkOpBuilder_Construct, C_SkOpBuilder_destruct, SkOpBuilder, SkPath, SkPathOp,
};
use std::mem;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PathOp {
    Difference = SkPathOp::kDifference_SkPathOp as _,
    Intersect = SkPathOp::kIntersect_SkPathOp as _,
    Union = SkPathOp::kUnion_SkPathOp as _,
    XOR = SkPathOp::kXOR_SkPathOp as _,
    ReverseDifference = SkPathOp::kReverseDifference_SkPathOp as _,
}

impl NativeTransmutable<SkPathOp> for PathOp {}
#[test]
fn test_path_op_layout() {
    PathOp::test_layout();
}

// TODO: I am not so sure if we should export these global functions.

pub fn op(one: &Path, two: &Path, op: PathOp) -> Option<Path> {
    let mut result = Path::default();
    unsafe {
        skia_bindings::Op(
            one.native(),
            two.native(),
            op.into_native(),
            result.native_mut(),
        )
    }
    .if_true_some(result)
}

pub fn simplify(path: &Path) -> Option<Path> {
    let mut result = Path::default();
    unsafe { skia_bindings::Simplify(path.native(), result.native_mut()) }.if_true_some(result)
}

pub fn tight_bounds(path: &Path) -> Option<Rect> {
    let mut result = Rect::default();
    unsafe { skia_bindings::TightBounds(path.native(), result.native_mut()) }.if_true_some(result)
}

pub fn as_winding(path: &Path) -> Option<Path> {
    let mut result = Path::default();
    unsafe { skia_bindings::AsWinding(path.native(), result.native_mut()) }.if_true_some(result)
}

pub type OpBuilder = Handle<SkOpBuilder>;

impl NativeDrop for SkOpBuilder {
    fn drop(&mut self) {
        unsafe { C_SkOpBuilder_destruct(self) }
    }
}

impl Default for Handle<SkOpBuilder> {
    fn default() -> Self {
        let mut op_builder = unsafe { mem::zeroed() };
        unsafe { C_SkOpBuilder_Construct(&mut op_builder) }
        op_builder.into_handle()
    }
}

impl Handle<SkOpBuilder> {
    pub fn add(&mut self, path: &Path, operator: PathOp) -> &mut Self {
        unsafe {
            self.native_mut().add(path.native(), operator.into_native());
        }
        self
    }

    pub fn resolve(&mut self) -> Option<Path> {
        let mut path = Path::default();
        unsafe { self.native_mut().resolve(path.native_mut()) }.if_true_some(path)
    }
}

impl Handle<SkPath> {
    pub fn op(&self, path: &Path, path_op: PathOp) -> Option<Self> {
        op(self, path, path_op)
    }

    pub fn simplify(&self) -> Option<Self> {
        simplify(self)
    }

    pub fn tight_bounds(&self) -> Option<Rect> {
        tight_bounds(self)
    }

    pub fn as_winding(&self) -> Option<Path> {
        as_winding(self)
    }
}
