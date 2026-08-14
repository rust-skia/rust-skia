use std::fmt;

use skia_bindings as sb;

use crate::{GlyphId, Rect, prelude::*, scalar};

/// `StrikeRef` is a lightweight, thread-safe handle to a resolved font strike.
///
/// It caches the result of looking up an `SkStrike` for a particular [`crate::Font`] configuration,
/// allowing repeated glyph metric queries (advances, bounds) without the overhead of descriptor
/// construction, hashing, and global cache lookup on each call.
///
/// Obtain an `StrikeRef` from [`crate::Font::make_strike_ref()`]. The returned object remains valid
/// as long as it is held; the underlying `SkStrike` is atomically reference counted.
///
/// `StrikeRef` does not track changes to the [`crate::Font`] it was created from. If the
/// [`crate::Font`]'s properties change (size, typeface, hinting, etc.), a new `StrikeRef` must be
/// obtained.
pub type StrikeRef = Handle<sb::SkStrikeRef>;
unsafe_send_sync!(StrikeRef);

impl NativeDrop for sb::SkStrikeRef {
    fn drop(&mut self) {
        unsafe { sb::C_SkStrikeRef_destruct(self) }
    }
}

impl NativeClone for sb::SkStrikeRef {
    fn clone(&self) -> Self {
        construct(|s| unsafe { sb::C_SkStrikeRef_CopyConstruct(s, self) })
    }
}

impl fmt::Debug for StrikeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StrikeRef").finish()
    }
}

impl StrikeRef {
    /// Retrieves the advance widths for each glyph.
    ///
    /// `widths` receives `min(widths.len(), glyphs.len())` values.
    ///
    /// - `glyphs`: array of glyph indices to be measured
    /// - `widths`: returns text advances for each glyph, in font units
    pub fn get_widths(&self, glyphs: &[GlyphId], widths: &mut [scalar]) {
        assert_eq!(glyphs.len(), widths.len());
        unsafe {
            sb::C_SkStrikeRef_getWidths(
                self.native(),
                glyphs.as_ptr(),
                glyphs.len(),
                widths.as_mut_ptr(),
                widths.len(),
            )
        }
    }

    /// Retrieves the advance width for a single glyph.
    ///
    /// - `glyph`: glyph index to be measured
    ///
    /// Returns advance width in font units.
    pub fn get_width(&self, glyph: GlyphId) -> scalar {
        unsafe { sb::C_SkStrikeRef_getWidth(self.native(), glyph) }
    }

    /// Retrieves the advance widths for each glyph, handling arbitrary strides for both input
    /// glyphs and output advances.
    ///
    /// - `count`: number of glyphs to measure
    /// - `glyphs`: slice containing the first glyph ID, glyph is 32-bit, instead of [`GlyphId`], in
    ///   preparation for large glyph ids.
    /// - `glyph_stride_32`: stride in 32-bit words between input glyph IDs
    /// - `advances`: slice containing the first output advance
    /// - `advance_stride_32`: stride in 32-bit words between output advances
    pub fn get_widths_strided(
        &self,
        count: usize,
        glyphs: &[u32],
        glyph_stride_32: usize,
        advances: &mut [scalar],
        advance_stride_32: usize,
    ) {
        if count == 0 {
            return;
        }

        let native_count = count.try_into().expect("count exceeds unsigned range");
        let native_glyph_stride = glyph_stride_32
            .try_into()
            .expect("glyph_stride_32 exceeds unsigned range");
        let native_advance_stride = advance_stride_32
            .try_into()
            .expect("advance_stride_32 exceeds unsigned range");

        let glyph_len = (count - 1)
            .checked_mul(glyph_stride_32)
            .and_then(|len| len.checked_add(1))
            .expect("glyph stride length overflow");
        let advance_len = (count - 1)
            .checked_mul(advance_stride_32)
            .and_then(|len| len.checked_add(1))
            .expect("advance stride length overflow");
        assert!(glyphs.len() >= glyph_len, "glyph slice is too short");
        assert!(advances.len() >= advance_len, "advance slice is too short");

        unsafe {
            sb::C_SkStrikeRef_getWidthsStrided(
                self.native(),
                native_count,
                glyphs.as_ptr(),
                native_glyph_stride,
                advances.as_mut_ptr(),
                native_advance_stride,
            )
        }
    }

