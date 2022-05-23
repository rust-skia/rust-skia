use crate::Rect;
use skia_bindings as sb;
use std::{
    cmp::{max, min},
    ops::Range,
};

pub use sb::skia_textlayout_Affinity as Affinity;
variant_name!(Affinity::Downstream, affinity_naming);
pub use sb::skia_textlayout_RectHeightStyle as RectHeightStyle;
variant_name!(
    RectHeightStyle::IncludeLineSpacingBottom,
    rect_height_style_naming
);
pub use sb::skia_textlayout_RectWidthStyle as RectWidthStyle;
variant_name!(RectWidthStyle::Max, rect_width_style_naming);
pub use sb::skia_textlayout_TextAlign as TextAlign;
variant_name!(TextAlign::End, text_align_naming);
pub use sb::skia_textlayout_TextDirection as TextDirection;
variant_name!(TextDirection::LTR, text_direction_naming);

pub use sb::skia_textlayout_PositionWithAffinity as PositionWithAffinity;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TextBox {
    pub rect: Rect,
    pub direct: TextDirection,
}

native_transmutable!(sb::skia_textlayout_TextBox, TextBox, text_box_layout);

pub const EMPTY_INDEX: usize = std::usize::MAX;

pub trait RangeExtensions {
    fn width(&self) -> usize;
    fn shift(&mut self, d: usize);
    fn contains(&self, other: &Self) -> bool;
    fn intersects(&self, other: &Self) -> bool;
    #[must_use]
    fn intersection(&self, other: &Self) -> Self;
    fn empty(&self) -> bool;
}

impl RangeExtensions for Range<usize> {
    fn width(&self) -> usize {
        self.end - self.start
    }

    fn shift(&mut self, d: usize) {
        self.start += d;
        self.end += d;
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn intersects(&self, other: &Self) -> bool {
        max(self.start, other.start) <= min(self.end, other.end)
    }

    fn intersection(&self, other: &Self) -> Self {
        Self {
            start: max(self.start, other.start),
            end: min(self.end, other.end),
        }
    }

    fn empty(&self) -> bool {
        self.start == EMPTY_INDEX && self.end == EMPTY_INDEX
    }
}

#[allow(clippy::reversed_empty_ranges)]
pub const EMPTY_RANGE: Range<usize> = Range {
    start: EMPTY_INDEX,
    end: EMPTY_INDEX,
};

pub use sb::skia_textlayout_TextBaseline as TextBaseline;
variant_name!(TextBaseline::Alphabetic, text_baseline_naming);

pub use sb::skia_textlayout_TextHeightBehavior as TextHeightBehavior;
variant_name!(
    TextHeightBehavior::DisableFirstAscent,
    text_height_behavior_naming
);

// m84: LineMetricStyle is declared but not used in the public API yet.
