//! Image utilities for Graphite
//!
//! This module provides functions for working with images in the Graphite backend.

use skia_bindings as sb;

use crate::graphite::{BackendTexture, Recorder};
use crate::prelude::*;
use crate::{AlphaType, ColorSpace, ColorType, IRect, Image};

/// Wrap an existing backend texture as an Image using Graphite
///
/// # Arguments
/// - `recorder` - The Graphite recorder to use
/// - `backend_texture` - The backend texture to wrap
/// - `color_type` - The color type of the image
/// - `alpha_type` - The alpha type of the image
/// - `color_space` - The color space, or `None` for sRGB
///
/// # Returns
/// A new `Image` wrapping the backend texture, or `None` if wrapping failed
pub fn wrap_texture(
    recorder: &mut Recorder,
    backend_texture: &BackendTexture,
    color_type: ColorType,
    alpha_type: AlphaType,
    color_space: impl Into<Option<ColorSpace>>,
) -> Option<Image> {
    // `C_SkImages_WrapTextureGraphite` adopts the color space (the shim wraps the
    // raw pointer in an `sk_sp` *without* adding a ref), so transfer an owned
    // reference via `into_ptr_or_null`. A borrowed pointer would let Skia release
    // a ref it never retained — a refcount underflow / use-after-free.
    let color_space_ptr = color_space.into().into_ptr_or_null();

    let image_ptr = unsafe {
        sb::C_SkImages_WrapTextureGraphite(
            recorder.native_mut(),
            backend_texture.native(),
            color_type.into_native(),
            alpha_type,
            color_space_ptr,
        )
    };

    Image::from_ptr(image_ptr)
}

/// Create a texture-backed image from an existing image using Graphite
///
/// This function uploads the image data to the GPU and creates a texture-backed image.
///
/// # Arguments
/// - `recorder` - The Graphite recorder to use
/// - `image` - The source image to upload
///
/// # Returns
/// A new texture-backed `Image`, or `None` if creation failed
pub fn texture_from_image(recorder: &mut Recorder, image: &Image) -> Option<Image> {
    let image_ptr =
        unsafe { sb::C_SkImages_TextureFromImageGraphite(recorder.native_mut(), image.native()) };

    Image::from_ptr(image_ptr)
}

/// Create a subset texture from an existing image using Graphite
///
/// This function creates a new texture-backed image containing only the specified
/// subset of the source image.
///
/// # Arguments
/// - `recorder` - The Graphite recorder to use
/// - `image` - The source image
/// - `subset` - The region to extract from the source image
///
/// # Returns
/// A new texture-backed `Image` containing the subset, or `None` if creation failed
pub fn subset_texture_from(
    recorder: &mut Recorder,
    image: &Image,
    subset: &IRect,
) -> Option<Image> {
    let image_ptr = unsafe {
        sb::C_SkImages_SubsetTextureFromGraphite(
            recorder.native_mut(),
            image.native(),
            subset.native(),
        )
    };

    Image::from_ptr(image_ptr)
}
