use super::Length;
use crate::{interop::AsStr, prelude::*};
use skia_bindings as sb;
use std::fmt;

pub type FontStyle = sb::SkSVGFontStyle_Type;
pub type FontWeight = sb::SkSVGFontWeight_Type;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FontSize {
    ty: sb::SkSVGFontSize_Type,
    size: sb::SkSVGLength,
}

native_transmutable!(sb::SkSVGFontSize, FontSize, svg_font_size_layout);

impl fmt::Debug for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgFontSize")
            .field("size", &self.size())
            .finish()
    }
}

impl FontSize {
    pub fn size(&self) -> Option<&Length> {
        if self.ty == sb::SkSVGFontSize_Type::Length {
            Some(Length::from_native_ref(&self.size))
        } else {
            None
        }
    }

    pub fn inherit() -> Self {
        Self {
            ty: sb::SkSVGFontSize_Type::Inherit,
            size: sb::SkSVGLength {
                fValue: 0.0,
                fUnit: sb::SkSVGLength_Unit::Unknown,
            },
        }
    }

    pub fn new(size: Length) -> Self {
        Self {
            ty: sb::SkSVGFontSize_Type::Length,
            size: size.into_native(),
        }
    }
}

#[repr(C)]
pub struct FontFamily {
    ty: sb::SkSVGFontFamily_Type,
    family: sb::SkString,
}

impl fmt::Debug for FontFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgFontFamily")
            .field("family", &self.family())
            .finish()
    }
}

impl FontFamily {
    pub fn family(&self) -> Option<&str> {
        if self.ty == sb::SkSVGFontFamily_Type::Family {
            Some(self.family.as_str())
        } else {
            None
        }
    }

    pub fn inherit() -> Self {
        Self {
            ty: sb::SkSVGFontFamily_Type::Inherit,
            family: crate::interop::String::default().into_native(),
        }
    }

    pub fn new<T: AsRef<str>>(family: T) -> Self {
        Self {
            ty: sb::SkSVGFontFamily_Type::Family,
            family: crate::interop::String::from_str(family.as_ref()).into_native(),
        }
    }
}

native_transmutable!(sb::SkSVGFontFamily, FontFamily, svg_font_family_layout);
