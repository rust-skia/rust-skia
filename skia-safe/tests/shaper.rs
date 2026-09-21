#![cfg(feature = "textlayout")]
use skia_safe::{
    GlyphId, Point, Vector,
    shaper::{
        RunHandler,
        run_handler::{Buffer, RunInfo},
    },
};

#[derive(Default, Debug)]
pub struct DebugRunHandler {
    glyphs: Vec<GlyphId>,
    points: Vec<Point>,
    advance: Vector,
}

impl RunHandler for DebugRunHandler {
    fn begin_line(&mut self) {
        println!("begin_line");
    }

    fn run_info(&mut self, info: &RunInfo) {
        println!("run_info: {:?} {:?}", info.advance, info.utf8_range);
        self.advance += info.advance;
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
    use skia_safe::{Font, Shaper, scalar, shapers};

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
        let text = "Hello";
        let untracked = shape(text, 0.0);
        let tracked = shape(text, 1.0);

        assert!(!tracked.glyphs.is_empty());
        assert_eq!(untracked.glyphs.len(), tracked.glyphs.len());
        assert!(tracked.advance.x > untracked.advance.x);
    }

    /// Shapes `text` with the `SkShaper::Options` based API and returns the handler that collected
    /// the runs.
    fn shape(text: &str, tracking: scalar) -> DebugRunHandler {
        let shaper = Shaper::new(None);
        let mut font_run_iterator =
            Shaper::new_trivial_font_run_iterator(&Font::default(), text.len());
        let mut bidi_run_iterator = shapers::primitive::trivial_bidi_run_iterator(0, text.len());
        let mut script_run_iterator =
            shapers::primitive::trivial_script_run_iterator(0, text.len());
        let mut language_run_iterator = Shaper::new_trivial_language_run_iterator("en", text.len());
        let mut run_handler = DebugRunHandler::default();
        shaper.shape_with_iterators_and_features_and_options(
            text,
            &mut font_run_iterator,
            &mut bidi_run_iterator,
            &mut script_run_iterator,
            &mut language_run_iterator,
            &[],
            10000.0,
            tracking,
            &mut run_handler,
        );
        run_handler
    }
}
