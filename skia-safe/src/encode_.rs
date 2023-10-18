use crate::{Bitmap, EncodedImageFormat, Pixmap};

pub mod jpeg_encoder;
pub mod png_encoder;
#[cfg(feature = "webp-encode")]
pub mod webp_encoder;

impl Pixmap<'_> {
    pub fn encode(
        &self,
        format: EncodedImageFormat,
        quality: impl Into<Option<u32>>,
    ) -> Option<Vec<u8>> {
        crate::encode::pixmap(self, format, quality)
    }
}

impl Bitmap {
    pub fn encode(
        &self,
        format: EncodedImageFormat,
        quality: impl Into<Option<u32>>,
    ) -> Option<Vec<u8>> {
        crate::encode::bitmap(self, format, quality)
    }
}

impl crate::Image {
    pub fn encode<'a>(
        &self,
        context: impl Into<Option<&'a mut crate::gpu::DirectContext>>,
        format: EncodedImageFormat,
        quality: impl Into<Option<u32>>,
    ) -> Option<crate::Data> {
        crate::encode::image(context, self, format, quality)
    }
}

pub mod encode {
    use super::{jpeg_encoder, png_encoder};
    use crate::{Bitmap, EncodedImageFormat, Pixmap};

    pub fn pixmap(
        bitmap: &Pixmap,
        format: EncodedImageFormat,
        quality: impl Into<Option<u32>>,
    ) -> Option<Vec<u8>> {
        let mut data = Vec::new();
        let quality = quality.into().unwrap_or(100).clamp(0, 100);
        match format {
            EncodedImageFormat::JPEG => {
                let opts = jpeg_encoder::Options {
                    quality,
                    ..jpeg_encoder::Options::default()
                };
                jpeg_encoder::encode(bitmap, &mut data, &opts)
            }
            EncodedImageFormat::PNG => {
                let opts = png_encoder::Options::default();
                png_encoder::encode(bitmap, &mut data, &opts)
            }
            #[cfg(feature = "webp-encode")]
            EncodedImageFormat::WEBP => {
                use super::webp_encoder;
                let mut opts = webp_encoder::Options::default();
                if quality == 100 {
                    opts.compression = webp_encoder::Compression::Lossless;
                    opts.quality = 75.0;
                } else {
                    opts.compression = webp_encoder::Compression::Lossy;
                    opts.quality = quality as _;
                }
                webp_encoder::encode(bitmap, &mut data, &opts)
            }
            _ => false,
        }
        .then_some(data)
    }

    pub fn bitmap(
        bitmap: &Bitmap,
        format: EncodedImageFormat,
        quality: impl Into<Option<u32>>,
    ) -> Option<Vec<u8>> {
        let pixels = bitmap.peek_pixels()?;
        pixmap(&pixels, format, quality)
    }

    pub fn image<'a>(
        context: impl Into<Option<&'a mut crate::gpu::DirectContext>>,
        image: &crate::Image,
        image_format: EncodedImageFormat,
        quality: impl Into<Option<u32>>,
    ) -> Option<crate::Data> {
        let quality = quality.into().unwrap_or(100).clamp(0, 100);
        match image_format {
            EncodedImageFormat::JPEG => {
                let opts = jpeg_encoder::Options {
                    quality,
                    ..jpeg_encoder::Options::default()
                };
                jpeg_encoder::encode_image(context, image, &opts)
            }
            EncodedImageFormat::PNG => {
                let opts = png_encoder::Options::default();
                png_encoder::encode_image(context, image, &opts)
            }
            #[cfg(feature = "webp-encode")]
            EncodedImageFormat::WEBP => {
                use super::webp_encoder;
                let mut opts = webp_encoder::Options::default();
                if quality == 100 {
                    opts.compression = webp_encoder::Compression::Lossless;
                    opts.quality = 75.0;
                } else {
                    opts.compression = webp_encoder::Compression::Lossy;
                    opts.quality = quality as _;
                }
                webp_encoder::encode_image(context, image, &opts)
            }
            _ => None,
        }
    }
}
