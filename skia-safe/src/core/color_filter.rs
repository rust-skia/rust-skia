use crate::core::{scalar, BlendMode, Color, Color4f, ColorSpace};
use crate::prelude::*;
use skia_bindings::{
    C_SkColorFilter_asColorMatrix, C_SkColorFilter_asColorMode, C_SkColorFilter_getFlags,
    C_SkColorFilter_makeComposed, C_SkColorFilters_Blend, C_SkColorFilters_Compose,
    C_SkColorFilters_Lerp, C_SkColorFilters_LinearToSRGBGamma, C_SkColorFilters_MatrixRowMajor255,
    C_SkColorFilters_SRGBToLinearGamma, SkBlendMode, SkColor, SkColorFilter,
    SkColorFilter_Flags_kAlphaUnchanged_Flag, SkRefCntBase,
};
use std::mem;

bitflags! {
    pub struct ColorFilterFlags: u32 {
        const ALPHA_UNCHANGED = SkColorFilter_Flags_kAlphaUnchanged_Flag as u32;
    }
}

pub type ColorFilter = RCHandle<SkColorFilter>;

impl NativeRefCountedBase for SkColorFilter {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl RCHandle<SkColorFilter> {
    pub fn as_color_mode(&self) -> Option<(Color, BlendMode)> {
        let mut color: SkColor = unsafe { mem::uninitialized() };
        let mut mode: SkBlendMode = unsafe { mem::uninitialized() };
        unsafe { C_SkColorFilter_asColorMode(self.native(), &mut color, &mut mode) }
            .if_true_some((Color::from_native(color), BlendMode::from_native(mode)))
    }

    pub fn as_color_matrix(&self) -> Option<[scalar; 20]> {
        let mut matrix: [scalar; 20] = unsafe { mem::uninitialized() };
        unsafe { C_SkColorFilter_asColorMatrix(self.native(), matrix.as_mut_ptr()) }
            .if_true_some(matrix)
    }

    pub fn flags(&self) -> ColorFilterFlags {
        ColorFilterFlags::from_bits_truncate(unsafe { C_SkColorFilter_getFlags(self.native()) })
    }

    pub fn filter_color<C: Into<Color>>(&self, color: C) -> Color {
        Color::from_native(unsafe { self.native().filterColor(color.into().into_native()) })
    }

    pub fn filter_color4f<C: AsRef<Color4f>>(&self, color: Color4f, color_space: &ColorSpace) -> Color4f {
        Color4f::from_native(unsafe {
            self.native()
                .filterColor4f(color.as_ref().native(), color_space.native_mut_force())
        })
    }

    #[must_use]
    pub fn composed(&self, inner: &ColorFilter) -> Option<Self> {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_makeComposed(self.native(), inner.shared_native())
        })
    }
}

pub enum ColorFilters {}

impl ColorFilters {
    pub fn compose(outer: &ColorFilter, inner: &ColorFilter) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilters_Compose(outer.shared_native(), inner.shared_native())
        })
    }

    pub fn matrix_row_major_255(array: &[scalar; 20]) -> ColorFilter {
        ColorFilter::from_ptr(unsafe { C_SkColorFilters_MatrixRowMajor255(array.as_ptr()) })
            .unwrap()
    }

    pub fn blend<C: Into<Color>>(c: C, mode: BlendMode) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilters_Blend(c.into().into_native(), mode.into_native())
        })
    }

    pub fn linear_to_srgb_gamma() -> ColorFilter {
        ColorFilter::from_ptr(unsafe { C_SkColorFilters_LinearToSRGBGamma() }).unwrap()
    }

    pub fn srgb_to_linear_gamma() -> ColorFilter {
        ColorFilter::from_ptr(unsafe { C_SkColorFilters_SRGBToLinearGamma() }).unwrap()
    }

    pub fn lerp(t: f32, dst: &ColorFilter, src: &ColorFilter) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilters_Lerp(t, dst.shared_native(), src.shared_native())
        })
    }
}

#[test]
fn color_mode_roundtrip() {
    let color = Color::CYAN;
    let mode = BlendMode::ColorBurn;
    let cf = ColorFilters::blend(color, mode).unwrap();
    let (c, m) = cf.as_color_mode().unwrap();
    assert!(color == c);
    assert_eq!(mode, m);
}

#[test]
fn ref_count() {
    let color = Color::CYAN;
    let mode = BlendMode::ColorBurn;
    let cf = ColorFilters::blend(color, mode).unwrap();
    let rc = cf.native()._ref_cnt();
    assert_eq!(1, rc);
}
