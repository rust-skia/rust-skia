use crate::{prelude::*, scalar, Color, Color4f, ColorSpace, Matrix, Point, Shader, TileMode};
use skia_bindings as sb;

impl Shader {
    pub fn linear_gradient<'a>(
        points: (impl Into<Point>, impl Into<Point>),
        colors: impl Into<GradientShaderColors<'a>>,
        pos: impl Into<Option<&'a [scalar]>>,
        mode: TileMode,
        flags: impl Into<Option<self::Flags>>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Self> {
        linear(points, colors, pos, mode, flags, local_matrix)
    }

    pub fn radial_gradient<'a>(
        center: impl Into<Point>,
        radius: scalar,
        colors: impl Into<GradientShaderColors<'a>>,
        pos: impl Into<Option<&'a [scalar]>>,
        mode: TileMode,
        flags: impl Into<Option<self::Flags>>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Self> {
        radial(center, radius, colors, pos, mode, flags, local_matrix)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn two_point_conical_gradient<'a>(
        start: impl Into<Point>,
        start_radius: scalar,
        end: impl Into<Point>,
        end_radius: scalar,
        colors: impl Into<GradientShaderColors<'a>>,
        pos: impl Into<Option<&'a [scalar]>>,
        mode: TileMode,
        flags: impl Into<Option<self::Flags>>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Self> {
        two_point_conical(
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

    pub fn sweep_gradient<'a>(
        center: impl Into<Point>,
        colors: impl Into<GradientShaderColors<'a>>,
        pos: impl Into<Option<&'a [scalar]>>,
        mode: TileMode,
        angles: impl Into<Option<(scalar, scalar)>>,
        flags: impl Into<Option<self::Flags>>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Self> {
        sweep(center, colors, pos, mode, angles, flags, local_matrix)
    }
}

bitflags! {
    pub struct Flags: u32 {
        const INTERPOLATE_COLORS_IN_PREMUL = sb::SkGradientShader_Flags_kInterpolateColorsInPremul_Flag as _;
    }
}

impl Default for self::Flags {
    fn default() -> Self {
        Self::empty()
    }
}

pub fn linear<'a>(
    points: (impl Into<Point>, impl Into<Point>),
    colors: impl Into<GradientShaderColors<'a>>,
    pos: impl Into<Option<&'a [scalar]>>,
    mode: TileMode,
    flags: impl Into<Option<self::Flags>>,
    local_matrix: impl Into<Option<&'a Matrix>>,
) -> Option<Shader> {
    let points = [points.0.into(), points.1.into()];
    let colors = colors.into();
    let pos = pos.into();
    assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));
    let flags = flags.into().unwrap_or_default();
    let local_matrix = local_matrix.into();

    Shader::from_ptr(unsafe {
        match colors {
            GradientShaderColors::Colors(colors) => sb::C_SkGradientShader_MakeLinear(
                points.native().as_ptr(),
                colors.native().as_ptr(),
                pos.as_ptr_or_null(),
                colors.len().try_into().unwrap(),
                mode,
                flags.bits(),
                local_matrix.native_ptr_or_null(),
            ),

            GradientShaderColors::ColorsInSpace(colors, color_space) => {
                sb::C_SkGradientShader_MakeLinear2(
                    points.native().as_ptr(),
                    colors.native().as_ptr(),
                    color_space.into_ptr(),
                    pos.as_ptr_or_null(),
                    colors.len().try_into().unwrap(),
                    mode,
                    flags.bits(),
                    local_matrix.native_ptr_or_null(),
                )
            }
        }
    })
}

