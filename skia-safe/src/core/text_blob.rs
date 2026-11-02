//! Combines multiple text runs into an immutable container. Each text run consists of glyphs,
//! [`Paint`], and position. Only parts of [`Paint`] related to fonts and text rendering are used by
//! a run.

use std::{fmt, ptr, slice};

use skia_bindings::{
    self as sb, SkTextBlob, SkTextBlob_Iter, SkTextBlob_Iter_Run, SkTextBlobBuilder, SkTypeface,
};

use crate::{
    EncodedText, Font, GlyphId, Paint, Point, RSXform, Rect, Typeface, prelude::*, scalar,
};

pub type TextBlob = RCHandle<SkTextBlob>;
unsafe_send_sync!(TextBlob);
require_base_type!(SkTextBlob, sb::SkNVRefCnt);

impl NativeRefCounted for SkTextBlob {
    unsafe fn _ref(ptr: *const Self) {
        unsafe { sb::C_SkTextBlob_ref(ptr) };
    }

    unsafe fn _unref(ptr: *const Self) {
        unsafe { sb::C_SkTextBlob_unref(ptr) }
    }

    unsafe fn unique(ptr: *const Self) -> bool {
        unsafe { sb::C_SkTextBlob_unique(ptr) }
    }
}

impl fmt::Debug for TextBlob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextBlob")
            .field("bounds", &self.bounds())
            .field("unique_id", &self.unique_id())
            .finish()
    }
}

impl TextBlob {
    pub fn new(str: impl AsRef<str>, font: &Font) -> Option<Self> {
        Self::from_str(str, font)
    }

    /// Returns the conservative bounding box. Uses the [`Paint`] associated with each glyph to
    /// determine glyph bounds, and unions all bounds. The returned bounds may be larger than the
    /// bounds of all glyphs in runs.
    pub fn bounds(&self) -> &Rect {
        Rect::from_native_ref(&self.native().fBounds)
    }

    /// Returns a non-zero value unique among all text blobs.
    pub fn unique_id(&self) -> u32 {
        self.native().fUniqueID
    }

