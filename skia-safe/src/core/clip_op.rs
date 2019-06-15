use crate::prelude::*;
use skia_bindings::SkClipOp;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ClipOp {
    Difference = SkClipOp::kDifference as _,
    Intersect = SkClipOp::kIntersect as _,
}

impl NativeTransmutable<SkClipOp> for ClipOp {}
#[test]
fn test_clip_op_layout() {
    ClipOp::test_layout()
}

// This is the default for the canvas's clip functions.
impl Default for ClipOp {
    fn default() -> Self {
        ClipOp::Intersect
    }
}