    /// Retrieves the advance widths and bounds for each glyph.
    ///
    /// `widths` receives `min(widths.len(), glyphs.len())` values.
    /// `bounds` receives `min(bounds.len(), glyphs.len())` values.
    ///
    /// - `glyphs`: array of glyph indices to be measured
    /// - `widths`: returns text advances for each glyph
    /// - `bounds`: returns bounds for each glyph relative to `(0, 0)`
    pub fn get_widths_bounds(
        &self,
        glyphs: &[GlyphId],
        mut widths: Option<&mut [scalar]>,
        mut bounds: Option<&mut [Rect]>,
    ) {
        let count = glyphs.len();

        {
            if let Some(slice) = &widths {
                assert_eq!(count, slice.len())
            };
            if let Some(slice) = &bounds {
                assert_eq!(count, slice.len())
            };
        }

        let widths_ptr = widths.as_ptr_or_null_mut();
        let widths_count = widths.map_or(0, |slice| slice.len());
        let bounds_ptr = bounds.native_mut().as_ptr_or_null_mut();
        let bounds_count = bounds.map_or(0, |slice| slice.len());

        unsafe {
            sb::C_SkStrikeRef_getWidthsBounds(
                self.native(),
                glyphs.as_ptr(),
                count,
                widths_ptr,
                widths_count,
                bounds_ptr,
                bounds_count,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Font, FontMgr, FontStyle};

    #[test]
    fn strike_ref_widths_match_font_widths() {
        let font_mgr = FontMgr::new();
        let typeface = font_mgr
            .legacy_make_typeface(None, FontStyle::normal())
            .unwrap();
        let font = Font::new(typeface, 14.0);
        let glyphs = font.text_to_glyphs_vec("StrikeRef");
        assert!(!glyphs.is_empty());

        let strike_ref = font.make_strike_ref();

        let mut font_widths = vec![0.0; glyphs.len()];
        font.get_widths(&glyphs, &mut font_widths);

        let mut strike_widths = vec![0.0; glyphs.len()];
        strike_ref.get_widths(&glyphs, &mut strike_widths);

        assert_eq!(strike_widths, font_widths);
        assert_eq!(strike_ref.get_width(glyphs[0]), font_widths[0]);

        let glyphs_32: Vec<u32> = glyphs.iter().map(|glyph| (*glyph).into()).collect();
        let strided_glyphs: Vec<u32> = glyphs_32
            .iter()
            .flat_map(|glyph| [*glyph, u32::MAX])
            .collect();
        let mut strided_widths = vec![f32::NAN; strided_glyphs.len()];
        strike_ref.get_widths_strided(glyphs_32.len(), &strided_glyphs, 2, &mut strided_widths, 2);
        assert_eq!(
            strided_widths
                .iter()
                .step_by(2)
                .copied()
                .collect::<Vec<_>>(),
            font_widths
        );
        assert!(
            strided_widths
                .iter()
                .skip(1)
                .step_by(2)
                .all(|width| width.is_nan())
        );

        let mut repeated_width = [0.0];
        strike_ref.get_widths_strided(2, &glyphs_32[..1], 0, &mut repeated_width, 0);
        assert_eq!(repeated_width[0], font_widths[0]);

        let mut font_bounds = vec![Default::default(); glyphs.len()];
        let mut strike_bounds = vec![Default::default(); glyphs.len()];
        font.get_widths_bounds(&glyphs, None, Some(&mut font_bounds), None);
        strike_ref.get_widths_bounds(&glyphs, None, Some(&mut strike_bounds));

        assert_eq!(strike_bounds, font_bounds);
    }
}
