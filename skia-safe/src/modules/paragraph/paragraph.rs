use std::{fmt, ops::Range};

use skia_bindings as sb;

use super::{
    LineMetrics, PositionWithAffinity, RectHeightStyle, RectWidthStyle, TextBox, TextDirection,
    TextIndex, TextRange,
};
use crate::{
    interop::{Sink, VecSink},
    prelude::*,
    scalar, Canvas, Font, Point, Rect, Unichar,
};

pub type Paragraph = RefHandle<sb::skia_textlayout_Paragraph>;
unsafe_send_sync!(Paragraph);

impl NativeDrop for sb::skia_textlayout_Paragraph {
    fn drop(&mut self) {
        unsafe { sb::C_Paragraph_delete(self) }
    }
}

impl fmt::Debug for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Paragraph")
            .field("max_width", &self.max_width())
            .field("height", &self.height())
            .field("min_intrinsic_width", &self.min_intrinsic_width())
            .field("max_intrinsic_width", &self.max_intrinsic_width())
            .field("alphabetic_baseline", &self.alphabetic_baseline())
            .field("ideographic_baseline", &self.ideographic_baseline())
            .field("longest_line", &self.longest_line())
            .field("did_exceed_max_lines", &self.did_exceed_max_lines())
            .field("line_number", &self.line_number())
            .finish()
    }
}

impl Paragraph {
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

    pub fn longest_line(&self) -> scalar {
        self.native().fLongestLine
    }

    pub fn did_exceed_max_lines(&self) -> bool {
        self.native().fExceededMaxLines
    }

    pub fn layout(&mut self, width: scalar) {
        unsafe { sb::C_Paragraph_layout(self.native_mut(), width) }
    }

    pub fn paint(&self, canvas: &mut Canvas, p: impl Into<Point>) {
        let p = p.into();
        unsafe { sb::C_Paragraph_paint(self.native_mut_force(), canvas.native_mut(), p.x, p.y) }
    }

    /// Returns a vector of bounding boxes that enclose all text between
    /// start and end glyph indexes, including start and excluding end
    pub fn get_rects_for_range(
        &self,
        range: Range<usize>,
        rect_height_style: RectHeightStyle,
        rect_width_style: RectWidthStyle,
    ) -> Vec<TextBox> {
        let mut result: Vec<TextBox> = Vec::new();

        let mut set_tb = |tbs: &[sb::skia_textlayout_TextBox]| {
            result = tbs.iter().map(TextBox::from_native_ref).cloned().collect();
        };

        unsafe {
            sb::C_Paragraph_getRectsForRange(
                self.native_mut_force(),
                range.start.try_into().unwrap(),
                range.end.try_into().unwrap(),
                rect_height_style.into_native(),
                rect_width_style.into_native(),
                VecSink::new(&mut set_tb).native_mut(),
            );
        }
        result
    }

    pub fn get_rects_for_placeholders(&self) -> Vec<TextBox> {
        let mut result = Vec::new();

        let mut set_tb = |tbs: &[sb::skia_textlayout_TextBox]| {
            result = tbs.iter().map(TextBox::from_native_ref).cloned().collect();
        };

        unsafe {
            sb::C_Paragraph_getRectsForPlaceholders(
                self.native_mut_force(),
                VecSink::new(&mut set_tb).native_mut(),
            )
        }
        result
    }

    /// Returns the index of the glyph that corresponds to the provided coordinate,
    /// with the top left corner as the origin, and +y direction as down
    pub fn get_glyph_position_at_coordinate(&self, p: impl Into<Point>) -> PositionWithAffinity {
        let p = p.into();
        let mut r = Default::default();
        unsafe {
            sb::C_Paragraph_getGlyphPositionAtCoordinate(self.native_mut_force(), p.x, p.y, &mut r)
        }
        r
    }

