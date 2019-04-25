use crate::prelude::*;
use crate::{scalar, Color, Color4f, ColorSpace, Matrix, Point, Shader, TileMode};
use skia_bindings::{
    C_SkGradientShader_MakeLinear, C_SkGradientShader_MakeLinear2, C_SkGradientShader_MakeRadial,
    C_SkGradientShader_MakeRadial2, C_SkGradientShader_MakeSweep, C_SkGradientShader_MakeSweep2,
    C_SkGradientShader_MakeTwoPointConical, C_SkGradientShader_MakeTwoPointConical2,
    SkGradientShader_Flags_kInterpolateColorsInPremul_Flag, SkShader,
};

pub enum GradientShader {}

bitflags! {
    pub struct GradientShaderFlags: u32 {
        const INTERPOLATE_COLORS_IN_PREMUL = SkGradientShader_Flags_kInterpolateColorsInPremul_Flag as _;
    }
}

impl Default for GradientShaderFlags {
    fn default() -> Self {
        Self::empty()
    }
}

impl GradientShader {
    pub fn linear<
        'a,
        P1: Into<Point>,
        P2: Into<Point>,
        C: Into<GradientShaderColors<'a>>,
        POS: Into<Option<&'a [scalar]>>,
        F: Into<Option<GradientShaderFlags>>,
        LM: Into<Option<&'a Matrix>>,
    >(
        points: (P1, P2),
        colors: C,
        pos: POS,
        mode: TileMode,
        flags: F,
        local_matrix: LM,
    ) -> Option<Shader> {
        let points = [points.0.into(), points.1.into()];
        let colors = colors.into();
        let pos = pos.into();
        assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));
        let flags = flags.into().unwrap_or_default();
        let local_matrix = local_matrix.into();

        Shader::from_ptr(unsafe {
            match colors {
                GradientShaderColors::Colors(colors) => C_SkGradientShader_MakeLinear(
                    points.native().as_ptr(),
                    colors.native().as_ptr(),
                    pos.as_ptr_or_null(),
                    colors.len().try_into().unwrap(),
                    mode.into_native(),
                    flags.bits(),
                    local_matrix.native_ptr_or_null(),
                ),

                GradientShaderColors::ColorsInSpace(colors, color_space) => {
                    C_SkGradientShader_MakeLinear2(
                        points.native().as_ptr(),
                        colors.native().as_ptr(),
                        color_space.shared_native(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        flags.bits(),
                        local_matrix.native_ptr_or_null(),
                    )
                }
            }
        })
    }

    pub fn radial<
        'a,
        P: Into<Point>,
        C: Into<GradientShaderColors<'a>>,
        POS: Into<Option<&'a [scalar]>>,
        F: Into<Option<GradientShaderFlags>>,
        LM: Into<Option<&'a Matrix>>,
    >(
        center: P,
        radius: scalar,
        colors: C,
        pos: POS,
        mode: TileMode,
        flags: F,
        local_matrix: LM,
    ) -> Option<Shader> {
        let colors = colors.into();
        let center = center.into();
        let pos = pos.into();
        assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));
        let flags = flags.into().unwrap_or_default();
        let local_matrix = local_matrix.into();

        Shader::from_ptr(unsafe {
            match colors {
                GradientShaderColors::Colors(colors) => C_SkGradientShader_MakeRadial(
                    center.native(),
                    radius,
                    colors.native().as_ptr(),
                    pos.as_ptr_or_null(),
                    colors.len().try_into().unwrap(),
                    mode.into_native(),
                    flags.bits(),
                    local_matrix.native_ptr_or_null(),
                ),

                GradientShaderColors::ColorsInSpace(colors, color_space) => {
                    C_SkGradientShader_MakeRadial2(
                        center.native(),
                        radius,
                        colors.native().as_ptr(),
                        color_space.shared_native(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        flags.bits(),
                        local_matrix.native_ptr_or_null(),
                    )
                }
            }
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn two_point_conical<
        'a,
        PS: Into<Point>,
        PE: Into<Point>,
        C: Into<GradientShaderColors<'a>>,
        POS: Into<Option<&'a [scalar]>>,
        F: Into<Option<GradientShaderFlags>>,
        LM: Into<Option<&'a Matrix>>,
    >(
        start: PS,
        start_radius: scalar,
        end: PE,
        end_radius: scalar,
        colors: C,
        pos: POS,
        mode: TileMode,
        flags: F,
        local_matrix: LM,
    ) -> Option<Shader> {
        let colors = colors.into();
        let start = start.into();
        let end = end.into();
        let pos = pos.into();
        assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));
        let flags = flags.into().unwrap_or_default();
        let local_matrix = local_matrix.into();

        Shader::from_ptr(unsafe {
            match colors {
                GradientShaderColors::Colors(colors) => C_SkGradientShader_MakeTwoPointConical(
                    start.native(),
                    start_radius,
                    end.native(),
                    end_radius,
                    colors.native().as_ptr(),
                    pos.as_ptr_or_null(),
                    colors.len().try_into().unwrap(),
                    mode.into_native(),
                    flags.bits(),
                    local_matrix.native_ptr_or_null(),
                ),

                GradientShaderColors::ColorsInSpace(colors, color_space) => {
                    C_SkGradientShader_MakeTwoPointConical2(
                        start.native(),
                        start_radius,
                        end.native(),
                        end_radius,
                        colors.native().as_ptr(),
                        color_space.shared_native(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        flags.bits(),
                        local_matrix.native_ptr_or_null(),
                    )
                }
            }
        })
    }

    pub fn sweep<
        'a,
        P: Into<Point>,
        C: Into<GradientShaderColors<'a>>,
        POS: Into<Option<&'a [scalar]>>,
        A: Into<Option<(scalar, scalar)>>,
        F: Into<Option<GradientShaderFlags>>,
        LM: Into<Option<&'a Matrix>>,
    >(
        center: P,
        colors: C,
        pos: POS,
        mode: TileMode,
        angles: A,
        flags: F,
        local_matrix: LM,
    ) -> Option<Shader> {
        let center = center.into();
        let colors = colors.into();
        let pos = pos.into();
        assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));
        let angles = angles.into();
        let flags = flags.into().unwrap_or_default();
        let local_matrix = local_matrix.into();

        let (start_angle, end_angle) = (
            angles.map(|a| a.0).unwrap_or(0.0),
            angles.map(|a| a.1).unwrap_or(360.0),
        );

        Shader::from_ptr(unsafe {
            match colors {
                GradientShaderColors::Colors(colors) => C_SkGradientShader_MakeSweep(
                    center.x,
                    center.y,
                    colors.native().as_ptr(),
                    pos.as_ptr_or_null(),
                    colors.len().try_into().unwrap(),
                    mode.into_native(),
                    start_angle,
                    end_angle,
                    flags.bits(),
                    local_matrix.native_ptr_or_null(),
                ),

                GradientShaderColors::ColorsInSpace(colors, color_space) => {
                    C_SkGradientShader_MakeSweep2(
                        center.x,
                        center.y,
                        colors.native().as_ptr(),
                        color_space.shared_native(),
                        pos.as_ptr_or_null(),
                        colors.len().try_into().unwrap(),
                        mode.into_native(),
                        start_angle,
                        end_angle,
                        flags.bits(),
                        local_matrix.native_ptr_or_null(),
                    )
                }
            }
        })
    }
}

