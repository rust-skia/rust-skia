//! Wrapper for pathops/SkPathOps.h
use crate::{prelude::*, Path, Rect};
use skia_bindings::{self as sb, SkOpBuilder};
use std::fmt;

pub use skia_bindings::SkPathOp as PathOp;
variant_name!(PathOp::XOR);

// TODO: I am not so sure if we should export these global functions.

pub fn op(one: &Path, two: &Path, op: PathOp) -> Option<Path> {
    let mut result = Path::default();
    unsafe { sb::Op(one.native(), two.native(), op, result.native_mut()) }.if_true_some(result)
}

pub fn simplify(path: &Path) -> Option<Path> {
    let mut result = Path::default();
    unsafe { sb::Simplify(path.native(), result.native_mut()) }.if_true_some(result)
}

pub fn tight_bounds(path: &Path) -> Option<Rect> {
    let mut result = Rect::default();
    unsafe { sb::TightBounds(path.native(), result.native_mut()) }.if_true_some(result)
}

pub fn as_winding(path: &Path) -> Option<Path> {
    let mut result = Path::default();
    unsafe { sb::AsWinding(path.native(), result.native_mut()) }.if_true_some(result)
}

pub type OpBuilder = Handle<SkOpBuilder>;
unsafe_send_sync!(OpBuilder);

impl NativeDrop for SkOpBuilder {
    fn drop(&mut self) {
        unsafe { sb::C_SkOpBuilder_destruct(self) }
    }
}

impl Default for Handle<SkOpBuilder> {
    fn default() -> Self {
        Self::construct(|opb| unsafe { sb::C_SkOpBuilder_Construct(opb) })
    }
}

impl fmt::Debug for OpBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpBuilder").finish()
    }
}

impl OpBuilder {
    pub fn add(&mut self, path: &Path, operator: PathOp) -> &mut Self {
        unsafe {
            self.native_mut().add(path.native(), operator);
        }
        self
    }

    pub fn resolve(&mut self) -> Option<Path> {
        let mut path = Path::default();
        unsafe { self.native_mut().resolve(path.native_mut()) }.if_true_some(path)
    }
}

impl Path {
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

#[test]
fn test_tight_bounds() {
    let mut path = Path::new();
    path.add_rect(Rect::from_point_and_size((10.0, 10.0), (10.0, 10.0)), None);
    path.add_rect(Rect::from_point_and_size((15.0, 15.0), (10.0, 10.0)), None);
    let tight_bounds: Rect = Rect::from_point_and_size((10.0, 10.0), (15.0, 15.0));
    assert_eq!(path.tight_bounds().unwrap(), tight_bounds);
}

#[test]
fn test_union() {
    let mut path = Path::new();
    path.add_rect(Rect::from_point_and_size((10.0, 10.0), (10.0, 10.0)), None);
    let mut path2 = Path::new();
    path2.add_rect(Rect::from_point_and_size((15.0, 15.0), (10.0, 10.0)), None);
    let union = path.op(&path2, PathOp::Union).unwrap();
    let expected: Rect = Rect::from_point_and_size((10.0, 10.0), (15.0, 15.0));
    assert_eq!(union.tight_bounds().unwrap(), expected);
}

#[test]
fn test_intersect() {
    let mut path = Path::new();
    path.add_rect(Rect::from_point_and_size((10.0, 10.0), (10.0, 10.0)), None);
    let mut path2 = Path::new();
    path2.add_rect(Rect::from_point_and_size((15.0, 15.0), (10.0, 10.0)), None);
    let intersected = path.op(&path2, PathOp::Intersect).unwrap();
    let expected: Rect = Rect::from_point_and_size((15.0, 15.0), (5.0, 5.0));
    assert_eq!(intersected.tight_bounds().unwrap(), expected);
}
