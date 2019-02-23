use std::ptr;
use crate::{
    skia::{
        Picture,
        Matrix,
        ColorType,
        ImageInfo,
        Data,
        Bitmap,
        IRect,
        YUVColorSpace,
        AlphaType,
        ColorSpace,
        YUVAIndex,
        ISize,
        Paint
    },
    prelude::*,
    graphics,
};
use rust_skia::{
    SkMatrix,
    C_SkImage_MakeFromPicture,
    C_SkImage_MakeFromTexture,
    C_SkImage_MakeFromEncoded,
    C_SkImage_MakeFromBitmap,
    SkImage,
    C_SkImage_encodeToData,
    C_SkImage_MakeRasterData,
    SkIRect,
    SkColorSpace,
    C_SkImage_MakeCrossContextFromEncoded,
    C_SkImage_MakeFromAdoptedTexture,
    C_SkImage_MakeFromYUVATexturesCopy,
    GrBackendTexture,
    SkYUVAIndex,
    C_SkImage_MakeFromYUVATexturesCopyWithExternalBackend,
    C_SkImage_MakeFromYUVATextures,
    C_SkImage_MakeFromNV12TexturesCopy,
    C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend,
    SkImage_BitDepth,
    SkPaint
};

#[derive(Copy, Clone)]
pub struct ImageBitDepth(pub(crate) SkImage_BitDepth);

impl ImageBitDepth {
    pub const U8: ImageBitDepth = ImageBitDepth(SkImage_BitDepth::kU8);
    pub const F16: ImageBitDepth = ImageBitDepth(SkImage_BitDepth::kF16);
}

pub type Image = RCHandle<SkImage>;

impl RefCounted for SkImage {
    fn _ref(&self) {
        unsafe { self._base._base.ref_() }
    }

    fn _unref(&self) {
        unsafe { self._base._base.unref() }
    }
}

impl Image {

    pub fn from_raster_data(info: &ImageInfo, pixels: Data, row_bytes: usize) -> Option<Image> {
        Image::from_ptr(unsafe { C_SkImage_MakeRasterData(&info.0, pixels.shared_native(), row_bytes) })
    }

    pub fn from_bitmap(bitmap: &Bitmap) -> Option<Image> {
        Image::from_ptr(unsafe { C_SkImage_MakeFromBitmap(&bitmap.0) })
    }

