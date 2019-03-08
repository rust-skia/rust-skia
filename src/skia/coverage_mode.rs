use crate::prelude::*;
use rust_skia::SkCoverageMode;

pub type CoverageMode = EnumHandle<SkCoverageMode>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkCoverageMode> {
    pub const Union: Self = Self(SkCoverageMode::kUnion);
    pub const Intersect: Self = Self(SkCoverageMode::kIntersect);
    pub const Difference: Self = Self(SkCoverageMode::kDifference);
    pub const ReverseDifference: Self = Self(SkCoverageMode::kReverseDifference);
    pub const Xor: Self = Self(SkCoverageMode::kXor);
}