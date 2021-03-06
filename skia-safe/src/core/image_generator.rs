#[cfg(feature = "gpu")]
use crate::gpu;
use crate::prelude::*;
use crate::{image, ColorSpace, Data, ISize, ImageInfo, Matrix, Paint, Picture};
use skia_bindings as sb;
use skia_bindings::SkImageGenerator;

pub type ImageGenerator = RefHandle<SkImageGenerator>;
unsafe impl Send for ImageGenerator {}
unsafe impl Sync for ImageGenerator {}

impl NativeDrop for SkImageGenerator {
    fn drop(&mut self) {
        unsafe { sb::C_SkImageGenerator_delete(self) }
    }
}

impl RefHandle<SkImageGenerator> {
    pub fn unique_id(&self) -> u32 {
        self.native().fUniqueID
    }

    pub fn encoded_data(&mut self) -> Option<Data> {
        Data::from_ptr(unsafe { sb::C_SkImageGenerator_refEncodedData(self.native_mut()) })
    }

    pub fn info(&self) -> &ImageInfo {
        ImageInfo::from_native_ref(&self.native().fInfo)
    }

    #[cfg(feature = "gpu")]
    pub fn is_valid(&self, mut context: Option<&mut gpu::RecordingContext>) -> bool {
        unsafe { sb::C_SkImageGenerator_isValid(self.native(), context.native_ptr_or_null_mut()) }
    }

    #[must_use]
    pub fn get_pixels(&mut self, info: &ImageInfo, pixels: &mut [u8], row_bytes: usize) -> bool {
        assert!(info.valid_pixels(row_bytes, pixels));
        unsafe {
            self.native_mut()
                .getPixels(info.native(), pixels.as_mut_ptr() as _, row_bytes)
        }
    }

    // TODO: m86: get_pixels(&Pixmap)

    // TODO: generateTexture()

    #[cfg(feature = "gpu")]
    #[deprecated(since = "0.29.0", note = "removed without replacement")]
    pub fn textures_are_cacheable(&self) -> ! {
        unimplemented!("removed without replacement")
    }

    pub fn from_encoded(encoded: impl Into<Data>) -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_SkImageGenerator_MakeFromEncoded(encoded.into().into_ptr()) })
    }

    pub fn from_picture(
        size: ISize,
        picture: impl Into<Picture>,
        matrix: Option<&Matrix>,
        paint: Option<&Paint>,
        bit_depth: image::BitDepth,
        color_space: impl Into<Option<ColorSpace>>,
    ) -> Option<Self> {
        Self::from_ptr(unsafe {
            sb::C_SkImageGenerator_MakeFromPicture(
                size.native(),
                picture.into().into_ptr(),
                matrix.native_ptr_or_null(),
                paint.native_ptr_or_null(),
                bit_depth,
                color_space.into().into_ptr_or_null(),
            )
        })
    }
}
