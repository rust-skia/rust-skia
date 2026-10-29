//! Encoding of images into the JPEG format, with configurable quality, downsampling, and alpha
//! handling.

use crate::{
    ColorSpace, Data, EncodedOrigin, Pixmap, YUVAPixmaps, interop::RustWStream, prelude::*,
};
use skia_bindings::{SkJpegEncoder_AlphaOption, SkJpegEncoder_Downsample};
use std::io;

/// How to handle input images with alpha (JPEGs must be opaque).
pub type AlphaOption = SkJpegEncoder_AlphaOption;
variant_name!(AlphaOption::BlendOnBlack);

/// The downsampling factor for the U and V components.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Downsample {
    /// Reduction by a factor of two in both the horizontal and vertical directions.
    BothDirections,
    /// Reduction by a factor of two in the horizontal direction.
    Horizontal,
    /// No downsampling.
    No,
}

impl Downsample {
    fn native(&self) -> SkJpegEncoder_Downsample {
        match self {
            Downsample::BothDirections => SkJpegEncoder_Downsample::k420,
            Downsample::Horizontal => SkJpegEncoder_Downsample::k422,
            Downsample::No => SkJpegEncoder_Downsample::k444,
        }
    }
}

/// JPEG encoding options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// Must be in `0..=100` where 0 corresponds to the lowest quality.
    pub quality: u32,
    /// Choose the downsampling factor for the U and V components. This is only meaningful if the
    /// `src` is not `kGray`, since `kGray` will not be encoded as YUV. This is ignored in favor of
    /// `src`'s subsampling when `src` is an [`YUVAPixmaps`].
    ///
    /// Our default value matches the libjpeg-turbo default.
    pub downsample: Downsample,
    /// Jpegs must be opaque. This instructs the encoder on how to handle input images with alpha.
    ///
    /// The default is to ignore the alpha channel and treat the image as opaque. Another option is
    /// to blend the pixels onto a black background before encoding. In the second case, the encoder
    /// supports linear or legacy blending.
    pub alpha_option: AlphaOption,
    /// Optional XMP metadata.
    pub xmp_metadata: Option<String>,
    // TODO: fHdrMetadata (skhdr::Metadata is private)
    pub origin: Option<EncodedOrigin>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            quality: 100,
            downsample: Downsample::BothDirections,
            alpha_option: AlphaOption::Ignore,
            xmp_metadata: None,
            origin: None,
        }
    }
}

/// Encode the `src` pixels to the `dst` stream.
/// `options` may be used to control the encoding behavior.
///
/// Returns `true` on success. Returns `false` on an invalid or unsupported `src`.
pub fn encode<W: io::Write>(pixmap: &Pixmap, writer: &mut W, options: &Options) -> bool {
    let xml_metadata = options.xmp_metadata.as_ref().map(Data::new_str);
    let mut stream = RustWStream::new(writer);

    unsafe {
        skia_bindings::C_SkJpegEncoder_Encode(
            stream.stream_mut(),
            pixmap.native(),
            options.quality as _,
            options.downsample.native(),
            options.alpha_option,
            xml_metadata.as_ref().native_ptr_or_null(),
            options.origin.as_ref().native_ptr_or_null(),
        )
    }
}

/// Encode the `src` pixels to the `dst` stream.
/// `options` may be used to control the encoding behavior.
///
/// Returns `true` on success. Returns `false` on an invalid or unsupported `src`.
pub fn encode_yuva_pixmaps<W: io::Write>(
    writer: &mut W,
    src: &YUVAPixmaps,
    src_color_space: Option<&ColorSpace>,
    options: &Options,
) -> bool {
    let xmp_metadata = options.xmp_metadata.as_ref().map(Data::new_str);
    let mut stream = RustWStream::new(writer);

    unsafe {
        skia_bindings::C_SkJpegEncoder_EncodeYUVAPixmaps(
            stream.stream_mut(),
            src.native(),
            src_color_space.native_ptr_or_null(),
            options.quality as _,
            options.downsample.native(),
            options.alpha_option,
            xmp_metadata.as_ref().native_ptr_or_null(),
            options.origin.as_ref().native_ptr_or_null(),
        )
    }
}

/// Returns the encoded data for the pixmap, or `None` on failure.
pub fn encode_pixmap(src: &Pixmap, options: &Options) -> Option<crate::Data> {
    let xmp_metadata = options.xmp_metadata.as_ref().map(Data::new_str);

    Data::from_ptr(unsafe {
        skia_bindings::C_SkJpegEncoder_EncodePixmap(
            src.native(),
            options.quality as _,
            options.downsample.native(),
            options.alpha_option,
            xmp_metadata.as_ref().native_ptr_or_null(),
            options.origin.as_ref().native_ptr_or_null(),
        )
    })
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
    let xmp_metadata = options.xmp_metadata.as_ref().map(Data::new_str);

    Data::from_ptr(unsafe {
        skia_bindings::C_SkJpegEncoder_EncodeImage(
            context.into().native_ptr_or_null_mut(),
            img.native(),
            options.quality as _,
            options.downsample.native(),
            options.alpha_option,
            xmp_metadata.as_ref().native_ptr_or_null(),
            options.origin.as_ref().native_ptr_or_null(),
        )
    })
}

// TODO: Make (Pixmap + SkYUVAPixmaps)
