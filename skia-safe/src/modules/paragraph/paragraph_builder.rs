use super::{FontCollection, Paragraph, ParagraphStyle, PlaceholderStyle, TextStyle};
use crate::prelude::*;
use skia_bindings as sb;
use std::{fmt, os::raw};

pub type ParagraphBuilder = RefHandle<sb::skia_textlayout_ParagraphBuilder>;
unsafe_send_sync!(ParagraphBuilder);

impl NativeDrop for sb::skia_textlayout_ParagraphBuilder {
    fn drop(&mut self) {
        unsafe { sb::C_ParagraphBuilder_delete(self) }
    }
}

impl fmt::Debug for ParagraphBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParagraphBuilder").finish()
    }
}

impl ParagraphBuilder {
    pub fn push_style(&mut self, style: &TextStyle) -> &mut Self {
        unsafe { sb::C_ParagraphBuilder_pushStyle(self.native_mut(), style.native()) }
        self
    }

    pub fn pop(&mut self) -> &mut Self {
        unsafe { sb::C_ParagraphBuilder_pop(self.native_mut()) }
        self
    }

    pub fn peek_style(&mut self) -> TextStyle {
        let mut ts = TextStyle::default();
        unsafe { sb::C_ParagraphBuilder_peekStyle(self.native_mut(), ts.native_mut()) }
        ts
    }

    pub fn add_text(&mut self, str: impl AsRef<str>) -> &mut Self {
        let str = str.as_ref();
        unsafe {
            sb::C_ParagraphBuilder_addText(
                self.native_mut(),
                str.as_ptr() as *const raw::c_char,
                str.len(),
            )
        }
        self
    }

    pub fn add_placeholder(&mut self, placeholder_style: &PlaceholderStyle) -> &mut Self {
        unsafe {
            sb::C_ParagraphBuilder_addPlaceholder(self.native_mut(), placeholder_style.native())
        }
        self
    }

    pub fn set_paragraph_style(&mut self, style: &ParagraphStyle) -> &mut Self {
        unsafe { sb::C_ParagraphBuilder_setParagraphStyle(self.native_mut(), style.native()) }
        self
    }

    pub fn build(&mut self) -> Paragraph {
        Paragraph::from_ptr(unsafe { sb::C_ParagraphBuilder_Build(self.native_mut()) }).unwrap()
    }

    pub fn new(style: &ParagraphStyle, font_collection: impl Into<FontCollection>) -> Self {
        #[cfg(feature = "embed-icudtl")]
        crate::icu::init();

        Self::from_ptr(unsafe {
            sb::C_ParagraphBuilder_make(style.native(), font_collection.into().into_ptr())
        })
        .expect("Unicode initialization error")
    }
}