    // TODO: consider to provide an inplace variant.
    /// Returns the number of intervals that intersect `bounds`. `bounds` describes a pair of lines
    /// parallel to the text advance. The return count is zero or a multiple of two, and is at most
    /// twice the number of glyphs in the blob.
    ///
    /// Runs within the blob that contain [`RSXform`] are ignored when computing intercepts.
    ///
    /// - `bounds` lower and upper line parallel to the advance
    /// - `paint` optional paint specifying stroking and path effect that affects the result
    pub fn get_intercepts(&self, bounds: [scalar; 2], paint: Option<&Paint>) -> Vec<scalar> {
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

    /// Creates a text blob with a single run. The string is encoded as UTF-8.
    ///
    /// `font` contains attributes used to define the run text.
    ///
    /// This function uses the default character-to-glyph mapping from the [`Typeface`] in `font`.
    /// It does not perform typeface fallback for characters not found in the [`Typeface`]. It does
    /// not perform kerning or other complex shaping; glyphs are positioned based on their default
    /// advances.
    ///
    /// - `str` character code points drawn
    /// - `font` text size, typeface, text scale, and so on, used to draw
    pub fn from_str(str: impl AsRef<str>, font: &Font) -> Option<TextBlob> {
        Self::from_text(str.as_ref(), font)
    }

    /// Creates a text blob with a single run.
    ///
    /// `font` contains attributes used to define the run text.
    ///
    /// When the encoding is UTF-8, UTF-16, or UTF-32, this function uses the default
    /// character-to-glyph mapping from the [`Typeface`] in `font`. It does not perform typeface
    /// fallback for characters not found in the [`Typeface`]. It does not perform kerning or other
    /// complex shaping; glyphs are positioned based on their default advances.
    ///
    /// - `text` character code points or glyphs drawn
    /// - `font` text size, typeface, text scale, and so on, used to draw
    pub fn from_text(text: impl EncodedText, font: &Font) -> Option<TextBlob> {
        let (ptr, size, encoding) = text.as_raw();
        TextBlob::from_ptr(unsafe {
            sb::C_SkTextBlob_MakeFromText(ptr, size, font.native(), encoding.into_native())
        })
    }

    /// Returns a text blob built from a single run of text with x-positions and a single y value.
    /// This is equivalent to using [`TextBlobBuilder`] and calling `alloc_run_pos_h`. Returns `None`
    /// if the text is empty.
    ///
    /// - `text` character code points or glyphs drawn (based on encoding)
    /// - `x_pos` array of x-positions, must contain values for all of the character points
    /// - `const_y` shared y-position for each character point, to be paired with each `x_pos`
    /// - `font` font used for this run
    pub fn from_pos_text_h(
        text: impl EncodedText,
        x_pos: &[scalar],
        const_y: scalar,
        font: &Font,
    ) -> Option<TextBlob> {
        // TODO: avoid that somehow.
        assert_eq!(x_pos.len(), font.count_text(&text));
        let (ptr, size, encoding) = text.as_raw();
        TextBlob::from_ptr(unsafe {
            sb::C_SkTextBlob_MakeFromPosTextH(
                ptr,
                size,
                x_pos.as_ptr(),
                x_pos.len(),
                const_y,
                font.native(),
                encoding.into_native(),
            )
        })
    }

    /// Returns a text blob built from a single run of text with positions. This is equivalent to
    /// using [`TextBlobBuilder`] and calling `alloc_run_pos`. Returns `None` if the text is empty.
    ///
    /// - `text` character code points or glyphs drawn (based on encoding)
    /// - `pos` array of positions, must contain values for all of the character points
    /// - `font` font used for this run
    pub fn from_pos_text(text: impl EncodedText, pos: &[Point], font: &Font) -> Option<TextBlob> {
        assert_eq!(pos.len(), font.count_text(&text));
        let (ptr, size, encoding) = text.as_raw();
        TextBlob::from_ptr(unsafe {
            sb::C_SkTextBlob_MakeFromPosText(
                ptr,
                size,
                pos.native().as_ptr(),
                pos.len(),
                font.native(),
                encoding.into_native(),
            )
        })
    }

    pub fn from_rsxform(
        text: impl EncodedText,
        xform: &[RSXform],
        font: &Font,
    ) -> Option<TextBlob> {
        assert_eq!(xform.len(), font.count_text(&text));
        let (ptr, size, encoding) = text.as_raw();
        TextBlob::from_ptr(unsafe {
            sb::C_SkTextBlob_MakeFromRSXform(
                ptr,
                size,
                xform.native().as_ptr(),
                xform.len(),
                font.native(),
                encoding.into_native(),
            )
        })
    }
}

/// Helper class for constructing [`TextBlob`].
pub type TextBlobBuilder = Handle<SkTextBlobBuilder>;
unsafe_send_sync!(TextBlobBuilder);

impl NativeDrop for SkTextBlobBuilder {
    fn drop(&mut self) {
        unsafe { sb::C_SkTextBlobBuilder_destruct(self) }
    }
}

impl fmt::Debug for TextBlobBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextBlobBuilder").finish()
    }
}

impl TextBlobBuilder {
    /// Constructs an empty text blob builder. By default, the text blob builder has no runs.
    pub fn new() -> Self {
        Self::from_native_c(unsafe { SkTextBlobBuilder::new() })
    }

    /// Returns a [`TextBlob`] built from runs of glyphs added by the builder. The returned
    /// [`TextBlob`] is immutable; it may be copied, but its contents may not be altered. Returns
    /// `None` if no runs of glyphs were added by the builder.
    ///
    /// Resets the text blob builder to its initial empty state, allowing it to be reused to build
    /// a new set of runs.
    pub fn make(&mut self) -> Option<TextBlob> {
        TextBlob::from_ptr(unsafe { sb::C_SkTextBlobBuilder_make(self.native_mut()) })
    }