    /// Finds the first and last glyphs that define a word containing
    /// the glyph at index offset
    pub fn get_word_boundary(&self, offset: u32) -> Range<usize> {
        let mut range: [usize; 2] = Default::default();
        unsafe {
            sb::C_Paragraph_getWordBoundary(self.native_mut_force(), offset, range.as_mut_ptr())
        }
        range[0]..range[1]
    }

    pub fn get_line_metrics(&self) -> Vec<LineMetrics> {
        let mut result: Vec<LineMetrics> = Vec::new();
        let mut set_lm = |lms: &[sb::skia_textlayout_LineMetrics]| {
            result = lms.iter().map(LineMetrics::from_native_ref).collect();
        };

        unsafe {
            sb::C_Paragraph_getLineMetrics(
                self.native_mut_force(),
                VecSink::new(&mut set_lm).native_mut(),
            )
        }

        result
    }

    pub fn line_number(&self) -> usize {
        unsafe { sb::C_Paragraph_lineNumber(self.native_mut_force()) }
    }

    pub fn mark_dirty(&mut self) {
        unsafe { sb::C_Paragraph_markDirty(self.native_mut()) }
    }

    /// This function will return the number of unresolved glyphs or
    /// `None` if not applicable (has not been shaped yet - valid case)
    pub fn unresolved_glyphs(&mut self) -> Option<usize> {
        unsafe { sb::C_Paragraph_unresolvedGlyphs(self.native_mut()) }
            .try_into()
            .ok()
    }

    pub fn unresolved_codepoints(&mut self) -> Vec<Unichar> {
        let mut result = Vec::new();

        let mut set_chars = |chars: &[Unichar]| {
            result = chars.to_vec();
        };

        unsafe {
            sb::C_Paragraph_unresolvedCodepoints(
                self.native_mut_force(),
                VecSink::new(&mut set_chars).native_mut(),
            )
        }

        result
    }

    // TODO: wrap visit()

    pub fn get_line_number_at(&self, code_unit_index: TextIndex) -> Option<usize> {
        // Returns -1 if `code_unit_index` is out of range.
        unsafe { sb::C_Paragraph_getLineNumberAt(self.native(), code_unit_index) }
            .try_into()
            .ok()
    }

    pub fn get_line_metrics_at(&self, line_number: usize) -> Option<LineMetrics> {
        let mut r = None;
        let mut set_lm = |lm: &sb::skia_textlayout_LineMetrics| {
            r = Some(LineMetrics::from_native_ref(lm));
        };
        unsafe {
            sb::C_Paragraph_getLineMetricsAt(
                self.native(),
                line_number,
                Sink::new(&mut set_lm).native_mut(),
            )
        }
        r
    }

    pub fn get_actual_text_range(&self, line_number: usize, include_spaces: bool) -> TextRange {
        let mut range = [0usize; 2];
        unsafe {
            sb::C_Paragraph_getActualTextRange(
                self.native(),
                line_number,
                include_spaces,
                range.as_mut_ptr(),
            )
        }
        TextRange {
            start: range[0],
            end: range[1],
        }
    }

    pub fn get_glyph_cluster_at(&self, code_unit_index: TextIndex) -> Option<GlyphClusterInfo> {
        let mut r = None;
        let mut set_fn = |gci: &sb::skia_textlayout_Paragraph_GlyphClusterInfo| {
            r = Some(GlyphClusterInfo::from_native_ref(gci))
        };
        unsafe {
            sb::C_Paragraph_getGlyphClusterAt(
                self.native(),
                code_unit_index,
                Sink::new(&mut set_fn).native_mut(),
            )
        }
        r
    }

    pub fn get_closest_glyph_cluster_at(&self, d: impl Into<Point>) -> Option<GlyphClusterInfo> {
        let mut r = None;
        let mut set_fn = |gci: &sb::skia_textlayout_Paragraph_GlyphClusterInfo| {
            r = Some(GlyphClusterInfo::from_native_ref(gci))
        };
        let d = d.into();
        unsafe {
            sb::C_Paragraph_getClosestGlyphClusterAt(
                self.native(),
                d.x,
                d.y,
                Sink::new(&mut set_fn).native_mut(),
            )
        }
        r
    }

