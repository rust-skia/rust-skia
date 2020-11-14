use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::{SkPathVerb, SkPath_Verb};

pub use skia_bindings::SkPathFillType as PathFillType;
#[test]
pub fn test_fill_type_naming() {
    let _ = PathFillType::InverseEvenOdd;
}

pub use skia_bindings::SkPathDirection as PathDirection;
#[test]
fn test_direction_naming() {
    let _ = PathDirection::CW;
}

bitflags! {
    pub struct PathSegmentMask: u32 {
        const LINE = sb::SkPathSegmentMask_kLine_SkPathSegmentMask as _;
        const QUAD = sb::SkPathSegmentMask_kQuad_SkPathSegmentMask as _;
        const CONIC = sb::SkPathSegmentMask_kConic_SkPathSegmentMask as _;
        const CUBIC = sb::SkPathSegmentMask_kCubic_SkPathSegmentMask as _;
    }
}

pub use skia_bindings::SkPathVerb as PathVerb;
#[test]
fn test_path_verb_naming() {
    let _ = PathVerb::Conic;
}

impl NativeTransmutable<SkPath_Verb> for SkPathVerb {}
#[test]
fn test_verb_layout() {
    SkPathVerb::test_layout();
}
