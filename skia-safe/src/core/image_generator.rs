use std::fmt;

use crate::{Data, ImageInfo, Recorder, YUVAPixmapInfo, prelude::*, yuva_pixmap_info};
use skia_bindings::{self as sb, SkImageGenerator};

pub type ImageGenerator = RefHandle<SkImageGenerator>;
unsafe_send_sync!(ImageGenerator);

impl NativeDelete for SkImageGenerator {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_SkImageGenerator_delete(ptr) }
    }
}

impl fmt::Debug for ImageGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageGenerator")
            .field("unique_id", &self.unique_id())
            .field("info", &self.info())
            .finish()
    }
}

impl ImageGenerator {
    pub fn unique_id(&self) -> u32 {
        self.native().fUniqueID
    }

    /// Returns a reference to the encoded (i.e. compressed) representation of this data, or `None`
    /// if there is none.
    pub fn encoded_data(&mut self) -> Option<Data> {
        Data::from_ptr_const(unsafe { sb::C_SkImageGenerator_refEncodedData(self.native_ptr()) })
    }

    /// Returns the [`ImageInfo`] associated with this generator.
    pub fn info(&self) -> &ImageInfo {
        ImageInfo::from_native_ref(&self.native().fInfo)
    }

    /// Returns true if this generator can be used to produce images that will be drawable to the
    /// specified recorder (or to CPU, if `recorder` is `None`).
    ///
    /// - `recorder` optional recorder
    pub fn is_valid(&self, recorder: Option<&mut dyn Recorder>) -> bool {
        unsafe {
            sb::C_SkImageGenerator_isValid(
                self.native(),
                recorder
                    .map(|r| r.as_recorder_ref())
                    .native_ptr_or_null_mut(),
            )
        }
    }

    /// Returns true if this generator will produce protected content.
    pub fn is_protected(self) -> bool {
        unsafe { sb::C_SkImageGenerator_isProtected(self.native()) }
    }

    /// Decodes into the given `pixels`, a block of memory of size at least
    /// `(info.height() - 1) * row_bytes + (info.width() * bytes_per_pixel)`.
    ///
    /// Repeated calls to this function should give the same results, allowing the pixel ref to be
    /// immutable.
    ///
    /// `info` is a description of the format expected by the caller. This can simply be identical
    /// to the info returned by [`Self::info()`]. This contract also allows the caller to specify
    /// different output configs, which the implementation can decide to support or not. A size that
    /// does not match [`Self::info()`] implies a request to scale. If the generator cannot perform
    /// this scale, it returns false.
    ///
    /// - `info` description of the expected format
    /// - `pixels` destination pixel buffer
    /// - `row_bytes` number of bytes per row
    #[must_use]
    pub fn get_pixels(&mut self, info: &ImageInfo, pixels: &mut [u8], row_bytes: usize) -> bool {
        assert!(info.valid_pixels(row_bytes, pixels));
        unsafe {
            sb::SkImageGenerator_getPixels(
                self.native_ptr(),
                info.native(),
                pixels.as_mut_ptr() as _,
                row_bytes,
            )
        }
    }

    // TODO: m86: get_pixels(&Pixmap)

    /// If decoding to YUV is supported, returns `Some` with the planar configuration. Otherwise,
    /// returns `None`.
    ///
    /// - `supported_data_types` indicates the data type/planar config combinations that are
    ///   supported by the caller. If the generator supports decoding to YUV(A), but not as a type
    ///   in `supported_data_types`, this method returns `None`
    ///
    /// The returned value specifies the planar configuration, subsampling, orientation, chroma
    /// siting, plane color types, and row bytes.
    pub fn query_yuva_info(
        &self,
        supported_data_types: &yuva_pixmap_info::SupportedDataTypes,
    ) -> Option<YUVAPixmapInfo> {
        YUVAPixmapInfo::new_if_valid(|info| unsafe {
            self.native()
                .queryYUVAInfo(supported_data_types.native(), info)
        })
    }

    // TODO: getYUVAPlanes()

    pub fn is_texture_generator(&self) -> bool {
        unsafe { sb::C_SkImageGenerator_isTextureGenerator(self.native()) }
    }

    #[deprecated(
        since = "0.64.0",
        note = "Removed, will return `None`. Use Image::deferred_from_encoded_data() or Codec::from_data()"
    )]
    pub fn from_encoded(_encoded: impl Into<Data>) -> Option<Self> {
        debug_assert!(false, "Removed, will return `None` in release builds");
        None
    }
}
