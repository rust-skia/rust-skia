use crate::prelude::*;
use skia_bindings::{
    SkTextEncoding,
    SkFontHinting
};

pub type TextEncoding = EnumHandle<SkTextEncoding>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkTextEncoding> {
    pub const UTF8: Self = Self(SkTextEncoding::kUTF8);
    pub const UTF16: Self = Self(SkTextEncoding::kUTF16);
    pub const UTF32: Self = Self(SkTextEncoding::kUTF32);
    pub const GlyphId: Self = Self(SkTextEncoding::kGlyphID);
}

pub type FontHinting = EnumHandle<SkFontHinting>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkFontHinting> {
    pub const None: Self = Self(SkFontHinting::kNone);
    pub const Slight: Self = Self(SkFontHinting::kSlight);
    pub const Normal: Self = Self(SkFontHinting::kNormal);
    pub const Full: Self = Self(SkFontHinting::kFull);
}
