#![cfg(feature = "textlayout")]
use skia_safe::{
    GlyphId, Point,
    shaper::{
        RunHandler,
        run_handler::{Buffer, RunInfo},
    },
};

#[derive(Default, Debug)]
pub struct DebugRunHandler {
    glyphs: Vec<GlyphId>,
    points: Vec<Point>,
}

impl RunHandler for DebugRunHandler {
    fn begin_line(&mut self) {
        println!("begin_line");
    }

    fn run_info(&mut self, info: &RunInfo) {
        println!("run_info: {:?} {:?}", info.advance, info.utf8_range);
    }

    fn commit_run_info(&mut self) {
        println!("commit_run_info");
    }

    fn run_buffer(&mut self, info: &RunInfo) -> Buffer<'_> {
        println!("run_buffer {}", info.glyph_count);
        let count = info.glyph_count;
        self.glyphs.resize(count, 0);
        self.points.resize(count, Point::default());
        Buffer::new(&mut self.glyphs, &mut self.points, None)
    }

    fn commit_run_buffer(&mut self, _info: &RunInfo) {
        println!("commit_run_buffer");
        println!("state: {self:?}");
    }

    fn commit_line(&mut self) {
        println!("commit_line");
    }
}

#[cfg(test)]
mod tests {
    use crate::DebugRunHandler;
    use skia_safe::{Font, Shaper, shapers};

    #[test]
    #[serial_test::serial]
    fn test_rtl_text_shaping() {
        let shaper = Shaper::new(None);
        shaper.shape(
            "العربية",
            &Font::default(),
            false,
            10000.0,
            &mut DebugRunHandler::default(),
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_skunicode_parameterized_shaper() {
        shapers::hb::shape_dont_wrap_or_reorder(None).expect("Shaper");
    }

    #[test]
    #[serial_test::serial]
    fn test_shape_with_options() {
        let shaper = Shaper::new(None);
        let text = "Hello";
        let mut font_run_iterator =
            Shaper::new_trivial_font_run_iterator(&Font::default(), text.len());
        let mut bidi_run_iterator = shapers::primitive::trivial_bidi_run_iterator(0, text.len());
        let mut script_run_iterator = shapers::primitive::trivial_script_run_iterator(0, text.len());
        let mut language_run_iterator = Shaper::new_trivial_language_run_iterator("en", text.len());
        shaper.shape_with_iterators_and_features_and_options(
            text,
            &mut font_run_iterator,
            &mut bidi_run_iterator,
            &mut script_run_iterator,
            &mut language_run_iterator,
            &[],
            10000.0,
            0.0,
            &mut DebugRunHandler::default(),
        );
    }
}