    /// Returns a run with storage for glyphs. The caller must write `count` glyphs to the returned
    /// glyph buffer before the next call to the text blob builder.
    ///
    /// Glyphs share metrics in `font`.
    ///
    /// Glyphs are positioned on a baseline at `offset`, using font metrics to determine their
    /// relative placement.
    ///
    /// `bounds` defines an optional bounding box, used to suppress drawing when the text blob bounds
    /// does not intersect the surface bounds. If `bounds` is `None`, the text blob bounds is
    /// computed from `offset` and the glyph metrics.
    ///
    /// - `font` font used for this run
    /// - `count` number of glyphs
    /// - `offset` horizontal and vertical offset within the blob
    /// - `bounds` optional run bounding box
    pub fn alloc_run(
        &mut self,
        font: &Font,
        count: usize,
        offset: impl Into<Point>,
        bounds: Option<&Rect>,
    ) -> &mut [GlyphId] {
        let offset = offset.into();
        unsafe {
            let buffer = &*self.native_mut().allocRun(
                font.native(),
                count.try_into().unwrap(),
                offset.x,
                offset.y,
                bounds.native_ptr_or_null(),
            );
            safer::from_raw_parts_mut(buffer.glyphs, count)
        }
    }

    /// Returns a run with storage for glyphs and positions along a baseline. The caller must write
    /// `count` glyphs to the returned glyph buffer and `count` scalars to the returned position
    /// buffer before the next call to the text blob builder.
    ///
    /// Glyphs share metrics in `font`.
    ///
    /// Glyphs are positioned on a baseline at `y`, using x-axis positions written by the caller to
    /// the returned position buffer.
    ///
    /// `bounds` defines an optional bounding box, used to suppress drawing when the text blob bounds
    /// does not intersect the surface bounds. If `bounds` is `None`, the text blob bounds is
    /// computed from `y`, the position buffer, and the glyph metrics.
    ///
    /// - `font` font used for this run
    /// - `count` number of glyphs
    /// - `y` vertical offset within the blob
    /// - `bounds` optional run bounding box
    pub fn alloc_run_pos_h(
        &mut self,
        font: &Font,
        count: usize,
        y: scalar,
        bounds: Option<&Rect>,
    ) -> (&mut [GlyphId], &mut [scalar]) {
        unsafe {
            let buffer = &*self.native_mut().allocRunPosH(
                font.native(),
                count.try_into().unwrap(),
                y,
                bounds.native_ptr_or_null(),
            );
            (
                safer::from_raw_parts_mut(buffer.glyphs, count),
                safer::from_raw_parts_mut(buffer.pos, count),
            )
        }
    }

    /// Returns a run with storage for glyphs and point positions. The caller must write `count`
    /// glyphs to the returned glyph buffer and `count` points to the returned position buffer
    /// before the next call to the text blob builder.
    ///
    /// Glyphs share metrics in `font`.
    ///
    /// Glyphs are positioned using the points written by the caller to the returned position
    /// buffer, using two scalar values for each point.
    ///
    /// `bounds` defines an optional bounding box, used to suppress drawing when the text blob bounds
    /// does not intersect the surface bounds. If `bounds` is `None`, the text blob bounds is
    /// computed from the position buffer and the glyph metrics.
    ///
    /// - `font` font used for this run
    /// - `count` number of glyphs
    /// - `bounds` optional run bounding box
    pub fn alloc_run_pos(
        &mut self,
        font: &Font,
        count: usize,
        bounds: Option<&Rect>,
    ) -> (&mut [GlyphId], &mut [Point]) {
        unsafe {
            let buffer = &*self.native_mut().allocRunPos(
                font.native(),
                count.try_into().unwrap(),
                bounds.native_ptr_or_null(),
            );
            (
                safer::from_raw_parts_mut(buffer.glyphs, count),
                safer::from_raw_parts_mut(buffer.pos as *mut Point, count),
            )
        }
    }

    /// Returns a run with storage for glyphs and [`RSXform`] positions. The caller must write
    /// `count` glyphs to the returned glyph buffer and `count` [`RSXform`]s to the returned position
    /// buffer before the next call to the text blob builder.
    ///
    /// Glyphs share metrics in `font`.
    ///
    /// - `font` font used for this run
    /// - `count` number of glyphs
    pub fn alloc_run_rsxform(
        &mut self,
        font: &Font,
        count: usize,
    ) -> (&mut [GlyphId], &mut [RSXform]) {
        unsafe {
            let buffer = &*self
                .native_mut()
                .allocRunRSXform(font.native(), count.try_into().unwrap());
            (
                safer::from_raw_parts_mut(buffer.glyphs, count),
                safer::from_raw_parts_mut(buffer.pos as *mut RSXform, count),
            )
        }
    }

