use crate::prelude::*;
use skia_bindings::SkClipOp;

pub type ClipOp = EnumHandle<SkClipOp>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkClipOp> {
    pub const Difference: Self = Self(SkClipOp::kDifference);
    pub const Intersect: Self = Self(SkClipOp::kIntersect);
}

// This is the default for the canvas's clip functions.
impl Default for EnumHandle<SkClipOp> {
    fn default() -> Self {
        ClipOp::Intersect
    }
}
