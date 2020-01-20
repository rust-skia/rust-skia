use crate::prelude::*;
use crate::{scalar, BlendMode, Color, Color4f, ColorSpace, NativeFlattenable};
use skia_bindings as sb;
use skia_bindings::{SkColorFilter, SkFlattenable, SkRefCntBase};

bitflags! {
    pub struct Flags: u32 {
        const ALPHA_UNCHANGED = sb::SkColorFilter_Flags_kAlphaUnchanged_Flag as u32;
    }
}

pub type ColorFilter = RCHandle<SkColorFilter>;

impl NativeBase<SkRefCntBase> for SkColorFilter {}

impl NativeRefCountedBase for SkColorFilter {
    type Base = SkRefCntBase;
}

impl NativeBase<SkFlattenable> for SkColorFilter {}

impl NativeFlattenable for SkColorFilter {
    fn native_flattenable(&self) -> &SkFlattenable {
        &self.base()
    }

    fn native_deserialize(data: &[u8]) -> *mut Self {
        unsafe { sb::C_SkColorFilter_Deserialize(data.as_ptr() as _, data.len()) }
    }
}

impl RCHandle<SkColorFilter> {
    #[deprecated(since = "0.14.0", note = "use to_a_color_mode()")]
    pub fn to_color_mode(&self) -> Option<(Color, BlendMode)> {
        self.to_a_color_mode()
    }

    pub fn to_a_color_mode(&self) -> Option<(Color, BlendMode)> {
        let mut color: Color = 0.into();
        let mut mode: BlendMode = Default::default();
        unsafe { sb::C_SkColorFilter_asAColorMode(self.native(), color.native_mut(), &mut mode) }
            .if_true_some((color, mode))
    }

    #[deprecated(since = "0.14.0", note = "use to_a_color_matrix()")]
    pub fn to_color_matrix(&self) -> Option<[scalar; 20]> {
        self.to_a_color_matrix()
    }

    pub fn to_a_color_matrix(&self) -> Option<[scalar; 20]> {
        let mut matrix: [scalar; 20] = Default::default();
        unsafe { sb::C_SkColorFilter_asAColorMatrix(self.native(), matrix.as_mut_ptr()) }
            .if_true_some(matrix)
    }

    pub fn flags(&self) -> self::Flags {
        Flags::from_bits_truncate(unsafe { sb::C_SkColorFilter_getFlags(self.native()) })
    }

    pub fn filter_color(&self, color: impl Into<Color>) -> Color {
        Color::from_native(unsafe { self.native().filterColor(color.into().into_native()) })
    }

    pub fn filter_color4f(
        &self,
        color: impl AsRef<Color4f>,
        src_color_space: &ColorSpace,
        dst_color_space: Option<&ColorSpace>,
    ) -> Color4f {
        Color4f::from_native(unsafe {
            self.native().filterColor4f(
                color.as_ref().native(),
                src_color_space.native_mut_force(),
                dst_color_space.native_ptr_or_null_mut_force(),
            )
        })
    }

    #[must_use]
    pub fn composed(&self, inner: impl Into<ColorFilter>) -> Option<Self> {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkColorFilter_makeComposed(self.native(), inner.into().into_ptr())
        })
    }

    // TODO: asFragmentProcessor()
    // TODO: affectsTransparentBlack()
}

pub mod color_filters {
    use crate::prelude::*;
    use crate::{scalar, BlendMode, Color, ColorFilter, ColorMatrix};
    use skia_bindings as sb;

    pub fn compose(outer: ColorFilter, inner: ColorFilter) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkColorFilters_Compose(outer.into_ptr(), inner.into_ptr())
        })
    }

    pub fn matrix(color_matrix: &ColorMatrix) -> ColorFilter {
        ColorFilter::from_ptr(unsafe { sb::C_SkColorFilters_Matrix(color_matrix.native()) })
            .unwrap()
    }

    #[deprecated(since = "0.14.0", note = "use matrix_row_major()")]
    pub fn matrix_row_major_255(array: &[scalar; 20]) -> ColorFilter {
        self::matrix_row_major(array)
    }

    pub fn matrix_row_major(array: &[scalar; 20]) -> ColorFilter {
        ColorFilter::from_ptr(unsafe { sb::C_SkColorFilters_MatrixRowMajor(array.as_ptr()) })
            .unwrap()
    }

    pub fn hsla_matrix(row_major: &[f32; 20]) -> ColorFilter {
        ColorFilter::from_ptr(unsafe { sb::C_SkColorFilters_HSLAMatrix(row_major.as_ptr()) })
            .unwrap()
    }

    pub fn blend(c: impl Into<Color>, mode: BlendMode) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe { sb::C_SkColorFilters_Blend(c.into().into_native(), mode) })
    }

    pub fn linear_to_srgb_gamma() -> ColorFilter {
        ColorFilter::from_ptr(unsafe { sb::C_SkColorFilters_LinearToSRGBGamma() }).unwrap()
    }

    pub fn srgb_to_linear_gamma() -> ColorFilter {
        ColorFilter::from_ptr(unsafe { sb::C_SkColorFilters_SRGBToLinearGamma() }).unwrap()
    }

    pub fn lerp(t: f32, dst: ColorFilter, src: ColorFilter) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkColorFilters_Lerp(t, dst.into_ptr(), src.into_ptr())
        })
    }
}

#[test]
fn color_mode_roundtrip() {
    let color = Color::CYAN;
    let mode = BlendMode::ColorBurn;
    let cf = color_filters::blend(color, mode).unwrap();
    let (c, m) = cf.to_a_color_mode().unwrap();
    assert_eq!(color, c);
    assert_eq!(mode, m);
}

#[test]
fn ref_count() {
    let color = Color::CYAN;
    let mode = BlendMode::ColorBurn;
    let cf = color_filters::blend(color, mode).unwrap();
    let rc = cf.native()._ref_cnt();
    assert_eq!(1, rc);
}
