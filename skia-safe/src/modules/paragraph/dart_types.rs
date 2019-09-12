use crate::prelude::*;
use crate::Rect;
use skia_bindings as sb;

pub use sb::skia_textlayout_Affinity as Affinity;
pub use sb::skia_textlayout_RectHeightStyle as RectHeightStyle;
pub use sb::skia_textlayout_RectWidthStyle as RectWidthStyle;
pub use sb::skia_textlayout_TextAlign as TextAlign;
pub use sb::skia_textlayout_TextDirection as TextDirection;

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
        self.start.max(other.start) <= self.end.min(other.end)
    }

    fn empty(&self) -> bool {
        self.start == EMPTY_INDEX && self.end == EMPTY_INDEX
    }
}

pub const EMPTY_RANGE: Range<usize> = Range {
    start: EMPTY_INDEX,
    end: EMPTY_INDEX,
};

pub use sb::skia_textlayout_TextBaseline as TextBaseline;
use std::ops::Range;
