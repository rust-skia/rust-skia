use crate::{scalar, Color4f, ColorSpace, TileMode};
use skia_bindings as sb;

/// Gradient interpolation settings.
///
/// Specifies how colors are interpolated in a gradient, including the color space
/// and premultiplication mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Interpolation {
    pub in_premul: interpolation::InPremul,
    pub color_space: interpolation::ColorSpace,
    pub hue_method: interpolation::HueMethod,
}

native_transmutable!(sb::SkGradient_Interpolation, Interpolation);

pub mod interpolation {
    use skia_bindings as sb;

    /// Whether to interpolate colors in premultiplied alpha space.
    pub type InPremul = sb::SkGradient_Interpolation_InPremul;
    variant_name!(InPremul::Yes);

    /// Color space for gradient interpolation.
    ///
    /// See <https://www.w3.org/TR/css-color-4/#interpolation-space>
    pub type ColorSpace = sb::SkGradient_Interpolation_ColorSpace;
    variant_name!(ColorSpace::HSL);

    /// Hue interpolation method for cylindrical color spaces (LCH, OKLCH, HSL, HWB).
    ///
    /// See <https://www.w3.org/TR/css-color-4/#hue-interpolation>
    pub type HueMethod = sb::SkGradient_Interpolation_HueMethod;
    variant_name!(HueMethod::Shorter);
}

impl Default for Interpolation {
    fn default() -> Self {
        Self {
            in_premul: interpolation::InPremul::No,
            color_space: interpolation::ColorSpace::Destination,
            hue_method: interpolation::HueMethod::Shorter,
        }
    }
}

impl Interpolation {
    /// Create interpolation settings from legacy flags.
    pub fn from_flags(flags: u32) -> Self {
        Self {
            in_premul: if flags & 1 != 0 {
                interpolation::InPremul::Yes
            } else {
                interpolation::InPremul::No
            },
            color_space: interpolation::ColorSpace::Destination,
            hue_method: interpolation::HueMethod::Shorter,
        }
    }
}

/// Specification for the colors in a gradient.
///
/// Holds color data, positions, tile mode, and color space for gradient construction.
/// All references must outlive any shader created from it.
#[derive(Debug, Clone)]
pub struct Colors<'a> {
    colors: &'a [Color4f],
    pos: Option<&'a [scalar]>,
    color_space: Option<ColorSpace>,
    tile_mode: TileMode,
}

impl<'a> Colors<'a> {
    /// Create gradient colors with explicit positions.
    ///
    /// - `colors`: The colors for the gradient.
    /// - `pos`: Relative positions of each color (0.0 to 1.0). Must be strictly increasing.
    ///          If `None`, colors are distributed evenly.
    /// - `tile_mode`: Tiling mode for the gradient.
    /// - `color_space`: Optional color space. If `None`, colors are treated as sRGB.
    pub fn new(
        colors: &'a [Color4f],
        pos: Option<&'a [scalar]>,
        tile_mode: TileMode,
        color_space: impl Into<Option<ColorSpace>>,
    ) -> Self {
        // Validate positions match colors if provided
        assert!(pos.map_or(true, |pos| pos.len() == colors.len()));

        Self {
            colors,
            pos,
            color_space: color_space.into(),
            tile_mode,
        }
    }

    /// Create gradient colors with evenly distributed positions.
    pub fn new_evenly_spaced(
        colors: &'a [Color4f],
        tile_mode: TileMode,
        color_space: impl Into<Option<ColorSpace>>,
    ) -> Self {
        Self::new(colors, None, tile_mode, color_space)
    }

    /// Returns a reference to the colors.
    pub fn colors(&self) -> &'a [Color4f] {
        self.colors
    }

    /// Returns a reference to the positions.
    pub fn positions(&self) -> Option<&'a [scalar]> {
        self.pos
    }

    /// Returns a reference to the color space.
    pub fn color_space(&self) -> Option<&ColorSpace> {
        self.color_space.as_ref()
    }

    /// Returns the tile mode.
    pub fn tile_mode(&self) -> TileMode {
        self.tile_mode
    }
}

/// Gradient specification combining colors and interpolation settings.
///
/// This type corresponds to the C++ `SkGradient` class and encapsulates
/// all parameters needed to define a gradient's appearance.
///
/// Note: This is a lightweight wrapper around [`Colors`] and [`Interpolation`].
/// The actual C++ `SkGradient` object is constructed on-demand when creating shaders.
#[derive(Debug, Clone)]
pub struct Gradient<'a> {
    colors: Colors<'a>,
    interpolation: Interpolation,
}

impl<'a> Gradient<'a> {
    pub fn new(colors: Colors<'a>, interpolation: impl Into<Interpolation>) -> Self {
        Self {
            colors,
            interpolation: interpolation.into(),
        }
    }

    pub fn colors(&self) -> &Colors<'a> {
        &self.colors
    }

    pub fn interpolation(&self) -> &Interpolation {
        &self.interpolation
    }
}

/// Shader factory functions that accept [`Gradient`] parameters.
///
/// These functions correspond to the C++ `SkShaders` namespace gradient functions.
pub mod shaders {
    use super::{scalar, Gradient};
    use crate::{prelude::*, Matrix, Point, Shader};
    use skia_bindings as sb;
    use std::ptr;

