use rust_skia::{
    C_SkImage_MakeFromEncoded,
    C_SkImage_MakeFromBitmap,
    SkImage,
    C_SkImage_encodeToData,
    C_SkImage_MakeRasterData,
    SkIRect
};
use std::ptr;
use crate::graphics;
use crate::{
    skia::IRect,
    skia::Bitmap,
    prelude::*,
    skia::Data,
    skia::ImageInfo,
    skia::ColorType,
    skia::AlphaType,
    skia::ColorSpace
};
use rust_skia::C_SkImage_MakeFromTexture;
use rust_skia::SkColorSpace;
use rust_skia::C_SkImage_MakeCrossContextFromEncoded;
use rust_skia::C_SkImage_MakeFromAdoptedTexture;

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

    pub fn encode_to_data(&self) -> Option<Data> {
        unsafe { C_SkImage_encodeToData(self.0) }
            .to_option()
            .map(Data)
    }
}
