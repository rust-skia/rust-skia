use crate::prelude::*;
use crate::{Rect, scalar, Paint, Font, TextEncoding, Point, GlyphId};
use std::{ptr, slice};
use skia_bindings::{SkTextBlob, C_SkTextBlob_MakeFromText, SkTextBlobBuilder, C_SkTextBlobBuilder_make, C_SkTextBlobBuilder_destruct};
use std::convert::TryInto;

pub type TextBlob = RCHandle<SkTextBlob>;

impl NativeRefCounted for SkTextBlob {
    fn _ref(&self) {
        unsafe { skia_bindings::C_SkTextBlob_ref(self) };
    }

    fn _unref(&self) {
        unsafe { skia_bindings::C_SkTextBlob_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { skia_bindings::C_SkTextBlob_unique(self) }
    }
}

impl RCHandle<SkTextBlob> {
    pub fn bounds(&self) -> &Rect {
        unsafe {
            Rect::from_native_ref(&*self.native().bounds())
        }
    }

    pub fn unique_id(&self) -> u32 {
        unsafe {
            self.native().uniqueID()
        }
    }

    #[deprecated(note = "use get_intercepts()")]
    pub fn interceps(&self, bounds: &[scalar; 2], paint: Option<&Paint>) -> Vec<scalar> {
        self.get_interceps(bounds, paint)
    }

    // TODO: consider to provide an inplace variant.
    pub fn get_interceps(&self, bounds: &[scalar; 2], paint: Option<&Paint>) -> Vec<scalar> {
        unsafe {
            let count = self.native().getIntercepts(bounds.as_ptr(), ptr::null_mut(), paint.native_ptr_or_null());
            let mut intervals = vec![Default::default(); count.try_into().unwrap()];
            let count_2 = self.native().getIntercepts(bounds.as_ptr(), intervals.as_mut_ptr(), paint.native_ptr_or_null());
            assert_eq!(count, count_2);
            intervals
        }
    }

    pub fn from_str(str: &str, font: &Font) -> Option<TextBlob> {
        Self::from_text(str.as_bytes(), TextEncoding::UTF8, font)
    }

    pub fn from_text(text: &[u8], encoding: TextEncoding, font: &Font) -> Option<TextBlob> {
        TextBlob::from_ptr(unsafe {
            C_SkTextBlob_MakeFromText(text.as_ptr() as _, text.len(), font.native(), encoding.into_native())
        })
    }

    // TODO: serialize, Deserialize
}

pub type TextBlobBuilder = Handle<SkTextBlobBuilder>;

impl NativeDrop for SkTextBlobBuilder {
    fn drop(&mut self) {
        unsafe {
            C_SkTextBlobBuilder_destruct(self)
        }
    }
}

impl Handle<SkTextBlobBuilder> {
    pub fn new() -> Self {
        Self::from_native(unsafe {
            SkTextBlobBuilder::new()
        })
    }

    pub fn make(&mut self) -> Option<TextBlob> {
        TextBlob::from_ptr(unsafe {
            C_SkTextBlobBuilder_make(self.native_mut())
        })
    }

    pub fn alloc_run(&mut self, font: &Font, count: usize, offset: impl Into<Point>, bounds: Option<&Rect>) -> &mut [GlyphId] {
        let offset = offset.into();
        unsafe {
            let buffer = self.native_mut().allocRun(
                font.native(),
                count.try_into().unwrap(),
                offset.x, offset.y,
                bounds.native_ptr_or_null());
            slice::from_raw_parts_mut((*buffer).glyphs, count)
        }
    }

    pub fn alloc_run_pos_h(&mut self, font: Font, count: usize, y: scalar, bounds: Option<&Rect>) -> (&mut [GlyphId], &mut [scalar]) {
        unsafe {
            let buffer = self.native_mut().allocRunPosH(
                font.native(),
                count.try_into().unwrap(),
                y,
                bounds.native_ptr_or_null());
            (
                slice::from_raw_parts_mut((*buffer).glyphs, count),
                slice::from_raw_parts_mut((*buffer).pos, count)
            )
        }
    }

    pub fn alloc_run_pos(&mut self, font: Font, count: usize, bounds: Option<&Rect>) -> (&mut [GlyphId], &mut [Point]) {
        unsafe {
            let buffer = self.native_mut().allocRunPos(
                font.native(),
                count.try_into().unwrap(),
                bounds.native_ptr_or_null());
            (
                slice::from_raw_parts_mut((*buffer).glyphs, count),
                slice::from_raw_parts_mut((*buffer).pos as *mut Point, count)
            )
        }
    }
}

#[test]
fn test_point_size_equals_size_of_two_scalars_used_in_alloc_run_pos() {
    use std::mem;
    assert_eq!(mem::size_of::<Point>(), mem::size_of::<[scalar; 2]>())
}
