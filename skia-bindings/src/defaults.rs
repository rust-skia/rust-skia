//! This file contains Default trait implementations and functions for types that are
//! used without a handle in skia-safe and get reexported from there.

use crate::{SkBlendMode, SkBlendModeCoeff};
use std::ffi::CStr;

impl Default for SkBlendMode {
    fn default() -> Self {
        SkBlendMode::SrcOver
    }
}

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

#[cfg(feature = "textlayout")]
pub mod textlayout {
    impl Default for crate::skia_textlayout_Affinity {
        fn default() -> Self {
            Self::Upstream
        }
    }

    impl Default for crate::skia_textlayout_RectHeightStyle {
        fn default() -> Self {
            Self::Tight
        }
    }

    impl Default for crate::skia_textlayout_RectWidthStyle {
        fn default() -> Self {
            Self::Tight
        }
    }

    impl Default for crate::skia_textlayout_TextAlign {
        fn default() -> Self {
            Self::Left
        }
    }

    impl Default for crate::skia_textlayout_PositionWithAffinity {
        fn default() -> Self {
            Self {
                position: 0,
                affinity: Default::default(),
            }
        }
    }

    impl Default for crate::skia_textlayout_TextBaseline {
        fn default() -> Self {
            Self::Alphabetic
        }
    }

    impl Default for crate::skia_textlayout_TextDecorationStyle {
        fn default() -> Self {
            Self::Solid
        }
    }

    impl Default for crate::skia_textlayout_StyleType {
        fn default() -> Self {
            Self::AllAttributes
        }
    }

    impl Default for crate::skia_textlayout_PlaceholderAlignment {
        fn default() -> Self {
            Self::Baseline
        }
    }
}
