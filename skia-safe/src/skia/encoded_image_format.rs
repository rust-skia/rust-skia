use crate::prelude::*;
use skia_bindings::SkEncodedImageFormat;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EncodedImageFormat {
    BMP = SkEncodedImageFormat::kBMP as _,
    GIF = SkEncodedImageFormat::kGIF as _,
    ICO = SkEncodedImageFormat::kICO as _,
    JPEG = SkEncodedImageFormat::kJPEG as _,
    PNG = SkEncodedImageFormat::kPNG as _,
    WBMP = SkEncodedImageFormat::kWBMP as _,
    WEBP = SkEncodedImageFormat::kWEBP as _,
    PKM = SkEncodedImageFormat::kPKM as _,
    KTX = SkEncodedImageFormat::kKTX as _,
    ASTC = SkEncodedImageFormat::kASTC as _,
    DNG = SkEncodedImageFormat::kDNG as _,
    HEIF = SkEncodedImageFormat::kHEIF as _
}

impl NativeTransmutable<SkEncodedImageFormat> for EncodedImageFormat {}
