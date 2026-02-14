//! Surface creation functions for Graphite
//!
//! This module provides functions for creating GPU surfaces using the Graphite backend.

use crate::graphite::{BackendTexture, Mipmapped, Recorder};
use crate::prelude::*;
use crate::{ColorSpace, ColorType, IRect, ImageInfo, Surface, SurfaceProps};
use skia_bindings as sb;

/// Create a render target surface using the Graphite backend
///
/// # Arguments
/// - `recorder` - The Graphite recorder to use for rendering
/// - `image_info` - Describes the dimensions and pixel format
/// - `mipmapped` - Whether the surface should have mipmaps
/// - `surface_props` - Surface properties, or `None` for defaults
/// - `label` - Optional label for debugging
///
/// # Returns
/// A new `Surface` backed by Graphite, or `None` if creation failed
pub fn render_target(
    recorder: &mut Recorder,
    image_info: &ImageInfo,
    mipmapped: Mipmapped,
    surface_props: Option<&SurfaceProps>,
    label: Option<&str>,
) -> Option<Surface> {
    let c_label = label.map(|s| std::ffi::CString::new(s).ok()).flatten();
    let label_ptr = c_label
        .as_ref()
        .map(|s| s.as_ptr())
        .unwrap_or(std::ptr::null());

    let surface_props_ptr = surface_props
        .map(|props| props.native() as *const _)
        .unwrap_or(std::ptr::null());

    let surface_ptr = unsafe {
        sb::C_SkSurfaces_RenderTargetGraphite(
            recorder.native_mut(),
            image_info.native(),
            mipmapped,
            surface_props_ptr,
            label_ptr,
        )
    };

    Surface::from_ptr(surface_ptr)
}

/// Wrap an existing backend texture as a Surface
///
/// # Arguments
/// - `recorder` - The Graphite recorder to use for rendering
/// - `backend_texture` - The backend texture to wrap
/// - `color_type` - The color type of the surface
/// - `color_space` - The color space, or `None` for sRGB
/// - `surface_props` - Surface properties, or `None` for defaults
///
/// # Returns
/// A new `Surface` wrapping the backend texture, or `None` if wrapping failed
pub fn wrap_backend_texture(
    recorder: &mut Recorder,
    backend_texture: &BackendTexture,
    color_type: ColorType,
    color_space: Option<&ColorSpace>,
    surface_props: Option<&SurfaceProps>,
) -> Option<Surface> {
    let color_space_ptr = color_space
        .map(|cs| unsafe { cs.native_mut_force() as *mut _ })
        .unwrap_or(std::ptr::null_mut());

    let surface_props_ptr = surface_props
        .map(|props| props.native() as *const _)
        .unwrap_or(std::ptr::null());

    let surface_ptr = unsafe {
        sb::C_SkSurfaces_WrapBackendTextureGraphite(
            recorder.native_mut(),
            backend_texture.native(),
            color_type.into_native(),
            color_space_ptr,
            surface_props_ptr,
        )
    };

    Surface::from_ptr(surface_ptr)
}

/// Convert a Surface to an Image using Graphite
///
/// This creates a snapshot of the surface as an image that can be used
/// for drawing operations.
///
/// # Arguments
/// - `surface` - The surface to convert to an image
///
/// # Returns
/// An `Image` representing the surface contents, or `None` if conversion failed
pub fn as_image(surface: &mut Surface) -> Option<crate::Image> {
    let image_ptr = unsafe { sb::C_SkSurfaces_AsImageGraphite(surface.native_mut()) };
    crate::Image::from_ptr(image_ptr)
}

/// Copy a subset of a Surface to an Image using Graphite
///
/// This creates a copy of the specified region of the surface as an image.
///
/// # Arguments
/// - `surface` - The surface to copy from
/// - `subset` - The region to copy, or `None` to copy the entire surface
/// - `mipmapped` - Whether the resulting image should have mipmaps
///
/// # Returns
/// An `Image` containing a copy of the surface region, or `None` if copying failed
pub fn as_image_copy(
    surface: &mut Surface,
    subset: Option<&IRect>,
    mipmapped: Mipmapped,
) -> Option<crate::Image> {
    let subset_ptr = subset
        .map(|rect| rect.native() as *const _)
        .unwrap_or(std::ptr::null());

    let image_ptr = unsafe {
        sb::C_SkSurfaces_AsImageCopyGraphite(surface.native_mut(), subset_ptr, mipmapped)
    };
    crate::Image::from_ptr(image_ptr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_functions_compile() {
        // Test that all surface creation functions compile correctly
        // We can't actually test them without a valid recorder and context
        let _f1 = render_target
            as fn(
                &mut Recorder,
                &ImageInfo,
                Mipmapped,
                Option<&SurfaceProps>,
                Option<&str>,
            ) -> Option<Surface>;

        let _f2 = wrap_backend_texture
            as fn(
                &mut Recorder,
                &BackendTexture,
                ColorType,
                Option<&ColorSpace>,
                Option<&SurfaceProps>,
            ) -> Option<Surface>;

        let _f3 = as_image as fn(&mut Surface) -> Option<crate::Image>;

        let _f4 =
            as_image_copy as fn(&mut Surface, Option<&IRect>, Mipmapped) -> Option<crate::Image>;
    }
}
