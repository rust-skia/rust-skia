use crate::{prelude::*, scalar, BlendMode, Color, Color4f, ColorSpace, NativeFlattenable};
use skia_bindings::{self as sb, SkColorFilter, SkFlattenable, SkRefCntBase};
use std::fmt;

pub type ColorFilter = RCHandle<SkColorFilter>;
unsafe impl Send for ColorFilter {}
unsafe impl Sync for ColorFilter {}

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

impl fmt::Debug for ColorFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ColorFilter")
            .field("as_a_color_mode", &self.to_a_color_mode())
            .field("as_a_color_matrix", &self.to_a_color_matrix())
            .field("is_alpha_unchanged", &self.is_alpha_unchanged())
            .finish()
    }
}

impl ColorFilter {
    pub fn to_a_color_mode(&self) -> Option<(Color, BlendMode)> {
        let mut color: Color = 0.into();
        let mut mode: BlendMode = Default::default();
        unsafe { self.native().asAColorMode(color.native_mut(), &mut mode) }
            .if_true_some((color, mode))
    }

    pub fn to_a_color_matrix(&self) -> Option<[scalar; 20]> {
        let mut matrix: [scalar; 20] = Default::default();
        unsafe { self.native().asAColorMatrix(&mut matrix[0]) }.if_true_some(matrix)
    }

    pub fn is_alpha_unchanged(&self) -> bool {
        unsafe { self.native().isAlphaUnchanged() }
    }

    pub fn filter_color(&self, color: impl Into<Color>) -> Color {
        // Color resolves to u32, so the C++ ABI can be used.
        Color::from_native_c(unsafe { self.native().filterColor(color.into().into_native()) })
    }

    pub fn filter_color4f(
        &self,
        color: impl AsRef<Color4f>,
        src_color_space: &ColorSpace,
        dst_color_space: Option<&ColorSpace>,
    ) -> Color4f {
        Color4f::from_native_c(unsafe {
            sb::C_SkColorFilter_filterColor4f(
                self.native(),
                color.as_ref().native(),
                src_color_space.native_mut_force(),
                dst_color_space.native_ptr_or_null_mut_force(),
            )
        })
    }

    pub fn composed(&self, inner: impl Into<ColorFilter>) -> Option<Self> {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkColorFilter_makeComposed(self.native(), inner.into().into_ptr())
        })
    }
}

pub mod color_filters {
    use crate::prelude::*;
    use crate::{scalar, BlendMode, Color, ColorFilter, ColorMatrix};
    use skia_bindings as sb;

    pub fn compose(
        outer: impl Into<ColorFilter>,
        inner: impl Into<ColorFilter>,
    ) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkColorFilters_Compose(outer.into().into_ptr(), inner.into().into_ptr())
        })
    }

    pub fn matrix(color_matrix: &ColorMatrix) -> ColorFilter {
        ColorFilter::from_ptr(unsafe { sb::C_SkColorFilters_Matrix(color_matrix.native()) })
            .unwrap()
    }

    pub fn matrix_row_major(array: &[scalar; 20]) -> ColorFilter {
        ColorFilter::from_ptr(unsafe { sb::C_SkColorFilters_MatrixRowMajor(array.as_ptr()) })
            .unwrap()
    }

    pub fn hsla_matrix_of_color_matrix(color_matrix: &ColorMatrix) -> ColorFilter {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkColorFilters_HSLAMatrixOfColorMatrix(color_matrix.native())
        })
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

    pub fn lerp(
        t: f32,
        dst: impl Into<ColorFilter>,
        src: impl Into<ColorFilter>,
    ) -> Option<ColorFilter> {
        ColorFilter::from_ptr(unsafe {
            sb::C_SkColorFilters_Lerp(t, dst.into().into_ptr(), src.into().into_ptr())
        })
    }
}

#[cfg(test)]

mod tests {
    use crate::prelude::*;
    use crate::{color_filters, BlendMode, Color, Color4f, ColorSpace};

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

    #[test]
    fn filter_color() {
        let color = Color::CYAN;
        let mode = BlendMode::ColorBurn;
        let cf = color_filters::blend(color, mode).unwrap();
        let _fc = cf.filter_color(Color::DARK_GRAY);
        let _fc = cf.filter_color4f(
            Color4f::new(0.0, 0.0, 0.0, 0.0),
            &ColorSpace::new_srgb(),
            None,
        );
    }
}
