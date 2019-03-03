use std::ptr;
use std::hash::{
    Hash,
    Hasher
};
use crate::prelude::*;
use crate::skia::{
    Color,
    FontHinting,
    FilterQuality,
    Color4f,
    ColorSpace,
    scalar,
    Path,
    Rect,
    ColorFilter,
    BlendMode,
    PathEffect,
    MaskFilter,
    Typeface
};
use rust_skia::{
    C_SkPaint_setMaskFilter,
    C_SkPaint_setPathEffect,
    C_SkPaint_setColorFilter,
    SkRect,
    SkPaint_Cap,
    SkPaint,
    C_SkPaint_destruct,
    SkPaint_Style,
    SkPaint_Flags,
    SkPaint_Join,
    C_SkPaint_Equals,
    C_SkPaint_setTypeface
};

bitflags! {
    pub struct PaintFlags: u32 {
        const AntiAlias = SkPaint_Flags::kAntiAlias_Flag as _;
        const Dither = SkPaint_Flags::kDither_Flag as _;
        const FakeBoldText = SkPaint_Flags::kFakeBoldText_Flag as _;
        const LinearText = SkPaint_Flags::kLinearText_Flag as _;
        const SubpixelText = SkPaint_Flags::kSubpixelText_Flag as _;
        const LCDRenderText = SkPaint_Flags::kLCDRenderText_Flag as _;
        const EmbeddedBitmapText = SkPaint_Flags::kEmbeddedBitmapText_Flag as _;
        const AutoHinting = SkPaint_Flags::kAutoHinting_Flag as _;
    }
}

pub type PaintStyle = EnumHandle<SkPaint_Style>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPaint_Style> {
    pub const Stroke: Self = Self(SkPaint_Style::kStroke_Style);
    pub const Fill: Self = Self(SkPaint_Style::kFill_Style);
    pub const StrokeAndFill: Self = Self(SkPaint_Style::kStrokeAndFill_Style);
}

pub type PaintCap = EnumHandle<SkPaint_Cap>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPaint_Cap> {
    pub const Butt: Self = Self(SkPaint_Cap::kButt_Cap);
    pub const Round: Self = Self(SkPaint_Cap::kRound_Cap);
    pub const Square: Self = Self(SkPaint_Cap::kSquare_Cap);
}

impl Default for EnumHandle<SkPaint_Cap> {
    fn default() -> Self {
        Self(SkPaint_Cap::kDefault_Cap)
    }
}

pub type PaintJoin = EnumHandle<SkPaint_Join>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPaint_Join> {
    pub const Miter: Self = Self(SkPaint_Join::kMiter_Join);
    pub const Round: Self = Self(SkPaint_Join::kRound_Join);
    pub const Bevel: Self = Self(SkPaint_Join::kBevel_Join);
}

