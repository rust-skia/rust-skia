use crate::{prelude::*, FontMgr};
use skia_bindings::{self as sb, RustOrderedFontMgr as SkOrderedFontMgr, SkRefCntBase};
use std::{
    fmt,
    mem::transmute,
    ops::{Deref, DerefMut},
};

pub type OrderedFontMgr = RCHandle<SkOrderedFontMgr>;
require_base_type!(SkOrderedFontMgr, sb::SkFontMgr);

impl NativeRefCountedBase for SkOrderedFontMgr {
    type Base = SkRefCntBase;
}

impl Deref for OrderedFontMgr {
    type Target = FontMgr;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref(self) }
    }
}

impl DerefMut for OrderedFontMgr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_ref_mut(self) }
    }
}

impl Default for OrderedFontMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OrderedFontMgr> for FontMgr {
    fn from(font_mgr: OrderedFontMgr) -> Self {
        unsafe { transmute(font_mgr) }
    }
}

impl fmt::Debug for OrderedFontMgr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OrderedFontMgr")
            .field("base", self as &FontMgr)
            .finish()
    }
}

impl OrderedFontMgr {
    pub fn new() -> Self {
        Self::from_ptr(unsafe { sb::C_RustOrderedFontMgr_new() }).unwrap()
    }

    pub fn append(&mut self, font_mgr: impl Into<FontMgr>) {
        let font_mgr = font_mgr.into();
        unsafe { sb::C_RustOrderedFontMgr_append(self.native_mut(), font_mgr.into_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::OrderedFontMgr;

    #[test]
    fn can_use_font_mgr_functions() {
        let ordered = OrderedFontMgr::default();
        let _families = ordered.count_families();
    }

    #[test]
    fn can_pass_ordered_font_mgr_where_a_font_mgr_is_expected() {
        let mut ordered = OrderedFontMgr::default();
        let another = OrderedFontMgr::default();
        ordered.append(another);
    }

    #[test]
    #[cfg(feature = "textlayout")]
    fn can_find_fonts_in_multiple_mgrs() {
        use super::FontMgr;
        use crate::textlayout::TypefaceFontProvider;
        use std::fs;
        use std::path::Path;

        let sys_mgr = FontMgr::default();
        let single_font_provider = |filename: &str| {
            let path = Path::new(filename);
            let font = fs::read(path)
                .ok()
                .and_then(|bytes| sys_mgr.new_from_data(&bytes, None))
                .expect(&format!("failed to load font: {}", filename));

            let mut provider = TypefaceFontProvider::new();
            provider.register_typeface(font, None);
            provider
        };

        // create two TypefaceFontProviders, each with only a single font
        let bungee_provider = single_font_provider(
            "../skia-bindings/skia/modules/canvaskit/tests/assets/Bungee-Regular.ttf",
        );
        let noto_provider = single_font_provider(
            "../skia-bindings/skia/modules/canvaskit/tests/assets/NotoSerif-Regular.ttf",
        );

        // add both providers to an OrderedFontMgr, then ensure both can be found
        let mut ordered = OrderedFontMgr::new();
        ordered.append(bungee_provider.clone());
        ordered.append(noto_provider.clone());
        assert_eq!(ordered.match_family("Bungee").count(), 1);
        assert_eq!(ordered.match_family("Noto Serif").count(), 1);
        assert_eq!(ordered.match_family("Nonesuch").count(), 0);
    }
}
