use std::{ffi, mem, ptr};

use skia_bindings::SkTextEncoding;

use crate::GlyphId;

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

/// Value representing a reference to text in a specific encoding.
///
/// Functions that expect encoded types accept implicit conversion through the `From` trait from
/// `&String``, `&str`, `&[u8]` (UTF8), and `&[u32]` (UTF32).
///
/// To pass a reference to a &[GlyphId] or `&[u16]` (UTF16) slice, use
/// `EncodedText::GlyphIds(slice)` or `EncodedText::UTF16(slice)`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum EncodedText<'a> {
    UTF8(&'a [u8]),
    UTF16(&'a [u16]),
    UTF32(&'a [u32]),
    GlyphId(&'a [GlyphId]),
}

impl<'a> From<&'a String> for EncodedText<'a> {
    fn from(value: &'a String) -> Self {
        value.as_bytes().into()
    }
}

impl<'a> From<&'a str> for EncodedText<'a> {
    fn from(value: &'a str) -> Self {
        value.as_bytes().into()
    }
}

impl<'a> From<&'a [u8]> for EncodedText<'a> {
    fn from(value: &'a [u8]) -> Self {
        EncodedText::UTF8(value)
    }
}

impl<'a> From<&'a [u32]> for EncodedText<'a> {
    fn from(value: &'a [u32]) -> Self {
        EncodedText::UTF32(value)
    }
}

impl EncodedText<'_> {
    pub fn encoding(self) -> TextEncoding {
        match self {
            EncodedText::UTF8(_) => TextEncoding::UTF8,
            EncodedText::UTF16(_) => TextEncoding::UTF16,
            EncodedText::UTF32(_) => TextEncoding::UTF32,
            EncodedText::GlyphId(_) => TextEncoding::GlyphId,
        }
    }

    pub(crate) fn raw(self) -> (*const ffi::c_void, usize, TextEncoding) {
        let size = self.size();
        let ptr = {
            if size == 0 {
                ptr::null()
            } else {
                match self {
                    EncodedText::UTF8(slice) => slice.as_ptr() as _,
                    EncodedText::UTF16(slice) => slice.as_ptr() as _,
                    EncodedText::UTF32(slice) => slice.as_ptr() as _,
                    EncodedText::GlyphId(slice) => slice.as_ptr() as _,
                }
            }
        };

        (ptr, size, self.encoding())
    }

    fn size(self) -> usize {
        match self {
            EncodedText::UTF8(slice) => mem::size_of_val(slice),
            EncodedText::UTF16(slice) => mem::size_of_val(slice),
            EncodedText::UTF32(slice) => mem::size_of_val(slice),
            EncodedText::GlyphId(slice) => mem::size_of_val(slice),
        }
    }
}

pub use skia_bindings::SkFontHinting as FontHinting;
variant_name!(FontHinting::Full);

#[cfg(test)]
mod tests {
    use super::*;

    /// Using `mem::size_of_val` on references may lead to subtle problems, for example a `&&[u8]`
    /// results in `16usize`` (the size of a fat pointer).
    #[test]
    fn text_sizes_match_expectations() {
        let skia = "SKIA";
        let encoded_text: EncodedText = skia.into();
        assert_eq!(encoded_text.size(), skia.len())
    }
}
