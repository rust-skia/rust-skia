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

native_transmutable!(SkTextEncoding, TextEncoding, text_encoding_layout);

impl Default for TextEncoding {
    fn default() -> Self {
        TextEncoding::UTF8
    }
}

pub use skia_bindings::SkFontHinting as FontHinting;
variant_name!(FontHinting::Full, font_hinting_naming);
