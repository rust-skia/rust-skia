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

pub use sb::skia_textlayout_TextBaseline as TextBaseline;
