use skia_bindings::SkTextEncoding;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[repr(i32)]
pub enum TextEncoding {
    #[default]
    UTF8 = SkTextEncoding::kUTF8 as _,
    UTF16 = SkTextEncoding::kUTF16 as _,
    UTF32 = SkTextEncoding::kUTF32 as _,
    // TODO: enum rewriter: ID -> Id
    GlyphId = SkTextEncoding::kGlyphID as _,
}

native_transmutable!(SkTextEncoding, TextEncoding, text_encoding_layout);

pub use skia_bindings::SkFontHinting as FontHinting;
variant_name!(FontHinting::Full);
