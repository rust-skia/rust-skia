use crate::prelude::*;
use crate::{scalar, Font, GlyphId, Paint, Point, RSXform, Rect, TextEncoding};
use skia_bindings::{
    C_SkTextBlobBuilder_destruct, C_SkTextBlobBuilder_make, C_SkTextBlob_MakeFromPosText,
    C_SkTextBlob_MakeFromPosTextH, C_SkTextBlob_MakeFromRSXform, C_SkTextBlob_MakeFromText,
    SkTextBlob, SkTextBlobBuilder,
};
use std::convert::TryInto;
use std::{ptr, slice};

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
    pub fn new(str: impl AsRef<str>, font: &Font) -> Option<Self> {
        Self::from_str(str, font)
    }

    pub fn bounds(&self) -> &Rect {
        Rect::from_native_ref(&self.native().fBounds)
    }

    pub fn unique_id(&self) -> u32 {
        self.native().fUniqueID
    }

    #[deprecated(note = "use get_intercepts()")]
    pub fn interceps(&self, bounds: [scalar; 2], paint: Option<&Paint>) -> Vec<scalar> {
        self.get_interceps(bounds, paint)
    }

    // TODO: consider to provide an inplace variant.
    pub fn get_interceps(&self, bounds: [scalar; 2], paint: Option<&Paint>) -> Vec<scalar> {
        unsafe {
            let count = self.native().getIntercepts(
                bounds.as_ptr(),
                ptr::null_mut(),
                paint.native_ptr_or_null(),
            );
            let mut intervals = vec![Default::default(); count.try_into().unwrap()];
            let count_2 = self.native().getIntercepts(
                bounds.as_ptr(),
                intervals.as_mut_ptr(),
                paint.native_ptr_or_null(),
            );
            assert_eq!(count, count_2);
            intervals
        }
    }

    pub fn from_str(str: impl AsRef<str>, font: &Font) -> Option<TextBlob> {
        Self::from_text(str.as_ref().as_bytes(), TextEncoding::UTF8, font)
    }

    pub fn from_text(text: &[u8], encoding: TextEncoding, font: &Font) -> Option<TextBlob> {
        TextBlob::from_ptr(unsafe {
            C_SkTextBlob_MakeFromText(
                text.as_ptr() as _,
                text.len(),
                font.native(),
                encoding.into_native(),
            )
        })
    }

    pub fn from_pos_text_h(
        text: &[u8],
        xpos: &[scalar],
        const_y: scalar,
        font: &Font,
        encoding: impl Into<Option<TextEncoding>>,
    ) -> Option<TextBlob> {
        let encoding = encoding.into().unwrap_or_default();
        // TODO: avoid that verification somehow.
        assert_eq!(xpos.len(), font.count_text(text, encoding));
        TextBlob::from_ptr(unsafe {
            C_SkTextBlob_MakeFromPosTextH(
                text.as_ptr() as _,
                text.len(),
                xpos.as_ptr(),
                const_y,
                font.native(),
                encoding.into_native(),
            )
        })
    }

    pub fn from_pos_text(
        text: &[u8],
        pos: &[Point],
        font: &Font,
        encoding: impl Into<Option<TextEncoding>>,
    ) -> Option<TextBlob> {
        let encoding = encoding.into().unwrap_or_default();
        // TODO: avoid that verification somehow.
        assert_eq!(pos.len(), font.count_text(text, encoding));
        TextBlob::from_ptr(unsafe {
            C_SkTextBlob_MakeFromPosText(
                text.as_ptr() as _,
                text.len(),
                pos.native().as_ptr(),
                font.native(),
                encoding.into_native(),
            )
        })
    }

    pub fn from_rsxform(
        text: &[u8],
        xform: &[RSXform],
        font: &Font,
        encoding: impl Into<Option<TextEncoding>>,
    ) -> Option<TextBlob> {
        let encoding = encoding.into().unwrap_or_default();
        // TODO: avoid that verification somehow.
        assert_eq!(xform.len(), font.count_text(text, encoding));
        TextBlob::from_ptr(unsafe {
            C_SkTextBlob_MakeFromRSXform(
                text.as_ptr() as _,
                text.len(),
                xform.native().as_ptr(),
                font.native(),
                encoding.into_native(),
            )
        })
    }
}

pub type TextBlobBuilder = Handle<SkTextBlobBuilder>;

impl NativeDrop for SkTextBlobBuilder {
    fn drop(&mut self) {
        unsafe { C_SkTextBlobBuilder_destruct(self) }
    }
}

impl Handle<SkTextBlobBuilder> {
    pub fn new() -> Self {
        Self::from_native(unsafe { SkTextBlobBuilder::new() })
    }

    pub fn make(&mut self) -> Option<TextBlob> {
        TextBlob::from_ptr(unsafe { C_SkTextBlobBuilder_make(self.native_mut()) })
    }

    pub fn alloc_run(
        &mut self,
        font: &Font,
        count: usize,
        offset: impl Into<Point>,
        bounds: Option<&Rect>,
    ) -> &mut [GlyphId] {
        let offset = offset.into();
        unsafe {
            let buffer = self.native_mut().allocRun(
                font.native(),
                count.try_into().unwrap(),
                offset.x,
                offset.y,
                bounds.native_ptr_or_null(),
            );
            slice::from_raw_parts_mut((*buffer).glyphs, count)
        }
    }

    pub fn alloc_run_pos_h(
        &mut self,
        font: Font,
        count: usize,
        y: scalar,
        bounds: Option<&Rect>,
    ) -> (&mut [GlyphId], &mut [scalar]) {
        unsafe {
            let buffer = self.native_mut().allocRunPosH(
                font.native(),
                count.try_into().unwrap(),
                y,
                bounds.native_ptr_or_null(),
            );
            (
                slice::from_raw_parts_mut((*buffer).glyphs, count),
                slice::from_raw_parts_mut((*buffer).pos, count),
            )
        }
    }

    pub fn alloc_run_pos(
        &mut self,
        font: Font,
        count: usize,
        bounds: Option<&Rect>,
    ) -> (&mut [GlyphId], &mut [Point]) {
        unsafe {
            let buffer = self.native_mut().allocRunPos(
                font.native(),
                count.try_into().unwrap(),
                bounds.native_ptr_or_null(),
            );
            (
                slice::from_raw_parts_mut((*buffer).glyphs, count),
                slice::from_raw_parts_mut((*buffer).pos as *mut Point, count),
            )
        }
    }

    pub fn alloc_run_rsxform(
        &mut self,
        font: &Font,
        count: usize,
    ) -> (&mut [GlyphId], &mut [RSXform]) {
        unsafe {
            let buffer = self
                .native_mut()
                .allocRunRSXform(font.native(), count.try_into().unwrap());
            (
                slice::from_raw_parts_mut((*buffer).glyphs, count),
                slice::from_raw_parts_mut((*buffer).pos as *mut RSXform, count),
            )
        }
    }
}

#[test]
fn test_point_size_equals_size_of_two_scalars_used_in_alloc_run_pos() {
    use std::mem;
    assert_eq!(mem::size_of::<Point>(), mem::size_of::<[scalar; 2]>())
}