impl Default for EnumHandle<SkPaint_Join> {
    fn default() -> Self {
        Self(SkPaint_Join::kDefault_Join)
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

    pub fn set_hinting(&mut self, hinting_level: FontHinting) -> &mut Self {
        unsafe { self.native_mut().setHinting(hinting_level.native()) }
        self
    }

    pub fn hinting(&self) -> FontHinting {
        unsafe { self.native().getHinting() }
            .into_handle()
    }

    pub fn flags(&self) -> PaintFlags {
        PaintFlags::from_bits_truncate(unsafe {
            self.native().getFlags()
        })
    }

    pub fn set_flags(&mut self, flags: PaintFlags) -> &mut Self {
        unsafe { self.native_mut().setFlags(flags.bits()) }
        self
    }

    pub fn anti_alias(&self) -> bool {
        unsafe { self.native().isAntiAlias() }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) -> &mut Self {
        unsafe { self.native_mut().setAntiAlias(anti_alias) }
        self
    }

    pub fn dither(&self) -> bool {
        unsafe { self.native().isDither() }
    }

    pub fn set_dither(&mut self, dither: bool) -> &mut Self {
        unsafe { self.native_mut().setDither(dither) }
        self
    }

    pub fn linear_text(&self) -> bool {
        // does not link
        // unsafe { self.native().isLinearText() }
        self.flags().contains(PaintFlags::LinearText)
    }

    pub fn set_linear_text(&mut self, linear_text: bool) -> &mut Self {
        unsafe { self.native_mut().setLinearText(linear_text) }
        self
    }

    pub fn subpixel_text(&self) -> bool {
        // does not link
        // unsafe { self.native().isSubpixelText() }
        self.flags().contains(PaintFlags::SubpixelText)
    }

    pub fn set_subpixel_text(&mut self, subpixel_text: bool) -> &mut Self {
        unsafe { self.native_mut().setSubpixelText(subpixel_text) }
        self
    }

    pub fn lcd_render_text(&self) -> bool {
        // does not link:
        // unsafe { self.native().isLCDRenderText() }
        self.flags().contains(PaintFlags::LCDRenderText)
    }

    pub fn set_lcd_render_text(&mut self, lcd_text: bool) -> &mut Self {
        unsafe { self.native_mut().setLCDRenderText(lcd_text) }
        self
    }

    pub fn embedded_bitmap_text(&self) -> bool {
        // does not link:
        // unsafe { self.native().isEmbeddedBitmapText() }
        self.flags().contains(PaintFlags::EmbeddedBitmapText)
    }

    pub fn set_embedded_bitmap_text(&mut self, use_embedded_bitmap_text: bool) -> &mut Self {
        unsafe { self.native_mut().setEmbeddedBitmapText(use_embedded_bitmap_text) }
        self
    }

    pub fn autohinted(&self) -> bool {
        // does not link:
        // unsafe { self.native().isAutohinted() }
        self.flags().contains(PaintFlags::AutoHinting)
    }

    pub fn set_autohinted(&mut self, use_autohinter: bool) -> &mut Self {
        unsafe { self.native_mut().setAutohinted(use_autohinter) }
        self
    }

    pub fn fake_bold_text(&self) -> bool {
        // does not link:
        // unsafe { self.native().isFakeBoldText() }
        self.flags().contains(PaintFlags::FakeBoldText)
    }

    pub fn set_fake_bold_text(&mut self, fake_bold_text: bool) -> &mut Self {
        unsafe { self.native_mut().setFakeBoldText(fake_bold_text) }
        self
    }

    pub fn filter_quality(&self) -> FilterQuality {
        unsafe { self.native().getFilterQuality() }
            .into_handle()
    }

    pub fn set_filter_quality(&mut self, quality: FilterQuality) -> &mut Self {
        unsafe { self.native_mut().setFilterQuality(quality.native()) }
        self
    }

    pub fn style(&self) -> PaintStyle {
        unsafe { self.native().getStyle() }.into_handle()
    }

    pub fn set_style(&mut self, style: PaintStyle) -> &mut Self {
        unsafe { self.native_mut().setStyle(style.native()) }
        self
    }

    pub fn color(&self) -> Color {
        Color::from_native(unsafe { self.native().getColor() })
    }

    pub fn color4f(&self) -> Color4f {
        Color4f::from_native(unsafe { self.native().getColor4f() })
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        unsafe { self.native_mut().setColor(color.into_native()) }
        self
    }

    // TODO: why is ColorSpace mutable?
    pub fn set_color4f(&mut self, color: Color4f, color_space: &mut ColorSpace) -> &mut Self {
        unsafe {
            self.native_mut().setColor4f(
                &color.into_native(),
                color_space.native_mut() )
        }
        self
    }

    pub fn alpha(&self) -> u8 {
        unsafe { self.native().getAlpha() }
    }

    pub fn set_alpha(&mut self, alpha: u8) -> &mut Self {
        unsafe { self.native_mut().setAlpha(alpha as _) }
        self
    }

    pub fn set_argb(&mut self, a: u8, r: u8, g: u8, b: u8) -> &mut Self {
        unsafe { self.native_mut().setARGB(a as _, r as _, g as _, b as _)}
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
        unsafe { self.native().getStrokeCap() }
            .into_handle()
    }

    pub fn set_stroke_cap(&mut self, cap: PaintCap) -> &mut Self {
        unsafe { self.native_mut().setStrokeCap(cap.native()) }
        self
    }

    pub fn stroke_join(&self) -> PaintJoin {
        unsafe { self.native().getStrokeJoin() }
            .into_handle()
    }

    pub fn set_stroke_join(&mut self, join: PaintJoin) -> &mut Self {
        unsafe { self.native_mut().setStrokeJoin(join.native()) }
        self
    }

    pub fn fill_path(&self, src: &Path, cull_rect: Option<&Rect>, res_scale: Option<scalar>) -> Option<Path> {
        let mut r = Path::new();

        let cull_rect_ptr : *const SkRect =
            cull_rect
                .map(|r| &r.into_native() as _)
                .unwrap_or(ptr::null());

        unsafe { self.native().getFillPath(
            src.native(),
            r.native_mut(),
            cull_rect_ptr,
            res_scale.unwrap_or(1.0))
        }
        .if_true_some(r)
    }

    pub fn color_filter(&self) -> Option<ColorFilter> {
        ColorFilter::from_unshared_ptr(unsafe {
            self.native().getColorFilter()
        })
    }


    pub fn set_color_filter(&mut self, color_filter: Option<&ColorFilter>) -> &mut Self {
        unsafe {
            C_SkPaint_setColorFilter(self.native_mut(), color_filter.shared_ptr())
        }
        self
    }

    pub fn blend_mode(&self) -> BlendMode {
        unsafe {
            self.native().getBlendMode()
        }.into_handle()
    }

    pub fn src_over(&self) -> bool {
        unsafe {
            self.native().isSrcOver()
        }
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        unsafe {
            self.native_mut().setBlendMode(mode.native())
        }
        self
    }

    pub fn path_effect(&self) -> Option<PathEffect> {
        PathEffect::from_unshared_ptr(unsafe {
            self.native().getPathEffect()
        })
    }

    pub fn set_path_effect(&mut self, path_effect: Option<&PathEffect>) -> &mut Self {
        unsafe {
            C_SkPaint_setPathEffect(self.native_mut(), path_effect.shared_ptr())
        }
        self
    }

    pub fn mask_filter(&self) -> Option<MaskFilter> {
        MaskFilter::from_unshared_ptr(unsafe {
            self.native().getMaskFilter()
        })
    }

    pub fn set_mask_filter(&mut self, mask_filter: Option<&MaskFilter>) -> &mut Self {
        unsafe {
            C_SkPaint_setMaskFilter(self.native_mut(), mask_filter.shared_ptr())
        }
        self
    }

    pub fn typeface(&self) -> Option<Typeface> {
        Typeface::from_unshared_ptr(unsafe {
            self.native().getTypeface()
        })
    }

    pub fn set_typeface(&mut self, typeface: Option<&Typeface>) -> &mut Self {
        unsafe {
            C_SkPaint_setTypeface(self.native_mut(), typeface.shared_ptr())
        }
        self
    }

    /* TODO: ImageFilter postponed

    pub fn image_filter(&self) -> Option<ImageFilter> {
        ImageFilter::from_unshared_ptr(unsafe {
            self.native().getImageFilter()
        })
    }

    pub fn set_image_filter(&mut self, image_filter: Option<&ImageFilter>) -> &mut Self {
        unsafe {
            C_SkPaint_setImageFilter(self.native_mut(), image_filter.shared_ptr())
        }
        self
    }

    */

    // TODO: getDrawLooper, setDrawLooper

    pub fn text_size(&self) -> scalar {
        unsafe { self.native().getTextSize() }
    }

    pub fn set_text_size(&mut self, text_size: scalar) -> &mut Self {
        unsafe { self.native_mut().setTextSize(text_size) }
        self
    }

    pub fn text_scale_x(&self) -> scalar {
        unsafe { self.native().getTextScaleX() }
    }

    pub fn set_text_scale_x(&mut self, scale_x: scalar) -> &mut Self {
        unsafe { self.native_mut().setTextScaleX(scale_x) }
        self
    }

    pub fn text_skew_x(&self) -> scalar {
        unsafe { self.native().getTextSkewX() }
    }

    pub fn set_text_skew_x(&mut self, skew_x: scalar) -> &mut Self {
        unsafe { self.native_mut().setTextSkewX(skew_x) }
        self
    }

    // TODO: getTextBlobIntercepts

    pub fn nothing_to_draw(&self) -> bool {
        unsafe { self.native().nothingToDraw() }
    }
}

#[test]
fn default_creation() {
    let mut paint = Paint::default();
}

fn method_chaining_compiles() {
    let mut paint = Paint::default();
    let paint = paint.reset().reset();
}