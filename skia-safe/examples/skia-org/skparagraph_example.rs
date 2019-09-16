use crate::artifact::DrawingDriver;
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use skia_safe::{icu, Canvas, FontMgr, Paint, Point};
use std::path;

pub fn draw<Driver: DrawingDriver>(path: &path::Path) {
    let path = path.join("SkParagraph-Example");

    icu::init();

    Driver::draw_image_256(&path, "lorem-ipsum", draw_lorem_ipsum);
}

fn draw_lorem_ipsum(canvas: &mut Canvas) {
    for _i in 0..1 {
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::new(), None);

        // As of Skia: 6d1c0d4196f19537cc64f74bacc7d123de3be454
        // 1000 runs of this function can be used to reliably reproduce a crash on macOS with a segmentation
        // fault if font fallback is not disabled in the font collection.
        // For more information: https://github.com/pragmatrix/rust-skia/pull/2#issuecomment-531819718
        font_collection.disable_font_fallback();

        let paragraph_style = ParagraphStyle::new();
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
        let mut ts = TextStyle::new();
        ts.set_foreground_color(Paint::default());
        paragraph_builder.push_style(&ts);
        paragraph_builder.add_text(LOREM_IPSUM);
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(256.0);
        paragraph.paint(canvas, Point::default());
    }
}

static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur at leo at nulla tincidunt placerat. Proin eget purus augue. Quisque et est ullamcorper, pellentesque felis nec, pulvinar massa. Aliquam imperdiet, nulla ut dictum euismod, purus dui pulvinar risus, eu suscipit elit neque ac est. Nullam eleifend justo quis placerat ultricies. Vestibulum ut elementum velit. Praesent et dolor sit amet purus bibendum mattis. Aliquam erat volutpat.";
