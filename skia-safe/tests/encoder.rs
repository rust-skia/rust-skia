//! Tests for the various image encoder and decoders skia-safe supports by default.
use skia_safe::{Bitmap, EncodedImageFormat};

/// The supported encoders of skia-safe.
const SUPPORTED_ENCODERS: &[EncodedImageFormat] =
    &[EncodedImageFormat::JPEG, EncodedImageFormat::PNG];

/// An exhaustive match for proving that we test all formats defined in Skia.
#[test]
fn test_encoder_enum() {
    match EncodedImageFormat::GIF {
        EncodedImageFormat::BMP => {}
        EncodedImageFormat::GIF => {}
        EncodedImageFormat::ICO => {}
        EncodedImageFormat::JPEG => {}
        EncodedImageFormat::PNG => {}
        EncodedImageFormat::WBMP => {}
        EncodedImageFormat::WEBP => {}
        EncodedImageFormat::PKM => {}
        EncodedImageFormat::KTX => {}
        EncodedImageFormat::ASTC => {}
        EncodedImageFormat::DNG => {}
        EncodedImageFormat::HEIF => {}
    }
}

const ALL: &[EncodedImageFormat] = &[
    EncodedImageFormat::BMP,
    EncodedImageFormat::GIF,
    EncodedImageFormat::ICO,
    EncodedImageFormat::JPEG,
    EncodedImageFormat::PNG,
    EncodedImageFormat::WBMP,
    EncodedImageFormat::WEBP,
    EncodedImageFormat::PKM,
    EncodedImageFormat::KTX,
    EncodedImageFormat::ASTC,
    EncodedImageFormat::DNG,
    EncodedImageFormat::HEIF,
];

#[test]
fn encode() {
    const DIM: i32 = 16;

    let mut bitmap = Bitmap::new();
    assert!(bitmap.try_alloc_n32_pixels((DIM, DIM), true));

    let supported: Vec<EncodedImageFormat> = ALL
        .iter()
        .copied()
        .filter(|format| bitmap.encode(*format, 100).is_some())
        .collect();

    assert_eq!(&supported, &SUPPORTED_ENCODERS);
}
