use crate::prelude::*;
use crate::{
    scalar, BlendMode, Color, Color4f, ColorFilter, ColorSpace, DrawLooper, FilterQuality,
    ImageFilter, MaskFilter, Path, PathEffect, Rect, Shader,
};
use skia_bindings::{
    C_SkPaint_Equals, C_SkPaint_destruct, C_SkPaint_getDrawLooper, C_SkPaint_setColorFilter,
    C_SkPaint_setDrawLooper, C_SkPaint_setImageFilter, C_SkPaint_setMaskFilter,
    C_SkPaint_setPathEffect, C_SkPaint_setShader, SkPaint, SkPaint_Cap, SkPaint_Join,
    SkPaint_Style,
};
use std::hash::{Hash, Hasher};
use std::ptr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Style {
    Stroke = SkPaint_Style::kStroke_Style as _,
    Fill = SkPaint_Style::kFill_Style as _,
    StrokeAndFill = SkPaint_Style::kStrokeAndFill_Style as _,
}

impl NativeTransmutable<SkPaint_Style> for Style {}
#[test]
fn test_paint_style_layout() {
    Style::test_layout()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Cap {
    Butt = SkPaint_Cap::kButt_Cap as _,
    Round = SkPaint_Cap::kRound_Cap as _,
    Square = SkPaint_Cap::kSquare_Cap as _,
}

impl NativeTransmutable<SkPaint_Cap> for Cap {}
#[test]
fn test_paint_cap_layout() {
    Cap::test_layout()
}

impl Default for Cap {
    fn default() -> Self {
        // SkPaint_Cap::kDefault_Cap
        Cap::Butt
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Join {
    Miter = SkPaint_Join::kMiter_Join as _,
    Round = SkPaint_Join::kRound_Join as _,
    Bevel = SkPaint_Join::kBevel_Join as _,
}

impl NativeTransmutable<SkPaint_Join> for Join {}
#[test]
fn test_paint_join_layout() {
    Join::test_layout()
}

impl Default for Join {
    fn default() -> Self {
        // SkPaint_Join::kDefault_Join
        Join::Miter
    }
}

pub type Paint = Handle<SkPaint>;

impl NativeDrop for SkPaint {
    fn drop(&mut self) {
        unsafe { C_SkPaint_destruct(self) }
    }
}

impl NativeClone for SkPaint {
    fn clone(&self) -> Self {
        unsafe { SkPaint::new1(self) }
    }
}

impl NativePartialEq for SkPaint {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkPaint_Equals(self, rhs) }
    }
}

impl NativeHash for SkPaint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { self.getHash() }.hash(state)
    }
}

impl Default for Handle<SkPaint> {
    fn default() -> Self {
        Paint::from_native(unsafe { SkPaint::new() })
    }
}

