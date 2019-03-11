use crate::prelude::*;
use std::{mem, ptr};
use skia_bindings::{
    C_SkFont_makeWithSize,
    C_SkFont_ConstructFromTypefaceWithSize,
    C_SkFont_ConstructFromTypeface,
    C_SkFont_equals,
    SkFont,
    SkFont_Edging,
    C_SkFont_destruct,
    C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew,
    C_SkFont_setTypeface,
};
use crate::skia::{
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

pub type FontEdging = EnumHandle<SkFont_Edging>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkFont_Edging> {
    pub const Alias: Self = Self(SkFont_Edging::kAlias);
    pub const AntiAlias: Self = Self(SkFont_Edging::kAntiAlias);
    pub const SubpixelAntiAlias: Self = Self(SkFont_Edging::kSubpixelAntiAlias);
}

pub type Font = Handle<SkFont>;

impl NativeDrop for SkFont {
    fn drop(&mut self) {
        unsafe { C_SkFont_destruct(self) }
    }
}

impl NativePartialEq for SkFont {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkFont_equals(self, rhs) }
    }
}

impl Default for Font {
    fn default() -> Self {
        unsafe { SkFont::new() }.into_handle()
    }
}

impl Handle<SkFont> {

    pub fn from_typeface(typeface: &Typeface) -> Self {
        let mut font : SkFont = unsafe { mem::uninitialized() };
        unsafe {
            C_SkFont_ConstructFromTypeface(&mut font, typeface.shared_native())
        }
        font.into_handle()
    }

    pub fn from_typeface_with_size(typeface: &Typeface, size: scalar) -> Self {
        let mut font : SkFont = unsafe { mem::uninitialized() };
        unsafe {
            C_SkFont_ConstructFromTypefaceWithSize(&mut font, typeface.shared_native(), size)
        }
        font.into_handle()
    }

    pub fn from_typeface_with_size_scale_and_skew(typeface: &Typeface, size: scalar, scale: scalar, skew: scalar) -> Self {
        let mut font : SkFont = unsafe { mem::uninitialized() };
        unsafe {
            C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew(&mut font, typeface.shared_native(), size, scale, skew)
        }
        font.into_handle()
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
            Some(font.into_handle())
        } else {
            None
        }
    }

    pub fn typeface(&self) -> Typeface {
        Typeface::from_unshared_ptr(unsafe {
            self.native().getTypeface()
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

    // we support UTF8 for now only.
    // TODO: can we return a slice _into_ glyphs?
    pub fn str_to_glyphs(&self, str: &str, glyphs: &mut[GlyphId]) -> usize {
        let bytes = str.as_bytes();

        unsafe { self.native().textToGlyphs(
            bytes.as_ptr() as _,
            bytes.len(),
            TextEncoding::UTF8.into_native(),
            glyphs.as_mut_ptr(),
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

    pub fn unichar_to_glyph(&self, uni: Unichar) -> u16 {
        unsafe { self.native().unicharToGlyph(uni) }
    }

    pub fn contains_str(&self, str: &str) -> bool {
        let bytes = str.as_bytes();
        unsafe {
            self.native().containsText(bytes.as_ptr() as _, bytes.len(), TextEncoding::UTF8.into_native())
        }
    }

    // note that the returned usize value is the bytes of the str that fits and not the characters.
    pub fn break_str(&self, str: &str, max_width: scalar) -> (usize, scalar) {
        let bytes = str.as_bytes();

        let mut measured_width = scalar::default();
        let bytes_fit = unsafe { self.native()
            .breakText(
                bytes.as_ptr() as _, bytes.len(), TextEncoding::UTF8.into_native(),
                max_width, &mut measured_width) };

        (bytes_fit, measured_width)
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
        glyphs: &[u16],
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

    pub fn pos(&self, glyphs: &[u16], pos: &mut [Point], origin: Option<Point>) {
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

    pub fn x_pos(&self, glyphs: &[u16], xpos: &mut [scalar], origin: Option<scalar>) {
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

    pub fn path(&self, glyph_id: u16) -> Option<Path> {
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