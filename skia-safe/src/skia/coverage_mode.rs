use crate::prelude::*;
use skia_bindings::SkCoverageMode;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CoverageMode {
    Union = SkCoverageMode::kUnion as _,
    Intersect = SkCoverageMode::kIntersect as _,
    Difference = SkCoverageMode::kDifference as _,
    ReverseDifference = SkCoverageMode::kReverseDifference as _,
    Xor = SkCoverageMode::kXor as _
}

impl NativeTransmutable<SkCoverageMode> for CoverageMode {}