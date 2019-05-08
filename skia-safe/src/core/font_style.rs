use std::mem;
use crate::prelude::*;
use skia_bindings::{
    SkFontStyle_Width,
    SkFontStyle_Weight,
    SkFontStyle_Slant,
    SkFontStyle,
    C_SkFontStyle_Construct,
    C_SkFontStyle_Equals
};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct FontStyleWeight(i32);

impl NativeTransmutable<i32> for FontStyleWeight {}

#[test]
fn test_font_style_weight_layout() {
    FontStyleWeight::test_layout()
}

#[allow(non_upper_case_globals)]
impl FontStyleWeight {
    pub const Invisible: Self = Self(SkFontStyle_Weight::kInvisible_Weight as _);
    pub const Thin: Self = Self(SkFontStyle_Weight::kThin_Weight as _);
    pub const ExtraLight: Self = Self(SkFontStyle_Weight::kExtraLight_Weight as _);
    pub const Light: Self = Self(SkFontStyle_Weight::kLight_Weight as _);
    pub const Normal: Self = Self(SkFontStyle_Weight::kNormal_Weight as _);
    pub const Medium: Self = Self(SkFontStyle_Weight::kMedium_Weight as _);
    pub const SemiBold: Self = Self(SkFontStyle_Weight::kSemiBold_Weight as _);
    pub const Bold: Self = Self(SkFontStyle_Weight::kBold_Weight as _);
    pub const ExtraBold: Self = Self(SkFontStyle_Weight::kExtraBold_Weight as _);
    pub const Black: Self = Self(SkFontStyle_Weight::kBlack_Weight as _);
    pub const ExtraBlack: Self = Self(SkFontStyle_Weight::kExtraBlack_Weight as _);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct FontStyleWidth(i32);

impl NativeTransmutable<i32> for FontStyleWidth {}
#[test] fn test_font_style_width_layout() { FontStyleWidth::test_layout() }

#[allow(non_upper_case_globals)]
impl FontStyleWidth {
    pub const UltraCondensed: Self = Self(SkFontStyle_Width::kUltraCondensed_Width as _);
    pub const ExtraCondensed: Self = Self(SkFontStyle_Width::kExtraCondensed_Width as _);
    pub const Condensed: Self = Self(SkFontStyle_Width::kCondensed_Width as _);
    pub const SemiCondensed: Self = Self(SkFontStyle_Width::kSemiCondensed_Width as _);
    pub const Normal: Self = Self(SkFontStyle_Width::kNormal_Width as _);
    pub const SemiExpanded: Self = Self(SkFontStyle_Width::kSemiExpanded_Width as _);
    pub const Expanded: Self = Self(SkFontStyle_Width::kExpanded_Width as _);
    pub const ExtraExpanded: Self = Self(SkFontStyle_Width::kExtraExpanded_Width as _);
    pub const UltraExpanded: Self = Self(SkFontStyle_Width::kUltraExpanded_Width as _);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FontStyleSlant {
    Upright = SkFontStyle_Slant::kUpright_Slant as _,
    Italic = SkFontStyle_Slant::kItalic_Slant as _,
    Oblique = SkFontStyle_Slant::kOblique_Slant as _
}

impl NativeTransmutable<SkFontStyle_Slant> for FontStyleSlant {}
#[test] fn test_font_style_slant_layout() { FontStyleSlant::test_layout() }

// TODO: implement Display
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FontStyle(SkFontStyle);

impl NativeTransmutable<SkFontStyle> for FontStyle {}
#[test] fn test_font_style_layout() { FontStyle::test_layout() }

impl PartialEq for FontStyle {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkFontStyle_Equals(self.native(), rhs.native()) }
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        // does not link under Linux:
        // unsafe { SkFontStyle::new1() }
        FontStyle::from_native(unsafe {
            let mut font_style = mem::uninitialized();
            C_SkFontStyle_Construct(&mut font_style);
            font_style
        })
    }
}

impl FontStyle {

    pub fn new(weight: FontStyleWeight, width: FontStyleWidth, slant: FontStyleSlant) -> Self {
        Self::from_native(unsafe {
            SkFontStyle::new(*weight.native(), *width.native(), *slant.native())
        })
    }

    pub fn weight(self) -> FontStyleWeight {
        FontStyleWeight::from_native(unsafe { self.native().weight() })
    }

    pub fn width(self) -> FontStyleWidth {
        FontStyleWidth::from_native(unsafe { self.native().width() })
    }

    pub fn slant(self) -> FontStyleSlant {
        FontStyleSlant::from_native(unsafe { self.native().slant() })
    }

    pub fn normal() -> FontStyle {
        *font_style_static::NORMAL
    }

    pub fn bold() -> FontStyle {
        *font_style_static::BOLD
    }

    pub fn italic() -> FontStyle {
        *font_style_static::ITALIC
    }

    pub fn bold_italic() -> FontStyle {
        *font_style_static::BOLD_ITALIC
    }
}

mod font_style_static {
    use super::{FontStyle, FontStyleWeight, FontStyleWidth, FontStyleSlant};

    lazy_static! {
        pub static ref NORMAL: FontStyle = FontStyle::new(FontStyleWeight::Normal, FontStyleWidth::Normal, FontStyleSlant::Upright);
        pub static ref BOLD: FontStyle = FontStyle::new(FontStyleWeight::Bold, FontStyleWidth::Normal, FontStyleSlant::Upright);
        pub static ref ITALIC: FontStyle = FontStyle::new(FontStyleWeight::Normal, FontStyleWidth::Normal, FontStyleSlant::Italic);
        pub static ref BOLD_ITALIC: FontStyle = FontStyle::new(FontStyleWeight::Bold , FontStyleWidth::Normal, FontStyleSlant::Italic);
    }
}

#[test]
fn test_equality() {
    let style : FontStyle = Default::default();
    let style2 : FontStyle = Default::default();
    assert!(style == style2);
}
