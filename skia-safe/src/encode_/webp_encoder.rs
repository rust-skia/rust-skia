//! Encoding of images into the WebP format, with lossy or lossless compression.

use crate::{Pixmap, interop::RustWStream, prelude::*};
use skia_bindings::SkWebpEncoder_Compression;
use std::io;

/// Whether webp lossy or lossless compression is used.
pub type Compression = SkWebpEncoder_Compression;
variant_name!(Compression::Lossy);

/// WebP encoding options.
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    /// Determines whether we will use webp lossy or lossless compression.
    pub compression: Compression,
    /// Must be in `0.0..=100.0`.
    ///
    /// If `compression` is [`Compression::Lossy`], `quality` corresponds to the visual quality of
    /// the encoding. Decreasing the quality will result in a smaller encoded image.
    /// If `compression` is [`Compression::Lossless`], `quality` corresponds to the amount of effort
    /// put into the encoding. Lower values will compress faster into larger files, while larger
    /// values will compress slower into smaller files.
    ///
    /// This scheme is designed to match the libwebp API.
    pub quality: f32,
    // TODO: fHdrMetadata (skhdr::Metadata is private)
    /// The libwebp compression method in `0..=6`. Lower values encode faster into larger files,
    /// while larger values encode slower into smaller files. `None` selects the default: 3 for
    /// [`Compression::Lossy`] and 0 for [`Compression::Lossless`].
    pub method: Option<u8>,
    // TODO: ICCProfile
    // TODO: ICCProfileDescription
}

impl Default for Options {
    fn default() -> Self {
        Self {
            compression: Compression::Lossy,
            quality: 100.0,
            method: None,
        }
    }
}

/// Encode the `src` pixels to the `dst` stream.
/// `options` may be used to control the encoding behavior.
///
/// Returns `true` on success. Returns `false` on an invalid or unsupported `src`.
pub fn encode<W: io::Write>(pixmap: &Pixmap, writer: &mut W, options: &Options) -> bool {
    let mut stream = RustWStream::new(writer);
    unsafe {
        skia_bindings::C_SkWebpEncoder_Encode(
            stream.stream_mut(),
            pixmap.native(),
            options.compression,
            options.quality,
            options.method.map_or(-1, i32::from),
        )
    }
}

/// Encode the provided image and return the resulting bytes. If the image was created as
/// a texture-backed image on a GPU context, that `context` must be provided so the pixels
/// can be read before being encoded. For raster-backed images, `context` can be `None`.
/// `options` may be used to control the encoding behavior.
///
/// Returns `None` if the pixels could not be read or encoding otherwise fails.
pub fn encode_image<'a>(
    context: impl Into<Option<&'a mut crate::gpu::DirectContext>>,
    img: &crate::Image,
    options: &Options,
) -> Option<crate::Data> {
    crate::Data::from_ptr(unsafe {
        skia_bindings::C_SkWebpEncoder_EncodeImage(
            context.into().native_ptr_or_null_mut(),
            img.native(),
            options.compression,
            options.quality,
            options.method.map_or(-1, i32::from),
        )
    })
}

// TODO: EncodeAnimated
