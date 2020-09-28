//! Tests for the various image encoder and decoders skia-safe supports by default.
use skia_safe::{codec, Bitmap, Data, EncodedImageFormat};

/// The supported encoders.
const STANDARD_ENCODERS: &[EncodedImageFormat] =
    &[EncodedImageFormat::JPEG, EncodedImageFormat::PNG];

/// The supported decoders.
const STANDARD_DECODERS: &[EncodedImageFormat] = &[
    EncodedImageFormat::BMP,
    EncodedImageFormat::GIF,
    EncodedImageFormat::ICO,
    EncodedImageFormat::JPEG,
    EncodedImageFormat::PNG,
    EncodedImageFormat::WBMP,
];

fn supported_encoders() -> Vec<EncodedImageFormat> {
    let mut r = STANDARD_ENCODERS.to_vec();
    if cfg!(feature = "webp-encode") {
        r.push(EncodedImageFormat::WEBP);
    }
    r
}

fn supported_decoders() -> Vec<EncodedImageFormat> {
    let mut r = STANDARD_DECODERS.to_vec();
    if cfg!(feature = "webp-decode") {
        r.push(EncodedImageFormat::WEBP);
    }
    r
}

#[test]
fn test_supported_encoders() {
    const DIM: i32 = 16;

    let mut bitmap = Bitmap::new();
    assert!(bitmap.try_alloc_n32_pixels((DIM, DIM), true));

    let supported: Vec<EncodedImageFormat> = ALL
        .iter()
        .copied()
        .filter(|format| bitmap.encode(*format, 100).is_some())
        .collect();

    assert_eq!(supported, supported_encoders());
}

#[test]
fn test_supported_decoders() {
    let supported: Vec<EncodedImageFormat> = DECODER_TESTS
        .iter()
        .filter(|(format, bytes)| {
            let data = Data::new_copy(bytes);
            if let Some(codec) = codec::Codec::from_data(data) {
                codec.encoded_format() == *format
            } else {
                false
            }
        })
        .map(|(format, _bytes)| *format)
        .collect();

    assert_eq!(supported, supported_decoders());
}

type DecoderTest = (EncodedImageFormat, &'static [u8]);

// image files copied from skia/resources/images
const DECODER_TESTS: &[DecoderTest] = &[
    (
        EncodedImageFormat::BMP,
        include_bytes!("images/randPixels.bmp"),
    ),
    (EncodedImageFormat::GIF, include_bytes!("images/box.gif")),
    (
        EncodedImageFormat::ICO,
        include_bytes!("images/color_wheel.ico"),
    ),
    (
        EncodedImageFormat::JPEG,
        include_bytes!("images/color_wheel.jpg"),
    ),
    (
        EncodedImageFormat::PNG,
        include_bytes!("images/mandrill_16.png"),
    ),
    (
        EncodedImageFormat::WBMP,
        include_bytes!("images/mandrill.wbmp"),
    ),
    (
        EncodedImageFormat::WEBP,
        include_bytes!("images/color_wheel.webp"),
    ),
    (
        EncodedImageFormat::DNG,
        include_bytes!("images/sample_1mp.dng"),
    ),
];

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