    pub fn from_encoded(data: &Data, subset: Option<IRect>) -> Option<Image> {

        let subset_ptr : *const SkIRect = {
            match subset {
                Some(subset) => &(subset.to_native()) as _,
                None => ptr::null()
            }
        };

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromEncoded(
                data.shared_native(),
                subset_ptr)
        })
    }

    pub fn from_texture(
        context: &mut graphics::Context,
        backend_texture: &graphics::BackendTexture,
        origin: graphics::SurfaceOrigin,
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: Option<&ColorSpace>) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromTexture(
                context.native_mut(),
                &backend_texture.0,
                origin.0,
                color_type.0,
                alpha_type.0,
                color_space.shared_ptr())
        })
    }

    pub fn from_encoded_cross_context(
        context: &mut graphics::Context,
        data: &Data,
        build_mips: bool,
        color_space: &mut ColorSpace,
        limit_to_max_texture_size: bool) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeCrossContextFromEncoded(
                context.native_mut(),
                data.shared_native(),
                build_mips,
                color_space.native_mut(), limit_to_max_texture_size)
        })
    }

    pub fn from_adopted_texture(
        context: &mut graphics::Context,
        backend_texture: &graphics::BackendTexture,
        origin: graphics::SurfaceOrigin,
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: Option<&ColorSpace>) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromAdoptedTexture(
                context.native_mut(),
                &backend_texture.0,
                origin.0,
                color_type.0,
                alpha_type.0,
                color_space.shared_ptr())
        })
    }

    pub fn from_yuva_textures_copy(
        context: &mut graphics::Context,
        yuv_color_space: YUVColorSpace,
        yuva_textures: &[graphics::BackendTexture],
        yuva_indices: &[YUVAIndex; 4],
        image_size: ISize,
        image_origin: graphics::SurfaceOrigin,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        let yuva_textures : Vec<GrBackendTexture> =
            yuva_textures.iter().map(|t| t.0.clone()).collect();

        let yuva_indices : Vec<SkYUVAIndex> =
            yuva_indices.iter().map(|i| i.0).collect();

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromYUVATexturesCopy(
                context.native_mut(),
                yuv_color_space.0,
                yuva_textures.as_ptr(),
                yuva_indices.as_ptr(),
                image_size.to_native(),
                image_origin.0,
                image_color_space.shared_ptr())
        })
    }

    pub fn from_yuva_textures_copy_with_external_backend(
        context: &mut graphics::Context,
        yuv_color_space: YUVColorSpace,
        yuva_textures: &[graphics::BackendTexture],
        yuva_indices: &[YUVAIndex; 4],
        image_size: ISize,
        image_origin: graphics::SurfaceOrigin,
        backend_texture: &graphics::BackendTexture,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        let yuva_textures : Vec<GrBackendTexture> =
            yuva_textures.iter().map(|t| t.0.clone()).collect();

        let yuva_indices : Vec<SkYUVAIndex> =
            yuva_indices.iter().map(|i| i.0).collect();

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromYUVATexturesCopyWithExternalBackend(
                context.native_mut(),
                yuv_color_space.0,
                yuva_textures.as_ptr(),
                yuva_indices.as_ptr(),
                image_size.to_native(),
                image_origin.0,
                &backend_texture.0,
                image_color_space.shared_ptr())
        })
    }

    pub fn from_yuva_textures(
        context: &mut graphics::Context,
        yuv_color_space: YUVColorSpace,
        yuva_textures: &[graphics::BackendTexture],
        yuva_indices: &[YUVAIndex; 4],
        image_size: ISize,
        image_origin: graphics::SurfaceOrigin,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        let yuva_textures : Vec<GrBackendTexture> =
            yuva_textures.iter().map(|t| t.0.clone()).collect();

        let yuva_indices : Vec<SkYUVAIndex> =
            yuva_indices.iter().map(|i| i.0).collect();

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromYUVATextures(
                context.native_mut(),
                yuv_color_space.0,
                yuva_textures.as_ptr(),
                yuva_indices.as_ptr(),
                image_size.to_native(),
                image_origin.0,
                image_color_space.shared_ptr())
        })
    }

    pub fn from_nv12_textures_copy(
        context: &mut graphics::Context,
        yuv_color_space: YUVColorSpace,
        nv12_textures: &[graphics::BackendTexture; 2],
        image_origin: graphics::SurfaceOrigin,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        let nv12_textures : Vec<GrBackendTexture> =
            nv12_textures.iter().map(|t| t.0.clone()).collect();

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromNV12TexturesCopy(
                context.native_mut(),
                yuv_color_space.0,
                nv12_textures.as_ptr(),
                image_origin.0,
                image_color_space.shared_ptr())
        })
    }

    pub fn from_nv12_textures_copy_with_external_backend(
        context: &mut graphics::Context,
        yuv_color_space: YUVColorSpace,
        nv12_textures: &[graphics::BackendTexture; 2],
        image_origin: graphics::SurfaceOrigin,
        backend_texture: &graphics::BackendTexture,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        let nv12_textures : Vec<GrBackendTexture> =
            nv12_textures.iter().map(|t| t.0.clone()).collect();

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend(
                context.native_mut(),
                yuv_color_space.0,
                nv12_textures.as_ptr(),
                image_origin.0,
                &backend_texture.0,
                image_color_space.shared_ptr())
        })
    }

    pub fn from_picture(
        picture: Picture,
        dimensions: ISize,
        matrix: Option<Matrix>,
        paint: Option<Paint>,
        bit_depth: ImageBitDepth,
        color_space: Option<ColorSpace>) -> Option<Image> {

        let matrix : *const SkMatrix =
            match matrix {
                Some(matrix) => &matrix.0,
                None => ptr::null()
            };

        let paint : *const SkPaint =
            match paint {
                Some(paint) => &paint.0,
                None => ptr::null()
            };

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromPicture(
                picture.shared_native(),
                &dimensions.to_native(),
                matrix,
                paint,
                bit_depth.0,
                color_space.shared_ptr())
        })
    }

    pub fn encode_to_data(&self) -> Option<Data> {
        Data::from_ptr(unsafe { C_SkImage_encodeToData(self.native()) })
    }
}
