use crate::prelude::*;
use crate::core::{
    Path,
    Paint,
    scalar,
    paint
};
use skia_bindings::{
    SkStrokeRec_InitStyle,
    SkStrokeRec,
    SkStrokeRec_Style,
    C_SkStrokeRec_destruct,
    C_SkStrokeRec_copy,
    C_SkStrokeRec_hasEqualEffect
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum InitStyle {
    Hairline = SkStrokeRec_InitStyle::kHairline_InitStyle as _,
    Fill = SkStrokeRec_InitStyle::kFill_InitStyle as _
}

impl NativeTransmutable<SkStrokeRec_InitStyle> for InitStyle {}
#[test] fn test_stroke_rec_init_style_layout() { InitStyle::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Style {
    Hairline = SkStrokeRec_Style::kHairline_Style as _,
    Fill = SkStrokeRec_Style::kFill_Style as _,
    Stroke = SkStrokeRec_Style::kStroke_Style as _,
    StrokeAndFill = SkStrokeRec_Style::kStrokeAndFill_Style as _
}

impl NativeTransmutable<SkStrokeRec_Style> for Style {}
#[test] fn test_stroke_rec_style_layout() { Style::test_layout() }

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
    pub fn new(init_style: InitStyle) -> Self {
        Self::from_native(unsafe { SkStrokeRec::new(init_style.into_native() )})
    }

    // for convenience
    pub fn new_hairline() -> Self {
        Self::new(InitStyle::Hairline)
    }

    // for convenience
    pub fn new_fill() -> Self {
        Self::new(InitStyle::Fill)
    }

    pub fn from_paint(paint: &Paint, style: impl Into<Option<paint::Style>>, res_scale: impl Into<Option<scalar>>) -> Self {
        let res_scale = res_scale.into().unwrap_or(1.0);
        Self::from_native(unsafe {match style.into() {
            Some(style) => {
                SkStrokeRec::new1(paint.native(), style.into_native(), res_scale)
            },
            None => SkStrokeRec::new2(paint.native(), res_scale)
        }})
    }

    pub fn style(&self) -> Style {
        Style::from_native(unsafe { self.native().getStyle() })
    }

    pub fn width(&self) -> scalar {
        unsafe { self.native().getWidth() }
    }

    pub fn miter(&self) -> scalar {
        unsafe { self.native().getMiter() }
    }

    pub fn cap(&self) -> paint::Cap {
        paint::Cap::from_native(unsafe { self.native().getCap() })
    }

    pub fn join(&self) -> paint::Join {
        paint::Join::from_native(unsafe { self.native().getJoin() })
    }

    pub fn is_hairline_style(&self) -> bool {
        unsafe { self.native().isHairlineStyle() }
    }

    pub fn is_fill_style(&self) -> bool {
        unsafe { self.native().isFillStyle() }
    }

    pub fn set_fill_style(&mut self) -> &mut Self {
        unsafe { self.native_mut().setFillStyle() }
        self
    }

    pub fn set_hairline_style(&mut self) -> &mut Self {
        unsafe { self.native_mut().setHairlineStyle() }
        self
    }

    pub fn set_stroke_style(&mut self, width: scalar, stroke_and_fill: impl Into<Option<bool>>) -> &mut Self {
        let stroke_and_fill = stroke_and_fill.into().unwrap_or(false);
        unsafe {
            self.native_mut().setStrokeStyle(width, stroke_and_fill )
        }
        self
    }

    pub fn set_stroke_params(&mut self, cap: paint::Cap, join: paint::Join, miter_limit: scalar) -> &mut Self {
        unsafe {
            self.native_mut().setStrokeParams(cap.into_native(), join.into_native(), miter_limit)
        }
        self
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

    pub fn apply_to_path(&self, dst: &mut Path, src: &Path) -> bool {
        unsafe { self.native().applyToPath(dst.native_mut(), src.native()) }
    }

    pub fn apply_to_path_inplace(&self, path: &mut Path) -> bool {
        unsafe { self.native().applyToPath(path.native_mut(), path.native()) }
    }

    pub fn apply_to_paint(&self, paint: &mut Paint) {
        unsafe { self.native().applyToPaint(paint.native_mut()) }
    }

    pub fn inflation_radius(&self) -> scalar {
        unsafe { self.native().getInflationRadius() }
    }

    pub fn inflation_radius_from_paint_and_style(paint: &Paint, style: paint::Style) -> scalar {
        unsafe { SkStrokeRec::GetInflationRadius(paint.native(), style.into_native() ) }
    }

    pub fn inflation_radius_from_params(join: paint::Join, miter_limit: scalar, cap: paint::Cap, stroke_width: scalar) -> scalar {
        unsafe {
            SkStrokeRec::GetInflationRadius1(
                join.into_native(),
                miter_limit,
                cap.into_native(),
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
