use crate::prelude::*;
use skia_bindings::{SkGradientShader_Flags, C_SkGradientShader_MakeLinear, C_SkGradientShader_MakeLinear2, C_SkGradientShader_MakeRadial, C_SkGradientShader_MakeRadial2, C_SkGradientShader_MakeTwoPointConical, C_SkGradientShader_MakeTwoPointConical2, C_SkGradientShader_MakeSweep, C_SkGradientShader_MakeSweep2};
use crate::skia::{Shader, ShaderTileMode, scalar, Color, Point, Matrix, Color4f, ColorSpace};

pub enum GradientShader {}

bitflags! {
    pub struct GradientShaderFlags: u32 {
        const INTERPOLATE_COLORS_IN_PREMUL = SkGradientShader_Flags::kInterpolateColorsInPremul_Flag as _;
    }
}

impl Default for GradientShaderFlags {
    fn default() -> Self {
        GradientShaderFlags::empty()
    }
}

impl GradientShader {

    pub fn linear<'a, P: Into<Point>, C: Into<GradientShaderColors<'a>>>(
        points: (P, P),
        colors: C,
        pos: Option<&[scalar]>,
        mode: ShaderTileMode,
        flags: GradientShaderFlags,
        local_matrix: Option<&Matrix>) -> Option<Shader> {

        let colors = colors.into();
        assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));

        let points = [points.0.into(), points.1.into()];

        Shader::from_ptr(unsafe {
            match colors {
                GradientShaderColors::Colors(colors) =>
                    C_SkGradientShader_MakeLinear(
                        points.native().as_ptr(),
                        colors.native().as_ptr(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        flags.bits(),
                        local_matrix.native_ptr_or_null()),

                GradientShaderColors::ColorsInSpace(colors, color_space) =>
                    C_SkGradientShader_MakeLinear2(
                        points.native().as_ptr(),
                        colors.native().as_ptr(),
                        color_space.shared_native(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        flags.bits(),
                        local_matrix.native_ptr_or_null())
            }
        })
    }

    pub fn radial<'a, P: Into<Point>, C: Into<GradientShaderColors<'a>>>(
        center: P,
        radius: scalar,
        colors: C,
        pos: Option<&[scalar]>,
        mode: ShaderTileMode,
        flags: GradientShaderFlags,
        local_matrix: Option<&Matrix>) -> Option<Shader> {

        let colors = colors.into();
        let center = center.into();
        assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));

        Shader::from_ptr(unsafe {
            match colors {
                GradientShaderColors::Colors(colors) =>
                    C_SkGradientShader_MakeRadial(
                        center.native(),
                        radius,
                        colors.native().as_ptr(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        flags.bits(),
                        local_matrix.native_ptr_or_null()),

                GradientShaderColors::ColorsInSpace(colors, color_space) =>
                    C_SkGradientShader_MakeRadial2(
                        center.native(),
                        radius,
                        colors.native().as_ptr(),
                        color_space.shared_native(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        flags.bits(),
                        local_matrix.native_ptr_or_null())
            }
        })
    }

    pub fn two_point_conical<'a, P: Into<Point>, C: Into<GradientShaderColors<'a>>>(
        start: P, start_radius: scalar,
        end: P, end_radius: scalar,
        colors: C,
        pos: Option<&[scalar]>,
        mode: ShaderTileMode,
        flags: GradientShaderFlags,
        local_matrix: Option<&Matrix>) -> Option<Shader> {

        let colors = colors.into();
        let start = start.into();
        let end = end.into();

        assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));

        Shader::from_ptr(unsafe {
            match colors {
                GradientShaderColors::Colors(colors) =>
                    C_SkGradientShader_MakeTwoPointConical(
                        start.native(), start_radius,
                        end.native(), end_radius,
                        colors.native().as_ptr(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        flags.bits(),
                        local_matrix.native_ptr_or_null()),

                GradientShaderColors::ColorsInSpace(colors, color_space) =>
                    C_SkGradientShader_MakeTwoPointConical2(
                        start.native(), start_radius,
                        end.native(), end_radius,
                        colors.native().as_ptr(),
                        color_space.shared_native(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        flags.bits(),
                        local_matrix.native_ptr_or_null())
            }
        })
    }

    pub fn sweep<'a, C: Into<GradientShaderColors<'a>>>(
        (cx, cy) : (scalar, scalar),
        colors: C,
        pos: Option<&[scalar]>,
        mode: ShaderTileMode,
        angles: Option<(scalar, scalar)>,
        flags: GradientShaderFlags,
        local_matrix: Option<&Matrix>) -> Option<Shader> {

        let colors = colors.into();
        assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));

        let (start_angle, end_angle) =
            (angles.map(|a| a.0).unwrap_or(0.0), angles.map(|a| a.1).unwrap_or(360.0));

        Shader::from_ptr(unsafe {
            match colors {
                GradientShaderColors::Colors(colors) =>
                    C_SkGradientShader_MakeSweep(
                        cx, cy,
                        colors.native().as_ptr(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        start_angle, end_angle,
                        flags.bits(),
                        local_matrix.native_ptr_or_null()),

                GradientShaderColors::ColorsInSpace(colors, color_space) =>
                    C_SkGradientShader_MakeSweep2(
                        cx, cy,
                        colors.native().as_ptr(),
                        color_space.shared_native(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        start_angle, end_angle,
                        flags.bits(),
                        local_matrix.native_ptr_or_null())
            }
        })
    }
}

/// Type that represents either a alice of Color, or a slice of Color4f and a color space.
/// Whenever this type is expected, it's either possible to directly pass a &[Color] , or
/// a tuple of type (&[Color4f], &ColorSpace).
pub enum GradientShaderColors<'a> {
    Colors(&'a [Color]),
    ColorsInSpace(&'a[Color4f], &'a ColorSpace)
}

impl<'a> GradientShaderColors<'a> {

    pub fn len(&self) -> usize {
        match self {
            GradientShaderColors::Colors(colors) => colors.len(),
            GradientShaderColors::ColorsInSpace(colors, _) => colors.len()
        }
    }
}

impl<'a> From<&'a [Color]> for GradientShaderColors<'a> {
    fn from(colors: &'a [Color]) -> Self {
        GradientShaderColors::<'a>::Colors(colors)
    }
}

impl<'a> From<(&'a [Color4f], &'a ColorSpace)> for GradientShaderColors<'a> {
    fn from(c: (&'a [Color4f], &'a ColorSpace)) -> Self {
        GradientShaderColors::<'a>::ColorsInSpace(c.0, c.1)
    }
}
