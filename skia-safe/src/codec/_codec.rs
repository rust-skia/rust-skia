use std::{
    ffi::{self, CStr},
    fmt, io,
    marker::PhantomData,
    mem, ptr, result,
};

use skia_bindings::{self as sb, SkCodec, SkCodec_FrameInfo, SkCodec_Options};

use super::codec_animation;
use crate::{
    AlphaType, Data, EncodedImageFormat, EncodedOrigin, IRect, ISize, Image, ImageInfo, Pixmap,
    YUVAPixmapInfo, YUVAPixmaps, interop::RustStream, prelude::*,
    yuva_pixmap_info::SupportedDataTypes,
};

/// Error codes for various [`Codec`] methods.
///
/// Variants:
/// - [`Result::Success`]: General return value for success.
/// - [`Result::IncompleteInput`]: The input is incomplete. A partial image was generated.
/// - [`Result::ErrorInInput`]: Like [`Result::IncompleteInput`], except the input had an error. If returned from an incremental decode, decoding cannot continue, even with more data.
/// - [`Result::InvalidConversion`]: The generator cannot convert to match the request, ignoring dimensions.
/// - [`Result::InvalidScale`]: The generator cannot scale to requested size.
/// - [`Result::InvalidParameters`]: Parameters (besides info) are invalid. e.g. `None` pixels, rowBytes too small, etc.
/// - [`Result::InvalidInput`]: The input did not contain a valid image.
/// - [`Result::CouldNotRewind`]: Fulfilling this request requires rewinding the input, which is not supported for this input.
/// - [`Result::InternalError`]: An internal error, such as OOM.
/// - [`Result::Unimplemented`]: This method is not implemented by this codec. FIXME: Perhaps this should be `kUnsupported`?
/// - [`Result::OutOfMemory`]: If the memory allocation exceeded the provided budget.
pub use sb::SkCodec_Result as Result;
variant_name!(Result::IncompleteInput);

// TODO: implement Display

/// Readable string representing the error code.
pub fn result_to_string(result: Result) -> &'static str {
    unsafe { CStr::from_ptr(skia_bindings::SkCodec_ResultToString(result)) }
        .to_str()
        .unwrap()
}

/// For container formats that contain both still images and image sequences, instruct the decoder
/// how the output should be selected. (Refer to comments for each value for more details.)
///
/// Variants:
/// - [`SelectionPolicy::PreferStillImage`]: If the container format contains both still images and image sequences, [`Codec`] should choose one of the still images. This is the default. Note that `PreferStillImage` may prevent use of the animation features if the input is not rewindable.
/// - [`SelectionPolicy::PreferAnimation`]: If the container format contains both still images and image sequences, [`Codec`] should choose one of the image sequences for animation.
pub use sb::SkCodec_SelectionPolicy as SelectionPolicy;
variant_name!(SelectionPolicy::PreferStillImage);

/// Whether or not the memory passed to [`Codec::get_pixels_with_options`] is zero initialized.
///
/// Variants:
/// - [`ZeroInitialized::Yes`]: The memory passed to [`Codec::get_pixels_with_options`] is zero initialized. The [`Codec`] may take advantage of this by skipping writing zeroes.
/// - [`ZeroInitialized::No`]: The memory passed to [`Codec::get_pixels_with_options`] has not been initialized to zero, so the [`Codec`] must write all zeroes to memory. This is the default. It will be used if no `Options` struct is used.
pub use sb::SkCodec_ZeroInitialized as ZeroInitialized;
variant_name!(ZeroInitialized::Yes);