    /// Returns a run with storage for glyphs, text, and clusters. The caller must write `count`
    /// glyphs to the returned glyph buffer, `text_byte_count` UTF-8 code units into the returned
    /// text buffer, and `count` monotonic indexes into the text buffer into the returned cluster
    /// buffer before the next call to the text blob builder.
    ///
    /// Glyphs share metrics in `font`.
    ///
    /// Glyphs are positioned on a baseline at `offset`, using font metrics to determine their
    /// relative placement.
    ///
    /// `bounds` defines an optional bounding box, used to suppress drawing when the text blob bounds
    /// does not intersect the surface bounds. If `bounds` is `None`, the text blob bounds is
    /// computed from `offset` and the glyph metrics.
    ///
    /// - `font` font used for this run
    /// - `count` number of glyphs
    /// - `offset` horizontal and vertical offset within the blob
    /// - `text_byte_count` number of UTF-8 code units
    /// - `bounds` optional run bounding box
    pub fn alloc_run_text(
        &mut self,
        font: &Font,
        count: usize,
        offset: impl Into<Point>,
        text_byte_count: usize,
        bounds: Option<&Rect>,
    ) -> (&mut [GlyphId], &mut [u8], &mut [u32]) {
        let offset = offset.into();
        unsafe {
            let buffer = &*self.native_mut().allocRunText(
                font.native(),
                count.try_into().unwrap(),
                offset.x,
                offset.y,
                text_byte_count.try_into().unwrap(),
                bounds.native_ptr_or_null(),
            );
            (
                safer::from_raw_parts_mut(buffer.glyphs, count),
                safer::from_raw_parts_mut(buffer.utf8text as *mut u8, text_byte_count),
                safer::from_raw_parts_mut(buffer.clusters, count),
            )
        }
    }

    /// Returns a run with storage for glyphs, positions along a baseline, text, and clusters. The
    /// caller must write `count` glyphs to the returned glyph buffer, `count` scalars to the
    /// returned position buffer, `text_byte_count` UTF-8 code units into the returned text buffer,
    /// and `count` monotonic indexes into the text buffer into the returned cluster buffer before
    /// the next call to the text blob builder.
    ///
    /// Glyphs share metrics in `font`.
    ///
    /// Glyphs are positioned on a baseline at `y`, using x-axis positions written by the caller to
    /// the returned position buffer.
    ///
    /// `bounds` defines an optional bounding box, used to suppress drawing when the text blob bounds
    /// does not intersect the surface bounds. If `bounds` is `None`, the text blob bounds is
    /// computed from `y`, the position buffer, and the glyph metrics.
    ///
    /// - `font` font used for this run
    /// - `count` number of glyphs
    /// - `y` vertical offset within the blob
    /// - `text_byte_count` number of UTF-8 code units
    /// - `bounds` optional run bounding box
    pub fn alloc_run_text_pos_h(
        &mut self,
        font: &Font,
        count: usize,
        y: scalar,
        text_byte_count: usize,
        bounds: Option<&Rect>,
    ) -> (&mut [GlyphId], &mut [scalar], &mut [u8], &mut [u32]) {
        unsafe {
            let buffer = &*self.native_mut().allocRunTextPosH(
                font.native(),
                count.try_into().unwrap(),
                y,
                text_byte_count.try_into().unwrap(),
                bounds.native_ptr_or_null(),
            );
            (
                safer::from_raw_parts_mut(buffer.glyphs, count),
                safer::from_raw_parts_mut(buffer.pos, count),
                safer::from_raw_parts_mut(buffer.utf8text as *mut u8, text_byte_count),
                safer::from_raw_parts_mut(buffer.clusters, count),
            )
        }
    }

