use crate::prelude::*;
use skia_bindings::SkClipOp;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ClipOp {
    Difference = SkClipOp::kDifference as _,
    Intersect = SkClipOp::kIntersect as _
}

impl NativeTransmutable<SkClipOp> for ClipOp {}

// This is the default for the canvas's clip functions.
impl Default for ClipOp {
    fn default() -> Self {
        ClipOp::Intersect
    }
}
