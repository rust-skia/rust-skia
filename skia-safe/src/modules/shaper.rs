//! TODO: Implement iterator traits if possible.
use crate::prelude::*;
use crate::{Font, FontMgr, FourByteTag};
use skia_bindings::{
    C_SkShaper_BiDiRunIterator_currentLevel, C_SkShaper_FontRunIterator_currentFont,
    C_SkShaper_LanguageRunIterator_currentLanguage, C_SkShaper_Make,
    C_SkShaper_MakeFontMgrRunIterator, C_SkShaper_MakeHbIcuScriptRunIterator,
    C_SkShaper_MakeIcuBidiRunIterator, C_SkShaper_MakePrimitive, C_SkShaper_MakeShapeThenWrap,
    C_SkShaper_MakeShaperDrivenWrapper, C_SkShaper_MakeStdLanguageRunIterator,
    C_SkShaper_RunIterator_atEnd, C_SkShaper_RunIterator_consume, C_SkShaper_RunIterator_delete,
    C_SkShaper_RunIterator_endOfCurrentRun, C_SkShaper_ScriptRunIterator_currentScript,
    C_SkShaper_delete, SkShaper, SkShaper_BiDiRunIterator, SkShaper_FontRunIterator,
    SkShaper_LanguageRunIterator, SkShaper_RunIterator, SkShaper_ScriptRunIterator,
};
use std::ffi::CStr;

pub struct Shaper(*mut SkShaper);

impl NativeAccess<SkShaper> for Shaper {
    fn native(&self) -> &SkShaper {
        unsafe { &*self.0 }
    }

    fn native_mut(&mut self) -> &mut SkShaper {
        unsafe { &mut *self.0 }
    }
}

impl Drop for Shaper {
    fn drop(&mut self) {
        unsafe { C_SkShaper_delete(self.0) }
    }
}

impl Shaper {
    pub fn new_primitive() -> Self {
        unsafe { C_SkShaper_MakePrimitive() }
            .to_option()
            .map(Shaper)
            .unwrap()
    }

    pub fn new_shaper_driven_wrapper() -> Option<Self> {
        unsafe { C_SkShaper_MakeShaperDrivenWrapper() }
            .to_option()
            .map(Shaper)
    }

    pub fn new_shape_then_wrap() -> Option<Self> {
        unsafe { C_SkShaper_MakeShapeThenWrap() }
            .to_option()
            .map(Shaper)
    }

    pub fn new() -> Self {
        unsafe { C_SkShaper_Make() }
            .to_option()
            .map(Shaper)
            .unwrap()
    }
}

pub trait RunIteratorNativeAccess {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator;
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator;
}

pub trait RunIterator {
    fn consume(&mut self);
    fn end_of_current_run(&self) -> usize;
    fn at_end(&self) -> bool;
}

impl<T> RunIterator for T
where
    T: RunIteratorNativeAccess,
{
    fn consume(&mut self) {
        unsafe { C_SkShaper_RunIterator_consume(self.access_run_iterator_mut()) }
    }

    fn end_of_current_run(&self) -> usize {
        unsafe { C_SkShaper_RunIterator_endOfCurrentRun(self.access_run_iterator()) }
    }

    fn at_end(&self) -> bool {
        unsafe { C_SkShaper_RunIterator_atEnd(self.access_run_iterator()) }
    }
}

pub struct FontRunIterator(*mut SkShaper_FontRunIterator);

impl Drop for FontRunIterator {
    fn drop(&mut self) {
        unsafe { C_SkShaper_RunIterator_delete(self.access_run_iterator_mut()) }
    }
}

impl NativeAccess<SkShaper_FontRunIterator> for FontRunIterator {
    fn native(&self) -> &SkShaper_FontRunIterator {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut SkShaper_FontRunIterator {
        unsafe { &mut *self.0 }
    }
}

impl RunIteratorNativeAccess for FontRunIterator {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator {
        &self.native()._base
    }
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator {
        &mut self.native_mut()._base
    }
}

impl FontRunIterator {
    pub fn current_font(&self) -> &Font {
        Font::from_native_ref(unsafe { &*C_SkShaper_FontRunIterator_currentFont(self.native()) })
    }
}

impl Shaper {
    pub fn new_font_mgr_run_iterator<'a>(
        utf8: &str,
        font: &Font,
        mut fallback: Option<&mut FontMgr>,
    ) -> FontRunIterator {
        let bytes = utf8.as_bytes();
        FontRunIterator(unsafe {
            C_SkShaper_MakeFontMgrRunIterator(
                bytes.as_ptr() as _,
                bytes.len(),
                font.native(),
                fallback.shared_ptr_mut(),
            )
        })
    }
}

