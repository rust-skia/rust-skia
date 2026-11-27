//! `freetype-alongside-native`: on macOS and Windows, a FreeType font manager next to the native
//! one (CoreText, DirectWrite).
//!
//! The fonts come from Skia's sources, which are always there: no prebuilt binaries include
//! FreeType on these platforms, so Skia is built from source.
#![cfg(any(target_os = "macos", target_os = "windows"))]

#[cfg(feature = "freetype-alongside-native")]
mod freetype_alongside_native {
    use skia_safe::{Data, Font, FontMgr, Typeface};

    const TTF: &[u8] =
        include_bytes!("../../skia-bindings/skia/resources/fonts/Roboto-Regular.ttf");
    #[cfg(feature = "freetype-woff2")]
    const WOFF2: &[u8] =
        include_bytes!("../../skia-bindings/skia/resources/fonts/SpiderSymbol.woff2");

    fn custom_empty() -> FontMgr {
        FontMgr::custom_empty().expect("FontMgr::custom_empty() returns a FreeType font manager")
    }

    /// Returns the advance of "Hamburgefons" at size 64.
    fn text_advance(typeface: Typeface) -> f32 {
        let font = Font::from_typeface(typeface, 64.0);
        let (advance, _) = font.measure_str("Hamburgefons", None);
        advance
    }

    #[test]
    fn system_font_manager_stays_native() {
        let system = FontMgr::new();
        assert!(system.count_families() > 0);

        #[cfg(target_os = "macos")]
        {
            let typeface = system
                .legacy_make_typeface(None, skia_safe::FontStyle::normal())
                .unwrap();
            assert!(typeface.ct_font().is_some(), "CoreText typeface");
        }
    }

    #[test]
    fn custom_empty_loads_ttf() {
        let typeface = custom_empty()
            .new_from_data(Data::new_copy(TTF), None)
            .expect("TTF font");
        assert_eq!(typeface.family_name(), "Roboto");
        assert!(text_advance(typeface.clone()) > 0.0);

        #[cfg(target_os = "macos")]
        assert!(typeface.ct_font().is_none(), "FreeType typeface");
    }

    #[test]
    #[cfg(feature = "freetype-woff2")]
    fn custom_empty_loads_woff2() {
        let typeface = custom_empty()
            .new_from_data(Data::new_copy(WOFF2), None)
            .expect("WOFF2 font");
        assert!(typeface.count_glyphs() > 0);
    }
}

/// Without the feature, the build does not change: Skia has no FreeType on these platforms.
#[test]
#[cfg(not(feature = "freetype-alongside-native"))]
fn custom_empty_is_none_without_the_feature() {
    assert!(skia_safe::FontMgr::custom_empty().is_none());
}
