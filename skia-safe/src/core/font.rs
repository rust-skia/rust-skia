use crate::prelude::*;
use std::{mem, ptr};
use skia_bindings::{
    C_SkFont_makeWithSize,
    C_SkFont_ConstructFromTypefaceWithSize,
    C_SkFont_ConstructFromTypeface,
    C_SkFont_Equals,
    SkFont,
    SkFont_Edging,
    C_SkFont_destruct,
    C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew,
    C_SkFont_setTypeface,
};
use crate::core::{
    Path,
    Point,
    Rect,
    Unichar,
    TextEncoding,
    GlyphId,
    Typeface,
    FontHinting,
    Paint,
    FontMetrics,
    scalar
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FontEdging {
    Alias = SkFont_Edging::kAlias as _,
    AntiAlias = SkFont_Edging::kAntiAlias as _,
    SubpixelAntiAlias = SkFont_Edging::kSubpixelAntiAlias as _
}

impl NativeTransmutable<SkFont_Edging> for FontEdging {}
#[test] fn test_font_edging_layout() { FontEdging::test_layout() }

pub type Font = Handle<SkFont>;

impl NativeDrop for SkFont {
    fn drop(&mut self) {
        unsafe { C_SkFont_destruct(self) }
    }
}

impl NativePartialEq for SkFont {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkFont_Equals(self, rhs) }
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::from_native(unsafe { SkFont::new() })
    }
}

impl Handle<SkFont> {

    pub fn from_typeface(typeface: &Typeface) -> Self {
        Self::construct(|font| unsafe { C_SkFont_ConstructFromTypeface(font, typeface.shared_native()) })
    }

    pub fn from_typeface_with_size(typeface: &Typeface, size: scalar) -> Self {
        Self::construct(|font| unsafe { C_SkFont_ConstructFromTypefaceWithSize(font, typeface.shared_native(), size) })
    }

    pub fn from_typeface_with_size_scale_and_skew(typeface: &Typeface, size: scalar, scale: scalar, skew: scalar) -> Self {
        Self::construct(|font| unsafe { C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew(font, typeface.shared_native(), size, scale, skew) })
    }

    pub fn is_force_auto_hinting(&self) -> bool {
        unsafe { self.native().isForceAutoHinting() }
    }

    pub fn is_embedded_bitmaps(&self) -> bool {
        unsafe { self.native().isEmbeddedBitmaps() }
    }

    pub fn is_subpixel(&self) -> bool {
        unsafe { self.native().isSubpixel() }
    }

    // has_linear_metrics?
    pub fn is_linear_metrics(&self) -> bool {
        unsafe { self.native().isLinearMetrics() }
    }

    pub fn is_embolden(&self) -> bool {
        unsafe { self.native().isEmbolden() }
    }

    pub fn set_force_autohinting(&mut self, force_auto_hinting: bool) -> &mut Self {
        unsafe {
            self.native_mut().setForceAutoHinting(force_auto_hinting)
        }
        self
    }

    pub fn set_embedded_bitmaps(&mut self, embedded_bitmaps: bool) -> &mut Self {
        unsafe {
            self.native_mut().setEmbeddedBitmaps(embedded_bitmaps)
        }
        self
    }

    pub fn set_subpixel(&mut self, subpixel: bool) -> &mut Self {
        unsafe {
            self.native_mut().setSubpixel(subpixel)
        }
        self
    }

    pub fn set_linear_metrics(&mut self, linear_metrics: bool) -> &mut Self {
        unsafe {
            self.native_mut().setLinearMetrics(linear_metrics)
        }
        self
    }

    pub fn set_embolden(&mut self, embolden: bool) -> &mut Self {
        unsafe {
            self.native_mut().setEmbolden(embolden)
        }
        self
    }

    pub fn edging(&self) -> FontEdging {
        FontEdging::from_native(unsafe {
            self.native().getEdging()
        })
    }

    pub fn set_edging(&mut self, edging: FontEdging) -> &mut Self {
        unsafe {
            self.native_mut().setEdging(edging.into_native())
        }
        self
    }

    pub fn set_hinting(&mut self, hinting: FontHinting) -> &mut Self {
        unsafe {
            self.native_mut().setHinting(hinting.into_native())
        }
        self
    }

    pub fn hinting(&self) -> FontHinting {
        FontHinting::from_native(unsafe {
            self.native().getHinting()
        })
    }

    #[must_use]
    pub fn with_size(&self, size: scalar) -> Option<Self> {
        if size >= 0.0 && !size.is_infinite() && !size.is_nan() {
            let mut font = unsafe { SkFont::new() };
            unsafe { C_SkFont_makeWithSize(self.native(), size, &mut font) }
            Some(Self::from_native(font))
        } else {
            None
        }
    }

    pub fn typeface(&self) -> Option<Typeface> {
        Typeface::from_unshared_ptr(unsafe {
            self.native().getTypeface()
        })
    }

    pub fn typeface_or_default(&self) -> Typeface {
        Typeface::from_unshared_ptr(unsafe {
            self.native().getTypefaceOrDefault()
        }).unwrap()
    }

    pub fn size(&self) -> scalar {
        unsafe { self.native().getSize() }
    }

    pub fn scale_x(&self) -> scalar {
        unsafe { self.native().getScaleX() }
    }

