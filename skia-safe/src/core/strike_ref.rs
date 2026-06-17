use std::fmt;

use skia_bindings as sb;

use crate::{GlyphId, Rect, prelude::*, scalar};

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
    pub fn get_width(&self, glyph: GlyphId) -> scalar {
        unsafe { sb::C_SkStrikeRef_getWidth(self.native(), glyph) }
    }

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

        let mut font_bounds = vec![Default::default(); glyphs.len()];
        let mut strike_bounds = vec![Default::default(); glyphs.len()];
        font.get_widths_bounds(&glyphs, None, Some(&mut font_bounds), None);
        strike_ref.get_widths_bounds(&glyphs, None, Some(&mut strike_bounds));

        assert_eq!(strike_bounds, font_bounds);
    }
}