/// Additional options to pass to [`Codec::get_pixels_with_options`].
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Options {
    pub zero_initialized: ZeroInitialized,
    /// If `Some`, represents a subset of the original image to decode. Must be within the bounds
    /// returned by [`Codec::info()`]. If the encoded format is [`EncodedImageFormat::WEBP`] (the only
    /// one which currently supports subsets), the top and left values must be even.
    ///
    /// In [`Codec::get_pixels_with_options`] and incremental decode, we will attempt to decode the
    /// exact rectangular subset specified by the subset.
    ///
    /// In a scanline decode, it does not make sense to specify a subset top or subset height, since
    /// the client already controls which rows to get and which rows to skip. During scanline
    /// decodes, we will require that the subset top be zero and the subset height be equal to the
    /// full height. We will, however, use the values of subset left and subset width to decode
    /// partial scanlines on calls to [`Codec::get_scanlines()`].
    pub subset: Option<IRect>,
    /// The frame to decode.
    ///
    /// Only meaningful for multi-frame images.
    pub frame_index: usize,
    /// If not `None`, the dst already contains the prior frame at this index.
    ///
    /// Only meaningful for multi-frame images.
    ///
    /// If `frame_index` needs to be blended with a prior frame (as reported by
    /// [`Codec::get_frame_info()`]`[`frame_index`]`.[`FrameInfo::required_frame`]), the client can
    /// set this to any non-`RestorePrevious` frame in `[`required_frame`, `frame_index`)` to
    /// indicate that that frame is already in the dst. `Options.zero_initialized` is ignored in this
    /// case.
    ///
    /// If set to `None`, the codec will decode any necessary required frame(s) first.
    pub prior_frame: Option<usize>,
    /// If non-zero, image decoding will fail if cumulative allocations exceed this many bytes.
    pub max_decode_memory: Option<usize>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            zero_initialized: ZeroInitialized::No,
            subset: None,
            frame_index: 0,
            prior_frame: None,
            max_decode_memory: None,
        }
    }
}

/// Sentinel value used when a frame index implies "no frame":
/// - [`FrameInfo::required_frame`] set to this value means the frame is independent.
/// - `Options.prior_frame` set to this value means no (relevant) prior frame is residing in dst's
///   memory.
pub const NO_FRAME: i32 = sb::SkCodec_kNoFrame;

/// Information about individual frames in a multi-framed image.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameInfo {
    /// The frame that this frame needs to be blended with, or [`NO_FRAME`] if this frame is
    /// independent (so it can be drawn over an uninitialized buffer).
    ///
    /// Note that this is the *earliest* frame that can be used for blending. Any frame from
    /// `[required_frame, i)` can be used, unless its `disposal_method` is
    /// [`codec_animation::DisposalMethod::RestorePrevious`].
    pub required_frame: i32,
    /// Number of milliseconds to show this frame.
    pub duration: i32,
    /// Whether the end marker for this frame is contained in the stream.
    ///
    /// Note: this does not guarantee that an attempt to decode will be complete. There could be an
    /// error in the stream.
    pub fully_received: bool,
    /// This is conservative; it will still return non-opaque if e.g. a color index-based frame has
    /// a color with alpha but does not use it.
    pub alpha_type: AlphaType,
    /// Whether the updated rectangle contains alpha.
    ///
    /// This is conservative; it will still be set to `true` if e.g. a color index-based frame has a
    /// color with alpha but does not use it. In addition, it may be set to `true`, even if the final
    /// frame, after blending, is opaque.
    pub has_alpha_within_bounds: bool,
    /// How this frame should be modified before decoding the next one.
    pub disposal_method: codec_animation::DisposalMethod,
    /// How this frame should blend with the prior frame.
    pub blend: codec_animation::Blend,
    /// The rectangle updated by this frame.
    ///
    /// It may be empty, if the frame does not change the image. It will always be contained by
    /// [`Codec::dimensions()`].
    pub rect: IRect,
}

native_transmutable!(SkCodec_FrameInfo, FrameInfo);

impl Default for FrameInfo {
    fn default() -> Self {
        Self::construct(|frame_info| unsafe { sb::C_SkFrameInfo_Construct(frame_info) })
    }
}

/// The order in which rows are output from the scanline decoder is not the same for all variations
/// of all image types. This explains the possible output row orderings.
///
/// Variants:
/// - [`ScanlineOrder::TopDown`]: By far the most common, this indicates that the image can be decoded reliably using the scanline decoder, and that rows will be output in the logical order.
/// - [`ScanlineOrder::BottomUp`]: This indicates that the scanline decoder reliably outputs rows, but they will be returned in reverse order. If the scanline format is `BottomUp`, the [`Codec::next_scanline()`] API can be used to determine the actual y-coordinate of the next output row, but the client is not forced to take advantage of this, given that it's not too tough to keep track independently.
///
///   For full image decodes, it is safe to get all of the scanlines at once, since the decoder will
///   handle inverting the rows as it decodes.
///
///   For subset decodes and sampling, it is simplest to get and skip scanlines one at a time, using
///   the [`Codec::next_scanline()`] API. It is possible to ask for larger chunks at a time, but this
///   should be used with caution. As with full image decodes, the decoder will handle inverting the
///   requested rows, but rows will still be delivered starting from the bottom of the image.
///
///   Upside down bmps are an example.
pub use sb::SkCodec_SkScanlineOrder as ScanlineOrder;
variant_name!(ScanlineOrder::BottomUp);

