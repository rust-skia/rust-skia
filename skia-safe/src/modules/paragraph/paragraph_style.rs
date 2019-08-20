use super::{FontFamilies, TextAlign, TextDirection, TextStyle};
use crate::interop::{AsStr, FromStrs, SetStr};
use crate::prelude::*;
use crate::{interop, scalar, FontStyle};
use skia_bindings as sb;
use std::slice;

pub type StrutStyle = Handle<sb::skia_textlayout_StrutStyle>;

impl NativeDrop for sb::skia_textlayout_StrutStyle {
    fn drop(&mut self) {
        unsafe { sb::C_StrutStyle_destruct(self) }
    }
}

impl NativeClone for sb::skia_textlayout_StrutStyle {
    fn clone(&self) -> Self {
        construct(|ss| unsafe { sb::C_StrutStyle_CopyConstruct(ss, self) })
    }
}

impl Default for Handle<sb::skia_textlayout_StrutStyle> {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle<sb::skia_textlayout_StrutStyle> {
    pub fn new() -> Self {
        StrutStyle::from_native(unsafe { sb::skia_textlayout_StrutStyle::new() })
    }

    pub fn font_families(&self) -> FontFamilies {
        unsafe {
            let mut count = 0;
            let ptr = sb::C_StrutStyle_getFontFamilies(self.native(), &mut count);
            FontFamilies(slice::from_raw_parts(ptr, count))
        }
    }

    pub fn set_font_families(&mut self, families: &[impl AsRef<str>]) {
        let families = interop::Strings::from_strs(families);
        let families = families.native();
        unsafe {
            sb::C_StrutStyle_setFontFamilies(self.native_mut(), families.as_ptr(), families.len());
        }
    }

    pub fn font_style(&self) -> FontStyle {
        FontStyle::from_native(self.native().fFontStyle)
    }

    pub fn set_font_style(&mut self, font_style: FontStyle) {
        self.native_mut().fFontStyle = font_style.into_native()
    }

    pub fn font_size(&self) -> scalar {
        self.native().fFontSize
    }

    pub fn set_font_size(&mut self, font_size: scalar) {
        self.native_mut().fFontSize = font_size;
    }

    pub fn set_height(&mut self, height: scalar) {
        self.native_mut().fHeight = height;
    }

    pub fn height(&self) -> scalar {
        self.native().fHeight
    }

    pub fn set_leading(&mut self, leading: scalar) {
        self.native_mut().fLeading = leading;
    }

    pub fn leading(&self) -> scalar {
        self.native().fLeading
    }

    pub fn strut_enabled(&self) -> bool {
        self.native().fStrutEnabled
    }

    pub fn set_strut_enabled(&mut self, strut_enabled: bool) {
        self.native_mut().fStrutEnabled = strut_enabled;
    }

    pub fn force_strut_height(&self) -> bool {
        self.native().fForceStrutHeight
    }

    pub fn set_force_strut_height(&mut self, force_strut_height: bool) {
        self.native_mut().fForceStrutHeight = force_strut_height;
    }
}

pub type ParagraphStyle = Handle<sb::skia_textlayout_ParagraphStyle>;

impl NativeDrop for sb::skia_textlayout_ParagraphStyle {
    fn drop(&mut self) {
        unsafe { sb::C_ParagraphStyle_destruct(self) }
    }
}

impl NativeClone for sb::skia_textlayout_ParagraphStyle {
    fn clone(&self) -> Self {
        construct(|ps| unsafe { sb::C_ParagraphStyle_CopyConstruct(ps, self) })
    }
}

impl NativePartialEq for sb::skia_textlayout_ParagraphStyle {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_ParagraphStyle_Equals(self, rhs) }
    }
}

impl Default for Handle<sb::skia_textlayout_ParagraphStyle> {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle<sb::skia_textlayout_ParagraphStyle> {
    pub fn new() -> Self {
        Self::from_native(unsafe { sb::skia_textlayout_ParagraphStyle::new() })
    }

    pub fn strut_style(&self) -> &StrutStyle {
        StrutStyle::from_native_ref(&self.native().fStrutStyle)
    }

    pub fn set_strut_style(&mut self, strut_style: StrutStyle) {
        self.native_mut().fStrutStyle.replace_with(strut_style);
    }

    pub fn text_style(&self) -> &TextStyle {
        TextStyle::from_native_ref(&self.native().fDefaultTextStyle)
    }

    pub fn set_text_style(&mut self, text_style: &TextStyle) {
        // TODO: implement the assignment operator in C.
        self.native_mut()
            .fDefaultTextStyle
            .replace_with(text_style.clone());
    }

    pub fn text_direction(&self) -> TextDirection {
        self.native().fTextDirection
    }

    pub fn set_text_direction(&mut self, direction: TextDirection) {
        self.native_mut().fTextDirection = direction;
    }

    pub fn text_align(&self) -> TextAlign {
        self.native().fTextAlign
    }

    pub fn set_text_align(&mut self, align: TextAlign) {
        self.native_mut().fTextAlign = align
    }

    pub fn max_lines(&self) -> Option<usize> {
        match self.native().fLinesLimit {
            std::usize::MAX => None,
            lines => Some(lines),
        }
    }

    pub fn set_max_lines(&mut self, lines: impl Into<Option<usize>>) {
        self.native_mut().fLinesLimit = lines.into().unwrap_or(usize::max_value())
    }

    pub fn ellipsis(&self) -> &str {
        self.native().fEllipsis.as_str()
    }

    pub fn set_ellipsis(&mut self, ellipsis: impl AsRef<str>) {
        self.native_mut().fEllipsis.set_str(ellipsis)
    }

    pub fn height(&self) -> scalar {
        self.native().fHeight
    }

    pub fn set_height(&mut self, height: scalar) {
        self.native_mut().fHeight = height;
    }

    pub fn unlimited_lines(&self) -> bool {
        self.max_lines().is_none()
    }

    pub fn ellipsized(&self) -> bool {
        !self.native().fEllipsis.as_str().is_empty()
    }

    pub fn effective_align(&self) -> TextAlign {
        unsafe { self.native().effective_align() }
    }

    pub fn hinting_is_on(&self) -> bool {
        self.native().fHintingIsOn
    }

    pub fn turn_hinting_off(&mut self) {
        self.native_mut().fHintingIsOn = false
    }
}
