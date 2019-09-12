use super::{PositionWithAffinity, RectHeightStyle, RectWidthStyle, TextBox};
use crate::prelude::*;
use crate::{scalar, Canvas, Point};
use skia_bindings as sb;
use std::ops::{Index, Range};

pub type Paragraph = RefHandle<sb::skia_textlayout_Paragraph>;

impl NativeDrop for sb::skia_textlayout_Paragraph {
    fn drop(&mut self) {
        unsafe { sb::C_Paragraph_delete(self) }
    }
}

impl RefHandle<sb::skia_textlayout_Paragraph> {
    pub fn max_width(&self) -> scalar {
        self.native().fWidth
    }

    pub fn height(&self) -> scalar {
        self.native().fHeight
    }

    pub fn min_intrinsic_width(&self) -> scalar {
        self.native().fMinIntrinsicWidth
    }

    pub fn max_intrinsic_width(&self) -> scalar {
        self.native().fMaxIntrinsicWidth
    }

    pub fn alphabetic_baseline(&self) -> scalar {
        self.native().fAlphabeticBaseline
    }

    pub fn ideographic_baseline(&self) -> scalar {
        self.native().fIdeographicBaseline
    }

    pub fn did_exceed_max_lines(&mut self) -> bool {
        unsafe { sb::C_Paragraph_didExceedMaxLines(self.native_mut()) }
    }

    pub fn layout(&mut self, width: scalar) {
        unsafe { sb::C_Paragraph_layout(self.native_mut(), width) }
    }

    pub fn paint(&mut self, canvas: &mut Canvas, p: impl Into<Point>) {
        let p = p.into();
        unsafe { sb::C_Paragraph_paint(self.native_mut(), canvas.native_mut(), p.x, p.y) }
    }

    pub fn get_rects_for_range(
        &mut self,
        range: Range<usize>,
        rect_height_style: RectHeightStyle,
        rect_width_style: RectWidthStyle,
    ) -> TextBoxes {
        TextBoxes::construct(|tb| unsafe {
            sb::C_Paragraph_getRectsForRange(
                self.native_mut(),
                range.start.try_into().unwrap(),
                range.end.try_into().unwrap(),
                rect_height_style,
                rect_width_style,
                tb,
            )
        })
    }

    pub fn get_rects_for_placeholders(&mut self) -> TextBoxes {
        TextBoxes::construct(|tb| unsafe {
            sb::C_Paragraph_GetRectsForPlaceholders(self.native_mut(), tb)
        })
    }

    pub fn get_glyph_position_at_coordinate(
        &mut self,
        p: impl Into<Point>,
    ) -> PositionWithAffinity {
        let p = p.into();
        let mut r = Default::default();
        unsafe { sb::C_Paragraph_getGlyphPositionAtCoordinate(self.native_mut(), p.x, p.y, &mut r) }
        r
    }

    pub fn get_word_boundary(&mut self, offset: u32) -> Range<usize> {
        let mut range: [usize; 2] = Default::default();
        unsafe { sb::C_Paragraph_getWordBoundary(self.native_mut(), offset, range.as_mut_ptr()) }
        range[0]..range[1]
    }

    pub fn line_number(&mut self) -> usize {
        unsafe { sb::C_Paragraph_lineNumber(self.native_mut()) }
    }

    pub fn mark_dirty(&mut self) {
        unsafe { sb::C_Paragraph_markDirty(self.native_mut()) }
    }
}

pub type TextBoxes = Handle<sb::TextBoxes>;

impl NativeDrop for sb::TextBoxes {
    fn drop(&mut self) {
        unsafe { sb::C_TextBoxes_destruct(self) }
    }
}

impl Index<usize> for Handle<sb::TextBoxes> {
    type Output = TextBox;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl AsRef<[TextBox]> for TextBoxes {
    fn as_ref(&self) -> &[TextBox] {
        self.as_slice()
    }
}

impl Handle<sb::TextBoxes> {
    pub fn iter(&self) -> impl Iterator<Item = &TextBox> {
        self.as_slice().iter()
    }

    pub fn as_slice(&self) -> &[TextBox] {
        unsafe {
            let mut count = 0;
            let ptr = sb::C_TextBoxes_ptr_count(self.native(), &mut count);
            std::slice::from_raw_parts(ptr as *const TextBox, count)
        }
    }
}
