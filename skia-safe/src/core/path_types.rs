use skia_bindings::{self as sb, SkPathVerb, SkPath_Verb};

pub use skia_bindings::SkPathFillType as PathFillType;
variant_name!(PathFillType::InverseEvenOdd);

pub use skia_bindings::SkPathDirection as PathDirection;
variant_name!(PathDirection::CW);

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PathSegmentMask: u32 {
        const LINE = sb::SkPathSegmentMask_kLine_SkPathSegmentMask as _;
        const QUAD = sb::SkPathSegmentMask_kQuad_SkPathSegmentMask as _;
        const CONIC = sb::SkPathSegmentMask_kConic_SkPathSegmentMask as _;
        const CUBIC = sb::SkPathSegmentMask_kCubic_SkPathSegmentMask as _;
    }
}

pub use skia_bindings::SkPathVerb as PathVerb;
variant_name!(PathVerb::Conic);

native_transmutable!(SkPath_Verb, SkPathVerb, path_verb_layout);