    pub fn get_font_at(&self, code_unit_index: TextIndex) -> Font {
        Font::construct(|f| unsafe { sb::C_Paragraph_getFontAt(self.native(), code_unit_index, f) })
    }

    pub fn get_fonts(&self) -> Vec<FontInfo> {
        let mut result = Vec::new();
        let mut set_fn = |fis: &[sb::skia_textlayout_Paragraph_FontInfo]| {
            result = fis.iter().map(FontInfo::from_native_ref).collect();
        };
        unsafe { sb::C_Paragraph_getFonts(self.native(), VecSink::new(&mut set_fn).native_mut()) }
        result
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphClusterInfo {
    pub bounds: Rect,
    pub text_range: TextRange,
    pub position: TextDirection,
}

impl GlyphClusterInfo {
    fn from_native_ref(native: &sb::skia_textlayout_Paragraph_GlyphClusterInfo) -> Self {
        unsafe {
            Self {
                bounds: *Rect::from_native_ptr(&native.fBounds),
                text_range: TextRange {
                    start: native.fClusterTextRange.start,
                    end: native.fClusterTextRange.end,
                },
                position: native.fGlyphClusterPosition,
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FontInfo {
    pub font: Font,
    pub text_range: TextRange,
}

impl FontInfo {
    pub fn new(font: Font, text_range: TextRange) -> Self {
        Self { font, text_range }
    }

    fn from_native_ref(native: &sb::skia_textlayout_Paragraph_FontInfo) -> Self {
        Self {
            font: Font::from_native_ref(&native.fFont).clone(),
            text_range: TextRange {
                start: native.fTextRange.start,
                end: native.fTextRange.end,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Paragraph;
    use crate::{
        icu,
        textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
        FontMgr,
    };

    #[test]
    #[serial_test::serial]
    fn test_line_metrics() {
        let paragraph = mk_lorem_ipsum_paragraph();
        let line_metrics = paragraph.get_line_metrics();
        for (line, lm) in line_metrics.iter().enumerate() {
            println!("line {}: width: {}", line + 1, lm.width)
        }
    }

    /// Regression test for <https://github.com/rust-skia/rust-skia/issues/585>
    #[test]
    #[serial_test::serial]
    fn test_style_metrics() {
        icu::init();

        let mut style = ParagraphStyle::new();
        let ts = TextStyle::new();
        style.set_text_style(&ts);
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::default(), None);
        let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
        paragraph_builder.add_text("Lorem ipsum dolor sit amet\n");
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(100.0);

        let line_metrics = &paragraph.get_line_metrics()[0];
        line_metrics.get_style_metrics(line_metrics.start_index..line_metrics.end_index);
    }

    #[test]
    #[serial_test::serial]
    fn test_font_infos() {
        let paragraph = mk_lorem_ipsum_paragraph();
        let infos = paragraph.get_fonts();
        assert!(!infos.is_empty())
    }

    fn mk_lorem_ipsum_paragraph() -> Paragraph {
        icu::init();

        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::new(), None);
        let paragraph_style = ParagraphStyle::new();
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
        let ts = TextStyle::new();
        paragraph_builder.push_style(&ts);
        paragraph_builder.add_text(LOREM_IPSUM);
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(256.0);

        return paragraph;

        static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur at leo at nulla tincidunt placerat. Proin eget purus augue. Quisque et est ullamcorper, pellentesque felis nec, pulvinar massa. Aliquam imperdiet, nulla ut dictum euismod, purus dui pulvinar risus, eu suscipit elit neque ac est. Nullam eleifend justo quis placerat ultricies. Vestibulum ut elementum velit. Praesent et dolor sit amet purus bibendum mattis. Aliquam erat volutpat.";
    }
}