    pub fn skew_y(&self) -> scalar {
        unsafe { self.native().getSkewX() }
    }

    pub fn set_typeface(&mut self, tf: &Typeface) -> &mut Self {
        unsafe {
            C_SkFont_setTypeface(self.native_mut(), tf.shared_native())
        }
        self
    }

    pub fn set_size(&mut self, size: scalar) -> &mut Self {
        unsafe { self.native_mut().setSize(size) }
        self
    }

    pub fn set_scale_x(&mut self, scale_x: scalar) -> &mut Self {
        unsafe { self.native_mut().setScaleX(scale_x) }
        self
    }

    pub fn set_skew_x(&mut self, skew_x: scalar) -> &mut Self {
        unsafe { self.native_mut().setSkewX(skew_x) }
        self
    }

    pub fn str_to_glyphs(&self, str: &str, glyphs: &mut[GlyphId]) -> usize {
        let bytes = str.as_bytes();

        unsafe { self.native().textToGlyphs(
            bytes.as_ptr() as _,
            bytes.len(),
            TextEncoding::UTF8.into_native(),
            glyphs.as_mut_ptr(),
            // don't fail if glyphs.len() is too large to fit into an i32.
            glyphs.len().min(i32::max_value().try_into().unwrap()).try_into().unwrap())
            .try_into().unwrap()
        }
    }

    pub fn count_str(&self, str: &str) -> usize {
        let bytes = str.as_bytes();
        unsafe { self.native().textToGlyphs(
            bytes.as_ptr() as _,
            bytes.len(),
            TextEncoding::UTF8.into_native(),
            ptr::null_mut(), i32::max_value()).try_into().unwrap()
        }
    }

    // slower, but sooo convenient.
    pub fn str_to_glyphs_vec(&self, str: &str) -> Vec<GlyphId> {
        let count = self.count_str(str);
        let mut glyphs : Vec<GlyphId> = vec![Default::default(); count];
        let resulting_count = self.str_to_glyphs(str, glyphs.as_mut_slice());
        assert_eq!(count, resulting_count);
        glyphs
    }

    pub fn unichar_to_glyph(&self, uni: Unichar) -> GlyphId {
        unsafe { self.native().unicharToGlyph(uni) }
    }

    pub fn unichar_to_glyphs(&self, uni: &[Unichar], glyphs: &mut [GlyphId]) {
        assert_eq!(uni.len(), glyphs.len());
        unsafe {
            self.native().unicharsToGlyphs(
                uni.as_ptr(),
                uni.len().try_into().unwrap(),
                glyphs.as_mut_ptr())
        }
    }

    pub fn measure_str(&self, str: &str, paint: Option<&Paint>) -> (scalar, Rect) {
        let bytes = str.as_bytes();
        let mut bounds = Rect::default();

        let width = unsafe { self.native()
            .measureText1(
                bytes.as_ptr() as _, bytes.len(), TextEncoding::UTF8.into_native(),
                bounds.native_mut(), paint.native_ptr_or_null()) };

        (width, bounds)
    }

    pub fn widths_bounds(
        &self,
        glyphs: &[GlyphId],
        mut widths: Option<&mut [scalar]>,
        mut bounds: Option<&mut [Rect]>,
        paint: Option<&Paint>) {
        let count = glyphs.len();

        {
            if let Some(slice) = &widths { assert_eq!(count, slice.len()) };
            if let Some(slice) = &bounds { assert_eq!(count, slice.len()) };
        }

        let bounds_ptr = bounds.native_mut().as_ptr_or_null_mut();
        let widths_ptr = widths.as_ptr_or_null_mut();
        let paint_ptr = paint.native_ptr_or_null();

        unsafe {
            self.native().getWidthsBounds(
                glyphs.as_ptr(),
                count.try_into().unwrap(),
                widths_ptr, bounds_ptr, paint_ptr)
        }
    }

    pub fn pos(&self, glyphs: &[GlyphId], pos: &mut [Point], origin: Option<Point>) {
        let count = glyphs.len();
        assert_eq!(count, pos.len());

        let origin = origin.unwrap_or_default();

        unsafe {
            self.native().getPos(
                glyphs.as_ptr(),
                count.try_into().unwrap(),
                pos.native_mut().as_mut_ptr(),
                origin.native().clone())
        }
    }

    pub fn x_pos(&self, glyphs: &[GlyphId], xpos: &mut [scalar], origin: Option<scalar>) {
        let count = glyphs.len();
        assert_eq!(count, xpos.len());
        let origin = origin.unwrap_or_default();

        unsafe {
            self.native().getXPos(
                glyphs.as_ptr(),
                count.try_into().unwrap(),
                xpos.as_mut_ptr(),
                origin)
        }
    }

    pub fn path(&self, glyph_id: GlyphId) -> Option<Path> {
        let mut path = Path::default();
        unsafe {
            self.native().getPath(glyph_id, path.native_mut())
        }.if_true_some(path)
    }

    pub fn metrics(&self) -> (scalar, FontMetrics) {
        let mut fm = unsafe { mem::zeroed() };
        let line_spacing = unsafe { self.native().getMetrics(&mut fm) };
        (line_spacing, FontMetrics::from_native(fm))
    }

    pub fn spacing(&self) -> scalar {
        unsafe {
            self.native().getMetrics(ptr::null_mut())
        }
    }
}