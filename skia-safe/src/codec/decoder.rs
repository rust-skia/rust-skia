// TODO: make the SkCodec wrapper complete

use crate::prelude::*;
use crate::{Data, EncodedImageFormat, EncodedOrigin, IRect, ISize, ImageInfo};
use bitflags::_core::ptr::null;
use skia_bindings as sb;
use skia_bindings::{SkCodec, SkCodec_Result, SkRefCntBase};
use std::ffi;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CodecResult {
    Success = SkCodec_Result::Success as _,
    IncompleteInput = SkCodec_Result::IncompleteInput as _,
    ErrorInInput = SkCodec_Result::ErrorInInput as _,
    InvalidConversion = SkCodec_Result::InvalidConversion as _,
    InvalidScale = SkCodec_Result::InvalidScale as _,
    InvalidParameters = SkCodec_Result::InvalidParameters as _,
    InvalidInput = SkCodec_Result::InvalidInput as _,
    CouldNotRewind = SkCodec_Result::CouldNotRewind as _,
    InternalError = SkCodec_Result::InternalError as _,
    Unimplemented = SkCodec_Result::Unimplemented as _,
}

impl NativeTransmutable<SkCodec_Result> for CodecResult {}

#[test]
fn test_codec_result_layout() {
    CodecResult::test_layout();
}

pub type Codec = RCHandle<SkCodec>;

impl NativeBase<SkRefCntBase> for SkCodec {}

impl NativeRefCountedBase for SkCodec {
    type Base = SkRefCntBase;
}

impl RCHandle<SkCodec> {
    pub fn from_data(data: impl Into<Data>) -> Option<Codec> {
        Codec::from_ptr(unsafe { sb::C_SkCodec_MakeFromData(data.into().into_ptr()) })
    }

    pub fn info(&self) -> ImageInfo {
        let mut info = ImageInfo::default();
        unsafe { sb::C_SkCodec_getInfo(self.native(), info.native_mut()) };
        info
    }

    pub fn dimensions(&self) -> ISize {
        ISize::from_native(unsafe { sb::C_SkCodec_dimensions(self.native()) })
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_native(unsafe { sb::C_SkCodec_bounds(self.native()) })
    }

    pub fn origin(&self) -> EncodedOrigin {
        EncodedOrigin::from_native(unsafe { sb::C_SkCodec_getOrigin(self.native()) })
    }

    pub fn encoded_format(&self) -> EncodedImageFormat {
        unsafe { sb::C_SkCodec_getEncodedFormat(self.native()) }
    }

    pub unsafe fn get_pixels(
        &mut self,
        info: &ImageInfo,
        pixels: *mut ffi::c_void,
        row_bytes: usize,
    ) -> CodecResult {
        CodecResult::from_native(self.native_mut().getPixels(
            info.native(),
            pixels,
            row_bytes,
            null(),
        ))
    }
}
