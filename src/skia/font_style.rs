use crate::prelude::*;
use rust_skia::{
    SkFontStyle_Width,
    SkFontStyle_Weight,
    SkFontStyle_Slant,
    SkFontStyle,
    C_SkFontStyle_equals
};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct FontStyleWeight(i32);

impl FromNative<i32> for FontStyleWeight {
    fn from_native(native: i32) -> Self {
        FontStyleWeight(native)
    }
}

impl NativeAccessValue<i32> for FontStyleWeight {
    fn  native(&self) -> i32 {
        self.0
    }
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

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct FontStyleWidth(i32);

impl FromNative<i32> for FontStyleWidth {
    fn from_native(native: i32) -> Self {
        FontStyleWidth(native)
    }
}

impl NativeAccessValue<i32> for FontStyleWidth {
    fn  native(&self) -> i32 {
        self.0
    }
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
        unsafe { SkFontStyle::new1() }.into_handle()
    }
}

impl ValueHandle<SkFontStyle> {
    pub fn new(weight: FontStyleWeight, width: FontStyleWidth, slant: FontStyleSlant) -> Self {
        unsafe { SkFontStyle::new(weight.native() as _, width.native() as _, slant.native()) }
            .into_handle()
    }

    pub fn weight(&self) -> FontStyleWeight {
        unsafe { self.native().weight() }.into_handle()
    }

    pub fn width(&self) -> FontStyleWidth {
        unsafe { self.native().width() }.into_handle()
    }

    pub fn slant(&self) -> FontStyleSlant {
        unsafe { self.native().slant() }.into_handle()
    }

    pub fn Normal() -> FontStyle {
        *FontStyle_Normal
    }

    pub fn Bold() -> FontStyle {
        *FontStyle_Bold
    }

    pub fn Italic() -> FontStyle {
        *FontStyle_Italic
    }

    pub fn BoldItalic() -> FontStyle {
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
