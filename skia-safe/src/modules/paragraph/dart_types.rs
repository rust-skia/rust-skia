use crate::prelude::*;
use crate::Rect;
use skia_bindings as sb;
use std::cmp::{max, min};
use std::ops::Range;

pub use sb::skia_textlayout_Affinity as Affinity;
pub use sb::skia_textlayout_RectHeightStyle as RectHeightStyle;
pub use sb::skia_textlayout_RectWidthStyle as RectWidthStyle;
pub use sb::skia_textlayout_TextAlign as TextAlign;
pub use sb::skia_textlayout_TextDirection as TextDirection;
#[test]
fn test_reexported_enum_name_conversion() {
    let _ = Affinity::Downstream;
    let _ = RectHeightStyle::IncludeLineSpacingBottom;
    let _ = RectWidthStyle::Max;
    let _ = TextAlign::End;
    let _ = TextDirection::LTR;
}

pub use sb::skia_textlayout_PositionWithAffinity as PositionWithAffinity;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct TextBox {
    pub rect: Rect,
    pub direct: TextDirection,
}

impl NativeTransmutable<sb::skia_textlayout_TextBox> for TextBox {}

#[test]
fn text_box_layout() {
    TextBox::test_layout()
}

pub const EMPTY_INDEX: usize = std::usize::MAX;

pub trait RangeExtensions {
    fn width(&self) -> usize;
    fn shift(&mut self, d: usize);
    fn contains(&self, other: &Self) -> bool;
    fn intersects(&self, other: &Self) -> bool;
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

#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::reversed_empty_ranges)] // 1.45 lint
pub const EMPTY_RANGE: Range<usize> = Range {
    start: EMPTY_INDEX,
    end: EMPTY_INDEX,
};

pub use sb::skia_textlayout_TextBaseline as TextBaseline;
#[test]
fn test_text_baseline_naming() {
    let _ = TextBaseline::Alphabetic;
}

pub use sb::skia_textlayout_TextHeightBehavior as TextHeightBehavior;
#[test]
fn test_text_height_behavior_naming() {
    let _ = TextHeightBehavior::DisableFirstAscent;
}

// m84: LineMetricStyle is declared but not used in the public API yet.

pub use sb::skia_textlayout_DrawOptions as DrawOptions;
#[test]
fn test_draw_options_naming() {
    let _ = DrawOptions::Replay;
}