/// Whether the full input is expected to contain an animated image (i.e. more than 1 image frame).
/// This can be used to disambiguate the meaning of [`Codec::get_repetition_count()`] returning `0`.
///
/// Variants:
/// - [`IsAnimated::Yes`]: The input is animated.
/// - [`IsAnimated::No`]: The input is not animated.
/// - [`IsAnimated::Unknown`]: It is not yet known whether the input is animated.
pub use sb::SkCodec_IsAnimated as IsAnimated;
variant_name!(IsAnimated::Yes);

/// Abstraction layer directly on top of an image codec.
pub struct Codec<'a> {
    inner: RefHandle<SkCodec>,
    pd: PhantomData<&'a mut dyn io::Read>,
}

impl NativeDelete for SkCodec {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_SkCodec_delete(ptr) }
    }
}

impl fmt::Debug for Codec<'_> {
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

impl Codec<'_> {
    /// If this stream represents an encoded image that we know how to decode, return a [`Codec`]
    /// that can decode it. Otherwise return an error [`Result`] explaining the reason for the
    /// failure.
    ///
    /// As stated above, this call must be able to peek or read `MinBufferedBytesNeeded` to determine
    /// the correct format, and then start reading from the beginning. First it will attempt to peek,
    /// and it assumes that if less than `MinBufferedBytesNeeded` bytes (but more than zero) are
    /// returned, this is because the stream is shorter than this, so falling back to reading would
    /// not provide more data. If `peek()` returns zero bytes, this call will instead attempt to
    /// `read()`. This will require that the stream can be `rewind()`ed.
    ///
    /// The returned [`Result`] is set to either [`Result::Success`] if a [`Codec`] is returned or a
    /// reason for the failure if an error is returned.
    pub fn from_stream<'a, T: io::Read + io::Seek>(
        stream: &'a mut T,
        decoders: &[codecs::Decoder],
        selection_policy: impl Into<Option<SelectionPolicy>>,
    ) -> result::Result<Codec<'a>, Result> {
        let stream = RustStream::new_seekable(stream);
        let mut result = Result::Unimplemented;
        let codec = unsafe {
            sb::C_SkCodec_MakeFromStream(
                // Transfer ownership of the SkStream to the Codec.
                stream.into_native(),
                decoders.as_ptr() as _,
                decoders.len(),
                &mut result,
                selection_policy
                    .into()
                    .unwrap_or(SelectionPolicy::PreferStillImage),
            )
        };
        if result != Result::Success {
            return Err(result);
        }
        Ok(Codec::from_ptr(codec).expect("Codec is null"))
    }

    // TODO: wrap from_data with SkPngChunkReader

    /// If this data represents an encoded image that we know how to decode, return a [`Codec`] that
    /// can decode it. Otherwise return `None`.
    // TODO: Deprecated in Skia
    pub fn from_data(data: impl Into<Data>) -> Option<Codec<'static>> {
        Self::from_ptr(unsafe { sb::C_SkCodec_MakeFromData(data.into().into_ptr()) })
    }

    /// If this data represents an encoded image that we know how to decode, return a [`Codec`] that
    /// can decode it. Otherwise return `None`.
    pub fn from_data_with_decoders(
        data: impl Into<Data>,
        decoders: &[codecs::Decoder],
    ) -> Option<Codec<'static>> {
        Self::from_ptr(unsafe {
            sb::C_SkCodec_MakeFromData2(
                data.into().into_ptr(),
                decoders.as_ptr() as _,
                decoders.len(),
            )
        })
    }

    /// Return a reasonable [`ImageInfo`] to decode into.
    ///
    /// If the image has an ICC profile that does not map to an [`crate::ColorSpace`], the returned
    /// [`ImageInfo`] will use SRGB.
    pub fn info(&self) -> ImageInfo {
        let mut info = ImageInfo::default();
        unsafe { sb::C_SkCodec_getInfo(self.native(), info.native_mut()) };
        info
    }

    pub fn dimensions(&self) -> ISize {
        ISize::from_native_c(unsafe { sb::C_SkCodec_dimensions(self.native()) })
    }

    pub fn bounds(&self) -> IRect {
        IRect::construct(|r| unsafe { sb::C_SkCodec_bounds(self.native(), r) })
    }

    // TODO: getICCProfile()
    // TODO: getHdrMetadata()

    /// Whether the encoded input uses 16 or more bits per component.
    pub fn has_high_bit_depth_encoded_data(&self) -> bool {
        unsafe { sb::C_SkCodec_hasHighBitDepthEncodedData(self.native()) }
    }

    /// Returns the image orientation stored in the EXIF data.
    /// If there is no EXIF data, or if we cannot read the EXIF data, returns
    /// [`EncodedOrigin::TopLeft`].
    pub fn origin(&self) -> EncodedOrigin {
        EncodedOrigin::from_native_c(unsafe { sb::C_SkCodec_getOrigin(self.native()) })
    }

    /// Return a size that approximately supports the desired scale factor.
    /// The codec may not be able to scale efficiently to the exact scale factor requested, so return
    /// a size that approximates that scale. The returned value is the codec's suggestion for the
    /// closest valid scale that it can natively support.
    pub fn get_scaled_dimensions(&self, desired_scale: f32) -> ISize {
        ISize::from_native_c(unsafe {
            sb::C_SkCodec_getScaledDimensions(self.native(), desired_scale)
        })
    }

    /// Return (via `desired_subset`) a subset which can be decoded from this codec, or `None` if
    /// this codec cannot decode subsets or anything similar to `desired_subset`.
    ///
    /// - `desired_subset` In/out parameter. As input, a desired subset of the original bounds (as
    ///   specified by [`Codec::info()`]). If `Some` is returned, `desired_subset` may have been
    ///   modified to a subset which is supported. Although a particular change may have been made
    ///   to `desired_subset` to create something supported, it is possible other changes could
    ///   result in a valid subset. If `None` is returned, `desired_subset`'s value is undefined.
    ///
    /// Returns `Some` if this codec supports decoding `desired_subset` (as returned, potentially
    /// modified).
    pub fn valid_subset(&self, desired_subset: impl AsRef<IRect>) -> Option<IRect> {
        let mut desired_subset = *desired_subset.as_ref();
        unsafe { sb::C_SkCodec_getValidSubset(self.native(), desired_subset.native_mut()) }
            .then_some(desired_subset)
    }

    /// Format of the encoded data.
    pub fn encoded_format(&self) -> EncodedImageFormat {
        unsafe { sb::C_SkCodec_getEncodedFormat(self.native()) }
    }

    /// Decode into the given pixels, a block of memory of size at least
    /// `([`ImageInfo::height()`] - 1) * row_bytes + ([`ImageInfo::width()`] * bytes_per_pixel)`.
    ///
    /// Repeated calls to this function should give the same results, allowing the PixelRef to be
    /// immutable.
    ///
    /// - `info` A description of the format (config, size) expected by the caller. This can simply
    ///   be identical to the info returned by [`Codec::info()`].
    ///
    ///   This contract also allows the caller to specify different output-configs, which the
    ///   implementation can decide to support or not.
    ///
    ///   A size that does not match [`Codec::info()`] implies a request to scale. If the generator
    ///   cannot perform this scale, it will return [`Result::InvalidScale`].
    ///
    ///   If the info contains a non-null [`crate::ColorSpace`], the codec will perform the
    ///   appropriate color space transformation.
    ///
    ///   If the caller passes in the [`crate::ColorSpace`] that maps to the ICC profile reported by
    ///   `getICCProfile()`, the color space transformation is a no-op.
    ///
    ///   If the caller passes a null [`crate::ColorSpace`], no color space transformation will be
    ///   done.
    ///
    /// If a scanline decode is in progress, scanline mode will end, requiring the client to call
    /// [`Codec::start_scanline_decode()`] in order to return to decoding scanlines.
    ///
    /// For certain codecs, reading into a smaller bitmap than the original dimensions may not
    /// produce correct results (e.g. animated webp).
    ///
    /// Returns [`Result::Success`], or another value explaining the type of failure.
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
            sb::SkCodec_getPixels(
                self.native_ptr(),
                info.native(),
                pixels.as_mut_ptr() as *mut _,
                row_bytes,
                native_options.as_ptr_or_null(),
            )
        }
    }

    /// Simplified version of [`Codec::get_pixels_with_options()`] that uses the default
    /// [`Options`].
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
        unsafe {
            sb::SkCodec_getPixels(
                self.native_ptr(),
                info.native(),
                pixels,
                row_bytes,
                ptr::null(),
            )
        }
    }

    /// Decode into the given [`Pixmap`]. See [`Codec::get_pixels_with_options()`] for details.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_pixels_to_pixmap(
        &mut self,
        pixmap: &Pixmap,
        options: Option<&Options>,
    ) -> Result {
        unsafe {
            let native_options = options.map(|options| Self::native_options(options));
            sb::SkCodec_getPixels(
                self.native_ptr(),
                pixmap.info().native(),
                pixmap.writable_addr(),
                pixmap.row_bytes(),
                native_options.as_ptr_or_null(),
            )
        }
    }

    unsafe fn native_options(options: &Options) -> SkCodec_Options {
        SkCodec_Options {
            fZeroInitialized: options.zero_initialized,
            fSubset: options.subset.native().as_ptr_or_null(),
            fFrameIndex: options.frame_index.try_into().unwrap(),
            fPriorFrame: match options.prior_frame {
                None => sb::SkCodec_kNoFrame,
                Some(frame) => frame.try_into().expect("invalid prior frame"),
            },
            fMaxDecodeMemory: options.max_decode_memory.unwrap_or(0),
        }
    }

    /// Return an image containing the pixels. If the codec's origin is not "upper left", this will
    /// rotate the output image accordingly.
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
                self.native_ptr(),
                info.native(),
                options.as_ptr_or_null(),
                &mut result,
            )
        })
        .ok_or(result)
    }

    /// If decoding to YUV is supported, this returns `Some`. Otherwise, this returns `None` and the
    /// caller will ignore the output parameter.
    ///
    /// - `supported_data_types` Indicates the data type/planar config combinations that are
    ///   supported by the caller. If the generator supports decoding to YUV(A), but not as a type in
    ///   `supported_data_types`, this method returns `None`.
    ///
    /// The returned [`YUVAPixmapInfo`] specifies the planar configuration, subsampling, orientation,
    /// chroma siting, plane color types, and row bytes.
    pub fn query_yuva_info(
        &self,
        supported_data_types: &SupportedDataTypes,
    ) -> Option<YUVAPixmapInfo> {
        YUVAPixmapInfo::new_if_valid(|pixmap_info| unsafe {
            self.native()
                .queryYUVAInfo(supported_data_types.native(), pixmap_info)
        })
    }

    /// Returns [`Result::Success`], or another value explaining the type of failure. This always
    /// attempts to perform a full decode. To get the planar configuration without decoding use
    /// [`Codec::query_yuva_info()`].
    ///
    /// - `pixmaps` Contains preallocated pixmaps configured according to a successful call to
    ///   [`Codec::query_yuva_info()`].
    pub fn get_yuva_planes(&mut self, pixmaps: &YUVAPixmaps) -> Result {
        unsafe { sb::SkCodec_getYUVAPlanes(self.native_ptr(), pixmaps.native()) }
    }

    /// Prepare for an incremental decode with the specified options.
    ///
    /// This may require a rewind.
    ///
    /// If [`Result::IncompleteInput`] is returned, may be called again after more data has been
    /// provided to the source stream.
    ///
    /// - `dst_info` Info of the destination. If the dimensions do not match those of
    ///   [`Codec::info()`], this implies a scale.
    /// - `dst` Memory to write to. Needs to be large enough to hold the subset, if present, or the
    ///   full image as described in `dst_info`.
    /// - `options` Contains decoding options, including if memory is zero initialized and whether to
    ///   decode a subset.
    ///
    /// Returns an enum representing success or reason for failure.
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
            sb::SkCodec_startIncrementalDecode(
                self.native_ptr(),
                dst_info.native(),
                dst.as_mut_ptr() as _,
                row_bytes,
                options.as_ptr_or_null(),
            )
        }
    }

    /// Start/continue the incremental decode.
    ///
    /// Not valid to call before a call to [`Codec::start_incremental_decode()`] returns
    /// [`Result::Success`].
    ///
    /// If [`Result::IncompleteInput`] is returned, may be called again after more data has been
    /// provided to the source stream.
    ///
    /// Unlike [`Codec::get_pixels_with_options`] and [`Codec::get_scanlines()`], this does not do
    /// any filling. This is left up to the caller, since they may be skipping lines or continuing
    /// the decode later. In the latter case, they may choose to initialize all lines first, or only
    /// initialize the remaining lines after the first call.
    ///
    /// The returned value is the total number of lines initialized, and is only `Some` if this
    /// method returns [`Result::IncompleteInput`]. Otherwise the implementation may not set it. Note
    /// that some implementations may have initialized this many rows, but not necessarily finished
    /// those rows (e.g. interlaced PNG). This may be useful for determining what rows the client
    /// needs to initialize.
    ///
    /// Returns [`Result::Success`] if all lines requested in [`Codec::start_incremental_decode()`]
    /// have been completely decoded. [`Result::IncompleteInput`] otherwise.
    pub fn incremental_decode(&mut self) -> (Result, Option<usize>) {
        let mut rows_decoded = Default::default();
        let r = unsafe { sb::C_SkCodec_incrementalDecode(self.native_ptr(), &mut rows_decoded) };
        if r == Result::IncompleteInput {
            (r, Some(rows_decoded.try_into().unwrap()))
        } else {
            (r, None)
        }
    }

    /// Prepare for a scanline decode with the specified options.
    ///
    /// After this call, this class will be ready to decode the first scanline.
    ///
    /// This must be called in order to call [`Codec::get_scanlines()`] or
    /// [`Codec::skip_scanlines()`].
    ///
    /// This may require rewinding the stream.
    ///
    /// Not all [`Codec`]s support this.
    ///
    /// - `dst_info` Info of the destination. If the dimensions do not match those of
    ///   [`Codec::info()`], this implies a scale.
    /// - `options` Contains decoding options, including if memory is zero initialized.
    ///
    /// Returns an enum representing success or reason for failure.
    pub fn start_scanline_decode<'a>(
        &mut self,
        dst_info: &ImageInfo,
        options: impl Into<Option<&'a Options>>,
    ) -> Result {
        let options = options
            .into()
            .map(|options| unsafe { Self::native_options(options) });
        unsafe {
            sb::SkCodec_startScanlineDecode(
                self.native_ptr(),
                dst_info.native(),
                options.as_ptr_or_null(),
            )
        }
    }

    /// Write the next `count_lines` scanlines into `dst`.
    ///
    /// Not valid to call before calling [`Codec::start_scanline_decode()`].
    ///
    /// - `dst` Must be large enough to hold `count_lines` scanlines of size `row_bytes`.
    /// - `count_lines` Number of lines to write.
    /// - `row_bytes` Number of bytes per row. Must be large enough to hold a scanline based on the
    ///   [`ImageInfo`] used to create this object.
    ///
    /// Returns the number of lines successfully decoded. If this value is less than `count_lines`,
    /// this will fill the remaining lines with a default value.
    pub fn get_scanlines(&mut self, dst: &mut [u8], count_lines: usize, row_bytes: usize) -> usize {
        assert!(mem::size_of_val(dst) >= count_lines * row_bytes);
        unsafe {
            sb::SkCodec_getScanlines(
                self.native_ptr(),
                dst.as_mut_ptr() as _,
                count_lines.try_into().unwrap(),
                row_bytes,
            )
        }
        .try_into()
        .unwrap()
    }

    /// Skip `count_lines` scanlines.
    ///
    /// Not valid to call before calling [`Codec::start_scanline_decode()`].
    ///
    /// The default version just calls `onGetScanlines` and discards the dst. NOTE: If skipped lines
    /// are the only lines with alpha, this default will make `reallyHasAlpha` return `true`, when it
    /// could have returned `false`.
    ///
    /// Returns `true` if the scanlines were successfully skipped, `false` on failure. Possible
    /// reasons for failure include: an incomplete input image stream; calling this function before
    /// calling [`Codec::start_scanline_decode()`]; if `count_lines` is so large that it moves the
    /// current scanline past the end of the image.
    pub fn skip_scanlines(&mut self, count_lines: usize) -> bool {
        unsafe { sb::SkCodec_skipScanlines(self.native_ptr(), count_lines.try_into().unwrap()) }
    }

    /// An enum representing the order in which scanlines will be returned by the scanline decoder.
    ///
    /// This is undefined before [`Codec::start_scanline_decode()`] is called.
    pub fn scanline_order(&self) -> ScanlineOrder {
        unsafe { sb::C_SkCodec_getScanlineOrder(self.native()) }
    }

    /// Returns the y-coordinate of the next row to be returned by the scanline decoder.
    ///
    /// This will equal the current scanline, except in the case of strangely encoded image types
    /// (bottom-up bmps).
    ///
    /// Results are undefined when not in scanline decoding mode.
    pub fn next_scanline(&self) -> i32 {
        unsafe { sb::C_SkCodec_nextScanline(self.native()) }
    }

    /// Returns the output y-coordinate of the row that corresponds to an input y-coordinate. The
    /// input y-coordinate represents where the scanline is located in the encoded data.
    ///
    /// This will equal `input_scanline`, except in the case of strangely encoded image types
    /// (bottom-up bmps, interlaced gifs).
    pub fn outbound_scanline(&self, input_scanline: i32) -> i32 {
        unsafe { self.native().outputScanline(input_scanline) }
    }

    /// Return the number of frames in the image.
    ///
    /// May require reading through the stream.
    ///
    /// Note that some codecs may be unable to gather `FrameInfo` for all frames in case of
    /// [`Result::IncompleteInput`]. For such codecs [`Codec::get_frame_count()`] may initially
    /// report a low frame count. After the underlying stream provides additional data, then calling
    /// [`Codec::get_frame_count()`] again may return an updated, increased frame count.
    pub fn get_frame_count(&mut self) -> usize {
        unsafe { sb::C_SkCodec_getFrameCount(self.native_ptr()) }
            .try_into()
            .unwrap()
    }

    /// Return info about a single frame.
    ///
    /// Does not read through the stream, so it should be called after [`Codec::get_frame_count()`]
    /// to parse any frames that have not already been parsed.
    ///
    /// Only supported by animated (multi-frame) codecs. Note that this is a property of the codec
    /// (the [`Codec`] subclass), not the image.
    ///
    /// To elaborate, some codecs support animation (e.g. GIF). Others do not (e.g. BMP). Animated
    /// codecs can still represent single frame images. Calling [`Codec::get_frame_info()`]`(0, etc)`
    /// will return `Some` for a single frame GIF even if the overall image is not animated (in that
    /// the pixels on screen do not change over time). When incrementally decoding a GIF image, we
    /// might only know that there's a single frame *so far*.
    ///
    /// For non-animated [`Codec`] subclasses, it's sufficient but not necessary for this method to
    /// always return `None`.
    pub fn get_frame_info(&mut self, index: usize) -> Option<FrameInfo> {
        let mut info = FrameInfo::default();
        unsafe {
            sb::C_SkCodec_getFrameInfo(
                self.native_ptr(),
                index.try_into().unwrap(),
                info.native_mut(),
            )
        }
        .then_some(info)
    }

    /// Return the number of times to repeat, if this image is animated. This number does not
    /// include the first play through of each frame. For example, a repetition count of 4 means that
    /// each frame is played 5 times and then the animation stops.
    ///
    /// It can return `None`, meaning that the animation should loop forever.
    ///
    /// May require reading the stream to find the repetition count.
    ///
    /// As such, future decoding calls may require a rewind.
    ///
    /// [`Codec::get_repetition_count()`] will return `Some(0)` in two cases:
    /// 1. Still (non-animated) images.
    /// 2. Animated images that only play the animation once (i.e. that don't repeat the animation).
    ///
    /// [`Codec::is_animated()`] can be used to disambiguate between these two cases.
    pub fn get_repetition_count(&mut self) -> Option<usize> {
        const REPETITION_COUNT_INFINITE: i32 = -1;
        let count = unsafe { sb::C_SkCodec_getRepetitionCount(self.native_ptr()) };
        if count != REPETITION_COUNT_INFINITE {
            Some(count.try_into().unwrap())
        } else {
            None
        }
    }

    /// Returns whether the full input is expected to contain an animated image (i.e. more than 1
    /// image frame). This can be used to disambiguate the meaning of
    /// [`Codec::get_repetition_count()`] returning `Some(0)` (see
    /// [`Codec::get_repetition_count()`]'s doc comment for more details).
    ///
    /// Note that in some codecs [`Codec::get_frame_count()`] only returns the number of frames for
    /// which all the metadata has been already successfully decoded. Therefore for a partial input
    /// [`Codec::is_animated()`] may return [`IsAnimated::Yes`], even though
    /// [`Codec::get_frame_count()`] may temporarily return `1` until more of the input is available.
    ///
    /// When handling partial input, some codecs may not know until later (e.g. until encountering
    /// additional image frames) whether the given image has more than one frame. Such codecs may
    /// initially return [`IsAnimated::Unknown`] and only later give a definitive "yes" or "no"
    /// answer. GIF format is one example where this may happen.
    ///
    /// Other codecs may be able to decode the information from the metadata present before the first
    /// image frame. Such codecs should be able to give a definitive "yes" or "no" answer as soon as
    /// they are constructed. PNG format is one example where this happens.
    pub fn is_animated(&mut self) -> IsAnimated {
        unsafe { sb::C_SkCodec_isAnimated(self.native_ptr()) }
    }

    // TODO: Register

    fn native(&self) -> &SkCodec {
        self.inner.native()
    }

    fn native_ptr(&mut self) -> *mut SkCodec {
        self.inner.native_ptr()
    }

    pub(crate) fn from_ptr<'a>(codec: *mut SkCodec) -> Option<Codec<'a>> {
        RefHandle::from_ptr(codec).map(|inner| Codec {
            inner,
            pd: PhantomData,
        })
    }
}

