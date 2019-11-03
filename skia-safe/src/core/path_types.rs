use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::{
    SkPathConvexityType, SkPathDirection, SkPathFillType, SkPathVerb, SkPath_Convexity,
    SkPath_Direction, SkPath_FillType, SkPath_Verb,
};
use std::mem;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PathFillType {
    Winding = SkPathFillType::kWinding as _,
    EvenOdd = SkPathFillType::kEvenOdd as _,
    InverseWinding = SkPathFillType::kInverseWinding as _,
    InverseEvenOdd = SkPathFillType::kInverseEvenOdd as _,
}

impl NativeTransmutable<SkPathFillType> for PathFillType {}
impl NativeTransmutable<SkPath_FillType> for SkPathFillType {}
#[test]
pub fn test_fill_type_layout() {
    PathFillType::test_layout();
    SkPathFillType::test_layout();
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PathConvexityType {
    Unknown = SkPathConvexityType::kUnknown as _,
    Convex = SkPathConvexityType::kConvex as _,
    Concave = SkPathConvexityType::kConcave as _,
}

impl NativeTransmutable<SkPathConvexityType> for PathConvexityType {}
#[test]
fn test_convexity_layout() {
    PathConvexityType::test_layout();
}

// The sizes of SkPath_Convexity and SkPathConvexityType differ in C++, so
// we provide custom conversions.
impl PathConvexityType {
    pub(crate) fn native_from_path(convexity: SkPath_Convexity) -> SkPathConvexityType {
        unsafe { mem::transmute(convexity as i32) }
    }

    pub(crate) fn native_to_path(convexity: SkPathConvexityType) -> SkPath_Convexity {
        unsafe { mem::transmute(convexity as u8) }
    }
}

#[test]
fn test_path_convexity_layout() {
    assert_eq!(mem::size_of::<SkPath_Convexity>(), mem::size_of::<u8>());
    assert_eq!(mem::size_of::<SkPathConvexityType>(), mem::size_of::<i32>());
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PathDirection {
    CW = SkPathDirection::kCW as _,
    CCW = SkPathDirection::kCCW as _,
}

impl NativeTransmutable<SkPathDirection> for PathDirection {}
impl NativeTransmutable<SkPath_Direction> for SkPathDirection {}
#[test]
fn test_direction_layout() {
    PathDirection::test_layout();
    SkPathDirection::test_layout();
}

impl Default for PathDirection {
    fn default() -> Self {
        PathDirection::CW
    }
}

bitflags! {
    pub struct PathSegmentMask: u32 {
        const LINE = sb::SkPathSegmentMask_kLine_SkPathSegmentMask as _;
        const QUAD = sb::SkPathSegmentMask_kQuad_SkPathSegmentMask as _;
        const CONIC = sb::SkPathSegmentMask_kConic_SkPathSegmentMask as _;
        const CUBIC = sb::SkPathSegmentMask_kCubic_SkPathSegmentMask as _;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PathVerb {
    Move = SkPathVerb::kMove as _,
    Line = SkPathVerb::kLine as _,
    Quad = SkPathVerb::kQuad as _,
    Conic = SkPathVerb::kConic as _,
    Qubic = SkPathVerb::kCubic as _,
    Close = SkPathVerb::kClose as _,
    Done = SkPathVerb::kDone as _,
}

impl NativeTransmutable<SkPathVerb> for PathVerb {}
impl NativeTransmutable<SkPath_Verb> for SkPathVerb {}
#[test]
fn test_verb_layout() {
    PathVerb::test_layout();
    SkPathVerb::test_layout();
}
