use crate::interop::{MemoryStream, NativeStreamBase};
use crate::prelude::*;
use crate::{font_arguments, interop, FontArguments};
use crate::{font_parameters::VariationAxis, Data, FontStyle, GlyphId, Rect, Unichar};
use skia_bindings as sb;
use skia_bindings::{SkRefCntBase, SkTypeface, SkTypeface_LocalizedStrings};
use std::{ffi, ptr};

pub type FontId = skia_bindings::SkFontID;
pub type FontTableTag = skia_bindings::SkFontTableTag;

pub use skia_bindings::SkTypeface_SerializeBehavior as SerializeBehavior;
#[test]
fn test_typeface_serialize_behavior_naming() {
    let _ = SerializeBehavior::DontIncludeData;
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedString {
    pub string: String,
    pub language: String,
}

pub type Typeface = RCHandle<SkTypeface>;
unsafe impl Send for Typeface {}
unsafe impl Sync for Typeface {}

impl NativeRefCountedBase for SkTypeface {
    type Base = SkRefCntBase;
}

impl Default for RCHandle<SkTypeface> {
    fn default() -> Self {
        Typeface::from_ptr(unsafe { sb::C_SkTypeface_MakeDefault() }).unwrap()
    }
}

impl RCHandle<SkTypeface> {
    // Canonical new:
    pub fn new(family_name: impl AsRef<str>, font_style: FontStyle) -> Option<Self> {
        Self::from_name(family_name, font_style)
    }

    pub fn font_style(&self) -> FontStyle {
        FontStyle::from_native_c(self.native().fStyle)
    }

    pub fn is_bold(&self) -> bool {
        unsafe { sb::C_SkTypeface_isBold(self.native()) }
    }

    pub fn is_italic(&self) -> bool {
        unsafe { sb::C_SkTypeface_isItalic(self.native()) }
    }

    pub fn is_fixed_pitch(&self) -> bool {
        self.native().fIsFixedPitch
    }

    pub fn variation_design_position(
        &self,
    ) -> Option<Vec<font_arguments::variation_position::Coordinate>> {
        unsafe {
            let r = self.native().getVariationDesignPosition(ptr::null_mut(), 0);
            if r != -1 {
                let mut v = vec![font_arguments::variation_position::Coordinate::default(); r as _];
                let elements = self
                    .native()
                    .getVariationDesignPosition(v.native_mut().as_mut_ptr(), r);
                assert_eq!(elements, r);
                Some(v)
            } else {
                None
            }
        }
    }

    pub fn variation_design_parameters(&self) -> Option<Vec<VariationAxis>> {
        unsafe {
            let r = self
                .native()
                .getVariationDesignParameters(ptr::null_mut(), 0);
            if r != -1 {
                let mut v = vec![VariationAxis::default(); r as _];
                let elements = self
                    .native()
                    .getVariationDesignParameters(v.native_mut().as_mut_ptr(), r);
                assert_eq!(elements, r);
                Some(v)
            } else {
                None
            }
        }
    }

    pub fn unique_id(self) -> FontId {
        self.native().fUniqueID
    }

    // TODO: wrap SkTypeface::UniqueID()?

    // Decided not to support PartialEq instead of this function,
    // because Skia does not support the operator ==.
    pub fn equal(face_a: impl AsRef<Typeface>, face_b: impl AsRef<Typeface>) -> bool {
        unsafe { SkTypeface::Equal(face_a.as_ref().native(), face_b.as_ref().native()) }
    }

    pub fn from_name(family_name: impl AsRef<str>, font_style: FontStyle) -> Option<Typeface> {
        let familiy_name = ffi::CString::new(family_name.as_ref()).ok()?;
        Typeface::from_ptr(unsafe {
            sb::C_SkTypeface_MakeFromName(familiy_name.as_ptr(), *font_style.native())
        })
    }

    // from_file is unsupported, because it is unclear what the
    // encoding of the path name is. from_data can be used instead.

    // TODO: MakeFromStream()?

    pub fn from_data(data: impl Into<Data>, index: impl Into<Option<usize>>) -> Option<Typeface> {
        Typeface::from_ptr(unsafe {
            sb::C_SkTypeface_MakeFromData(
                data.into().into_ptr(),
                index.into().unwrap_or_default().try_into().unwrap(),
            )
        })
    }

    pub fn clone_with_arguments(&self, arguments: &FontArguments) -> Option<Typeface> {
        Typeface::from_ptr(unsafe { sb::C_SkTypeface_makeClone(self.native(), arguments.native()) })
    }

    // TODO: serialize(Write)?

    // TODO: return Data as impl Deref<[u8]> / Borrow<[u8]> here?
    pub fn serialize(&self, behavior: SerializeBehavior) -> Data {
        Data::from_ptr(unsafe { sb::C_SkTypeface_serialize(self.native(), behavior) }).unwrap()
    }

    // TODO: Deserialize(Read?)

    pub fn deserialize(data: &[u8]) -> Option<Typeface> {
        let mut stream = MemoryStream::from_bytes(data);
        Typeface::from_ptr(unsafe {
            sb::C_SkTypeface_MakeDeserialize(stream.native_mut().as_stream_mut())
        })
    }

    pub fn unichars_to_glyphs(&self, uni: &[Unichar], glyphs: &mut [GlyphId]) {
        assert_eq!(uni.len(), glyphs.len());
        unsafe {
            self.native().unicharsToGlyphs(
                uni.as_ptr(),
                uni.len().try_into().unwrap(),
                glyphs.as_mut_ptr(),
            )
        }
    }

    pub fn unichar_to_glyph(&self, unichar: Unichar) -> GlyphId {
        unsafe { self.native().unicharToGlyph(unichar) }
    }

    pub fn count_glyphs(&self) -> usize {
        unsafe { self.native().countGlyphs().try_into().unwrap() }
    }

    pub fn count_tables(&self) -> usize {
        unsafe { self.native().countTables().try_into().unwrap() }
    }

    pub fn table_tags(&self) -> Option<Vec<FontTableTag>> {
        let mut v: Vec<FontTableTag> = vec![0; self.count_tables()];
        (unsafe { self.native().getTableTags(v.as_mut_ptr()) } != 0).if_true_some(v)
    }

    pub fn get_table_size(&self, tag: FontTableTag) -> Option<usize> {
        let size = unsafe { self.native().getTableSize(tag) };
        if size != 0 {
            Some(size)
        } else {
            None
        }
    }

    pub fn get_table_data(&self, tag: FontTableTag, data: &mut [u8]) -> usize {
        unsafe {
            self.native()
                .getTableData(tag, 0, data.len(), data.as_mut_ptr() as _)
        }
    }

    pub fn copy_table_data(&self, tag: FontTableTag) -> Option<Data> {
        Data::from_ptr(unsafe { sb::C_SkTypeface_copyTableData(self.native(), tag) })
    }

    pub fn units_per_em(&self) -> Option<i32> {
        let units = unsafe { self.native().getUnitsPerEm() };
        if units != 0 {
            Some(units)
        } else {
            None
        }
    }

    // note: adjustments slice length must be equal to glyph's len - 1.
    pub fn get_kerning_pair_adjustments(
        &self,
        glyphs: &[GlyphId],
        adjustments: &mut [i32],
    ) -> bool {
        (adjustments.len() + 1 == glyphs.len())
            && unsafe {
                self.native().getKerningPairAdjustments(
                    glyphs.as_ptr(),
                    glyphs.len().try_into().unwrap(),
                    adjustments.as_mut_ptr(),
                )
            }
    }

    pub fn new_family_name_iterator(&self) -> impl Iterator<Item = LocalizedString> {
        LocalizedStringsIter::from_ptr(unsafe { self.native().createFamilyNameIterator() }).unwrap()
    }

    pub fn family_name(&self) -> String {
        let mut name = interop::String::default();
        unsafe {
            self.native().getFamilyName(name.native_mut());
        };
        name.as_str().into()
    }

    pub fn post_script_name(&self) -> Option<String> {
        let mut name = interop::String::default();
        unsafe { self.native().getPostScriptName(name.native_mut()) }
            .if_true_then_some(|| name.as_str().into())
    }

    // TODO: openStream()
    // TODO: createScalerContext()

    pub fn bounds(&self) -> Rect {
        Rect::from_native_c(unsafe { sb::C_SkTypeface_getBounds(self.native()) })
    }
}

pub type LocalizedStringsIter = RefHandle<SkTypeface_LocalizedStrings>;

impl NativeDrop for SkTypeface_LocalizedStrings {
    fn drop(&mut self) {
        unsafe { sb::C_SkTypeface_LocalizedStrings_unref(self) }
    }
}

impl Iterator for RefHandle<SkTypeface_LocalizedStrings> {
    type Item = LocalizedString;

    fn next(&mut self) -> Option<Self::Item> {
        let mut string = interop::String::default();
        let mut language = interop::String::default();
        unsafe {
            sb::C_SkTypeface_LocalizedStrings_next(
                self.native_mut(),
                string.native_mut(),
                language.native_mut(),
            )
        }
        .if_true_then_some(|| LocalizedString {
            string: string.as_str().into(),
            language: language.as_str().into(),
        })
    }
}

#[test]
fn serialize_and_deserialize_default_typeface() {
    let tf = Typeface::default();
    let serialized = tf.serialize(SerializeBehavior::DoIncludeData);
    // On Android, the deserialized typeface name changes from sans-serif to Roboto.
    // (which is probably OK, because Roboto _is_ the default font, so we do another
    // serialization / deserialization and compare the family name with already deserialized
    // one.)
    let deserialized = Typeface::deserialize(&serialized).unwrap();
    let serialized2 = deserialized.serialize(SerializeBehavior::DoIncludeData);
    let deserialized2 = Typeface::deserialize(&serialized2).unwrap();

    // why aren't they not equal?
    // assert!(Typeface::equal(&tf, &deserialized));
    assert_eq!(deserialized.family_name(), deserialized2.family_name());
}

#[test]
fn family_name_iterator_owns_the_strings_and_returns_at_least_one_name_for_the_default_typeface() {
    let tf = Typeface::default();
    let family_names = tf.new_family_name_iterator();
    drop(tf);

    let mut any = false;
    for name in family_names {
        println!("family: {}, language: {}", name.string, name.language);
        any = true
    }
    assert!(any);
}