pub struct BiDiRunIterator(*mut SkShaper_BiDiRunIterator);

impl Drop for BiDiRunIterator {
    fn drop(&mut self) {
        unsafe { C_SkShaper_RunIterator_delete(self.access_run_iterator_mut()) }
    }
}

impl NativeAccess<SkShaper_BiDiRunIterator> for BiDiRunIterator {
    fn native(&self) -> &SkShaper_BiDiRunIterator {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut SkShaper_BiDiRunIterator {
        unsafe { &mut *self.0 }
    }
}

impl RunIteratorNativeAccess for BiDiRunIterator {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator {
        &self.native()._base
    }
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator {
        &mut self.native_mut()._base
    }
}

impl BiDiRunIterator {
    pub fn current_level(&self) -> u8 {
        unsafe { C_SkShaper_BiDiRunIterator_currentLevel(self.native()) }
    }
}

impl Shaper {
    pub fn new_icu_bidi_run_iterator(utf8: &str, level: u8) -> Option<BiDiRunIterator> {
        let bytes = utf8.as_bytes();
        unsafe { C_SkShaper_MakeIcuBidiRunIterator(bytes.as_ptr() as _, bytes.len(), level) }
            .to_option()
            .map(BiDiRunIterator)
    }
}

pub struct ScriptRunIterator(*mut SkShaper_ScriptRunIterator);

impl Drop for ScriptRunIterator {
    fn drop(&mut self) {
        unsafe { C_SkShaper_RunIterator_delete(self.access_run_iterator_mut()) }
    }
}

impl NativeAccess<SkShaper_ScriptRunIterator> for ScriptRunIterator {
    fn native(&self) -> &SkShaper_ScriptRunIterator {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut SkShaper_ScriptRunIterator {
        unsafe { &mut *self.0 }
    }
}

impl RunIteratorNativeAccess for ScriptRunIterator {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator {
        &self.native()._base
    }
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator {
        &mut self.native_mut()._base
    }
}

impl ScriptRunIterator {
    pub fn current_script(&self) -> FourByteTag {
        FourByteTag::from_native(unsafe {
            C_SkShaper_ScriptRunIterator_currentScript(self.native())
        })
    }
}

impl Shaper {
    pub fn new_hb_icu_script_run_iterator(utf8: &str) -> ScriptRunIterator {
        let bytes = utf8.as_bytes();
        unsafe { C_SkShaper_MakeHbIcuScriptRunIterator(bytes.as_ptr() as _, bytes.len()) }
            .to_option()
            .map(ScriptRunIterator)
            .unwrap()
    }
}

pub struct LanguageRunIterator(*mut SkShaper_LanguageRunIterator);

impl Drop for LanguageRunIterator {
    fn drop(&mut self) {
        unsafe { C_SkShaper_RunIterator_delete(self.access_run_iterator_mut()) }
    }
}

impl NativeAccess<SkShaper_LanguageRunIterator> for LanguageRunIterator {
    fn native(&self) -> &SkShaper_LanguageRunIterator {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut SkShaper_LanguageRunIterator {
        unsafe { &mut *self.0 }
    }
}

impl RunIteratorNativeAccess for LanguageRunIterator {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator {
        &self.native()._base
    }
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator {
        &mut self.native_mut()._base
    }
}

impl LanguageRunIterator {
    pub fn current_language(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(C_SkShaper_LanguageRunIterator_currentLanguage(
                self.native(),
            ))
        }
    }
}

impl Shaper {
    pub fn new_std_language_run_iterator(utf8: &str) -> Option<LanguageRunIterator> {
        let bytes = utf8.as_bytes();
        unsafe { C_SkShaper_MakeStdLanguageRunIterator(bytes.as_ptr() as _, bytes.len()) }
            .to_option()
            .map(LanguageRunIterator)
    }
}

// TODO: RunHandler
