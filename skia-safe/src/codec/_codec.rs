use crate::{prelude::*, Image};
use crate::{Data, EncodedImageFormat, EncodedOrigin, IRect, ISize, ImageInfo, Pixmap};
use ffi::CStr;
use skia_bindings as sb;
use skia_bindings::{SkCodec, SkCodec_Options, SkRefCntBase};
use std::{ffi, ptr};

pub use sb::SkCodec_Result as Result;

// TODO: implement Display

pub fn result_to_string(result: Result) -> &'static str {
    unsafe { CStr::from_ptr(skia_bindings::SkCodec_ResultToString(result)) }
        .to_str()
        .unwrap()
}

pub use sb::SkCodec_SelectionPolicy as SelectionPolicy;

pub use sb::SkCodec_ZeroInitialized as ZeroInitialized;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Options {
    pub zero_initialized: ZeroInitialized,
    pub subset: IRect,
    pub frame_index: usize,
    pub prior_frame: usize,
}

impl NativeTransmutable<SkCodec_Options> for Options {}

pub type Codec = RCHandle<SkCodec>;

impl NativeBase<SkRefCntBase> for SkCodec {}

impl NativeRefCountedBase for SkCodec {
    type Base = SkRefCntBase;
}

impl Codec {
    // TODO: wrap MakeFromStream
    // TODO: wrap from_data with SkPngChunkReader

    pub fn from_data(data: impl Into<Data>) -> Option<Codec> {
        Codec::from_ptr(unsafe { sb::C_SkCodec_MakeFromData(data.into().into_ptr()) })
    }

    pub fn info(&self) -> ImageInfo {
        let mut info = ImageInfo::default();
        unsafe { sb::C_SkCodec_getInfo(self.native(), info.native_mut()) };
        info
    }

    pub fn dimensions(&self) -> ISize {
        ISize::from_native_c(unsafe { sb::C_SkCodec_dimensions(self.native()) })
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_native_c(unsafe { sb::C_SkCodec_bounds(self.native()) })
    }

    // TODO: getICCProfile

    pub fn origin(&self) -> EncodedOrigin {
        EncodedOrigin::from_native_c(unsafe { sb::C_SkCodec_getOrigin(self.native()) })
    }

    pub fn get_scaled_dimensions(&self, desired_scale: f32) -> ISize {
        ISize::from_native_c(unsafe {
            sb::C_SkCodec_getScaledDimensions(self.native(), desired_scale)
        })
    }

    pub fn valid_subset(&self, desired_subset: impl AsRef<IRect>) -> Option<IRect> {
        let mut desired_subset = *desired_subset.as_ref();
        unsafe { sb::C_SkCodec_getValidSubset(self.native(), desired_subset.native_mut()) }
            .if_true_some(desired_subset)
    }

    pub fn encoded_format(&self) -> EncodedImageFormat {
        unsafe { sb::C_SkCodec_getEncodedFormat(self.native()) }
    }

    pub fn get_pixels_with_options(
        &mut self,
        info: &ImageInfo,
        pixels: &mut [u8],
        row_bytes: usize,
        options: Option<&Options>,
    ) -> Result {
        assert_eq!(pixels.len(), info.compute_byte_size(row_bytes));
        unsafe {
            let native_options = options.map(|options| Self::native_options(options));
            self.native_mut().getPixels(
                info.native(),
                pixels.as_mut_ptr() as *mut _,
                row_bytes,
                native_options.as_ptr_or_null(),
            )
        }
    }

    #[deprecated(
        since = "0.33.1",
        note = "Use the safe variant get_pixels_with_options()."
    )]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_pixels(
        &mut self,
        info: &ImageInfo,
        pixels: *mut ffi::c_void,
        row_bytes: usize,
    ) -> Result {
        self.native_mut()
            .getPixels(info.native(), pixels, row_bytes, ptr::null())
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_pixels_to_pixmap(
        &mut self,
        pixmap: &Pixmap,
        options: Option<&Options>,
    ) -> Result {
        let native_options = options.map(|options| Self::native_options(options));
        self.native_mut().getPixels(
            pixmap.info().native(),
            pixmap.writable_addr(),
            pixmap.row_bytes(),
            native_options.as_ptr_or_null(),
        )
    }

    unsafe fn native_options(options: &Options) -> SkCodec_Options {
        SkCodec_Options {
            fZeroInitialized: options.zero_initialized,
            fSubset: options.subset.native(),
            fFrameIndex: options.frame_index.try_into().unwrap(),
            fPriorFrame: options.prior_frame.try_into().unwrap(),
        }
    }

    pub fn get_image<'a>(
        &mut self,
        info: impl Into<Option<ImageInfo>>,
        options: impl Into<Option<&'a Options>>,
    ) -> std::result::Result<Image, Result> {
        let info = info.into().unwrap_or_else(|| self.info());
        let options: Option<&Options> = options.into();
        let mut result = Result::InternalError;
        match Image::from_ptr(unsafe {
            sb::C_SkCodec_getImage(
                self.native_mut(),
                info.native(),
                options.native_ptr_or_null(),
                &mut result,
            )
        }) {
            Some(image) => Ok(image),
            None => Err(result),
        }
    }

    // TODO: queryYUVAInfo
    // TODO: getYUVAPlanes
    // TODO: startIncrementalDecode
    // TODO: incrementalDecode
    // TODO: startScanlineDecode
    // TODO: getScanlines
    // TODO: skipScanlines
    // TODO: ScanlineOrder
    // TODO: getScanlineOrder
    // TODO: nextScanline
    // TODO: outputScanline
    // TODO: getFrameCount
    // TODO: NoFrame
    // TODO: FrameInfo
    // TODO: getFrameInfo
    // TODO: RepetitionCountInfinite
    // TODO: getRepetitionCount
    // TODO: Register
}

#[cfg(test)]
mod tests {
    use super::{Options, Result, SelectionPolicy, ZeroInitialized};
    use crate::prelude::*;

    #[test]
    fn test_layout() {
        Options::test_layout();
    }

    #[test]
    fn test_naming() {
        let _ = Result::IncompleteInput;
        let _ = SelectionPolicy::PreferStillImage;
        let _ = ZeroInitialized::Yes;
    }
}
