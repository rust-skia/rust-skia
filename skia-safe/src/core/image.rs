use crate::prelude::*;
use crate::gpu;
use crate::core::{Picture, Matrix, ColorType, ImageInfo, Data, Bitmap, IRect, YUVColorSpace, AlphaType, ColorSpace, YUVAIndex, ISize, Paint, EncodedImageFormat, IPoint, TileMode, Shader};
use skia_bindings::{
    C_SkImage_MakeFromPicture,
    C_SkImage_MakeFromTexture,
    C_SkImage_MakeFromCompressed,
    C_SkImage_MakeFromEncoded,
    C_SkImage_MakeFromBitmap,
    SkImage,
    C_SkImage_encodeToData,
    C_SkImage_MakeRasterData,
    C_SkImage_MakeCrossContextFromEncoded,
    C_SkImage_MakeFromAdoptedTexture,
    C_SkImage_MakeFromYUVATexturesCopy,
    SkYUVAIndex,
    C_SkImage_MakeFromYUVATexturesCopyWithExternalBackend,
    C_SkImage_MakeFromYUVATextures,
    C_SkImage_MakeFromNV12TexturesCopy,
    C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend,
    SkImage_BitDepth,
    SkImage_CachingHint,
    SkImage_CompressionType,
    C_SkImage_refEncodedData,
    C_SkImage_makeSubset,
    C_SkImage_makeTextureImage,
    C_SkImage_makeNonTextureImage,
    C_SkImage_makeRasterImage,
    C_SkImage_makeColorSpace,
    SkRefCntBase,
    C_SkImage_makeShader
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ImageBitDepth {
    U8 = SkImage_BitDepth::kU8 as _,
    F16 = SkImage_BitDepth::kF16 as _
}

impl NativeTransmutable<SkImage_BitDepth> for ImageBitDepth {}
#[test] fn test_image_bit_depth_layout() { ImageBitDepth::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ImageCachingHint {
    Allow = SkImage_CachingHint::kAllow_CachingHint as _,
    Disallow = SkImage_CachingHint::kDisallow_CachingHint as _
}

impl NativeTransmutable<SkImage_CachingHint> for ImageCachingHint {}
#[test] fn test_caching_hint_layout() { ImageCachingHint::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ImageCompressionType {
    ETC1 = SkImage_CompressionType::kETC1_CompressionType as _
}

impl NativeTransmutable<SkImage_CompressionType> for ImageCompressionType {}
#[test] fn test_compression_type_layout() { ImageCompressionType::test_layout() }

pub type Image = RCHandle<SkImage>;

impl NativeRefCountedBase for SkImage {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl RCHandle<SkImage> {

    pub fn from_raster_data(info: &ImageInfo, pixels: &mut Data, row_bytes: usize) -> Option<Image> {
        Image::from_ptr(unsafe {
            C_SkImage_MakeRasterData(info.native(), pixels.shared_native_mut(), row_bytes)
        })
    }

    pub fn from_bitmap(bitmap: &Bitmap) -> Option<Image> {
        Image::from_ptr(unsafe {
            C_SkImage_MakeFromBitmap(bitmap.native())
        })
    }

    pub fn from_encoded(data: &Data, subset: Option<IRect>) -> Option<Image> {
        Image::from_ptr(unsafe {
            C_SkImage_MakeFromEncoded(
                data.shared_native(),
                subset.native().as_ptr_or_null())
        })
    }

    pub fn from_compressed<IS: Into<ISize>>(
        context: &mut gpu::Context,
        data: &Data,
        size: IS,
        c_type: ImageCompressionType) -> Option<Image> {
        let size = size.into();
        Image::from_ptr(unsafe {
            C_SkImage_MakeFromCompressed(
                context.native_mut(),
                data.shared_native(),
                size.width, size.height,
                c_type.into_native())
        })
    }