pub mod codecs {
    //! The registry of decoder implementations for the image formats that Skia supports natively.
    //!
    //! A [`Decoder`] can be passed to [`Codec::from_stream`] or [`Codec::from_data_with_decoders`]
    //! to decode a specific format.

    use std::{fmt, io, ptr, result, str};

    use skia_bindings::{self as sb, SkCodecs_Decoder};

    use super::Result;
    use crate::{AlphaType, Codec, Image, interop::RustStream, prelude::*};

    /// A decoder implementation for a specific image format that can be passed to
    /// [`Codec::from_stream`] or [`Codec::from_data_with_decoders`] to decode that format.
    pub type Decoder = Handle<SkCodecs_Decoder>;
    unsafe_send_sync!(Decoder);

    impl fmt::Debug for Decoder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Decoder").field("id", &self.id()).finish()
        }
    }

    impl NativeDrop for SkCodecs_Decoder {
        fn drop(&mut self) {
            unsafe { sb::C_SkCodecs_Decoder_destruct(self) }
        }
    }

    impl NativeClone for SkCodecs_Decoder {
        fn clone(&self) -> Self {
            construct(|d| unsafe { sb::C_SkCodecs_Decoder_CopyConstruct(d, self) })
        }
    }

    impl Decoder {
        /// Returns the name identifying this decoder (e.g. "png", "jpeg", ...).
        pub fn id(&self) -> &'static str {
            let mut len: usize = 0;
            let ptr = unsafe { sb::C_SkCodecs_Decoder_getId(self.native(), &mut len) };
            let chars = unsafe { safer::from_raw_parts(ptr as _, len) };
            str::from_utf8(chars).expect("Invalid UTF-8 decoder id")
        }

        /// Whether the given data is in this decoder's format.
        pub fn is_format(&self, data: &[u8]) -> bool {
            unsafe {
                (self.native().isFormat.expect("Decoder::isFormat is null"))(
                    data.as_ptr() as _,
                    data.len(),
                )
            }
        }

        /// If this stream represents an encoded image in this decoder's format, return a [`Codec`]
        /// that can decode it. Otherwise return an error [`Result`] explaining the reason for the
        /// failure.
        pub fn from_stream<'a>(
            &self,
            stream: &'a mut impl io::Read,
        ) -> result::Result<Codec<'a>, Result> {
            let stream = RustStream::new(stream);
            let mut result = Result::Unimplemented;
            let codec = unsafe {
                sb::C_SkCodecs_Decoder_MakeFromStream(
                    self.native(),
                    // Transfer ownership of the SkStream to the Codec.
                    stream.into_native(),
                    &mut result,
                    ptr::null_mut(),
                )
            };
            if result != Result::Success {
                return Err(result);
            }
            Ok(Codec::from_ptr(codec).expect("Codec is null"))
        }
    }

    // TODO: wrap Register()

    /// Return an image using the encoded data, but attempts to defer decoding until the image is
    /// actually used/drawn. This deferral allows the system to cache the result, either on the CPU
    /// or on the GPU, depending on where the image is drawn. If memory is low, the cache may be
    /// purged, causing the next draw of the image to have to re-decode.
    ///
    /// If `alpha_type` is `None`, the image's alpha type will be chosen automatically based on the
    /// image format. Transparent images will default to `[`AlphaType::Premul`]`. If `alpha_type`
    /// contains `[`AlphaType::Premul`]` or `[`AlphaType::Unpremul`]`, that alpha type will be used.
    /// Forcing opaque (passing `[`AlphaType::Opaque`]`) is not allowed, and will return `None`.
    ///
    /// If the encoded format is not supported, `None` is returned.
    pub fn deferred_image(
        codec: Codec<'_>,
        alpha_type: impl Into<Option<AlphaType>>,
    ) -> Option<Borrows<'_, Image>> {
        let alpha_type: Option<AlphaType> = alpha_type.into();
        // Even though codec is getting consumed / moved here, we need to preserve the borrow of the
        // input stream.
        Image::from_ptr(unsafe {
            sb::C_SkCodecs_DeferredImage(codec.inner.into_ptr(), alpha_type.as_ptr_or_null())
        })
        .map(|h| unsafe { Borrows::unchecked_new(h) })
    }
}
