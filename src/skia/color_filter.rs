use std::mem;
use crate::prelude::*;
use crate::skia::{
    Color,
    BlendMode,
    Bitmap,
    Color4f,
    ColorSpace,
    scalar
};
use rust_skia::{
    C_SkColorFilter_MakeLinearToSRGBGamma,
    C_SkColorFilter_MakeMatrixFilterRowMajor255,
    C_SkColorFilter_makeComposed,
    C_SkColorFilter_getFlags,
    SkColorFilter_Flags,
    C_SkColorFilter_asComponentTable,
    C_SkColorFilter_asColorMatrix,
    C_SkColorFilter_asColorMode,
    SkColor,
    SkBlendMode,
    C_SkColorFilter_MakeModeFilter,
    SkRefCntBase,
    SkColorFilter,
    C_SkColorFilter_MakeSRGBToLinearGamma
};

bitflags! {
    pub struct ColorFilterFlags: u32 {
        const AlphaUnchanged = SkColorFilter_Flags::kAlphaUnchanged_Flag as u32;
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
        let mut color : SkColor = unsafe { mem::uninitialized() };
        let mut mode: SkBlendMode = unsafe { mem::uninitialized() };
        unsafe { C_SkColorFilter_asColorMode(self.native(), &mut color, &mut mode) }
            .if_true_some((Color::from_native(color), BlendMode::from_native(mode)))
    }

    pub fn as_color_matrix(&self) -> Option<[scalar; 20]> {
        let mut matrix : [scalar; 20] = unsafe { mem::uninitialized() };
        unsafe { C_SkColorFilter_asColorMatrix(self.native(), matrix.as_mut_ptr())}
            .if_true_some(matrix)
    }

    pub fn as_component_table(&self) -> Option<Bitmap> {
        let mut bitmap = Bitmap::new();
        unsafe { C_SkColorFilter_asComponentTable(self.native(), bitmap.native_mut())}
            .if_true_some(bitmap)
    }

    pub fn flags(&self) -> ColorFilterFlags {
        ColorFilterFlags::from_bits_truncate(unsafe {
            C_SkColorFilter_getFlags(self.native())
        })
    }

    pub fn filter_color(&self, color: Color) -> Color {
        Color::from_native(unsafe {
            self.native().filterColor(color.into_native())
        })
    }

    // TODO: check why and if ColorSpace needs to be mutable here.
    pub fn filter_color4f(&self, color: Color4f, color_space: &mut ColorSpace) -> Color4f {
        Color4f::from_native(unsafe {
            self.native().filterColor4f(color.native(), color_space.native_mut())
        })
    }

    pub fn new_mode_filter(c: Color, mode: BlendMode) -> Option<Self> {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_MakeModeFilter(c.native(), mode.native())
        })
    }

    #[warn(unused)]
    pub fn composed(&self, inner: &ColorFilter) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_makeComposed(self.native(), inner.shared_native() )
        })
    }

    pub fn from_matrix_row_major_255(matrix: [scalar; 20]) -> ColorFilter {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_MakeMatrixFilterRowMajor255(matrix.as_ptr())
        }).unwrap()
    }

    // TODO: not sure if we need the new_ prefix here.
    pub fn new_linear_to_srgb_gamma() -> ColorFilter {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_MakeLinearToSRGBGamma()
        }).unwrap()
    }

    // TODO: not sure if we need the new_ prefix here.
    pub fn new_srgb_to_linear_gamma() -> ColorFilter {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_MakeSRGBToLinearGamma()
        }).unwrap()
    }
}

#[test]
fn color_mode_roundtrip() {
    let color = Color::CYAN;
    let mode = BlendMode::ColorBurn;
    let cf = ColorFilter::new_mode_filter(color, mode).unwrap();
    let (c, m) = cf.as_color_mode().unwrap();
    assert!(color == c);
    assert_eq!(mode, m);
}

#[test]
fn ref_count() {
    let color = Color::CYAN;
    let mode = BlendMode::ColorBurn;
    let cf = ColorFilter::new_mode_filter(color, mode).unwrap();
    let rc = cf.native()._ref_cnt();
    assert_eq!(1, rc);
}