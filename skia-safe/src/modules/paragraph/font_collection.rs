use crate::prelude::*;
use crate::{interop, FontMgr, FontStyle, Typeface, Unichar};
use skia_bindings as sb;
use std::ffi;

pub type FontCollection = RCHandle<sb::skia_textlayout_FontCollection>;

impl NativeRefCountedBase for sb::skia_textlayout_FontCollection {
    type Base = sb::SkRefCntBase;
}

impl RCHandle<sb::skia_textlayout_FontCollection> {
    pub fn new() -> Self {
        Self::from_ptr(unsafe { sb::C_FontCollection_new() }).unwrap()
    }

    pub fn font_managers_count(&self) -> usize {
        unsafe { self.native().getFontManagersCount() }
    }

    pub fn set_asset_font_manager(&mut self, font_manager: impl Into<Option<FontMgr>>) {
        unsafe {
            sb::C_FontCollection_setAssetFontManager(
                self.native_mut(),
                font_manager.into().into_ptr_or_null(),
            )
        }
    }

    pub fn set_dynamic_font_manager(&mut self, font_manager: impl Into<Option<FontMgr>>) {
        unsafe {
            sb::C_FontCollection_setDynamicFontManager(
                self.native_mut(),
                font_manager.into().into_ptr_or_null(),
            )
        }
    }

    pub fn set_test_font_manager(&mut self, font_manager: impl Into<Option<FontMgr>>) {
        unsafe {
            sb::C_FontCollection_setTestFontManager(
                self.native_mut(),
                font_manager.into().into_ptr_or_null(),
            )
        }
    }

    pub fn set_default_font_manager<'a>(
        &mut self,
        font_manager: impl Into<Option<FontMgr>>,
        default_family_name: impl Into<Option<&'a str>>,
    ) {
        let font_manager = font_manager.into();
        unsafe {
            match default_family_name.into() {
                Some(fname) => {
                    let fname = ffi::CString::new(fname).unwrap();
                    sb::C_FontCollection_setDefaultFontManager2(
                        self.native_mut(),
                        font_manager.into_ptr_or_null(),
                        fname.as_ptr(),
                    )
                }
                None => sb::C_FontCollection_setDefaultFontManager(
                    self.native_mut(),
                    font_manager.into_ptr_or_null(),
                ),
            }
        }
    }

    pub fn fallback_manager(&self) -> Option<FontMgr> {
        FontMgr::from_unshared_ptr(self.native().fDefaultFontManager.fPtr)
    }

    pub fn match_typeface(
        &mut self,
        family_name: impl AsRef<str>,
        font_style: FontStyle,
        locale: impl AsRef<str>,
    ) -> Option<Typeface> {
        let family_name = ffi::CString::new(family_name.as_ref()).unwrap();
        let locale = interop::String::from_str(locale);
        Typeface::from_ptr(unsafe {
            sb::C_FontCollection_matchTypeface(
                self.native_mut(),
                family_name.as_ptr(),
                font_style.into_native(),
                locale.native(),
            )
        })
    }

    pub fn match_default_typeface(
        &mut self,
        font_style: FontStyle,
        locale: impl AsRef<str>,
    ) -> Option<Typeface> {
        let locale = interop::String::from_str(locale);
        Typeface::from_ptr(unsafe {
            sb::C_FontCollection_matchDefaultTypeface(
                self.native_mut(),
                font_style.into_native(),
                locale.native(),
            )
        })
    }

    pub fn default_fallback(
        &mut self,
        unicode: Unichar,
        font_style: FontStyle,
        locale: impl AsRef<str>,
    ) -> Option<Typeface> {
        let locale = interop::String::from_str(locale.as_ref());
        Typeface::from_ptr(unsafe {
            sb::C_FontCollection_defaultFallback(
                self.native_mut(),
                unicode,
                font_style.into_native(),
                locale.native(),
            )
        })
    }

    pub fn disable_font_fallback(&mut self) {
        unsafe { self.native_mut().disableFontFallback() }
    }

    pub fn font_fallback_enabled(&self) -> bool {
        self.native().fEnableFontFallback
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::textlayout::FontCollection;
    use crate::FontMgr;

    #[test]
    #[serial_test_derive::serial]
    fn ref_counts() {
        let mut fc = FontCollection::new();
        assert_eq!(fc.native().ref_counted_base()._ref_cnt(), 1);

        let fm = FontMgr::new();
        let fm_base = fm.native().ref_counted_base()._ref_cnt();

        fc.set_default_font_manager(fm.clone(), None);
        assert_eq!(fm.native().ref_counted_base()._ref_cnt(), fm_base + 1);

        let cloned_fc = fc.clone();
        assert_eq!(fm.native().ref_counted_base()._ref_cnt(), fm_base + 1);
        assert_eq!(fc.native().ref_counted_base()._ref_cnt(), 2);
        drop(cloned_fc);
        assert_eq!(fc.native().ref_counted_base()._ref_cnt(), 1);
        assert_eq!(fm.native().ref_counted_base()._ref_cnt(), fm_base + 1);

        {
            let fmc = fc.fallback_manager().unwrap();
            assert_eq!(fmc.native().ref_counted_base()._ref_cnt(), fm_base + 2);
            drop(fmc);
        }

        fc.set_default_font_manager(None, None);
        assert_eq!(fm.native().ref_counted_base()._ref_cnt(), fm_base);
        drop(fm);
        drop(fc);
    }
}
