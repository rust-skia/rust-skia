use std::ptr;
use std::hash::{
    Hash,
    Hasher
};
use crate::prelude::*;
use crate::{Color, FilterQuality, Color4f, ColorSpace, scalar, Path, Rect, ColorFilter, BlendMode, PathEffect, MaskFilter, Shader, ImageFilter, DrawLooper};
use skia_bindings::{C_SkPaint_setMaskFilter, C_SkPaint_setPathEffect, C_SkPaint_setColorFilter, SkPaint_Cap, SkPaint, C_SkPaint_destruct, SkPaint_Style, SkPaint_Join, C_SkPaint_Equals, C_SkPaint_setShader, C_SkPaint_setImageFilter, C_SkPaint_setDrawLooper, C_SkPaint_getDrawLooper };

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PaintStyle {
    Stroke = SkPaint_Style::kStroke_Style as _,
    Fill = SkPaint_Style::kFill_Style as _,
    StrokeAndFill = SkPaint_Style::kStrokeAndFill_Style as _
}

impl NativeTransmutable<SkPaint_Style> for PaintStyle {}
#[test] fn test_paint_style_layout() { PaintStyle::test_layout() }


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PaintCap {
    Butt = SkPaint_Cap::kButt_Cap as _,
    Round = SkPaint_Cap::kRound_Cap as _,
    Square = SkPaint_Cap::kSquare_Cap as _
}

impl NativeTransmutable<SkPaint_Cap> for PaintCap {}
#[test] fn test_paint_cap_layout() { PaintCap::test_layout() }

impl Default for PaintCap {
    fn default() -> Self {
        // SkPaint_Cap::kDefault_Cap
        PaintCap::Butt
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PaintJoin {
    Miter = SkPaint_Join::kMiter_Join as _,
    Round = SkPaint_Join::kRound_Join as _,
    Bevel = SkPaint_Join::kBevel_Join as _,
}

impl NativeTransmutable<SkPaint_Join> for PaintJoin {}
#[test] fn test_paint_join_layout() { PaintStyle::test_layout() }

impl Default for PaintJoin {
    fn default() -> Self {
        // SkPaint_Join::kDefault_Join
        PaintJoin::Miter
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

    pub fn style(&self) -> PaintStyle {
        PaintStyle::from_native(unsafe { self.native().getStyle() })
    }

    pub fn set_style(&mut self, style: PaintStyle) -> &mut Self {
        unsafe { self.native_mut().setStyle(style.into_native()) }
        self
    }

    pub fn color(&self) -> Color {
        Color::from_native(unsafe { self.native().getColor() })
    }

    pub fn color4f(&self) -> Color4f {
        Color4f::from_native(unsafe { self.native().getColor4f() })
    }

    pub fn set_color<C: Into<Color>>(&mut self, color: C) -> &mut Self {
        let color = color.into();
        unsafe { self.native_mut().setColor(color.into_native()) }
        self
    }

    pub fn set_color4f<C: AsRef<Color4f>>(&mut self, color: C, color_space: &ColorSpace) -> &mut Self {
        unsafe {
            self.native_mut().setColor4f(
                color.as_ref().native(),
                color_space.native_mut_force() )
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
        unsafe { self.native_mut().setARGB(a.into(), r.into(), g.into(), b.into())}
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

    pub fn stroke_cap(&self) -> PaintCap {
        PaintCap::from_native(unsafe { self.native().getStrokeCap() })
    }

    pub fn set_stroke_cap(&mut self, cap: PaintCap) -> &mut Self {
        unsafe { self.native_mut().setStrokeCap(cap.into_native()) }
        self
    }

    pub fn stroke_join(&self) -> PaintJoin {
        PaintJoin::from_native(unsafe { self.native().getStrokeJoin() })
    }

    pub fn set_stroke_join(&mut self, join: PaintJoin) -> &mut Self {
        unsafe { self.native_mut().setStrokeJoin(join.into_native()) }
        self
    }

    // getFillPath is not the typical getter we can strip the prefix "get" from, so we
    // stick to the original name of the Skia method.
    pub fn get_fill_path(&self, src: &Path, cull_rect: Option<&Rect>, res_scale: Option<scalar>) -> Option<Path> {
        let mut r = Path::default();

        let cull_rect_ptr =
            cull_rect
                .map(|r| r.native() as *const _)
                .unwrap_or(ptr::null());

        unsafe { self.native().getFillPath(
            src.native(),
            r.native_mut(),
            cull_rect_ptr,
            res_scale.unwrap_or(1.0))
        }
        .if_true_some(r)
    }

    pub fn shader(&self) -> Option<Shader> {
        Shader::from_unshared_ptr(unsafe {
            self.native().getShader()
        })
    }

    pub fn set_shader<'a, S: Into<Option<&'a Shader>>>(&mut self, shader: S) -> &mut Self {
        unsafe {
            C_SkPaint_setShader(self.native_mut(), shader.into().shared_ptr())
        }
        self
    }

    pub fn color_filter(&self) -> Option<ColorFilter> {
        ColorFilter::from_unshared_ptr(unsafe {
            self.native().getColorFilter()
        })
    }


    pub fn set_color_filter<'a, CF: Into<Option<&'a ColorFilter>>>(&mut self, color_filter: CF) -> &mut Self {
        unsafe {
            C_SkPaint_setColorFilter(self.native_mut(), color_filter.into().shared_ptr())
        }
        self
    }

    pub fn blend_mode(&self) -> BlendMode {
        BlendMode::from_native(unsafe {
            self.native().getBlendMode()
        })
    }

    pub fn is_src_over(&self) -> bool {
        unsafe {
            self.native().isSrcOver()
        }
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        unsafe {
            self.native_mut().setBlendMode(mode.into_native())
        }
        self
    }

    pub fn path_effect(&self) -> Option<PathEffect> {
        PathEffect::from_unshared_ptr(unsafe {
            self.native().getPathEffect()
        })
    }

    pub fn set_path_effect<'a, PE: Into<Option<&'a PathEffect>>>(&mut self, path_effect: PE) -> &mut Self {
        unsafe {
            C_SkPaint_setPathEffect(self.native_mut(), path_effect.into().shared_ptr())
        }
        self
    }

    pub fn mask_filter(&self) -> Option<MaskFilter> {
        MaskFilter::from_unshared_ptr(unsafe {
            self.native().getMaskFilter()
        })
    }

    pub fn set_mask_filter<'a, MF: Into<Option<&'a MaskFilter>>>(&mut self, mask_filter: MF) -> &mut Self {
        unsafe {
            C_SkPaint_setMaskFilter(self.native_mut(), mask_filter.into().shared_ptr())
        }
        self
    }

    pub fn image_filter(&self) -> Option<ImageFilter> {
        ImageFilter::from_unshared_ptr(unsafe {
            self.native().getImageFilter()
        })
    }

    pub fn set_image_filter<'a, IF: Into<Option<&'a ImageFilter>>>(&mut self, image_filter: IF) -> &mut Self {
        unsafe {
            C_SkPaint_setImageFilter(self.native_mut(), image_filter.into().shared_ptr())
        }
        self
    }

    pub fn draw_looper(&self) -> Option<DrawLooper> {
        DrawLooper::from_unshared_ptr(unsafe {
            // does not link on Windows:
            // self.native().getDrawLooper()
            C_SkPaint_getDrawLooper(self.native())
        })
    }

    pub fn set_draw_looper<'a, IDL: Into<Option<&'a DrawLooper>>>(&mut self, draw_looper: IDL) -> &mut Self {
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