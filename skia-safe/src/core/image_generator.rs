use crate::prelude::*;
use crate::{
    gpu, image, ColorSpace, Data, ISize, ImageInfo, Matrix, Paint, Picture, YUVAIndex,
    YUVASizeInfo, YUVColorSpace,
};
use skia_bindings as sb;
use skia_bindings::SkImageGenerator;
use std::ffi::c_void;

pub type ImageGenerator = RefHandle<SkImageGenerator>;

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

    pub fn is_valid(&self, mut context: Option<&mut gpu::Context>) -> bool {
        unsafe { sb::C_SkImageGenerator_isValid(self.native(), context.native_ptr_or_null_mut()) }
    }

    #[must_use]
    pub fn get_pixels(&mut self, info: &ImageInfo, pixels: &mut [u8], row_bytes: usize) -> bool {
        // TODO: check if other functions similar to get_pixels use the same asserts:
        assert!(info.height() > 0);
        assert!(
            pixels.len()
                >= ((info.height() - 1) as usize) * row_bytes
                    + ((info.width() as usize) * info.bytes_per_pixel())
        );
        unsafe {
            self.native_mut()
                .getPixels(info.native(), pixels.as_mut_ptr() as _, row_bytes)
        }
    }

    pub fn query_yuva8(
        &self,
    ) -> Option<(
        YUVASizeInfo,
        [YUVAIndex; YUVAIndex::INDEX_COUNT],
        YUVColorSpace,
    )> {
        let mut size_info = YUVASizeInfo::default();
        let mut indices = [YUVAIndex::default(); YUVAIndex::INDEX_COUNT];
        let mut cs = YUVColorSpace::Identity;
        unsafe {
            self.native().queryYUVA8(
                size_info.native_mut(),
                indices.native_mut().as_mut_ptr(),
                &mut cs,
            )
        }
        .if_true_some((size_info, indices, cs))
    }

    // TODO: why does planes need to be a mutable reference?
    pub fn get_yuva8_planes(
        &mut self,
        size_info: &YUVASizeInfo,
        yuva_indices: &[YUVAIndex; YUVAIndex::INDEX_COUNT],
        planes: &mut [&mut [u8]],
    ) -> bool {
        for index in yuva_indices {
            if index.is_valid() {
                let index = index.index as usize;
                let height = size_info.sizes[index].height;
                let width_bytes = size_info.width_bytes[index];
                let plane_size = width_bytes * height as usize;
                if planes[index].len() < plane_size {
                    return false;
                }
            }
        }

        let mut planes: Vec<*mut c_void> = planes
            .iter_mut()
            .map(|p| p.as_mut_ptr() as *mut c_void)
            .collect();

        unsafe {
            self.native_mut().getYUVA8Planes(
                size_info.native(),
                yuva_indices.native().as_ptr(),
                planes.as_mut_ptr(),
            )
        }
    }

    // TODO: generateTexture()

    pub fn from_encoded(encoded: Data) -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_SkImageGenerator_MakeFromEncoded(encoded.into_ptr()) })
    }

    pub fn from_picture(
        size: ISize,
        picture: Picture,
        matrix: Option<&Matrix>,
        paint: Option<&Paint>,
        bit_depth: image::BitDepth,
        color_space: impl Into<Option<ColorSpace>>,
    ) -> Option<Self> {
        Self::from_ptr(unsafe {
            sb::C_SkImageGenerator_MakeFromPicture(
                size.native(),
                picture.into_ptr(),
                matrix.native_ptr_or_null(),
                paint.native_ptr_or_null(),
                bit_depth,
                color_space.into().into_ptr_or_null(),
            )
        })
    }
}
