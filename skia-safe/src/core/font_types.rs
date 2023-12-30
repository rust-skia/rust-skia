use std::{ffi, mem};

use skia_bindings::SkTextEncoding;

use crate::{prelude::*, GlyphId};

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

/// Trait representing a text in a specific encoding.
///
/// Functions that expect `EncodedText` may be passed `String`, `&String``, `str`, or `&str`
/// representing UTF-8 encoded text. In addition to that, &[u16], [u16], or &[GlyphId], [GlyphId],
/// are interpreted as `GlyphId` slices.
///
/// To use UTF16 or UTF32 encodings, use [`skia_safe::as_utf16_unchecked`] or
/// [`skia_safe::as_utf32_unchecked`].
pub trait EncodedText {
    fn as_raw(&self) -> (*const ffi::c_void, usize, TextEncoding);
}

/// Treat a `&[u16]` as UTF16 encoded text.
pub unsafe fn as_utf16_unchecked<'a>(slice: &'a [u16]) -> impl EncodedText + Captures<&'a [u16]> {
    UTF16Slice(slice)
}

/// Treat a `&[u32]` as UTF32 encoded text.
pub unsafe fn as_utf32_unchecked<'a>(slice: &'a [u32]) -> impl EncodedText + Captures<&'a [u32]> {
    UTF32Slice(slice)
}

struct UTF16Slice<'a>(&'a [u16]);

impl EncodedText for UTF16Slice<'_> {
    fn as_raw(&self) -> (*const ffi::c_void, usize, TextEncoding) {
        let slice = self.0;
        let size = mem::size_of_val(slice);
        (slice.as_ptr() as _, size, TextEncoding::UTF16)
    }
}

struct UTF32Slice<'a>(&'a [u32]);

impl EncodedText for UTF32Slice<'_> {
    fn as_raw(&self) -> (*const ffi::c_void, usize, TextEncoding) {
        let slice = self.0;
        let size = mem::size_of_val(slice);
        (slice.as_ptr() as _, size, TextEncoding::UTF32)
    }
}

impl EncodedText for String {
    fn as_raw(&self) -> (*const ffi::c_void, usize, TextEncoding) {
        self.as_str().as_raw()
    }
}

impl EncodedText for &str {
    fn as_raw(&self) -> (*const ffi::c_void, usize, TextEncoding) {
        let bytes = self.as_bytes();
        (bytes.as_ptr() as _, bytes.len(), TextEncoding::UTF8)
    }
}

impl EncodedText for [GlyphId] {
    fn as_raw(&self) -> (*const ffi::c_void, usize, TextEncoding) {
        (
            self.as_ptr() as _,
            mem::size_of_val(self),
            TextEncoding::GlyphId,
        )
    }
}

impl<T: EncodedText> EncodedText for &T {
    fn as_raw(&self) -> (*const ffi::c_void, usize, TextEncoding) {
        (**self).as_raw()
    }
}

pub use skia_bindings::SkFontHinting as FontHinting;
variant_name!(FontHinting::Full);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_id_size() {
        let glyphs = [0u16, 1u16];
        assert_eq!(glyphs.as_raw().1, 4);
    }

    #[test]
    fn utf16_size() {
        let utf16 = unsafe { as_utf16_unchecked(&[0u16, 1u16]) };
        assert_eq!(utf16.as_raw().1, 4);
    }

    #[test]
    fn utf32_size() {
        let utf16 = unsafe { as_utf32_unchecked(&[0u32, 1u32]) };
        assert_eq!(utf16.as_raw().1, 8);
    }
}
