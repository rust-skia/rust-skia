use std::{io, result};

use skia_bindings as sb;

use super::{decode_stream, Codec, Decoder, Result};

#[derive(Debug)]
pub struct BmpDecoder;

impl Decoder for BmpDecoder {
    const ID: &'static str = "bmp";

    fn is_format(data: &[u8]) -> bool {
        unsafe { sb::C_SkBmpDecoder_IsBmp(data.as_ptr() as _, data.len()) }
    }

    fn decode_stream(stream: &mut impl io::Read) -> result::Result<Codec, Result> {
        decode_stream(stream, sb::C_SkBmpDecoder_Decode)
    }
}

#[derive(Debug)]
pub struct GifDecoder;

impl Decoder for GifDecoder {
    const ID: &'static str = "gif";

    fn is_format(data: &[u8]) -> bool {
        unsafe { sb::C_SkGifDecoder_IsGif(data.as_ptr() as _, data.len()) }
    }

    fn decode_stream(stream: &mut impl io::Read) -> result::Result<Codec, Result> {
        decode_stream(stream, sb::C_SkGifDecoder_Decode)
    }
}

#[derive(Debug)]
pub struct IcoDecoder;

impl Decoder for IcoDecoder {
    const ID: &'static str = "ico";

    fn is_format(data: &[u8]) -> bool {
        unsafe { sb::C_SkIcoDecoder_IsIco(data.as_ptr() as _, data.len()) }
    }

    fn decode_stream(stream: &mut impl io::Read) -> result::Result<Codec, Result> {
        decode_stream(stream, sb::C_SkIcoDecoder_Decode)
    }
}

#[derive(Debug)]
pub struct JpegDecoder;

impl Decoder for JpegDecoder {
    const ID: &'static str = "jpeg";

    fn is_format(data: &[u8]) -> bool {
        unsafe { sb::C_SkJpegDecoder_IsJpeg(data.as_ptr() as _, data.len()) }
    }

    fn decode_stream(stream: &mut impl io::Read) -> result::Result<Codec, Result> {
        decode_stream(stream, sb::C_SkJpegDecoder_Decode)
    }
}

#[derive(Debug)]
pub struct PngDecoder;

impl Decoder for PngDecoder {
    const ID: &'static str = "png";

    fn is_format(data: &[u8]) -> bool {
        unsafe { sb::C_SkPngDecoder_IsPng(data.as_ptr() as _, data.len()) }
    }

    fn decode_stream(stream: &mut impl io::Read) -> result::Result<Codec, Result> {
        decode_stream(stream, sb::C_SkPngDecoder_Decode)
    }
}

#[derive(Debug)]
pub struct WbmpDecoder;

impl Decoder for WbmpDecoder {
    const ID: &'static str = "Wbmp";

    fn is_format(data: &[u8]) -> bool {
        unsafe { sb::C_SkWbmpDecoder_IsWbmp(data.as_ptr() as _, data.len()) }
    }

    fn decode_stream(stream: &mut impl io::Read) -> result::Result<Codec, Result> {
        decode_stream(stream, sb::C_SkWbmpDecoder_Decode)
    }
}

#[cfg(feature = "webp-decode")]
#[derive(Debug)]
pub struct WebpDecoder;

#[cfg(feature = "webp-decode")]
impl Decoder for WebpDecoder {
    const ID: &'static str = "Webp";

    fn is_format(data: &[u8]) -> bool {
        unsafe { sb::C_SkWebpDecoder_IsWebp(data.as_ptr() as _, data.len()) }
    }

    fn decode_stream(stream: &mut impl io::Read) -> result::Result<Codec, Result> {
        decode_stream(stream, sb::C_SkWebpDecoder_Decode)
    }
}