/// Type that represents either a slice of Color, or a slice of Color4f and a color space.
/// Whenever this type is expected, it's either possible to directly pass a &[Color] , or
/// a tuple of type (&[Color4f], &ColorSpace).
pub enum GradientShaderColors<'a> {
    Colors(&'a [Color]),
    ColorsInSpace(&'a [Color4f], &'a ColorSpace),
}

impl<'a> GradientShaderColors<'a> {
    pub fn len(&self) -> usize {
        match self {
            GradientShaderColors::Colors(colors) => colors.len(),
            GradientShaderColors::ColorsInSpace(colors, _) => colors.len(),
        }
    }

    // to keep clippy happy.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

impl RCHandle<SkShader> {
    pub fn linear_gradient<
        'a,
        P1: Into<Point>,
        P2: Into<Point>,
        C: Into<GradientShaderColors<'a>>,
        POS: Into<Option<&'a [scalar]>>,
        F: Into<Option<GradientShaderFlags>>,
        LM: Into<Option<&'a Matrix>>,
    >(
        points: (P1, P2),
        colors: C,
        pos: POS,
        mode: TileMode,
        flags: F,
        local_matrix: LM,
    ) -> Option<Self> {
        GradientShader::linear(points, colors, pos, mode, flags, local_matrix)
    }

    pub fn radial_gradient<
        'a,
        P: Into<Point>,
        C: Into<GradientShaderColors<'a>>,
        POS: Into<Option<&'a [scalar]>>,
        F: Into<Option<GradientShaderFlags>>,
        LM: Into<Option<&'a Matrix>>,
    >(
        center: P,
        radius: scalar,
        colors: C,
        pos: POS,
        mode: TileMode,
        flags: F,
        local_matrix: LM,
    ) -> Option<Self> {
        GradientShader::radial(center, radius, colors, pos, mode, flags, local_matrix)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn two_point_conical_gradient<
        'a,
        PS: Into<Point>,
        PE: Into<Point>,
        C: Into<GradientShaderColors<'a>>,
        POS: Into<Option<&'a [scalar]>>,
        F: Into<Option<GradientShaderFlags>>,
        LM: Into<Option<&'a Matrix>>,
    >(
        start: PS,
        start_radius: scalar,
        end: PE,
        end_radius: scalar,
        colors: C,
        pos: POS,
        mode: TileMode,
        flags: F,
        local_matrix: LM,
    ) -> Option<Self> {
        GradientShader::two_point_conical(
            start,
            start_radius,
            end,
            end_radius,
            colors,
            pos,
            mode,
            flags,
            local_matrix,
        )
    }

    pub fn sweep_gradient<
        'a,
        P: Into<Point>,
        C: Into<GradientShaderColors<'a>>,
        POS: Into<Option<&'a [scalar]>>,
        A: Into<Option<(scalar, scalar)>>,
        F: Into<Option<GradientShaderFlags>>,
        LM: Into<Option<&'a Matrix>>,
    >(
        center: P,
        colors: C,
        pos: POS,
        mode: TileMode,
        angles: A,
        flags: F,
        local_matrix: LM,
    ) -> Option<Self> {
        GradientShader::sweep(center, colors, pos, mode, angles, flags, local_matrix)
    }
}
