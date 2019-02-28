use crate::prelude::*;
use std::{mem, ptr};
use rust_skia::{
    C_SkFont_makeWithSize,
    C_SkFont_ConstructFromTypefaceWithSize,
    C_SkFont_ConstructFromTypeface,
    C_SkFont_equals,
    SkFont,
    SkFont_Edging,
    C_SkFont_Destruct,
    C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew,
    C_SkFont_setTypeface,
    SkRect,
    SkPoint
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
    FontMetrics
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
        unsafe { C_SkFont_Destruct(self) }
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
        unsafe { C_SkFont_ConstructFromTypeface(&mut font, typeface.shared_native()) }
        font.into_handle()
    }

    pub fn from_typeface_with_size(typeface: &Typeface, size: f32) -> Self {
        let mut font : SkFont = unsafe { mem::uninitialized() };
        unsafe { C_SkFont_ConstructFromTypefaceWithSize(&mut font, typeface.shared_native(), size) }
        font.into_handle()
    }

    pub fn from_typeface_with_size_scale_and_skew(typeface: &Typeface, size: f32, scale: f32, skew: f32) -> Self {
        let mut font : SkFont = unsafe { mem::uninitialized() };
        unsafe { C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew(&mut font, typeface.shared_native(), size, scale, skew) }
        font.into_handle()
    }

    // suggestion: forces_auto_hinting().
    pub fn is_force_auto_hinting(&self) -> bool {
        unsafe { self.native().isForceAutoHinting() }
    }

    // suggestion: has_embedded_bitmaps().
    pub fn is_embedded_bitmaps(&self) -> bool {
        unsafe { self.native().isEmbeddedBitmaps() }
    }

    // supports_subpixel?
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

    pub fn set_force_autohinting(&mut self, force_auto_hinting: bool) {
        unsafe { self.native_mut().setForceAutoHinting(force_auto_hinting) }
    }

    pub fn set_embedded_bitmaps(&mut self, embedded_bitmaps: bool) {
        unsafe { self.native_mut().setEmbeddedBitmaps(embedded_bitmaps) }
    }

    pub fn set_subpixel(&mut self, subpixel: bool) {
        unsafe { self.native_mut().setSubpixel(subpixel) }
    }

    pub fn set_linear_metrics(&mut self, linear_metrics: bool) {
        unsafe { self.native_mut().setLinearMetrics(linear_metrics) }
    }

    pub fn set_embolden(&mut self, embolden: bool) {
        unsafe { self.native_mut().setEmbolden(embolden) }
    }

    pub fn edging(&self) -> FontEdging {
        unsafe { self.native().getEdging() }.into_handle()
    }

    pub fn set_edging(&mut self, edging: FontEdging) {
        unsafe { self.native_mut().setEdging(edging.native()) }
    }

    pub fn set_hinting(&mut self, hinting: FontHinting) {
        unsafe { self.native_mut().setHinting(hinting.native()) }
    }

    pub fn hinting(&self) -> FontHinting {
        unsafe { self.native().getHinting() }.into_handle()
    }

    #[warn(unused)]
    pub fn with_size(&self, size: f32) -> Option<Self> {
        if size >= 0.0 && !size.is_infinite() && !size.is_nan() {
            let mut font = unsafe { SkFont::new() };
            unsafe { C_SkFont_makeWithSize(self.native(), size, &mut font) }
            Some(font.into_handle())
        } else {
            None
        }
    }

    pub fn typeface(&self) -> Typeface {
        Typeface::from_unshared_ptr(unsafe { self.native().getTypeface() }).unwrap()
    }

    pub fn size(&self) -> f32 {
        unsafe { self.native().getSize() }
    }

    pub fn scale_x(&self) -> f32 {
        unsafe { self.native().getScaleX() }
    }

    pub fn skew_y(&self) -> f32 {
        unsafe { self.native().getSkewX() }
    }

    pub fn set_typeface(&mut self, tf: &Typeface) {
        unsafe { C_SkFont_setTypeface(self.native_mut(), tf.shared_native()) }
    }

    pub fn set_size(&mut self, size: f32) {
        unsafe { self.native_mut().setSize(size) }
    }

    pub fn set_scale_x(&mut self, scale_x: f32) {
        unsafe { self.native_mut().setScaleX(scale_x) }
    }

    pub fn set_skew_x(&mut self, skew_x: f32) {
        unsafe { self.native_mut().setSkewX(skew_x) }
    }

    // we support only UTF8 for now.
    pub fn str_to_glyphs(&self, str: &str, glyphs: &mut[GlyphId]) -> Option<usize> {
        let bytes = str.as_bytes();
        if bytes.len() > i32::max_value() as usize {
            return None;
        }

        Some(unsafe { self.native().textToGlyphs(
            bytes.as_ptr() as _,
            bytes.len(),
            TextEncoding::UTF8.native(),
            glyphs.as_mut_ptr(), glyphs.len().max(i32::max_value() as usize) as i32) as usize })
    }

    pub fn count_str(&self, str: &str) -> Option<usize> {
        let bytes = str.as_bytes();
        if bytes.len() > i32::max_value() as usize {
            return None;
        }
        Some(unsafe { self.native().textToGlyphs(
            bytes.as_ptr() as _,
            bytes.len(),
            TextEncoding::UTF8.native(),
            ptr::null_mut(), i32::max_value()) as usize })
    }

    // slower, but sooo convenient.
    pub fn str_to_glyphs_vec(&self, str: &str) -> Option<Vec<GlyphId>> {
        let count = self.count_str(str)?;
        let mut glyphs : Vec<GlyphId> = vec![Default::default(); count];
        let resulting_count = self.str_to_glyphs(str, glyphs.as_mut_slice())?;
        assert_eq!(count, resulting_count);
        Some(glyphs)
    }

    pub fn unichar_to_glyph(&self, uni: Unichar) -> u16 {
        unsafe { self.native().unicharToGlyph(uni) }
    }

    pub fn contains_str(&self, str: &str) -> bool {
        let bytes = str.as_bytes();
        if bytes.len() > i32::max_value() as usize {
            return false;
        }
        unsafe { self.native().containsText(bytes.as_ptr() as _, bytes.len(), TextEncoding::UTF8.native()) }
    }

    // note that the returned usize value is the bytes of the str that fits and not the characters.
    pub fn break_str(&self, str: &str, max_width: f32) -> Option<(usize, f32)> {
        let bytes = str.as_bytes();
        if bytes.len() > i32::max_value() as usize {
            return None;
        }

        let mut measured_width = 0.0;
        let bytes_fit = unsafe { self.native()
            .breakText(
                bytes.as_ptr() as _, bytes.len(), TextEncoding::UTF8.native(),
                max_width, &mut measured_width) };

        Some((bytes_fit, measured_width))
    }

    pub fn measure_str(&self, str: &str, paint: Option<&Paint>) -> Option<(f32, Rect)> {
        let bytes = str.as_bytes();
        if bytes.len() > i32::max_value() as usize {
            return None;
        }

        let mut bounds = SkRect {
            fLeft: 0.0,
            fTop: 0.0,
            fRight: 0.0,
            fBottom: 0.0
        };

        let width = unsafe { self.native()
            .measureText1(
                bytes.as_ptr() as _, bytes.len(), TextEncoding::UTF8.native(),
                &mut bounds, paint.native_ptr_or_null()) };

        Some((width, Rect::from_native(bounds)))
    }

    pub fn widths_bounds(
        &self,
        glyphs: &[u16],
        widths: Option<&mut [f32]>,
        bounds: Option<&mut [Rect]>,
        paint: Option<&Paint>) {
        // tbd: if we assert here when we go over i32::max_value() elements, we should problem
        // assert anywhere else, too. Chunking would be an option.
        assert!(glyphs.len() <= i32::max_value() as usize);
        let count = glyphs.len();
        let mut widths = widths;
        let mut bounds = bounds;

        if let Some(slice) = &widths { assert_eq!(count, slice.len()) };
        if let Some(slice) = &bounds { assert_eq!(count, slice.len()) };

        let mut bounds_vec : Option<Vec<SkRect>> = match &bounds {
            Some(_) => {
                Some(vec![SkRect {
                    fLeft: 0.0,
                    fTop: 0.0,
                    fRight: 0.0,
                    fBottom: 0.0
                }; count])
            },
            None => None
        };

        let bounds_ptr = bounds_vec.as_ptr_or_null_mut();
        let widths_ptr = widths.as_ptr_or_null_mut();
        let paint_ptr = paint.native_ptr_or_null();

        unsafe { self.native().getWidthsBounds(glyphs.as_ptr(), count as i32, widths_ptr, bounds_ptr, paint_ptr) }

        if let (Some(vec), Some(bounds)) = (&bounds_vec, &mut bounds) {
            vec.iter().enumerate().for_each(|(i, r)| bounds[i] = Rect::from_native(*r))
        }
    }

    pub fn pos(&self, glyphs: &[u16], pos: &mut [Point], origin: Option<Point>) {
        assert!(glyphs.len() <= i32::max_value() as usize);
        let count = glyphs.len();
        assert_eq!(count, pos.len());

        let mut pos_vec : Vec<SkPoint> =
            vec![SkPoint {
                fX: 0.0,
                fY: 0.0
            }; count];

        let pos_ptr = pos_vec.as_mut_ptr();

        let origin = origin.map(|p| p.into_native()).unwrap_or(SkPoint { fX: 0.0, fY: 0.0 });

        unsafe { self.native().getPos(glyphs.as_ptr(), count as i32, pos_vec.as_mut_ptr(), origin) }

        pos_vec
            .iter()
            .enumerate()
            .for_each(|(i, p)| pos[i] = Point::from_native(*p))
    }

    pub fn x_pos(&self, glyphs: &[u16], xpos: &mut [f32], origin: Option<f32>) {
        assert!(glyphs.len() <= i32::max_value() as usize);
        let count = glyphs.len();
        assert_eq!(count, xpos.len());
        let origin = origin.unwrap_or_default();

        unsafe { self.native().getXPos(glyphs.as_ptr(), count as i32, xpos.as_mut_ptr(), origin) }
    }

    pub fn path(&self, glyph_id: u16) -> Option<Path> {
        let mut path = Path::new();
        if unsafe { self.native().getPath(glyph_id, path.native_mut())} {
            Some(path)
        } else {
            None
        }
    }

    pub fn metrics(&self) -> (f32, FontMetrics) {
        let mut fm = unsafe { mem::zeroed() };
        let line_spacing = unsafe { self.native().getMetrics(&mut fm) };
        (line_spacing, FontMetrics::from_native(fm))
    }

    pub fn spacing(&self) -> f32 {
        unsafe { self.native().getMetrics(ptr::null_mut()) }
    }
}