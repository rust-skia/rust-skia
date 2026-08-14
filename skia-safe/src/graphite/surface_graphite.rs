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
    let c_label = label.and_then(|s| std::ffi::CString::new(s).ok());
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
    color_space: impl Into<Option<ColorSpace>>,
    surface_props: Option<&SurfaceProps>,
) -> Option<Surface> {
    // `C_SkSurfaces_WrapBackendTextureGraphite` adopts the color space (the shim
    // wraps the raw pointer in an `sk_sp` *without* adding a ref), so an owned
    // reference must be transferred via `into_ptr_or_null`. Passing a borrowed
    // pointer would let Skia release a ref it never retained — a refcount
    // underflow / use-after-free of the color space.
    let color_space_ptr = color_space.into().into_ptr_or_null();

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
pub fn as_image(surface: &Surface) -> Option<crate::Image> {
    // `SkSurfaces::AsImage` takes an owning `sk_sp<const SkSurface>` (the shim
    // adopts the pointer) while the caller keeps using `surface`, so transfer a
    // *fresh* reference: `clone` bumps the refcount and `into_ptr` hands that ref
    // over. Passing the borrowed `native_mut()` pointer would make Skia release
    // the caller's ref.
    let surface_ptr = surface.clone().into_ptr();
    let image_ptr = unsafe { sb::C_SkSurfaces_AsImageGraphite(surface_ptr) };
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
    surface: &Surface,
    subset: Option<&IRect>,
    mipmapped: Mipmapped,
) -> Option<crate::Image> {
    let subset_ptr = subset
        .map(|rect| rect.native() as *const _)
        .unwrap_or(std::ptr::null());

    // Transfer a fresh surface reference (clone bumps, `into_ptr` hands it over):
    // the shim adopts an owning `sk_sp<const SkSurface>` and the caller keeps
    // `surface`. See `as_image` for the full rationale.
    let surface_ptr = surface.clone().into_ptr();
    let image_ptr =
        unsafe { sb::C_SkSurfaces_AsImageCopyGraphite(surface_ptr, subset_ptr, mipmapped) };
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
                Option<ColorSpace>,
                Option<&SurfaceProps>,
            ) -> Option<Surface>;

        let _f3 = as_image as fn(&Surface) -> Option<crate::Image>;

        let _f4 = as_image_copy as fn(&Surface, Option<&IRect>, Mipmapped) -> Option<crate::Image>;
    }
}
