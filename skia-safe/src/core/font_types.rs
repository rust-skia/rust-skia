use crate::prelude::*;
use skia_bindings::{SkFontHinting, SkTextEncoding};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum TextEncoding {
    UTF8 = SkTextEncoding::kUTF8 as _,
    UTF16 = SkTextEncoding::kUTF16 as _,
    UTF32 = SkTextEncoding::kUTF32 as _,
    GlyphId = SkTextEncoding::kGlyphID as _,
}

impl NativeTransmutable<SkTextEncoding> for TextEncoding {}
#[test]
fn test_text_encoding_layout() {
    TextEncoding::test_layout()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FontHinting {
    None = SkFontHinting::kNone as _,
    Slight = SkFontHinting::kSlight as _,
    Normal = SkFontHinting::kNormal as _,
    Full = SkFontHinting::kFull as _,
}

impl NativeTransmutable<SkFontHinting> for FontHinting {}
#[test]
fn test_font_hinting_layout() {
    FontHinting::test_layout();
}
