//! This file contains implementations for types that are
//! re-exported in skia-safe.
//!
//! We could provide trait implementations in skia-safe, but then users of the library would have to
//! import the implementation type _and_ the trait.
//!
//! See also: https://github.com/rust-lang/rfcs/issues/1880

use crate::{SkAlphaType, SkBlendMode, SkBlendModeCoeff, SkPathFillType, SkPathVerb};
use std::ffi::CStr;

impl SkBlendMode {
    pub fn as_coeff(self) -> Option<(SkBlendModeCoeff, SkBlendModeCoeff)> {
        let mut src = SkBlendModeCoeff::Zero;
        let mut dst = SkBlendModeCoeff::Zero;
        if unsafe { crate::SkBlendMode_AsCoeff(self, &mut src, &mut dst) } {
            Some((src, dst))
        } else {
            None
        }
    }

    pub fn name(self) -> &'static str {
        unsafe {
            let name_ptr = crate::SkBlendMode_Name(self);
            CStr::from_ptr(name_ptr).to_str().unwrap()
        }
    }
}

impl SkPathVerb {
    /// The maximum number of points an iterator will return for the verb.
    pub const MAX_POINTS: usize = 4;
    /// The number of points an iterator will return for the verb.
    pub fn points(self) -> usize {
        match self {
            SkPathVerb::Move => 1,
            SkPathVerb::Line => 2,
            SkPathVerb::Quad => 3,
            SkPathVerb::Conic => 4,
            SkPathVerb::Cubic => 4,
            SkPathVerb::Close => 0,
            SkPathVerb::Done => 0,
        }
    }
}

impl SkPathFillType {
    pub fn is_even_odd(self) -> bool {
        (self as i32 & 1) != 0
    }

    pub fn is_inverse(self) -> bool {
        (self as i32 & 2) != 0
    }

    pub fn to_non_inverse(self) -> Self {
        match self {
            SkPathFillType::Winding => self,
            SkPathFillType::EvenOdd => self,
            SkPathFillType::InverseWinding => SkPathFillType::Winding,
            SkPathFillType::InverseEvenOdd => SkPathFillType::EvenOdd,
        }
    }
}

impl SkAlphaType {
    pub fn is_opaque(self) -> bool {
        self == SkAlphaType::Opaque
    }
}
