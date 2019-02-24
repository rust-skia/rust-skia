use crate::prelude::*;
use rust_skia::SkEncodedImageFormat;

pub type EncodedImageFormat = EnumHandle<SkEncodedImageFormat>;

impl EnumHandle<SkEncodedImageFormat> {
    pub const BMP: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kBMP);
    pub const GIF: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kGIF);
    pub const ICO: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kICO);
    pub const JPEG: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kJPEG);
    pub const PNG: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kPNG);
    pub const WBMP: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kWBMP);
    pub const WEBP: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kWEBP);
    pub const PKM: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kPKM);
    pub const KTX: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kKTX);
    pub const ASTC: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kASTC);
    pub const DNG: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kDNG);
    pub const HEIF: EncodedImageFormat = EnumHandle(SkEncodedImageFormat::kHEIF);
}
