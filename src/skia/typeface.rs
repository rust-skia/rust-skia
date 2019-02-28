use std::ffi;
use crate::prelude::*;
use crate::skia::{
    FontStyle,
    FontStyleWeight,
    FontStyleSlant,
    Data,
    GlyphId,
    Unichar,
    FontTableTag,
    Rect
};
use rust_skia::{
    C_SkTypeface_MakeDefault,
    SkTypeface,
    C_SkTypeface_MakeFromName,
    C_SkTypeface_MakeFromData,
    SkTypeface_SerializeBehavior,
    C_SkTypeface_serialize,
};

pub type TypefaceSerializeBehavior = EnumHandle<SkTypeface_SerializeBehavior>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkTypeface_SerializeBehavior> {
    pub const DoIncludeData: Self = Self(SkTypeface_SerializeBehavior::kDoIncludeData);
    pub const DontIncludeData: Self = Self(SkTypeface_SerializeBehavior::kDontIncludeData);
    pub const IncludeDataIfLocal: Self = Self(SkTypeface_SerializeBehavior::kIncludeDataIfLocal);
}

/*
// not sure if we need to export that yet.
type TypefaceEncoding = EnumHandle<SkTypeface_Encoding>;

impl EnumHandle<SkTypeface_Encoding> {
    pub const UTF8: Self = Self(SkTypeface_Encoding::kUTF8_Encoding);
    pub const UTF16: Self = Self(SkTypeface_Encoding::kUTF16_Encoding);
    pub const UTF32: Self = Self(SkTypeface_Encoding::kUTF32_Encoding);
}
*/

pub type Typeface = RCHandle<SkTypeface>;

impl NativeRefCounted for SkTypeface {
    fn _ref(&self) {
        unsafe { self._base._base._base.ref_() }
    }

    fn _unref(&self) {
        unsafe { self._base._base._base.unref() }
    }
}

impl RCHandle<SkTypeface> {
    pub fn font_style(&self) -> FontStyle {
        unsafe { FontStyle::from_native(self.native().fontStyle()) }
    }

    pub fn is_bold(&self) -> bool {
        // does not link
        // unsafe { self.native().isBold() }
        self.font_style().weight() >= FontStyleWeight::SemiBold
    }

    pub fn is_italic(&self) -> bool {
        // unsafe { self.native().isItalic() }
        self.font_style().slant() != FontStyleSlant::Upright
    }

    pub fn is_fixed_pitch(&self) -> bool {
        unsafe { self.native().isFixedPitch() }
    }

    pub fn equal(facea: &Typeface, faceb: &Typeface) -> bool {
        unsafe { SkTypeface::Equal(facea.native(), faceb.native()) }
    }

    pub fn from_name(familiy_name: &str, font_style: FontStyle) -> Option<Typeface> {
        let familiy_name = ffi::CString::new(familiy_name);
        if let Result::Ok(familiy_name) = familiy_name {
            Typeface::from_ptr(unsafe {
                C_SkTypeface_MakeFromName(familiy_name.as_ptr(), *font_style.native())
            })
        } else {
            None
        }
    }

    // from_file is unsupported, because it is unclear what's the
    // encoding of the path name. from_data can be used instead.

    pub fn from_data(data: &Data, index: i32) {
        Typeface::from_ptr(
            unsafe { C_SkTypeface_MakeFromData(data.shared_native(), index) } );
    }

    pub fn serialize(&self, behavior: TypefaceSerializeBehavior) -> Data {
        Data::from_ptr(unsafe {
            C_SkTypeface_serialize(self.native(), behavior.native())
        }).unwrap()
    }

    // chars_to_glyphs is unsupported, because the documentation does not make sense to me:
    // The returnvalue does not seem to actually count the required elements of the array.
    // Use Font's text_to_glyphs 's function instead.

    pub fn unichar_to_glyph(&self, unichar: Unichar) -> GlyphId {
        unsafe { self.native().unicharToGlyph(unichar) }
    }

    pub fn count_glyphs(&self) -> usize {
        unsafe { self.native().countGlyphs() as usize }
    }

    pub fn count_tables(&self) -> usize {
        unsafe { self.native().countTables() as usize }
    }

    pub fn table_tags(&self) -> Option<Vec<FontTableTag>> {
        let mut v: Vec<FontTableTag> = vec![0; self.count_tables()];
        if unsafe { self.native().getTableTags(v.as_mut_ptr()) } != 0 {
            Some(v)
        } else {
            None
        }
    }

    pub fn table_size(&self, tag: FontTableTag) -> Option<usize> {
        let size = unsafe { self.native().getTableSize(tag) as usize };
        if size != 0 {
            Some(size)
        } else {
            None
        }
    }

    pub fn table_data(&self, tag: FontTableTag, data: &mut [u8]) -> usize {
        unsafe { self.native().getTableData(tag, 0, data.len(), data.as_mut_ptr() as _) }
    }

    pub fn units_per_em(&self) -> Option<i32> {
        let units = unsafe { self.native().getUnitsPerEm() };
        if units != 0 {
            Some(units)
        } else {
            None
        }
    }

    pub fn may_support_kerning(&self) -> bool {
        true
    }

    // note: adjustments slice length must be equal to glyph's len - 1.
    pub fn kerning_pair_adjustments(&self, glyphs: &[GlyphId], adjustments: &mut [i32]) -> bool {
        if glyphs.len() <= (i32::max_value() as _) && adjustments.len() == glyphs.len() + 1 {
            unsafe { self.native().
                getKerningPairAdjustments(glyphs.as_ptr(), glyphs.len() as i32, adjustments.as_mut_ptr()) }
        } else {
            false
        }
    }

    pub fn bounds(&self) -> Rect {
        Rect::from_native(unsafe { self.native().getBounds() })
    }
}

impl Default for RCHandle<SkTypeface> {
    fn default() -> Self {
        Typeface::from_ptr(unsafe { C_SkTypeface_MakeDefault() }).unwrap()
    }
}
