use std::mem;
use crate::prelude::*;
use skia_bindings::{
    SkFontStyle_Width,
    SkFontStyle_Weight,
    SkFontStyle_Slant,
    SkFontStyle,
    C_SkFontStyle_Construct,
    C_SkFontStyle_equals
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

#[test]
fn test_font_style_width_layout() {
    FontStyleWidth::test_layout()
}

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

pub type FontStyleSlant = EnumHandle<SkFontStyle_Slant>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkFontStyle_Slant> {
    pub const Upright: Self = Self(SkFontStyle_Slant::kUpright_Slant);
    pub const Italic: Self = Self(SkFontStyle_Slant::kItalic_Slant);
    pub const Oblique: Self = Self(SkFontStyle_Slant::kOblique_Slant);
}

pub type FontStyle = ValueHandle<SkFontStyle>;

impl NativePartialEq for SkFontStyle {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkFontStyle_equals(self, rhs) }
    }
}

impl Default for ValueHandle<SkFontStyle> {
    fn default() -> Self {
        // does not link under Linux:
        // unsafe { SkFontStyle::new1() }.into_handle()
        FontStyle::from_native(unsafe {
            let mut font_style = mem::uninitialized();
            C_SkFontStyle_Construct(&mut font_style);
            font_style
        })

    }
}

impl ValueHandle<SkFontStyle> {

    pub fn new(weight: FontStyleWeight, width: FontStyleWidth, slant: FontStyleSlant) -> Self {
        unsafe {
            SkFontStyle::new(*weight.native(), *width.native(), *slant.native())
        }.into_handle()
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
        *FontStyle_Normal
    }

    pub fn bold() -> FontStyle {
        *FontStyle_Bold
    }

    pub fn italic() -> FontStyle {
        *FontStyle_Italic
    }

    pub fn bold_italic() -> FontStyle {
        *FontStyle_BoldItalic
    }
}

lazy_static! {
    static ref FontStyle_Normal : FontStyle = FontStyle::new(FontStyleWeight::Normal, FontStyleWidth::Normal, FontStyleSlant::Upright);
    static ref FontStyle_Bold : FontStyle = FontStyle::new(FontStyleWeight::Bold, FontStyleWidth::Normal, FontStyleSlant::Upright);
    static ref FontStyle_Italic : FontStyle = FontStyle::new(FontStyleWeight::Normal, FontStyleWidth::Normal, FontStyleSlant::Italic);
    static ref FontStyle_BoldItalic : FontStyle = FontStyle::new(FontStyleWeight::Bold , FontStyleWidth::Normal, FontStyleSlant::Italic);
}


#[test]
fn test_equality() {
    let style : FontStyle = Default::default();
    let style2 : FontStyle = Default::default();
    assert!(style == style2);
}
