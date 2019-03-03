use crate::skia::scalar;
use crate::prelude::*;
use rust_skia::{
    SkIRect,
    SkRect
};
use crate::skia::ISize;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32
}

impl NativeTransmutable<SkIRect> for IRect {}

#[test]
fn test_irect_layout() {
    IRect::test_layout();
}

impl Default for IRect {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl IRect {

    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> IRect {
        IRect { left, top, right, bottom }
    }

    pub fn from_size(size: ISize) -> IRect {
        Self::new(0, 0, size.width, size.height)
    }

    pub fn intersects(a: &IRect, b: &IRect) -> bool {
        unsafe { SkIRect::Intersects(a.native(), b.native()) }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rect {
    pub left: scalar,
    pub top: scalar,
    pub right: scalar,
    pub bottom: scalar
}

impl NativeTransmutable<SkRect> for Rect {}

#[test]
fn test_rect_layout() {
    IRect::test_layout();
}
