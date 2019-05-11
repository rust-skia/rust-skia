use crate::interop;
use crate::interop::DynamicMemoryWStream;
use crate::prelude::*;
use crate::{FontStyle, Typeface, Unichar};
use skia_bindings::{
    C_SkFontMgr_RefDefault, C_SkFontMgr_makeFromStream, C_SkFontStyleSet_count,
    C_SkFontStyleSet_createTypeface, C_SkFontStyleSet_getStyle, C_SkFontStyleSet_matchStyle,
    SkFontMgr, SkFontStyleSet, SkRefCntBase,
};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;

pub type FontStyleSet = RCHandle<SkFontStyleSet>;

impl NativeRefCountedBase for SkFontStyleSet {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl Default for RCHandle<SkFontStyleSet> {
    fn default() -> Self {
        FontStyleSet::new_empty()
    }
}

impl RCHandle<SkFontStyleSet> {
    pub fn count(&mut self) -> usize {
        unsafe {
            C_SkFontStyleSet_count(self.native_mut())
                .try_into()
                .unwrap()
        }
    }

    pub fn style(&mut self, index: usize) -> (FontStyle, String) {
        assert!(index < self.count());

        let mut font_style = FontStyle::default();
        let mut style = interop::String::default();
        unsafe {
            C_SkFontStyleSet_getStyle(
                self.native_mut(),
                index.try_into().unwrap(),
                font_style.native_mut(),
                style.native_mut(),
            )
        }
        (font_style, style.as_str().into())
    }

    // TODO: use the original name create_typeface() ?
    pub fn new_typeface(&mut self, index: usize) -> Option<Typeface> {
        assert!(index < self.count());

        Typeface::from_ptr(unsafe {
            C_SkFontStyleSet_createTypeface(self.native_mut(), index.try_into().unwrap())
        })
    }

    pub fn match_style(&mut self, index: usize, pattern: &FontStyle) -> Option<Typeface> {
        assert!(index < self.count());
        Typeface::from_ptr(unsafe {
            C_SkFontStyleSet_matchStyle(self.native_mut(), pattern.native())
        })
    }

    // TODO: use the original name create_empty() ?
    pub fn new_empty() -> Self {
        FontStyleSet::from_ptr(unsafe { SkFontStyleSet::CreateEmpty() }).unwrap()
    }
}

pub type FontMgr = RCHandle<SkFontMgr>;

impl NativeRefCountedBase for SkFontMgr {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl Default for RCHandle<SkFontMgr> {
    fn default() -> Self {
        FontMgr::from_ptr(unsafe { C_SkFontMgr_RefDefault() }).unwrap()
    }
}

impl RCHandle<SkFontMgr> {
    pub fn count_families(&self) -> usize {
        unsafe { self.native().countFamilies().try_into().unwrap() }
    }

    pub fn family_name(&self, index: usize) -> String {
        assert!(index < self.count_families());
        let mut family_name = interop::String::default();
        unsafe {
            self.native()
                .getFamilyName(index.try_into().unwrap(), family_name.native_mut())
        }
        family_name.as_str().into()
    }

    pub fn new_styleset(&self, index: usize) -> FontStyleSet {
        assert!(index < self.count_families());
        FontStyleSet::from_ptr(unsafe { self.native().createStyleSet(index.try_into().unwrap()) })
            .unwrap()
    }

    pub fn match_family(&self, family_name: &str) -> FontStyleSet {
        let family_name = CString::new(family_name).unwrap();
        FontStyleSet::from_ptr(unsafe { self.native().matchFamily(family_name.as_ptr()) }).unwrap()
    }

    pub fn match_family_style(&self, family_name: &str, style: FontStyle) -> Option<Typeface> {
        let family_name = CString::new(family_name).unwrap();
        Typeface::from_ptr(unsafe {
            self.native()
                .matchFamilyStyle(family_name.as_ptr(), style.native())
        })
    }

    // TODO: support IntoIterator / AsRef<str> for bcp_47?
    pub fn match_family_style_character(
        &self,
        family_name: &str,
        style: FontStyle,
        bcp_47: &[&str],
        character: Unichar,
    ) -> Option<Typeface> {
        let family_name = CString::new(family_name).unwrap();
        // create backing store for the pointer array.
        let bcp_47: Vec<CString> = bcp_47.iter().map(|s| CString::new(*s).unwrap()).collect();
        // note: mutability needed to comply to the C type "const char* bcp47[]".
        let mut bcp_47: Vec<*const c_char> = bcp_47.iter().map(|cs| cs.as_ptr()).collect();

        Typeface::from_ptr(unsafe {
            self.native().matchFamilyStyleCharacter(
                family_name.as_ptr(),
                style.native(),
                bcp_47.as_mut_ptr(),
                bcp_47.len().try_into().unwrap(),
                character,
            )
        })
    }

    pub fn match_face_style(&self, typeface: &Typeface, style: FontStyle) -> Option<Typeface> {
        Typeface::from_ptr(unsafe {
            self.native()
                .matchFaceStyle(typeface.native(), style.native())
        })
    }

    pub fn new_from_bytes(&self, bytes: &[u8], ttc_index: Option<usize>) -> Option<Typeface> {
        let mut stream = DynamicMemoryWStream::from_bytes(bytes);
        let mut stream = stream.detach_as_stream();
        Typeface::from_ptr(unsafe {
            let stream_ptr = stream.native_mut() as *mut _;
            // makeFromStream takes ownership of the stream, so don't call drop on it.
            mem::forget(stream);
            C_SkFontMgr_makeFromStream(
                self.native(),
                stream_ptr,
                ttc_index.unwrap_or_default().try_into().unwrap(),
            )
        })
    }
}

#[test]
fn create_all_typefaces() {
    let font_mgr = FontMgr::default();
    let families = font_mgr.count_families();
    println!("FontMgr families: {}", families);
    // test requires that the font manager returns at least one family for now.
    assert!(families > 0);
    // get all family names
    for i in 0..families {
        let name = font_mgr.family_name(i);
        println!("font_family: {}", name);
        let mut style_set = font_mgr.new_styleset(i);
        for style_index in 0..style_set.count() {
            let (_, style_name) = style_set.style(style_index);
            println!("  style: {}", style_name);
            let face = style_set.new_typeface(style_index);
            drop(face);
        }
    }
}
