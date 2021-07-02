use crate::{prelude::*, yuva_pixmap_info::SupportedDataTypes, Image, YUVAPixmapInfo, YUVAPixmaps};
use crate::{Data, EncodedImageFormat, EncodedOrigin, IRect, ISize, ImageInfo, Pixmap};
use ffi::CStr;
use skia_bindings as sb;
use skia_bindings::{SkCodec, SkCodec_Options, SkRefCntBase};
use std::{ffi, fmt, mem, ptr};

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
    pub subset: Option<IRect>,
    pub frame_index: usize,
    pub prior_frame: usize,
}

pub use sb::SkCodec_SkScanlineOrder as ScanlineOrder;

pub type Codec = RCHandle<SkCodec>;

impl NativeBase<SkRefCntBase> for SkCodec {}

impl NativeRefCountedBase for SkCodec {
    type Base = SkRefCntBase;
}

impl fmt::Debug for Codec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Codec")
            .field("info", &self.info())
            .field("dimensions", &self.dimensions())
            .field("bounds", &self.bounds())
            .field("origin", &self.origin())
            .field("encoded_format", &self.encoded_format())
            .field("scanline_order", &self.scanline_order())
            .field("next_scanline", &self.next_scanline())
            .finish()
    }
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
            fSubset: options.subset.native().as_ptr_or_null(),
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
        let options = options
            .into()
            .map(|options| unsafe { Self::native_options(options) });
        let mut result = Result::InternalError;
        Image::from_ptr(unsafe {
            sb::C_SkCodec_getImage(
                self.native_mut(),
                info.native(),
                options.as_ptr_or_null(),
                &mut result,
            )
        })
        .ok_or(result)
    }

    pub fn query_yuva_info(
        &self,
        supported_data_types: &SupportedDataTypes,
    ) -> Option<YUVAPixmapInfo> {
        let mut pixmap_info = YUVAPixmapInfo::new_invalid();
        let r = unsafe {
            self.native()
                .queryYUVAInfo(supported_data_types.native(), &mut pixmap_info)
        };
        (r && YUVAPixmapInfo::native_is_valid(&pixmap_info))
            .if_true_then_some(|| YUVAPixmapInfo::from_native_c(pixmap_info))
    }

    pub fn get_yuva_planes(&mut self, pixmaps: &YUVAPixmaps) -> Result {
        unsafe { self.native_mut().getYUVAPlanes(pixmaps.native()) }
    }

    pub fn start_incremental_decode<'a>(
        &mut self,
        dst_info: &ImageInfo,
        dst: &mut [u8],
        row_bytes: usize,
        options: impl Into<Option<&'a Options>>,
    ) -> Result {
        if !dst_info.valid_pixels(row_bytes, dst) {
            return Result::InvalidParameters;
        }
        let options = options
            .into()
            .map(|options| unsafe { Self::native_options(options) });
        unsafe {
            self.native_mut().startIncrementalDecode(
                dst_info.native(),
                dst.as_mut_ptr() as _,
                row_bytes,
                options.as_ptr_or_null(),
            )
        }
    }

    pub fn incremental_decode(&mut self) -> (Result, Option<usize>) {
        let mut rows_decoded = Default::default();
        let r = unsafe { sb::C_SkCodec_incrementalDecode(self.native_mut(), &mut rows_decoded) };
        if r == Result::IncompleteInput {
            (r, Some(rows_decoded.try_into().unwrap()))
        } else {
            (r, None)
        }
    }

    pub fn start_scanline_decode<'a>(
        &mut self,
        dst_info: &ImageInfo,
        options: impl Into<Option<&'a Options>>,
    ) -> Result {
        let options = options
            .into()
            .map(|options| unsafe { Self::native_options(options) });
        unsafe {
            self.native_mut()
                .startScanlineDecode(dst_info.native(), options.as_ptr_or_null())
        }
    }

    pub fn get_scanlines(&mut self, dst: &mut [u8], count_lines: usize, row_bytes: usize) -> usize {
        assert!(mem::size_of_val(dst) >= count_lines * row_bytes);
        unsafe {
            self.native_mut().getScanlines(
                dst.as_mut_ptr() as _,
                count_lines.try_into().unwrap(),
                row_bytes,
            )
        }
        .try_into()
        .unwrap()
    }

    pub fn skip_scanlines(&mut self, count_lines: usize) -> bool {
        unsafe {
            self.native_mut()
                .skipScanlines(count_lines.try_into().unwrap())
        }
    }

    pub fn scanline_order(&self) -> ScanlineOrder {
        unsafe { sb::C_SkCodec_getScanlineOrder(self.native()) }
    }

    pub fn next_scanline(&self) -> i32 {
        unsafe { sb::C_SkCodec_nextScanline(self.native()) }
    }

    pub fn outbound_scanline(&self, input_scanline: i32) -> i32 {
        unsafe { self.native().outputScanline(input_scanline) }
    }

    pub fn get_frame_count(&mut self) -> usize {
        unsafe { sb::C_SkCodec_getFrameCount(self.native_mut()) }
            .try_into()
            .unwrap()
    }

    // TODO: FrameInfo
    // TODO: getFrameInfo

    pub fn get_repetition_count(&mut self) -> Option<usize> {
        const REPETITION_COUNT_INFINITE: i32 = -1;
        let count = unsafe { sb::C_SkCodec_getRepetitionCount(self.native_mut()) };
        if count != REPETITION_COUNT_INFINITE {
            Some(count.try_into().unwrap())
        } else {
            None
        }
    }

    // TODO: Register
}

#[cfg(test)]
mod tests {
    use super::{Result, ScanlineOrder, SelectionPolicy, ZeroInitialized};

    #[test]
    fn test_naming() {
        let _ = Result::IncompleteInput;
        let _ = SelectionPolicy::PreferStillImage;
        let _ = ZeroInitialized::Yes;
        let _ = ScanlineOrder::BottomUp;
    }
}
