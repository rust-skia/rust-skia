use crate::prelude::*;
use crate::{Rect, scalar, Paint, Font, TextEncoding};
use std::ptr;
use skia_bindings::{SkTextBlob, C_SkTextBlob_MakeFromText, SkTextEncoding};

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