    pub fn from_texture(
        context: &mut gpu::Context,
        backend_texture: &gpu::BackendTexture,
        origin: gpu::SurfaceOrigin,
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: Option<&ColorSpace>) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromTexture(
                context.native_mut(),
                backend_texture.native(),
                origin.into_native(),
                color_type.into_native(),
                alpha_type.into_native(),
                color_space.shared_ptr())
        })
    }

    pub fn from_encoded_cross_context(
        context: &mut gpu::Context,
        data: &Data,
        build_mips: bool,
        // not mentions in the docs, but implementation indicates that
        // this can be null.
        color_space: Option<&ColorSpace>,
        limit_to_max_texture_size: bool) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeCrossContextFromEncoded(
                context.native_mut(),
                data.shared_native(),
                build_mips,
                color_space.native_ptr_or_null(), limit_to_max_texture_size)
        })
    }

    pub fn from_adopted_texture(
        context: &mut gpu::Context,
        backend_texture: &gpu::BackendTexture,
        origin: gpu::SurfaceOrigin,
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: Option<&ColorSpace>) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromAdoptedTexture(
                context.native_mut(),
                backend_texture.native(),
                origin.into_native(),
                color_type.into_native(),
                alpha_type.into_native(),
                color_space.shared_ptr())
        })
    }

    pub fn from_yuva_textures_copy(
        context: &mut gpu::Context,
        yuv_color_space: YUVColorSpace,
        yuva_textures: &[gpu::BackendTexture],
        yuva_indices: &[YUVAIndex; 4],
        image_size: ISize,
        image_origin: gpu::SurfaceOrigin,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromYUVATexturesCopy(
                context.native_mut(),
                yuv_color_space.into_native(),
                yuva_textures.native().as_ptr(),
                yuva_indices.native().as_ptr(),
                image_size.into_native(),
                image_origin.into_native(),
                image_color_space.shared_ptr())
        })
    }

    // TODO: consider clippy!
    #[allow(clippy::too_many_arguments)]
    pub fn from_yuva_textures_copy_with_external_backend(
        context: &mut gpu::Context,
        yuv_color_space: YUVColorSpace,
        yuva_textures: &[gpu::BackendTexture],
        yuva_indices: &[YUVAIndex; 4],
        image_size: ISize,
        image_origin: gpu::SurfaceOrigin,
        backend_texture: &gpu::BackendTexture,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        let yuva_indices : Vec<SkYUVAIndex> =
            yuva_indices.iter().map(|i| i.0).collect();

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromYUVATexturesCopyWithExternalBackend(
                context.native_mut(),
                yuv_color_space.into_native(),
                yuva_textures.native().as_ptr(),
                yuva_indices.as_ptr(),
                image_size.into_native(),
                image_origin.into_native(),
                backend_texture.native(),
                image_color_space.shared_ptr())
        })
    }

    pub fn from_yuva_textures(
        context: &mut gpu::Context,
        yuv_color_space: YUVColorSpace,
        yuva_textures: &[gpu::BackendTexture],
        yuva_indices: &[YUVAIndex; 4],
        image_size: ISize,
        image_origin: gpu::SurfaceOrigin,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        let yuva_indices : Vec<SkYUVAIndex> =
            yuva_indices.iter().map(|i| i.0).collect();

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromYUVATextures(
                context.native_mut(),
                yuv_color_space.into_native(),
                yuva_textures.native().as_ptr(),
                yuva_indices.as_ptr(),
                image_size.into_native(),
                image_origin.into_native(),
                image_color_space.shared_ptr())
        })
    }

    pub fn from_nv12_textures_copy(
        context: &mut gpu::Context,
        yuv_color_space: YUVColorSpace,
        nv12_textures: &[gpu::BackendTexture; 2],
        image_origin: gpu::SurfaceOrigin,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromNV12TexturesCopy(
                context.native_mut(),
                yuv_color_space.into_native(),
                nv12_textures.native().as_ptr(),
                image_origin.into_native(),
                image_color_space.shared_ptr())
        })
    }

    pub fn from_nv12_textures_copy_with_external_backend(
        context: &mut gpu::Context,
        yuv_color_space: YUVColorSpace,
        nv12_textures: &[gpu::BackendTexture; 2],
        image_origin: gpu::SurfaceOrigin,
        backend_texture: &gpu::BackendTexture,
        image_color_space: Option<ColorSpace>) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend(
                context.native_mut(),
                yuv_color_space.into_native(),
                nv12_textures.native().as_ptr(),
                image_origin.into_native(),
                backend_texture.native(),
                image_color_space.shared_ptr())
        })
    }

    pub fn from_picture(
        picture: Picture,
        dimensions: ISize,
        matrix: Option<&Matrix>,
        paint: Option<&Paint>,
        bit_depth: ImageBitDepth,
        color_space: Option<ColorSpace>) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_MakeFromPicture(
                picture.shared_native(),
                &dimensions.into_native(),
                matrix.native_ptr_or_null(),
                paint.native_ptr_or_null(),
                bit_depth.into_native(),
                color_space.shared_ptr())
        })
    }

    pub fn image_info(&self) -> ImageInfo {
        ImageInfo::from_native(unsafe {
            (*self.native().imageInfo()).clone()
        })
    }

    pub fn width(&self) -> i32 {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { self.native().height() }
    }

    pub fn dimensions(&self) -> ISize {
        ISize::from_native(unsafe { self.native().dimensions() })
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_native(unsafe { self.native().bounds() })
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { self.native().uniqueID() }
    }

    pub fn alpha_type(&self) -> AlphaType {
        AlphaType::from_native(unsafe { self.native().alphaType() })
    }

    pub fn color_type(&self) -> ColorType {
        ColorType::from_native(unsafe { self.native().colorType() })
    }

    pub fn color_space(&self) -> ColorSpace {
        ColorSpace::from_unshared_ptr(unsafe {
            self.native().colorSpace()
        }).unwrap()
    }

    pub fn is_alpha_only(&self) -> bool {
        unsafe { self.native().isAlphaOnly() }
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { self.native().isOpaque() }
    }

    pub fn as_shader<
        'a, TM: Into<Option<(TileMode, TileMode)>>,
        OM: Into<Option<&'a Matrix>>>(
        &self, tile_modes: TM, local_matrix: OM) -> Shader {
        let tile_modes = tile_modes.into();
        let tm1 = tile_modes.map(|m| m.0).unwrap_or_default();
        let tm2 = tile_modes.map(|m| m.1).unwrap_or_default();
        let local_matrix = local_matrix.into();

        Shader::from_ptr(unsafe {
            C_SkImage_makeShader(self.native(), tm1.into_native(), tm2.into_native(), local_matrix.native_ptr_or_null())
        }).unwrap()
    }

    pub fn is_texture_backed(&self) -> bool {
        unsafe { self.native().isTextureBacked() }
    }

    pub fn is_valid(&self, context: &mut gpu::Context) -> bool {
        unsafe { self.native().isValid(context.native_mut()) }
    }

    pub fn backend_texture(&self, flush_pending_gr_context_io: bool)
        -> (gpu::BackendTexture, gpu::SurfaceOrigin) {

        let mut origin = gpu::SurfaceOrigin::TopLeft;
        let texture = gpu::BackendTexture::from_native(unsafe {
            self.native()
                .getBackendTexture(flush_pending_gr_context_io, origin.native_mut())
        });
        (texture, origin)
    }

    pub fn read_pixels<P>(
        &self,
        dst_info: &ImageInfo,
        pixels: &mut[P],
        dst_row_bytes: usize,
        src: IPoint,
        caching_hint: ImageCachingHint) -> bool {

        if pixels.elements_size_of() !=
            (usize::try_from(dst_info.height()).unwrap() * dst_row_bytes) {
            return false
        }

        unsafe {
            self.native().readPixels(
                dst_info.native(),
                pixels.as_mut_ptr() as _, dst_row_bytes,
                src.x, src.y,
                caching_hint.native().to_owned())
        }
    }

    pub fn encode_to_data(&self, image_format: EncodedImageFormat) -> Option<Data> {
        Data::from_ptr(unsafe {
            C_SkImage_encodeToData(self.native(), image_format.into_native())
        })
    }

    pub fn ref_encoded_data(&self) -> Option<Data> {
        Data::from_ptr(unsafe {
            C_SkImage_refEncodedData(self.native())
        })
    }

    pub fn new_subset(&self, rect: IRect) -> Option<Image> {
        Image::from_ptr(unsafe {
            C_SkImage_makeSubset(self.native(), rect.native())
        })
    }

    pub fn new_texture_image<'a, ICS: Into<Option<&'a ColorSpace>>>(
        &self,
        context: &mut gpu::Context,
        dst_color_space: ICS,
        mip_mapped: gpu::MipMapped) -> Option<Image> {

        Image::from_ptr(unsafe {
            C_SkImage_makeTextureImage(
                self.native(),
                context.native_mut(),
                dst_color_space.into().native_ptr_or_null(),
                mip_mapped.into_native())
        })
    }

    pub fn new_non_texture_image(&self) -> Option<Image> {
        Image::from_ptr(unsafe {
            C_SkImage_makeNonTextureImage(self.native())
        })
    }

    pub fn new_raster_image(&self) -> Option<Image> {
        Image::from_ptr(unsafe {
            C_SkImage_makeRasterImage(self.native())
        })
    }

    pub fn is_lazy_generated(&self) -> bool {
        unsafe { self.native().isLazyGenerated() }
    }

    pub fn new_color_space(&self, color_space: Option<&ColorSpace>) -> Option<Image> {
        Image::from_ptr(unsafe {
            C_SkImage_makeColorSpace(self.native(), color_space.shared_ptr())
        })
    }
}
