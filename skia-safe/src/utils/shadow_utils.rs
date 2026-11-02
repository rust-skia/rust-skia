//! Utilities for drawing and computing drop shadows.

use crate::{Canvas, Color, Matrix, Path, Point3, Rect, prelude::*, scalar};
use skia_bindings::{self as sb, SkShadowUtils};

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShadowFlags: u32 {
        /// The occluding object is not opaque. Knowing that the occluder is opaque allows
        /// us to cull shadow geometry behind it and improve performance.
        #[allow(clippy::unnecessary_cast)]
        const TRANSPARENT_OCCLUDER = sb::SkShadowFlags_kTransparentOccluder_ShadowFlag as u32;
        /// Don't try to use analytic shadows.
        #[allow(clippy::unnecessary_cast)]
        const GEOMETRIC_ONLY = sb::SkShadowFlags_kGeometricOnly_ShadowFlag as u32;
        /// Light position represents a direction, light radius is blur radius at elevation 1
        #[allow(clippy::unnecessary_cast)]
        const DIRECTIONAL_LIGHT = sb::SkShadowFlags_kDirectionalLight_ShadowFlag as u32;
        /// Concave paths will only use blur to generate the shadow
        #[allow(clippy::unnecessary_cast)]
        const CONCAVE_BLUR_ONLY = sb::SkShadowFlags_kConcaveBlurOnly_ShadowFlag as u32;
        /// mask for all shadow flags
        const ALL = Self::TRANSPARENT_OCCLUDER.bits() | Self::GEOMETRIC_ONLY.bits()
            | Self::DIRECTIONAL_LIGHT.bits() | Self::CONCAVE_BLUR_ONLY.bits();
    }
}

/// Draw an offset spot shadow and outlining ambient shadow for the given path using a disc
/// light. The shadow may be cached, depending on the path type and canvas matrix. If the
/// matrix is perspective or the path is volatile, it will not be cached.
///
/// - `canvas` The canvas on which to draw the shadows.
/// - `path` The occluder used to generate the shadows.
/// - `z_plane_params` Values for the plane function which returns the Z offset of the
///   occluder from the canvas based on local x and y values (the current matrix is not applied).
/// - `light_pos` Generally, the 3D position of the light relative to the canvas plane.
///   If [`self::ShadowFlags::DIRECTIONAL_LIGHT`] is set, this specifies a vector pointing
///   towards the light.
/// - `light_radius` Generally, the radius of the disc light.
///   If [`self::ShadowFlags::DIRECTIONAL_LIGHT`] is set, this specifies the amount of blur when
///   the occluder is at Z offset == 1. The blur will grow linearly as the Z value increases.
/// - `ambient_color` The color of the ambient shadow.
/// - `spot_color` The color of the spot shadow.
/// - `flags` Options controlling opaque occluder optimizations, shadow appearance, and light
///   position. See [`self::ShadowFlags`].
#[allow(clippy::too_many_arguments)]
pub fn draw_shadow(
    canvas: &Canvas,
    path: &Path,
    z_plane_params: impl Into<Point3>,
    light_pos: impl Into<Point3>,
    light_radius: scalar,
    ambient_color: impl Into<Color>,
    spot_color: impl Into<Color>,
    flags: impl Into<Option<ShadowFlags>>,
) {
    unsafe {
        SkShadowUtils::DrawShadow(
            canvas.native_ptr(),
            path.native(),
            z_plane_params.into().native(),
            light_pos.into().native(),
            light_radius,
            ambient_color.into().into_native(),
            spot_color.into().into_native(),
            flags.into().unwrap_or_else(ShadowFlags::empty).bits(),
        )
    }
}

/// Generate bounding box for shadows relative to path. Includes both the ambient and spot
/// shadow bounds.
///
/// - `ctm` Current transformation matrix to device space.
/// - `path` The occluder used to generate the shadows.
/// - `z_plane_params` Values for the plane function which returns the Z offset of the
///   occluder from the canvas based on local x and y values (the current matrix is not applied).
/// - `light_pos` Generally, the 3D position of the light relative to the canvas plane.
///   If [`self::ShadowFlags::DIRECTIONAL_LIGHT`] is set, this specifies a vector pointing
///   towards the light.
/// - `light_radius` Generally, the radius of the disc light.
///   If [`self::ShadowFlags::DIRECTIONAL_LIGHT`] is set, this specifies the amount of blur when
///   the occluder is at Z offset == 1. The blur will grow linearly as the Z value increases.
/// - `flags` Options controlling opaque occluder optimizations, shadow appearance, and light
///   position. See [`self::ShadowFlags`].
///
/// Returns the shadow bounding box if successful, `None` otherwise.
pub fn local_bounds(
    ctm: &Matrix,
    path: &Path,
    z_plane_params: impl Into<Point3>,
    light_pos: impl Into<Point3>,
    light_radius: scalar,
    flags: u32,
) -> Option<Rect> {
    let mut r = crate::Rect::default();
    unsafe {
        SkShadowUtils::GetLocalBounds(
            ctm.native(),
            path.native(),
            z_plane_params.into().native(),
            light_pos.into().native(),
            light_radius,
            flags,
            r.native_mut(),
        )
    }
    .then_some(r)
}

impl Canvas {
    #[allow(clippy::too_many_arguments)]
    pub fn draw_shadow(
        &self,
        path: &Path,
        z_plane_params: impl Into<Point3>,
        light_pos: impl Into<Point3>,
        light_radius: scalar,
        ambient_color: impl Into<Color>,
        spot_color: impl Into<Color>,
        flags: impl Into<Option<ShadowFlags>>,
    ) -> &Self {
        draw_shadow(
            self,
            path,
            z_plane_params,
            light_pos,
            light_radius,
            ambient_color,
            spot_color,
            flags,
        );
        self
    }
}

/// Helper routine to compute color values for one-pass tonal alpha.
///
/// - `ambient_color` Original ambient color
/// - `spot_color` Original spot color
///
/// Returns the modified ambient and spot colors.
pub fn compute_tonal_colors(
    ambient_color: impl Into<Color>,
    spot_color: impl Into<Color>,
) -> (Color, Color) {
    let mut out_ambient_color = Color::default();
    let mut out_spot_color = Color::default();
    unsafe {
        SkShadowUtils::ComputeTonalColors(
            ambient_color.into().into_native(),
            spot_color.into().into_native(),
            out_ambient_color.native_mut(),
            out_spot_color.native_mut(),
        )
    }
    (out_ambient_color, out_spot_color)
}
