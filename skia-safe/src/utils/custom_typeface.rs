use crate::{prelude::*, FontMetrics, FontStyle};
use crate::{GlyphId, Image, Paint, Path, Picture, Typeface};
use skia_bindings as sb;
use skia_bindings::SkCustomTypefaceBuilder;

pub type CustomTypefaceBuilder = Handle<SkCustomTypefaceBuilder>;
unsafe impl Send for CustomTypefaceBuilder {}
unsafe impl Sync for CustomTypefaceBuilder {}

impl NativeDrop for SkCustomTypefaceBuilder {
    fn drop(&mut self) {
        unsafe { sb::C_SkCustomTypefaceBuilder_destruct(self) }
    }
}

impl Handle<SkCustomTypefaceBuilder> {
    pub fn new() -> Self {
        Self::from_native(unsafe { SkCustomTypefaceBuilder::new() })
    }

    pub fn set_glyph<'a>(
        &mut self,
        glyph_id: GlyphId,
        advance: f32,
        typeface_glyph: impl Into<TypefaceGlyph<'a>>,
    ) -> &mut Self {
        unsafe {
            match typeface_glyph.into() {
                TypefaceGlyph::Path(path) => {
                    self.native_mut().setGlyph(glyph_id, advance, path.native())
                }
                TypefaceGlyph::PathAndPaint(_path, _paint) => {
                    unimplemented!("TypefaceGlyph::PathAndPaint is not supported yet, Skia implementation is missing (last checked: m86)")
                }
                TypefaceGlyph::Image { .. } => {
                    unimplemented!("TypefaceGlyph::PathAndPaint is not supported yet, Skia implementation is missing (last checked: m86)")
                }
                TypefaceGlyph::Picture(_picture) => {
                    unimplemented!("TypefaceGlyph::Picture is not supported yet, Skia implementation is missing (last checked: m86)")
                }
            }
        }
        self
    }

    pub fn set_metrics(
        &mut self,
        font_metrics: &FontMetrics,
        scale: impl Into<Option<f32>>,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .setMetrics(font_metrics.native(), scale.into().unwrap_or(1.0))
        }
        self
    }

    pub fn set_font_style(&mut self, font_style: FontStyle) -> &mut Self {
        unsafe { self.native_mut().setFontStyle(font_style.into_native()) }
        self
    }

    pub fn detach(&mut self) -> Option<Typeface> {
        Typeface::from_ptr(unsafe { sb::C_SkCustomTypefaceBuilder_detach(self.native_mut()) })
    }
}

pub enum TypefaceGlyph<'a> {
    Path(&'a Path),
    PathAndPaint(&'a Path, &'a Paint),
    Image { image: Image, scale: f32 },
    Picture(Picture),
}

impl<'a> From<&'a Path> for TypefaceGlyph<'a> {
    fn from(path: &'a Path) -> Self {
        Self::Path(path)
    }
}

impl<'a> From<(&'a Path, &'a Paint)> for TypefaceGlyph<'a> {
    fn from((path, paint): (&'a Path, &'a Paint)) -> Self {
        Self::PathAndPaint(path, paint)
    }
}

impl From<(Image, f32)> for TypefaceGlyph<'_> {
    fn from((image, scale): (Image, f32)) -> Self {
        Self::Image { image, scale }
    }
}

impl From<(&Image, f32)> for TypefaceGlyph<'_> {
    fn from((image, scale): (&Image, f32)) -> Self {
        Self::Image {
            image: image.clone(),
            scale,
        }
    }
}

#[test]
fn build_custom_typeface() {
    let mut builder = CustomTypefaceBuilder::new();
    let path = Path::new();
    builder.set_glyph(10u16, 0.0, &path);
    builder.set_glyph(11u16, 0.0, &path);
    let typeface = builder.detach().unwrap();
    assert_eq!(typeface.native().ref_counted_base()._ref_cnt(), 1);
}
