use once_cell::sync::OnceCell;

use skia_safe::{FontMgr, FontStyle, Typeface};

pub fn default_typeface() -> Typeface {
    DEFAULT_TYPEFACE
        .get_or_init(|| {
            let font_mgr = FontMgr::new();
            font_mgr
                .legacy_make_typeface(None, FontStyle::default())
                .unwrap()
        })
        .clone()
}

static DEFAULT_TYPEFACE: OnceCell<Typeface> = OnceCell::new();
