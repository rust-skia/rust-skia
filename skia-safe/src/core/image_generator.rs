use crate::prelude::*;
use crate::{
    gpu, image, ColorSpace, Data, ISize, ImageInfo, Matrix, Paint, Picture, YUVAIndex,
    YUVASizeInfo, YUVColorSpace,
};
use skia_bindings::{
    C_SkImageGenerator_MakeFromEncoded, C_SkImageGenerator_MakeFromPicture,
    C_SkImageGenerator_delete, C_SkImageGenerator_refEncodedData, SkImageGenerator,
};
use std::ffi::c_void;

pub struct ImageGenerator(*mut SkImageGenerator);

impl NativeAccess<SkImageGenerator> for ImageGenerator {
    fn native(&self) -> &SkImageGenerator {
        unsafe { &*self.0 }
    }

    fn native_mut(&mut self) -> &mut SkImageGenerator {
        unsafe { &mut *self.0 }
    }
}

impl Drop for ImageGenerator {
    fn drop(&mut self) {
        unsafe { C_SkImageGenerator_delete(self.native_mut()) }
    }
}

impl ImageGenerator {
    pub fn unique_id(&self) -> u32 {
        unsafe { self.native().uniqueID() }
    }

    pub fn encoded_data(&mut self) -> Option<Data> {
        Data::from_ptr(unsafe { C_SkImageGenerator_refEncodedData(self.native_mut()) })
    }

    pub fn info(&self) -> ImageInfo {
        ImageInfo::from_native(unsafe { (*self.native().getInfo()).clone() })
    }

    pub fn is_valid(&self, mut context: Option<&mut gpu::Context>) -> bool {
        unsafe { self.native().isValid(context.native_ptr_or_null_mut()) }
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
                cs.native_mut(),
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

    pub fn from_encoded(encoded: &Data) -> Option<ImageGenerator> {
        unsafe { C_SkImageGenerator_MakeFromEncoded(encoded.shared_native()) }
            .to_option()
            .map(ImageGenerator)
    }

    pub fn from_picture(
        size: ISize,
        picture: &Picture,
        matrix: Option<&Matrix>,
        paint: Option<&Paint>,
        bit_depth: image::BitDepth,
        color_space: Option<&ColorSpace>,
    ) -> Option<ImageGenerator> {
        unsafe {
            C_SkImageGenerator_MakeFromPicture(
                size.native(),
                picture.native(),
                matrix.native_ptr_or_null(),
                paint.native_ptr_or_null(),
                bit_depth.into_native(),
                color_space.native_ptr_or_null(),
            )
        }
        .to_option()
        .map(ImageGenerator)
    }
}