pub fn radial<'a>(
    center: impl Into<Point>,
    radius: scalar,
    colors: impl Into<GradientShaderColors<'a>>,
    pos: impl Into<Option<&'a [scalar]>>,
    mode: TileMode,
    flags: impl Into<Option<self::Flags>>,
    local_matrix: impl Into<Option<&'a Matrix>>,
) -> Option<Shader> {
    let colors = colors.into();
    let center = center.into();
    let pos = pos.into();
    assert!(pos.is_none() || (pos.unwrap().len() == colors.len()));
    let flags = flags.into().unwrap_or_default();
    let local_matrix = local_matrix.into();

    Shader::from_ptr(unsafe {
        match colors {
            GradientShaderColors::Colors(colors) => sb::C_SkGradientShader_MakeRadial(
                center.native(),
                radius,
                colors.native().as_ptr(),
                pos.as_ptr_or_null(),
                colors.len().try_into().unwrap(),
                mode,
                flags.bits(),
                local_matrix.native_ptr_or_null(),
            ),

            GradientShaderColors::ColorsInSpace(colors, color_space) => {
                sb::C_SkGradientShader_MakeRadial2(
                    center.native(),
                    radius,
                    colors.native().as_ptr(),
                    color_space.into_ptr(),
                    pos.as_ptr_or_null(),
                    colors.len().try_into().unwrap(),
                    mode,
                    flags.bits(),
                    local_matrix.native_ptr_or_null(),
                )
            }
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub fn two_point_conical<'a>(
    start: impl Into<Point>,
    start_radius: scalar,
    end: impl Into<Point>,
    end_radius: scalar,
    colors: impl Into<GradientShaderColors<'a>>,
    pos: impl Into<Option<&'a [scalar]>>,
    mode: TileMode,
    flags: impl Into<Option<self::Flags>>,
    local_matrix: impl Into<Option<&'a Matrix>>,
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
            GradientShaderColors::Colors(colors) => sb::C_SkGradientShader_MakeTwoPointConical(
                start.native(),
                start_radius,
                end.native(),
                end_radius,
                colors.native().as_ptr(),
                pos.as_ptr_or_null(),
                colors.len().try_into().unwrap(),
                mode,
                flags.bits(),
                local_matrix.native_ptr_or_null(),
            ),

            GradientShaderColors::ColorsInSpace(colors, color_space) => {
                sb::C_SkGradientShader_MakeTwoPointConical2(
                    start.native(),
                    start_radius,
                    end.native(),
                    end_radius,
                    colors.native().as_ptr(),
                    color_space.into_ptr(),
                    pos.as_ptr_or_null(),
                    colors.len().try_into().unwrap(),
                    mode,
                    flags.bits(),
                    local_matrix.native_ptr_or_null(),
                )
            }
        }
    })
}

pub fn sweep<'a>(
    center: impl Into<Point>,
    colors: impl Into<GradientShaderColors<'a>>,
    pos: impl Into<Option<&'a [scalar]>>,
    mode: TileMode,
    angles: impl Into<Option<(scalar, scalar)>>,
    flags: impl Into<Option<self::Flags>>,
    local_matrix: impl Into<Option<&'a Matrix>>,
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
            GradientShaderColors::Colors(colors) => sb::C_SkGradientShader_MakeSweep(
                center.x,
                center.y,
                colors.native().as_ptr(),
                pos.as_ptr_or_null(),
                colors.len().try_into().unwrap(),
                mode,
                start_angle,
                end_angle,
                flags.bits(),
                local_matrix.native_ptr_or_null(),
            ),

            GradientShaderColors::ColorsInSpace(colors, color_space) => {
                sb::C_SkGradientShader_MakeSweep2(
                    center.x,
                    center.y,
                    colors.native().as_ptr(),
                    color_space.into_ptr(),
                    pos.as_ptr_or_null(),
                    colors.len().try_into().unwrap(),
                    mode,
                    start_angle,
                    end_angle,
                    flags.bits(),
                    local_matrix.native_ptr_or_null(),
                )
            }
        }
    })
}

/// Type that represents either a slice of [`Color`], or a slice of [`Color4f`] and a color space.
/// Whenever this type is expected, it's either possible to directly pass a `&[Color]` , or
/// a tuple of type `(&[Color4f], &ColorSpace)`.
#[derive(Debug)]
pub enum GradientShaderColors<'a> {
    Colors(&'a [Color]),
    ColorsInSpace(&'a [Color4f], ColorSpace),
}

impl GradientShaderColors<'_> {
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

impl<'a> From<(&'a [Color4f], ColorSpace)> for GradientShaderColors<'a> {
    fn from(c: (&'a [Color4f], ColorSpace)) -> Self {
        GradientShaderColors::<'a>::ColorsInSpace(c.0, c.1)
    }
}
