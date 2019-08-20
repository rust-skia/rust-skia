use super::{FontFamilies, TextBaseline, TextShadow};
use crate::interop::{AsStr, FromStrs, SetStr};
use crate::prelude::*;
use crate::{interop, scalar, Color, FontMetrics, FontStyle, Paint, Typeface};
use skia_bindings as sb;
use std::slice;

bitflags! {
    pub struct TextDecoration: u32 {
        const NO_DECORATION = sb::skia_textlayout_TextDecoration::kNoDecoration as _;
        const UNDERLINE = sb::skia_textlayout_TextDecoration::kUnderline as _;
        const OVERLINE = sb::skia_textlayout_TextDecoration::kOverline as _;
        const LINE_THROUGH = sb::skia_textlayout_TextDecoration::kOverline as _;
    }
}

pub const ALL_TEXT_DECORATIONS: TextDecoration = TextDecoration::ALL;

impl Default for TextDecoration {
    fn default() -> Self {
        TextDecoration::NO_DECORATION
    }
}

impl TextDecoration {
    pub const ALL: TextDecoration = TextDecoration::all();
}

pub use sb::skia_textlayout_TextDecorationStyle as TextDecorationStyle;

pub use sb::skia_textlayout_StyleType as StyleType;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Decoration {
    pub ty: TextDecoration,
    pub color: Color,
    pub style: TextDecorationStyle,
    pub thickness_multiplier: scalar,
}

impl NativeTransmutable<sb::skia_textlayout_Decoration> for Decoration {}

#[test]
fn decoration_layout() {
    Decoration::test_layout();
}

pub type TextStyle = Handle<sb::skia_textlayout_TextStyle>;

impl NativeDrop for sb::skia_textlayout_TextStyle {
    fn drop(&mut self) {
        unsafe { sb::C_TextStyle_destruct(self) }
    }
}

impl NativeClone for sb::skia_textlayout_TextStyle {
    fn clone(&self) -> Self {
        construct(|ts| unsafe { sb::C_TextStyle_CopyConstruct(ts, self) })
    }
}

impl NativePartialEq for sb::skia_textlayout_TextStyle {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { self.equals(rhs) }
    }
}

impl Default for Handle<sb::skia_textlayout_TextStyle> {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle<sb::skia_textlayout_TextStyle> {
    pub fn new() -> Self {
        TextStyle::from_native(unsafe { sb::skia_textlayout_TextStyle::new() })
    }

    pub fn equals(&self, other: &TextStyle) -> bool {
        *self == *other
    }

    pub fn match_one_attribute(&self, style_type: StyleType, other: &TextStyle) -> bool {
        unsafe { self.native().matchOneAttribute(style_type, other.native()) }
    }

    pub fn color(&self) -> Color {
        Color::from_native(self.native().fColor)
    }

    pub fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.native_mut().fColor = color.into().into_native();
        self
    }

    pub fn foreground(&self) -> Option<&Paint> {
        self.native()
            .fHasForeground
            .if_true_some(Paint::from_native_ref(&self.native().fForeground))
    }

    pub fn set_foreground_color(&mut self, paint: impl Into<Option<Paint>>) -> &mut Self {
        let n = self.native_mut();
        n.fHasForeground = paint
            .into()
            .map(|paint| n.fForeground.replace_with(paint))
            .is_some();
        self
    }

    pub fn background(&self) -> Option<&Paint> {
        self.native()
            .fHasBackground
            .if_true_some(Paint::from_native_ref(&self.native().fBackground))
    }

    pub fn set_background_color(&mut self, paint: impl Into<Option<Paint>>) -> &mut Self {
        let n = self.native_mut();
        n.fHasBackground = paint
            .into()
            .map(|paint| n.fBackground.replace_with(paint))
            .is_some();
        self
    }

    pub fn decoration(&self) -> &Decoration {
        Decoration::from_native_ref(&self.native().fDecoration)
    }

    pub fn decoration_mut(&mut self) -> &mut Decoration {
        Decoration::from_native_ref_mut(&mut self.native_mut().fDecoration)
    }

    pub fn font_style(&self) -> FontStyle {
        FontStyle::from_native(self.native().fFontStyle)
    }

    pub fn set_font_style(&mut self, font_style: FontStyle) -> &mut Self {
        self.native_mut().fFontStyle = font_style.into_native();
        self
    }

    pub fn shadows(&self) -> &[TextShadow] {
        unsafe {
            let ts: &sb::TextShadows = transmute_ref(&self.native().fTextShadows);
            let mut cnt = 0;
            let ptr = TextShadow::from_native_ref(&*sb::C_TextShadows_ptr_count(ts, &mut cnt));
            slice::from_raw_parts(ptr, cnt)
        }
    }

    pub fn add_shadow(&mut self, shadow: TextShadow) -> &mut Self {
        unsafe { sb::C_TextStyle_addShadow(self.native_mut(), shadow.native()) }
        self
    }

    pub fn reset_shadows(&mut self) -> &mut Self {
        unsafe { sb::C_TextStyle_resetShadows(self.native_mut()) }
        self
    }

    pub fn font_families(&self) -> FontFamilies {
        unsafe {
            let mut count = 0;
            let ptr = sb::C_TextStyle_getFontFamilies(self.native(), &mut count);
            FontFamilies(slice::from_raw_parts(ptr, count))
        }
    }

    pub fn set_font_families(&mut self, families: &[impl AsRef<str>]) -> &mut Self {
        let families: Vec<interop::String> = FromStrs::from_strs(families);
        let families = families.native();
        unsafe {
            sb::C_TextStyle_setFontFamilies(self.native_mut(), families.as_ptr(), families.len())
        }
        self
    }

    pub fn set_height(&mut self, height: scalar) -> &mut Self {
        self.native_mut().fHeight = height;
        self
    }

    pub fn height(&self) -> scalar {
        self.native().fHeight
    }

    pub fn set_letter_spacing(&mut self, letter_spacing: scalar) -> &mut Self {
        self.native_mut().fLetterSpacing = letter_spacing;
        self
    }

    pub fn letter_spacing(&self) -> scalar {
        self.native().fLetterSpacing
    }

    pub fn set_word_spacing(&mut self, word_spacing: scalar) -> &mut Self {
        self.native_mut().fWordSpacing = word_spacing;
        self
    }

    pub fn word_spacing(&self) -> scalar {
        self.native().fWordSpacing
    }

    pub fn typeface(&self) -> Option<Typeface> {
        Typeface::from_unshared_ptr(self.native().fTypeface.fPtr)
    }

    pub fn set_typeface(&mut self, typeface: impl Into<Option<Typeface>>) -> &mut Self {
        unsafe {
            sb::C_TextStyle_setTypeface(self.native_mut(), typeface.into().into_ptr_or_null())
        }
        self
    }

    pub fn locale(&self) -> &str {
        self.native().fLocale.as_str()
    }

    pub fn set_locale(&mut self, locale: impl AsRef<str>) -> &mut Self {
        self.native_mut().fLocale.set_str(locale);
        self
    }

    pub fn text_baseline(&self) -> TextBaseline {
        self.native().fTextBaseline
    }

    pub fn set_text_baseline(&mut self, baseline: TextBaseline) -> &mut Self {
        self.native_mut().fTextBaseline = baseline;
        self
    }

    pub fn font_metrics(&self) -> FontMetrics {
        let mut m = FontMetrics::default();
        unsafe { sb::C_TextStyle_getFontMetrics(self.native(), m.native_mut()) }
        m
    }
}
