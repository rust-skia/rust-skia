use crate::prelude::*;
use crate::Rect;
use skia_bindings as sb;

pub type Affinity = sb::skia_textlayout_Affinity;
pub type RectHeightStyle = sb::skia_textlayout_RectHeightStyle;
pub type RectWidthStyle = sb::skia_textlayout_RectWidthStyle;
pub type TextAlign = sb::skia_textlayout_TextAlign;
pub type TextDirection = sb::skia_textlayout_TextDirection;

pub type PositionWithAffinity = sb::skia_textlayout_PositionWithAffinity;

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

pub type TextBaseline = sb::skia_textlayout_TextBaseline;
