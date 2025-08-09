use skia_bindings as sb;

pub use sb::SkPathFillType as PathFillType;
variant_name!(PathFillType::InverseEvenOdd);

pub use sb::SkPathDirection as PathDirection;
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

pub use sb::SkPathVerb as PathVerb;
variant_name!(PathVerb::Conic);
