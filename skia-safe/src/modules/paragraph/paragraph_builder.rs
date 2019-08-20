use super::{FontCollection, Paragraph, ParagraphStyle, TextStyle};
use crate::prelude::*;
use skia_bindings as sb;
use std::ffi;

pub type ParagraphBuilder = RefHandle<sb::skia_textlayout_ParagraphBuilder>;

impl NativeDrop for sb::skia_textlayout_ParagraphBuilder {
    fn drop(&mut self) {
        unsafe { sb::C_ParagraphBuilder_delete(self) }
    }
}

impl RefHandle<sb::skia_textlayout_ParagraphBuilder> {
    pub fn push_style(&mut self, style: &TextStyle) {
        unsafe { sb::C_ParagraphBuilder_pushStyle(self.native_mut(), style.native()) }
    }

    pub fn pop(&mut self) {
        unsafe { sb::C_ParagraphBuilder_pop(self.native_mut()) }
    }

    pub fn peek_style(&mut self) -> TextStyle {
        let mut ts = TextStyle::default();
        unsafe { sb::C_ParagraphBuilder_peekStyle(self.native_mut(), ts.native_mut()) }
        ts
    }

    pub fn add_text(&mut self, str: impl AsRef<str>) {
        let cstr = ffi::CString::new(str.as_ref()).unwrap();
        unsafe { sb::C_ParagraphBuilder_addText(self.native_mut(), cstr.as_ptr()) }
    }

    pub fn set_paragraph_style(&mut self, style: &ParagraphStyle) {
        unsafe { sb::C_ParagraphBuilder_setParagraphStyle(self.native_mut(), style.native()) }
    }

    pub fn build(&mut self) -> Paragraph {
        Paragraph::from_ptr(unsafe { sb::C_ParagraphBuilder_Build(self.native_mut()) }).unwrap()
    }

    pub fn new(style: &ParagraphStyle, font_collection: FontCollection) -> Self {
        Self::from_ptr(unsafe {
            sb::C_ParagraphBuilder_make(style.native(), font_collection.into_ptr())
        })
        .unwrap()
    }
}