impl Handle<SkPaint> {
    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.native_mut().reset() }
        self
    }

    pub fn is_anti_alias(&self) -> bool {
        unsafe { self.native().isAntiAlias() }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) -> &mut Self {
        unsafe { self.native_mut().setAntiAlias(anti_alias) }
        self
    }

    pub fn is_dither(&self) -> bool {
        unsafe { self.native().isDither() }
    }

    pub fn set_dither(&mut self, dither: bool) -> &mut Self {
        unsafe { self.native_mut().setDither(dither) }
        self
    }

    pub fn filter_quality(&self) -> FilterQuality {
        FilterQuality::from_native(unsafe { self.native().getFilterQuality() })
    }

    pub fn set_filter_quality(&mut self, quality: FilterQuality) -> &mut Self {
        unsafe { self.native_mut().setFilterQuality(quality.into_native()) }
        self
    }

    pub fn style(&self) -> Style {
        Style::from_native(unsafe { self.native().getStyle() })
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        unsafe { self.native_mut().setStyle(style.into_native()) }
        self
    }

    pub fn color(&self) -> Color {
        Color::from_native(unsafe { self.native().getColor() })
    }

    pub fn color4f(&self) -> Color4f {
        Color4f::from_native(unsafe { self.native().getColor4f() })
    }

    pub fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        let color = color.into();
        unsafe { self.native_mut().setColor(color.into_native()) }
        self
    }

    pub fn set_color4f(
        &mut self,
        color: impl AsRef<Color4f>,
        color_space: &ColorSpace,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .setColor4f(color.as_ref().native(), color_space.native_mut_force())
        }
        self
    }

    pub fn alpha_f(&self) -> f32 {
        unsafe { self.native().getAlphaf() }
    }

    pub fn alpha(&self) -> u8 {
        unsafe { self.native().getAlpha() }
    }

    pub fn set_alpha_f(&mut self, alpha: f32) -> &mut Self {
        unsafe { self.native_mut().setAlphaf(alpha) }
        self
    }

    pub fn set_alpha(&mut self, alpha: u8) -> &mut Self {
        unsafe { self.native_mut().setAlpha(alpha.into()) }
        self
    }

    pub fn set_argb(&mut self, a: u8, r: u8, g: u8, b: u8) -> &mut Self {
        unsafe {
            self.native_mut()
                .setARGB(a.into(), r.into(), g.into(), b.into())
        }
        self
    }

    pub fn stroke_width(&self) -> scalar {
        unsafe { self.native().getStrokeWidth() }
    }

    pub fn set_stroke_width(&mut self, width: scalar) -> &mut Self {
        unsafe { self.native_mut().setStrokeWidth(width) }
        self
    }

    pub fn stroke_miter(&self) -> scalar {
        unsafe { self.native().getStrokeMiter() }
    }

    pub fn set_stroke_miter(&mut self, miter: scalar) -> &mut Self {
        unsafe { self.native_mut().setStrokeMiter(miter) }
        self
    }

    pub fn stroke_cap(&self) -> Cap {
        Cap::from_native(unsafe { self.native().getStrokeCap() })
    }

    pub fn set_stroke_cap(&mut self, cap: Cap) -> &mut Self {
        unsafe { self.native_mut().setStrokeCap(cap.into_native()) }
        self
    }

    pub fn stroke_join(&self) -> Join {
        Join::from_native(unsafe { self.native().getStrokeJoin() })
    }

    pub fn set_stroke_join(&mut self, join: Join) -> &mut Self {
        unsafe { self.native_mut().setStrokeJoin(join.into_native()) }
        self
    }

    pub fn get_fill_path(
        &self,
        src: &Path,
        cull_rect: Option<&Rect>,
        res_scale: impl Into<Option<scalar>>,
    ) -> Option<Path> {
        let mut r = Path::default();

        let cull_rect_ptr = cull_rect
            .map(|r| r.native() as *const _)
            .unwrap_or(ptr::null());

        unsafe {
            self.native().getFillPath(
                src.native(),
                r.native_mut(),
                cull_rect_ptr,
                res_scale.into().unwrap_or(1.0),
            )
        }
        .if_true_some(r)
    }

    pub fn shader(&self) -> Option<Shader> {
        Shader::from_unshared_ptr(unsafe { self.native().getShader() })
    }

    pub fn set_shader<'a>(&mut self, shader: impl Into<Option<&'a Shader>>) -> &mut Self {
        unsafe { C_SkPaint_setShader(self.native_mut(), shader.into().shared_ptr()) }
        self
    }

    pub fn color_filter(&self) -> Option<ColorFilter> {
        ColorFilter::from_unshared_ptr(unsafe { self.native().getColorFilter() })
    }

    pub fn set_color_filter<'a>(
        &mut self,
        color_filter: impl Into<Option<&'a ColorFilter>>,
    ) -> &mut Self {
        unsafe { C_SkPaint_setColorFilter(self.native_mut(), color_filter.into().shared_ptr()) }
        self
    }

    pub fn blend_mode(&self) -> BlendMode {
        BlendMode::from_native(unsafe { self.native().getBlendMode() })
    }

    pub fn is_src_over(&self) -> bool {
        unsafe { self.native().isSrcOver() }
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        unsafe { self.native_mut().setBlendMode(mode.into_native()) }
        self
    }

    pub fn path_effect(&self) -> Option<PathEffect> {
        PathEffect::from_unshared_ptr(unsafe { self.native().getPathEffect() })
    }

    pub fn set_path_effect<'a>(
        &mut self,
        path_effect: impl Into<Option<&'a PathEffect>>,
    ) -> &mut Self {
        unsafe { C_SkPaint_setPathEffect(self.native_mut(), path_effect.into().shared_ptr()) }
        self
    }

    pub fn mask_filter(&self) -> Option<MaskFilter> {
        MaskFilter::from_unshared_ptr(unsafe { self.native().getMaskFilter() })
    }

    pub fn set_mask_filter<'a>(
        &mut self,
        mask_filter: impl Into<Option<&'a MaskFilter>>,
    ) -> &mut Self {
        unsafe { C_SkPaint_setMaskFilter(self.native_mut(), mask_filter.into().shared_ptr()) }
        self
    }

    pub fn image_filter(&self) -> Option<ImageFilter> {
        ImageFilter::from_unshared_ptr(unsafe { self.native().getImageFilter() })
    }

    pub fn set_image_filter<'a>(
        &mut self,
        image_filter: impl Into<Option<&'a ImageFilter>>,
    ) -> &mut Self {
        unsafe { C_SkPaint_setImageFilter(self.native_mut(), image_filter.into().shared_ptr()) }
        self
    }

    pub fn draw_looper(&self) -> Option<DrawLooper> {
        DrawLooper::from_unshared_ptr(unsafe {
            // does not link on Windows:
            // self.native().getDrawLooper()
            C_SkPaint_getDrawLooper(self.native())
        })
    }

    pub fn set_draw_looper<'a>(
        &mut self,
        draw_looper: impl Into<Option<&'a DrawLooper>>,
    ) -> &mut Self {
        unsafe {
            C_SkPaint_setDrawLooper(self.native_mut(), draw_looper.into().shared_ptr());
        }
        self
    }

    pub fn nothing_to_draw(&self) -> bool {
        unsafe { self.native().nothingToDraw() }
    }
}

#[test]
fn default_creation() {
    let paint = Paint::default();
    drop(paint)
}

#[test]
fn method_chaining_compiles() {
    let mut paint = Paint::default();
    let _paint = paint.reset().reset();
}
