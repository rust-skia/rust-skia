//! This file contains Default trait implementations for types that are
//! used without a handle in skia-safe and get reexported from there.

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
