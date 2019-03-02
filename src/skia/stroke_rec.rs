use crate::prelude::*;
use crate::skia::{
    Path,
    PaintJoin,
    PaintCap,
    PaintStyle,
    Paint,
    scalar
};
use rust_skia::{
    SkStrokeRec_InitStyle,
    SkStrokeRec,
    SkStrokeRec_Style,
    C_SkStrokeRec_destruct,
    C_SkStrokeRec_copy,
    C_SkStrokeRec_hasEqualEffect
};

pub type StrokeRecInitStyle = EnumHandle<SkStrokeRec_InitStyle>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkStrokeRec_InitStyle> {
    pub const Hairline: Self = Self(SkStrokeRec_InitStyle::kHairline_InitStyle);
    pub const Fill: Self = Self(SkStrokeRec_InitStyle::kFill_InitStyle);
}

pub type StrokeRecStyle = EnumHandle<SkStrokeRec_Style>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkStrokeRec_Style> {
    pub const Hairline: Self = Self(SkStrokeRec_Style::kHairline_Style);
    pub const Fill: Self = Self(SkStrokeRec_Style::kFill_Style);
    pub const Stroke: Self = Self(SkStrokeRec_Style::kStroke_Style);
    pub const StrokeAndFill: Self = Self(SkStrokeRec_Style::kStrokeAndFill_Style);
}

pub type StrokeRec = Handle<SkStrokeRec>;

impl NativeDrop for SkStrokeRec {
    fn drop(&mut self) {
        unsafe { C_SkStrokeRec_destruct(self) };
    }
}

impl NativeClone for SkStrokeRec {
    fn clone(&self) -> Self {
        let mut copy = StrokeRec::new_hairline();
        unsafe { C_SkStrokeRec_copy(self, copy.native_mut()) }
        *copy.native()
    }
}

impl Handle<SkStrokeRec> {
    pub fn new(init_style: StrokeRecInitStyle) -> Self {
        unsafe { SkStrokeRec::new(init_style.native() )}
            .into_handle()
    }

    // for convenience
    pub fn new_hairline() -> Self {
        Self::new(StrokeRecInitStyle::Hairline)
    }

    // for convenience
    pub fn new_fill() -> Self {
        Self::new(StrokeRecInitStyle::Fill)
    }

    pub fn from_paint(paint: &Paint, style: Option<PaintStyle>, res_scale: Option<scalar>) -> Self {
        let res_scale = res_scale.unwrap_or(1.0);
        match style {
            Some(style) => {
                unsafe { SkStrokeRec::new1(paint.native(), style.native(), res_scale)}
                    .into_handle()
            },
            None => {
                unsafe { SkStrokeRec::new2(paint.native(), res_scale)}
                    .into_handle()
            }
        }
    }

    pub fn style(&self) -> StrokeRecStyle {
        unsafe { self.native().getStyle() }
            .into_handle()
    }

    pub fn width(&self) -> scalar {
        unsafe { self.native().getWidth() }
    }

    pub fn miter(&self) -> scalar {
        unsafe { self.native().getMiter() }
    }

    pub fn cap(&self) -> PaintCap {
        unsafe { self.native().getCap() }
            .into_handle()
    }

    pub fn join(&self) -> PaintJoin {
        unsafe { self.native().getJoin() }
            .into_handle()
    }

    pub fn is_hairline_style(&self) -> bool {
        unsafe { self.native().isHairlineStyle() }
    }

    pub fn is_fill_style(&self) -> bool {
        unsafe { self.native().isFillStyle() }
    }

    pub fn set_fill_style(&mut self) {
        unsafe { self.native_mut().setFillStyle() }
    }

    pub fn set_hairline_style(&mut self) {
        unsafe { self.native_mut().setHairlineStyle() }
    }

    pub fn set_stroke_style(&mut self, width: scalar, stroke_and_fill: Option<bool>) {
        let stroke_and_fill = stroke_and_fill.unwrap_or(false);
        unsafe { self.native_mut().setStrokeStyle(width, stroke_and_fill )}
    }

    pub fn set_stroke_params(&mut self, cap: PaintCap, join: PaintJoin, miter_limit: scalar) {
        unsafe {
            self.native_mut().setStrokeParams(cap.native(), join.native(), miter_limit)
        }
    }

    pub fn res_scale(&self) -> scalar {
        unsafe { self.native().getResScale() }
    }

    pub fn set_res_scale(&mut self, rs: scalar) {
        unsafe { self.native_mut().setResScale(rs) }
    }

    pub fn need_to_apply(&self) -> bool {
        unsafe { self.native().needToApply() }
    }

    pub fn apply_to_path(&self, path: &mut Path) -> bool {
        unsafe { self.native().applyToPath(path.native_mut(), path.native()) }
    }

    pub fn apply_to_paint(&self, paint: &mut Paint) {
        unsafe { self.native().applyToPaint(paint.native_mut()) }
    }

    pub fn inflation_radius(&self) -> scalar {
        unsafe { self.native().getInflationRadius() }
    }

    pub fn inflation_radius_from_paint_and_style(paint: &Paint, style: PaintStyle) -> scalar {
        unsafe { SkStrokeRec::GetInflationRadius(paint.native(), style.native() ) }
    }

    pub fn inflation_radius_from_params(join: PaintJoin, miter_limit: scalar, cap: PaintCap, stroke_width: scalar) -> scalar {
        unsafe {
            SkStrokeRec::GetInflationRadius1(
                join.native(),
                miter_limit,
                cap.native(),
                stroke_width)
        }
    }

    pub fn has_equal_effect(&self, other: &StrokeRec) -> bool {
        // does not link:
        // unsafe {
        //     self.native().hasEqualEffect(other.native())
        // }
        unsafe {
            C_SkStrokeRec_hasEqualEffect(self.native(), other.native())
        }
    }
}