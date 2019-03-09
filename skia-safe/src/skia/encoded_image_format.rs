use crate::prelude::*;
use skia_bindings::SkEncodedImageFormat;

pub type EncodedImageFormat = EnumHandle<SkEncodedImageFormat>;

impl EnumHandle<SkEncodedImageFormat> {
    pub const BMP: Self = Self(SkEncodedImageFormat::kBMP);
    pub const GIF: Self = Self(SkEncodedImageFormat::kGIF);
    pub const ICO: Self = Self(SkEncodedImageFormat::kICO);
    pub const JPEG: Self = Self(SkEncodedImageFormat::kJPEG);
    pub const PNG: Self = Self(SkEncodedImageFormat::kPNG);
    pub const WBMP: Self = Self(SkEncodedImageFormat::kWBMP);
    pub const WEBP: Self = Self(SkEncodedImageFormat::kWEBP);
    pub const PKM: Self = Self(SkEncodedImageFormat::kPKM);
    pub const KTX: Self = Self(SkEncodedImageFormat::kKTX);
    pub const ASTC: Self = Self(SkEncodedImageFormat::kASTC);
    pub const DNG: Self = Self(SkEncodedImageFormat::kDNG);
    pub const HEIF: Self = Self(SkEncodedImageFormat::kHEIF);
}
