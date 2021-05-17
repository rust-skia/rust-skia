use crate::prelude::*;
use skia_bindings::SkTextEncoding;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(i32)]
pub enum TextEncoding {
    UTF8 = SkTextEncoding::kUTF8 as _,
    UTF16 = SkTextEncoding::kUTF16 as _,
    UTF32 = SkTextEncoding::kUTF32 as _,
    // TODO: enum rewriter: ID -> Id
    GlyphId = SkTextEncoding::kGlyphID as _,
}

impl NativeTransmutable<SkTextEncoding> for TextEncoding {}
#[test]
fn test_text_encoding_layout() {
    TextEncoding::test_layout()
}

impl Default for TextEncoding {
    fn default() -> Self {
        TextEncoding::UTF8
    }
}

pub use skia_bindings::SkFontHinting as FontHinting;
#[test]
fn test_font_hinting_naming() {
    let _ = FontHinting::Full;
}