    /// Returns a run with storage for glyphs, point positions, text, and clusters. The caller must
    /// write `count` glyphs to the returned glyph buffer, `count` points to the returned position
    /// buffer, `text_byte_count` UTF-8 code units into the returned text buffer, and `count`
    /// monotonic indexes into the text buffer into the returned cluster buffer before the next call
    /// to the text blob builder.
    ///
    /// Glyphs share metrics in `font`.
    ///
    /// Glyphs are positioned using the points written by the caller to the returned position
    /// buffer, using two scalar values for each point.
    ///
    /// `bounds` defines an optional bounding box, used to suppress drawing when the text blob bounds
    /// does not intersect the surface bounds. If `bounds` is `None`, the text blob bounds is
    /// computed from the position buffer and the glyph metrics.
    ///
    /// - `font` font used for this run
    /// - `count` number of glyphs
    /// - `text_byte_count` number of UTF-8 code units
    /// - `bounds` optional run bounding box
    pub fn alloc_run_text_pos(
        &mut self,
        font: &Font,
        count: usize,
        text_byte_count: usize,
        bounds: Option<&Rect>,
    ) -> (&mut [GlyphId], &mut [Point], &mut [u8], &mut [u32]) {
        unsafe {
            let buffer = &*self.native_mut().allocRunTextPos(
                font.native(),
                count.try_into().unwrap(),
                text_byte_count.try_into().unwrap(),
                bounds.native_ptr_or_null(),
            );
            (
                safer::from_raw_parts_mut(buffer.glyphs, count),
                safer::from_raw_parts_mut(buffer.pos as *mut Point, count),
                safer::from_raw_parts_mut(buffer.utf8text as *mut u8, text_byte_count),
                safer::from_raw_parts_mut(buffer.clusters, count),
            )
        }
    }

    pub fn alloc_run_text_rsxform(
        &mut self,
        font: &Font,
        count: usize,
        text_byte_count: usize,
        bounds: Option<&Rect>,
    ) -> (&mut [GlyphId], &mut [RSXform], &mut [u8], &mut [u32]) {
        unsafe {
            let buffer = &*self.native_mut().allocRunTextPos(
                font.native(),
                count.try_into().unwrap(),
                text_byte_count.try_into().unwrap(),
                bounds.native_ptr_or_null(),
            );
            (
                safer::from_raw_parts_mut(buffer.glyphs, count),
                safer::from_raw_parts_mut(buffer.pos as *mut RSXform, count),
                safer::from_raw_parts_mut(buffer.utf8text as *mut u8, text_byte_count),
                safer::from_raw_parts_mut(buffer.clusters, count),
            )
        }
    }
}

pub type TextBlobIter<'a> = Borrows<'a, Handle<SkTextBlob_Iter>>;

pub struct TextBlobRun<'a> {
    typeface: *mut SkTypeface,
    pub glyph_indices: &'a [GlyphId],
}

impl fmt::Debug for TextBlobRun<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextBlobRun")
            .field("typeface", self.typeface())
            .field("glyph_indices", &self.glyph_indices)
            .finish()
    }
}

impl TextBlobRun<'_> {
    pub fn typeface(&self) -> &Option<Typeface> {
        Typeface::from_unshared_ptr_ref(&self.typeface)
    }
}

impl<'a> Borrows<'a, Handle<SkTextBlob_Iter>> {
    pub fn new(text_blob: &'a TextBlob) -> Self {
        Handle::from_native_c(unsafe { SkTextBlob_Iter::new(text_blob.native()) })
            .borrows(text_blob)
    }
}

impl NativeDrop for SkTextBlob_Iter {
    fn drop(&mut self) {
        unsafe { sb::C_SkTextBlob_Iter_destruct(self) }
    }
}

impl<'a> Iterator for Borrows<'a, Handle<SkTextBlob_Iter>> {
    type Item = TextBlobRun<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut run = SkTextBlob_Iter_Run {
            fTypeface: ptr::null_mut(),
            fGlyphCount: 0,
            fGlyphIndices: ptr::null_mut(),
        };
        unsafe {
            if self.native_mut().next(&mut run) {
                let indices = if !run.fGlyphIndices.is_null() && run.fGlyphCount != 0 {
                    slice::from_raw_parts(run.fGlyphIndices, run.fGlyphCount.try_into().unwrap())
                } else {
                    &[]
                };

                Some(TextBlobRun {
                    typeface: run.fTypeface,
                    glyph_indices: indices,
                })
            } else {
                None
            }
        }
    }
}

#[test]
fn test_point_size_and_alignment_equals_size_of_two_scalars_used_in_alloc_run_pos() {
    use std::mem;
    assert_eq!(mem::size_of::<Point>(), mem::size_of::<[scalar; 2]>());
    assert_eq!(mem::align_of::<Point>(), mem::align_of::<[scalar; 2]>());
}
