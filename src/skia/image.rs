use std::ptr;
use rust_skia::{
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
    C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend
};
use crate::{
    prelude::*,
    skia::{
        ColorType,
        ImageInfo,
        Data,
        Bitmap,
        IRect,
        YUVColorSpace,
        AlphaType,
        ColorSpace,
        YUVAIndex,
        ISize
    },
    graphics
};

#[derive(RCCloneDrop)]
pub struct Image(pub (crate) *mut SkImage);

impl RefCounted for Image {
    fn _ref(&self) {
        unsafe { (*self.0)._base._base.ref_() }
    }

    fn _unref(&self) {
        unsafe { (*self.0)._base._base.unref() }
    }
}

impl Image {

    pub fn from_raster_data(info: &ImageInfo, pixels: &mut Data, row_bytes: usize) -> Option<Image> {
        pixels._ref();
        unsafe { C_SkImage_MakeRasterData(&info.0, pixels.0, row_bytes) }
            .to_option()
            .map(Image)
    }

    pub fn from_bitmap(bitmap: &Bitmap) -> Option<Image> {
        unsafe { C_SkImage_MakeFromBitmap(&bitmap.0) }
            .to_option()
            .map(Image)
    }

    pub fn from_encoded(data: &Data, subset: Option<IRect>) -> Option<Image> {

        let subset_ptr : *const SkIRect = {
            match subset {
                Some(subset) => &(subset.to_native()) as _,
                None => ptr::null()
            }
        };

        data._ref();
        unsafe { C_SkImage_MakeFromEncoded(data.0, subset_ptr) }
            .to_option()
            .map(Image)
    }

    pub fn from_texture(
        context: &mut graphics::Context,
        backend_texture: &graphics::BackendTexture,
        origin: graphics::SurfaceOrigin,
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: Option<&ColorSpace>) -> Option<Image> {

        let cs_ptr : *mut SkColorSpace = {
            match color_space {
                Some (cs) => { cs._ref(); cs.0 },
                None => ptr::null_mut()
            }
        };

        unsafe { C_SkImage_MakeFromTexture(
            context.0, &backend_texture.0, origin.0, color_type.0, alpha_type.0, cs_ptr) }
            .to_option()
            .map(Image)
    }

    pub fn from_encoded_cross_context(
        context: &mut graphics::Context,
        data: &Data,
        build_mips: bool,
        color_space: &mut ColorSpace,
        limit_to_max_texture_size: bool) -> Option<Image> {

        data._ref();
        unsafe { C_SkImage_MakeCrossContextFromEncoded(
            context.0, data.0, build_mips, color_space.0, limit_to_max_texture_size)}
            .to_option()
            .map(Image)
    }

    pub fn from_adopted_texture(
        context: &mut graphics::Context,
        backend_texture: &graphics::BackendTexture,
        origin: graphics::SurfaceOrigin,
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: Option<&ColorSpace>) -> Option<Image> {

        let cs_ptr : *mut SkColorSpace = {
            match color_space {
                Some (cs) => { cs._ref(); cs.0 },
                None => ptr::null_mut()
            }
        };

        unsafe { C_SkImage_MakeFromAdoptedTexture(
            context.0, &backend_texture.0, origin.0, color_type.0, alpha_type.0, cs_ptr) }
            .to_option()
            .map(Image)
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

        let image_color_space : *mut SkColorSpace = {
            match image_color_space {
                Some (cs) => { cs._ref(); cs.0 },
                None => ptr::null_mut()
            }
        };

        unsafe { C_SkImage_MakeFromYUVATexturesCopy(
            context.0,
            yuv_color_space.0,
            yuva_textures.as_ptr(),
            yuva_indices.as_ptr(),
            image_size.to_native(),
            image_origin.0,
            image_color_space) }
            .to_option()
            .map(Image)
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

        let image_color_space : *mut SkColorSpace = {
            match image_color_space {
                Some (cs) => { cs._ref(); cs.0 },
                None => ptr::null_mut()
            }
        };

        unsafe { C_SkImage_MakeFromYUVATexturesCopyWithExternalBackend(
            context.0,
            yuv_color_space.0,
            yuva_textures.as_ptr(),
            yuva_indices.as_ptr(),
            image_size.to_native(),
            image_origin.0,
            &backend_texture.0,
            image_color_space) }
            .to_option()
            .map(Image)
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

        let image_color_space : *mut SkColorSpace = {
            match image_color_space {
                Some (cs) => { cs._ref(); cs.0 },
                None => ptr::null_mut()
            }
        };

        unsafe { C_SkImage_MakeFromYUVATextures(
            context.0,
            yuv_color_space.0,
            yuva_textures.as_ptr(),
            yuva_indices.as_ptr(),
            image_size.to_native(),
            image_origin.0,
            image_color_space) }
            .to_option()
            .map(Image)
    }

    pub fn from_nv12_textures_copy(
        context: &mut graphics::Context,
        yuv_color_space: YUVColorSpace,
        nv12_textures: &[graphics::BackendTexture; 2],
        image_origin: graphics::SurfaceOrigin,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        let nv12_textures : Vec<GrBackendTexture> =
            nv12_textures.iter().map(|t| t.0.clone()).collect();

        let image_color_space : *mut SkColorSpace = {
            match image_color_space {
                Some (cs) => { cs._ref(); cs.0 },
                None => ptr::null_mut()
            }
        };

        unsafe { C_SkImage_MakeFromNV12TexturesCopy(
            context.0,
            yuv_color_space.0,
            nv12_textures.as_ptr(),
            image_origin.0,
            image_color_space) }
            .to_option()
            .map(Image)
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

        let image_color_space : *mut SkColorSpace = {
            match image_color_space {
                Some (cs) => { cs._ref(); cs.0 },
                None => ptr::null_mut()
            }
        };

        unsafe { C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend(
            context.0,
            yuv_color_space.0,
            nv12_textures.as_ptr(),
            image_origin.0,
            &backend_texture.0,
            image_color_space) }
            .to_option()
            .map(Image)
    }

    pub fn encode_to_data(&self) -> Option<Data> {
        unsafe { C_SkImage_encodeToData(self.0) }
            .to_option()
            .map(Data)
    }
}
