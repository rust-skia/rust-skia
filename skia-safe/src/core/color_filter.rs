use std::mem;
use crate::prelude::*;
use crate::core::{
    Color,
    BlendMode,
    Bitmap,
    Color4f,
    ColorSpace,
    scalar
};
use skia_bindings::{C_SkColorFilter_MakeLinearToSRGBGamma, C_SkColorFilter_MakeMatrixFilterRowMajor255, C_SkColorFilter_makeComposed, C_SkColorFilter_getFlags, C_SkColorFilter_asComponentTable, C_SkColorFilter_asColorMatrix, C_SkColorFilter_asColorMode, SkColor, SkBlendMode, C_SkColorFilter_MakeModeFilter, SkRefCntBase, SkColorFilter, C_SkColorFilter_MakeSRGBToLinearGamma, SkColorFilter_Flags_kAlphaUnchanged_Flag, C_SkColorFilter_MakeMixer};

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

    #[deprecated(since = "0.11.0", note = "use to_color_mode()")]
    pub fn as_color_mode(&self) -> Option<(Color, BlendMode)> {
        self.to_color_mode()
    }

    pub fn to_color_mode(&self) -> Option<(Color, BlendMode)> {
        let mut color: Color = 0.into();
        let mut mode: BlendMode = BlendMode::default();
        unsafe { C_SkColorFilter_asColorMode(self.native(), color.native_mut(), mode.native_mut()) }
            .if_true_some((color, mode))
    }

    pub fn to_color_matrix(&self) -> Option<[scalar; 20]> {
        let mut matrix : [scalar; 20] = Default::default();
        unsafe { C_SkColorFilter_asColorMatrix(self.native(), matrix.as_mut_ptr())}
            .if_true_some(matrix)
    }

    pub fn to_component_table(&self) -> Option<Bitmap> {
        let mut bitmap = Bitmap::new();
        unsafe { C_SkColorFilter_asComponentTable(self.native(), bitmap.native_mut())}
            .if_true_some(bitmap)
    }

    pub fn flags(&self) -> ColorFilterFlags {
        ColorFilterFlags::from_bits_truncate(unsafe {
            C_SkColorFilter_getFlags(self.native())
        })
    }

    pub fn filter_color(&self, color: impl Into<Color>) -> Color {
        Color::from_native(unsafe {
            self.native().filterColor(color.into().into_native())
        })
    }

    pub fn filter_color4f(&self, color: impl AsRef<Color4f>, color_space: &ColorSpace) -> Color4f {
        Color4f::from_native(unsafe {
            self.native().filterColor4f(color.as_ref().native(), color_space.native_mut_force())
        })
    }

    pub fn new_mode_filter(c: impl Into<Color>, mode: BlendMode) -> Option<Self> {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_MakeModeFilter(c.into().native(), mode.native())
        })
    }

    // TODO: name this function to_composed()?
    #[must_use]
    pub fn composed(&self, inner: &ColorFilter) -> Option<Self> {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_makeComposed(self.native(), inner.shared_native() )
        })
    }

    pub fn from_matrix_row_major_255(matrix: &[scalar; 20]) -> Self {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_MakeMatrixFilterRowMajor255(matrix.as_ptr())
        }).unwrap()
    }

    pub fn new_linear_to_srgb_gamma() -> Self {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_MakeLinearToSRGBGamma()
        }).unwrap()
    }

    pub fn new_srgb_to_linear_gamma() -> Self {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_MakeSRGBToLinearGamma()
        }).unwrap()
    }

    pub fn new_mixer(cf0: &ColorFilter, cf1: &ColorFilter, weight: f32) -> Option<Self> {
        ColorFilter::from_ptr(unsafe {
            C_SkColorFilter_MakeMixer(cf0.shared_native(), cf1.shared_native(), weight)
        })
    }

    // TODO: asFragmentProcessor()
    // TODO: affectsTransparentBlack()
    // TODO: Deserialize (via Flattenable).
}

#[test]
fn color_mode_roundtrip() {
    let color = Color::CYAN;
    let mode = BlendMode::ColorBurn;
    let cf = ColorFilter::new_mode_filter(color, mode).unwrap();
    let (c, m) = cf.to_color_mode().unwrap();
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