    /// Returns a shader that generates a linear gradient between the two specified points.
    ///
    /// - `points`: Array of 2 points, the end-points of the line segment
    /// - `gradient`: Description of the colors and interpolation method
    /// - `local_matrix`: Optional local matrix
    pub fn linear_gradient<'a>(
        points: (impl Into<Point>, impl Into<Point>),
        gradient: &Gradient<'_>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Shader> {
        let points = [points.0.into(), points.1.into()];
        let local_matrix = local_matrix.into();
        let colors = gradient.colors();
        let interpolation = gradient.interpolation();
        let positions = colors.positions();

        Shader::from_ptr(unsafe {
            sb::C_SkShaders_LinearGradient(
                points.native().as_ptr(),
                colors.colors().as_ptr() as *const _,
                colors.colors().len(),
                positions.map_or(ptr::null(), |pos| pos.as_ptr()),
                positions.map_or(0, |pos| pos.len()),
                colors.tile_mode(),
                colors.color_space().native_ptr_or_null() as *mut _,
                interpolation.native(),
                local_matrix.native_ptr_or_null(),
            )
        })
    }

    /// Returns a shader that generates a radial gradient given the center and radius.
    ///
    /// - `center`: The center of the circle for this gradient
    /// - `radius`: Must be positive. The radius of the circle for this gradient
    /// - `gradient`: Description of the colors and interpolation method
    /// - `local_matrix`: Optional local matrix
    pub fn radial_gradient<'a>(
        (center, radius): (impl Into<Point>, scalar),
        gradient: &Gradient<'_>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Shader> {
        let center = center.into();
        let local_matrix = local_matrix.into();
        let colors = gradient.colors();
        let interpolation = gradient.interpolation();
        let positions = colors.positions();

        Shader::from_ptr(unsafe {
            sb::C_SkShaders_RadialGradient(
                center.native(),
                radius,
                colors.colors().as_ptr() as *const _,
                colors.colors().len(),
                positions.map_or(ptr::null(), |pos| pos.as_ptr()),
                positions.map_or(0, |pos| pos.len()),
                colors.tile_mode(),
                colors.color_space().native_ptr_or_null() as *mut _,
                interpolation.native(),
                local_matrix.native_ptr_or_null(),
            )
        })
    }

    /// Returns a shader that generates a conical gradient given two circles.
    ///
    /// The gradient interprets the two circles according to the following HTML spec:
    /// <http://dev.w3.org/html5/2dcontext/#dom-context-2d-createradialgradient>
    ///
    /// - `start`: The center of the start circle
    /// - `start_radius`: Must be positive. The radius of the start circle
    /// - `end`: The center of the end circle
    /// - `end_radius`: Must be positive. The radius of the end circle
    /// - `gradient`: Description of the colors and interpolation method
    /// - `local_matrix`: Optional local matrix
    #[allow(clippy::too_many_arguments)]
    pub fn two_point_conical_gradient<'a>(
        (start, start_radius): (impl Into<Point>, scalar),
        (end, end_radius): (impl Into<Point>, scalar),
        gradient: &Gradient<'_>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Shader> {
        let start = start.into();
        let end = end.into();
        let local_matrix = local_matrix.into();
        let colors = gradient.colors();
        let interpolation = gradient.interpolation();
        let positions = colors.positions();

        Shader::from_ptr(unsafe {
            sb::C_SkShaders_TwoPointConicalGradient(
                start.native(),
                start_radius,
                end.native(),
                end_radius,
                colors.colors().as_ptr() as *const _,
                colors.colors().len(),
                positions.map_or(ptr::null(), |pos| pos.as_ptr()),
                positions.map_or(0, |pos| pos.len()),
                colors.tile_mode(),
                colors.color_space().native_ptr_or_null() as *mut _,
                interpolation.native(),
                local_matrix.native_ptr_or_null(),
            )
        })
    }

    /// Returns a shader that generates a sweep gradient given a center.
    ///
    /// The shader accepts negative angles and angles larger than 360, draws between 0 and 360
    /// degrees, similar to the CSS conic-gradient semantics. 0 degrees means horizontal
    /// positive x axis. The start angle must be less than the end angle.
    ///
    /// - `center`: The center of the sweep
    /// - `start_angle`: Start of the angular range, corresponding to pos == 0
    /// - `end_angle`: End of the angular range, corresponding to pos == 1
    /// - `gradient`: Description of the colors and interpolation method
    /// - `local_matrix`: Optional local matrix
    pub fn sweep_gradient<'a>(
        center: impl Into<Point>,
        (start_angle, end_angle): (scalar, scalar),
        gradient: &Gradient<'_>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Shader> {
        let center = center.into();
        let local_matrix = local_matrix.into();
        let colors = gradient.colors();
        let interpolation = gradient.interpolation();
        let positions = colors.positions();

        Shader::from_ptr(unsafe {
            sb::C_SkShaders_SweepGradient(
                center.native(),
                start_angle,
                end_angle,
                colors.colors().as_ptr() as *const _,
                colors.colors().len(),
                positions.map_or(ptr::null(), |pos| pos.as_ptr()),
                positions.map_or(0, |pos| pos.len()),
                colors.tile_mode(),
                colors.color_space().native_ptr_or_null() as *mut _,
                interpolation.native(),
                local_matrix.native_ptr_or_null(),
            )
        })
    }
}
