#[cfg(feature = "gpu")]
use crate::gpu;
use crate::prelude::*;
use crate::{
    AlphaType, Bitmap, ColorSpace, ColorType, Data, EncodedImageFormat, IPoint, IRect, ISize,
    ImageInfo, Matrix, Paint, Picture, SamplingOptions, Shader, TileMode,
};
use crate::{FilterQuality, ImageFilter, ImageGenerator, Pixmap};
use skia_bindings as sb;
use skia_bindings::{SkImage, SkRefCntBase};
use std::{mem, ptr};

#[deprecated(since = "0.37.0", note = "Use skia_safe::SamplingMode")]
pub use super::SamplingMode;

#[deprecated(since = "0.37.0", note = "Use skia_safe::SkMipmapMode")]
pub use super::MipmapMode;

#[deprecated(since = "0.37.0", note = "Use skia_safe::FilterOptions")]
pub use super::FilterOptions;

pub use super::CubicResampler;

// TODO: Add MipmapBuilder as soon it's documented or
//       SkMipmap made its way into the public interface.

pub use skia_bindings::SkImage_BitDepth as BitDepth;
#[test]
fn test_bit_depth_naming() {
    let _ = BitDepth::F16;
}

pub use skia_bindings::SkImage_CachingHint as CachingHint;
#[test]
fn test_caching_hint_naming() {
    let _ = CachingHint::Allow;
}

pub use skia_bindings::SkImage_CompressionType as CompressionType;
#[test]
fn test_compression_type_naming() {
    // legacy type (replaced in m81 by ETC2_RGB8_UNORM)
    #[allow(deprecated)]
    let _ = CompressionType::ETC1;
    // m81: preserve the underscore characters for consistency.
    let _ = CompressionType::BC1_RGBA8_UNORM;
}

pub type Image = RCHandle<SkImage>;
unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl NativeBase<SkRefCntBase> for SkImage {}

impl NativeRefCountedBase for SkImage {
    type Base = SkRefCntBase;
}

impl RCHandle<SkImage> {
    // TODO: MakeRasterCopy()

    pub fn from_raster_data(
        info: &ImageInfo,
        pixels: impl Into<Data>,
        row_bytes: usize,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeRasterData(info.native(), pixels.into().into_ptr(), row_bytes)
        })
    }

    // TODO: MakeFromRaster()

    pub fn from_bitmap(bitmap: &Bitmap) -> Option<Image> {
        Image::from_ptr(unsafe { sb::C_SkImage_MakeFromBitmap(bitmap.native()) })
    }

    pub fn from_generator(mut image_generator: ImageGenerator) -> Option<Image> {
        let image = Image::from_ptr(unsafe {
            sb::C_SkImage_MakeFromGenerator(image_generator.native_mut())
        });
        mem::forget(image_generator);
        image
    }

    pub fn from_encoded(data: impl Into<Data>) -> Option<Image> {
        Image::from_ptr(unsafe { sb::C_SkImage_MakeFromEncoded(data.into().into_ptr()) })
    }

    #[deprecated(since = "0.35.0", note = "Removed without replacement")]
    pub fn decode_to_raster(_encoded: &[u8], _subset: impl Into<Option<IRect>>) -> ! {
        panic!("Removed without replacement")
    }

    #[cfg(feature = "gpu")]
    pub fn new_texture_from_compressed(
        context: &mut gpu::DirectContext,
        data: Data,
        dimensions: impl Into<ISize>,
        ct: CompressionType,
        mipmapped: impl Into<Option<gpu::Mipmapped>>,
        protected: impl Into<Option<gpu::Protected>>,
    ) -> Option<Image> {
        let dimensions = dimensions.into();
        let mipmapped = mipmapped.into().unwrap_or(gpu::Mipmapped::No);
        let protected = protected.into().unwrap_or(gpu::Protected::No);

        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeTextureFromCompressed(
                context.native_mut(),
                data.into_ptr(),
                dimensions.width,
                dimensions.height,
                ct,
                mipmapped,
                protected,
            )
        })
    }

    #[deprecated(since = "0.35.0", note = "Removed without replacement")]
    #[cfg(feature = "gpu")]
    pub fn from_compressed(
        _context: &mut gpu::RecordingContext,
        _data: Data,
        _dimensions: impl Into<ISize>,
        _ct: CompressionType,
    ) -> ! {
        panic!("Removed without replacement.")
    }

    pub fn new_raster_from_compressed(
        data: impl Into<Data>,
        dimensions: impl Into<ISize>,
        ct: CompressionType,
    ) -> Option<Image> {
        let dimensions = dimensions.into();
        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeRasterFromCompressed(
                data.into().into_ptr(),
                dimensions.width,
                dimensions.height,
                ct,
            )
        })
    }

    #[cfg(feature = "gpu")]
    // TODO: add variant with TextureReleaseProc
    pub fn from_texture(
        context: &mut gpu::RecordingContext,
        backend_texture: &gpu::BackendTexture,
        origin: gpu::SurfaceOrigin,
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: impl Into<Option<ColorSpace>>,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeFromTexture(
                context.native_mut(),
                backend_texture.native(),
                origin,
                color_type.into_native(),
                alpha_type,
                color_space.into().into_ptr_or_null(),
            )
        })
    }

    // TODO: MakeFromCompressedTexture

    #[deprecated(since = "0.27.0", note = "renamed, use new_cross_context_from_pixmap")]
    #[cfg(feature = "gpu")]
    pub fn from_pixmap_cross_context(
        context: &mut gpu::DirectContext,
        pixmap: &Pixmap,
        build_mips: bool,
        limit_to_max_texture_size: impl Into<Option<bool>>,
    ) -> Option<Image> {
        Self::new_cross_context_from_pixmap(context, pixmap, build_mips, limit_to_max_texture_size)
    }

    #[cfg(feature = "gpu")]
    pub fn new_cross_context_from_pixmap(
        context: &mut gpu::DirectContext,
        pixmap: &Pixmap,
        build_mips: bool,
        limit_to_max_texture_size: impl Into<Option<bool>>,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeCrossContextFromPixmap(
                context.native_mut(),
                pixmap.native(),
                build_mips,
                limit_to_max_texture_size.into().unwrap_or(false),
            )
        })
    }

    #[cfg(feature = "gpu")]
    pub fn from_adopted_texture(
        context: &mut gpu::RecordingContext,
        backend_texture: &gpu::BackendTexture,
        texture_origin: gpu::SurfaceOrigin,
        color_type: ColorType,
        alpha_type: AlphaType,
        color_space: impl Into<Option<ColorSpace>>,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeFromAdoptedTexture(
                context.native_mut(),
                backend_texture.native(),
                texture_origin,
                color_type.into_native(),
                alpha_type,
                color_space.into().into_ptr_or_null(),
            )
        })
    }

    #[cfg(feature = "gpu")]
    pub fn from_yuva_textures(
        context: &mut gpu::RecordingContext,
        yuva_textures: &gpu::YUVABackendTextures,
        image_color_space: impl Into<Option<ColorSpace>>,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeFromYUVATextures(
                context.native_mut(),
                yuva_textures.native(),
                image_color_space.into().into_ptr_or_null(),
            )
        })
    }

    #[cfg(feature = "gpu")]
    pub fn from_yuva_pixmaps(
        context: &mut gpu::RecordingContext,
        yuva_pixmaps: &crate::YUVAPixmaps,
        build_mips: impl Into<Option<gpu::Mipmapped>>,
        limit_to_max_texture_size: impl Into<Option<bool>>,
        image_color_space: impl Into<Option<ColorSpace>>,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeFromYUVAPixmaps(
                context.native_mut(),
                yuva_pixmaps.native(),
                build_mips.into().unwrap_or(gpu::Mipmapped::No),
                limit_to_max_texture_size.into().unwrap_or(false),
                image_color_space.into().into_ptr_or_null(),
            )
        })
    }

    #[cfg(feature = "gpu")]
    #[deprecated(since = "0.37.0", note = "Removed without replacement")]
    pub fn from_nv12_textures_copy(
        _context: &mut gpu::DirectContext,
        _yuv_color_space: crate::YUVColorSpace,
        _nv12_textures: &[gpu::BackendTexture; 2],
        _image_origin: gpu::SurfaceOrigin,
        _image_color_space: impl Into<Option<ColorSpace>>,
    ) -> ! {
        panic!("Removed without replacement")
    }

    #[cfg(feature = "gpu")]
    pub fn from_nv12_textures_copy_with_external_backend(
        context: &mut gpu::RecordingContext,
        yuv_color_space: crate::YUVColorSpace,
        nv12_textures: &[gpu::BackendTexture; 2],
        texture_origin: gpu::SurfaceOrigin,
        backend_texture: &gpu::BackendTexture,
        image_color_space: impl Into<Option<ColorSpace>>,
        // TODO: m78 introduced textureReleaseProc and releaseContext here.
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend(
                context.native_mut(),
                yuv_color_space,
                nv12_textures.native().as_ptr(),
                texture_origin,
                backend_texture.native(),
                image_color_space.into().into_ptr_or_null(),
            )
        })
    }

    pub fn from_picture(
        picture: impl Into<Picture>,
        dimensions: impl Into<ISize>,
        matrix: Option<&Matrix>,
        paint: Option<&Paint>,
        bit_depth: BitDepth,
        color_space: impl Into<Option<ColorSpace>>,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_MakeFromPicture(
                picture.into().into_ptr(),
                dimensions.into().native(),
                matrix.native_ptr_or_null(),
                paint.native_ptr_or_null(),
                bit_depth,
                color_space.into().into_ptr_or_null(),
            )
        })
    }

    pub fn image_info(&self) -> &ImageInfo {
        ImageInfo::from_native_ref(&self.native().fInfo)
    }

    pub fn width(&self) -> i32 {
        self.image_info().width()
    }

    pub fn height(&self) -> i32 {
        self.image_info().height()
    }

    pub fn dimensions(&self) -> ISize {
        self.image_info().dimensions()
    }

    pub fn bounds(&self) -> IRect {
        self.image_info().bounds()
    }

    pub fn unique_id(&self) -> u32 {
        self.native().fUniqueID
    }

    pub fn alpha_type(&self) -> AlphaType {
        unsafe { self.native().alphaType() }
    }

    pub fn color_type(&self) -> ColorType {
        ColorType::from_native_c(unsafe { self.native().colorType() })
    }

    pub fn color_space(&self) -> ColorSpace {
        ColorSpace::from_unshared_ptr(unsafe { self.native().colorSpace() }).unwrap()
    }

    pub fn is_alpha_only(&self) -> bool {
        unsafe { self.native().isAlphaOnly() }
    }

    pub fn is_opaque(&self) -> bool {
        self.alpha_type().is_opaque()
    }

    pub fn to_shader_with_sampling_options<'a>(
        &self,
        tile_modes: impl Into<Option<(TileMode, TileMode)>>,
        sampling_options: &SamplingOptions,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Shader {
        let tile_modes = tile_modes.into();
        let tm1 = tile_modes.map(|m| m.0).unwrap_or_default();
        let tm2 = tile_modes.map(|m| m.1).unwrap_or_default();

        Shader::from_ptr(unsafe {
            sb::C_SkImage_makeShaderWithSamplingOptions(
                self.native(),
                tm1,
                tm2,
                sampling_options.native(),
                local_matrix.into().native_ptr_or_null(),
            )
        })
        .unwrap()
    }

    pub fn to_shader_with_cubic_resampler<'a>(
        &self,
        tile_modes: impl Into<Option<(TileMode, TileMode)>>,
        cubic_resampler: CubicResampler,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Option<Shader> {
        let tile_modes = tile_modes.into();
        let tm1 = tile_modes.map(|m| m.0).unwrap_or_default();
        let tm2 = tile_modes.map(|m| m.1).unwrap_or_default();

        Shader::from_ptr(unsafe {
            sb::C_SkImage_makeShaderWithCubicResampler(
                self.native(),
                tm1,
                tm2,
                cubic_resampler.into_native(),
                local_matrix.into().native_ptr_or_null(),
            )
        })
    }

    pub fn to_shader_with_filter_options<'a>(
        &self,
        tile_modes: impl Into<Option<(TileMode, TileMode)>>,
        filter_options: FilterOptions,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Shader {
        let tile_modes = tile_modes.into();
        let tm1 = tile_modes.map(|m| m.0).unwrap_or_default();
        let tm2 = tile_modes.map(|m| m.1).unwrap_or_default();

        Shader::from_ptr(unsafe {
            sb::C_SkImage_makeShaderWithFilterOptions(
                self.native(),
                tm1,
                tm2,
                filter_options.into_native(),
                local_matrix.into().native_ptr_or_null(),
            )
        })
        .unwrap()
    }

    pub fn to_shader<'a>(
        &self,
        tile_modes: impl Into<Option<(TileMode, TileMode)>>,
        local_matrix: impl Into<Option<&'a Matrix>>,
    ) -> Shader {
        let tile_modes = tile_modes.into();
        let tm1 = tile_modes.map(|m| m.0).unwrap_or_default();
        let tm2 = tile_modes.map(|m| m.1).unwrap_or_default();

        Shader::from_ptr(unsafe {
            sb::C_SkImage_makeShader(
                self.native(),
                tm1,
                tm2,
                local_matrix.into().native_ptr_or_null(),
            )
        })
        .unwrap()
    }

    pub fn to_shader_with_quality<'a>(
        &self,
        tile_modes: impl Into<Option<(TileMode, TileMode)>>,
        local_matrix: impl Into<Option<&'a Matrix>>,
        filter_quality: FilterQuality,
    ) -> Shader {
        let tile_modes = tile_modes.into();
        let tm1 = tile_modes.map(|m| m.0).unwrap_or_default();
        let tm2 = tile_modes.map(|m| m.1).unwrap_or_default();
        Shader::from_ptr(unsafe {
            sb::C_SkImage_makeShaderWithQuality(
                self.native(),
                tm1,
                tm2,
                local_matrix.into().native_ptr_or_null(),
                filter_quality,
            )
        })
        .unwrap()
    }

    pub fn peek_pixels(&self) -> Option<Borrows<Pixmap>> {
        let mut pixmap = Pixmap::default();
        unsafe { self.native().peekPixels(pixmap.native_mut()) }
            .if_true_then_some(|| pixmap.borrows(self))
    }

    pub fn is_texture_backed(&self) -> bool {
        unsafe { self.native().isTextureBacked() }
    }

    #[cfg(feature = "gpu")]
    pub fn is_valid(&self, context: &mut gpu::RecordingContext) -> bool {
        unsafe { self.native().isValid(context.native_mut()) }
    }

    #[cfg(feature = "gpu")]
    pub fn flush_with_info(
        &mut self,
        context: &mut gpu::DirectContext,
        flush_info: &gpu::FlushInfo,
    ) -> gpu::SemaphoresSubmitted {
        unsafe {
            self.native_mut()
                .flush(context.native_mut(), flush_info.native())
        }
    }

    // TODO: m86: implement new flush() variant that is based on flush_with_info() as soon the old
    // flush() is removed.
    #[cfg(feature = "gpu")]
    #[deprecated(
        since = "0.33.0",
        note = "use flushAndSubmit() or flush_with_info(,&gpu::FlushInfo::default())"
    )]
    pub fn flush(&mut self, context: &mut gpu::DirectContext) {
        self.flush_and_submit(context)
    }

    #[cfg(feature = "gpu")]
    pub fn flush_and_submit(&mut self, context: &mut gpu::DirectContext) {
        unsafe { self.native_mut().flushAndSubmit(context.native_mut()) }
    }

    #[cfg(feature = "gpu")]
    pub fn backend_texture(
        &self,
        flush_pending_gr_context_io: bool,
    ) -> Option<(gpu::BackendTexture, gpu::SurfaceOrigin)> {
        let mut origin = gpu::SurfaceOrigin::TopLeft;
        let mut backend_texture = unsafe { sb::GrBackendTexture::new() };
        unsafe {
            sb::C_SkImage_getBackendTexture(
                self.native(),
                flush_pending_gr_context_io,
                &mut origin,
                &mut backend_texture,
            );
            gpu::BackendTexture::from_native_if_valid(backend_texture)
        }
        .map(|texture| (texture, origin))
    }

    #[cfg(feature = "gpu")]
    pub fn read_pixels_with_context<'a, P>(
        &self,
        context: impl Into<Option<&'a mut gpu::DirectContext>>,
        dst_info: &ImageInfo,
        pixels: &mut [P],
        dst_row_bytes: usize,
        src: impl Into<IPoint>,
        caching_hint: CachingHint,
    ) -> bool {
        if pixels.elements_size_of()
            != (usize::try_from(dst_info.height()).unwrap() * dst_row_bytes)
        {
            return false;
        }

        let src = src.into();

        unsafe {
            self.native().readPixels(
                context.into().native_ptr_or_null_mut(),
                dst_info.native(),
                pixels.as_mut_ptr() as _,
                dst_row_bytes,
                src.x,
                src.y,
                caching_hint,
            )
        }
    }

    #[cfg(feature = "gpu")]
    pub fn read_pixels_to_pixmap_with_context<'a>(
        &self,
        context: impl Into<Option<&'a mut gpu::DirectContext>>,
        dst: &Pixmap,
        src: impl Into<IPoint>,
        caching_hint: CachingHint,
    ) -> bool {
        let src = src.into();

        unsafe {
            self.native().readPixels1(
                context.into().native_ptr_or_null_mut(),
                dst.native(),
                src.x,
                src.y,
                caching_hint,
            )
        }
    }

    // _not_ deprecated, because we support separate functions in `gpu` feature builds.
    pub fn read_pixels<P>(
        &self,
        dst_info: &ImageInfo,
        pixels: &mut [P],
        dst_row_bytes: usize,
        src: impl Into<IPoint>,
        caching_hint: CachingHint,
    ) -> bool {
        if pixels.elements_size_of()
            != (usize::try_from(dst_info.height()).unwrap() * dst_row_bytes)
        {
            return false;
        }

        let src = src.into();

        unsafe {
            self.native().readPixels(
                ptr::null_mut(),
                dst_info.native(),
                pixels.as_mut_ptr() as _,
                dst_row_bytes,
                src.x,
                src.y,
                caching_hint,
            )
        }
    }

    #[cfg(feature = "gpu")]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn read_pixels_to_pixmap(
        &self,
        dst: &Pixmap,
        src: impl Into<IPoint>,
        caching_hint: CachingHint,
    ) -> bool {
        let src = src.into();

        self.native()
            .readPixels1(ptr::null_mut(), dst.native(), src.x, src.y, caching_hint)
    }

    // TODO:
    // AsyncReadResult, ReadPixelsContext, ReadPixelsCallback, RescaleGamma,
    // asyncRescaleAndReadPixels, asyncRescaleAndReadPixelsYUV420

    #[must_use]
    pub fn scale_pixels(
        &self,
        dst: &Pixmap,
        filter_quality: FilterQuality,
        caching_hint: impl Into<Option<CachingHint>>,
    ) -> bool {
        unsafe {
            self.native().scalePixels(
                dst.native(),
                filter_quality,
                caching_hint.into().unwrap_or(CachingHint::Allow),
            )
        }
    }

    pub fn encode_to_data(&self, image_format: EncodedImageFormat) -> Option<Data> {
        self.encode_to_data_with_quality(image_format, 100)
    }

    pub fn encode_to_data_with_quality(
        &self,
        image_format: EncodedImageFormat,
        quality: i32,
    ) -> Option<Data> {
        Data::from_ptr(unsafe { sb::C_SkImage_encodeToData(self.native(), image_format, quality) })
    }

    pub fn encoded_data(&self) -> Option<Data> {
        Data::from_ptr(unsafe { sb::C_SkImage_refEncodedData(self.native()) })
    }

    pub fn new_subset(&self, rect: impl AsRef<IRect>) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_makeSubset(self.native(), rect.as_ref().native(), ptr::null_mut())
        })
    }

    #[cfg(feature = "gpu")]
    pub fn new_subset_with_context<'a>(
        &self,
        rect: impl AsRef<IRect>,
        direct: impl Into<Option<&'a mut gpu::DirectContext>>,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_makeSubset(
                self.native(),
                rect.as_ref().native(),
                direct.into().native_ptr_or_null_mut(),
            )
        })
    }

    pub fn has_mipmaps(&self) -> bool {
        unsafe { self.native().hasMipmaps() }
    }

    pub fn with_default_mipmaps(&self) -> Option<Image> {
        Image::from_ptr(unsafe { sb::C_SkImage_withDefaultMipmaps(self.native()) })
    }

    #[cfg(feature = "gpu")]
    pub fn new_texture_image(
        &self,
        context: &mut gpu::DirectContext,
        mipmapped: gpu::Mipmapped,
    ) -> Option<Image> {
        self.new_texture_image_budgeted(context, mipmapped, crate::Budgeted::Yes)
    }

    #[cfg(feature = "gpu")]
    pub fn new_texture_image_budgeted(
        &self,
        context: &mut gpu::DirectContext,
        mipmapped: gpu::Mipmapped,
        budgeted: crate::Budgeted,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_makeTextureImage(
                self.native(),
                context.native_mut(),
                mipmapped,
                budgeted.into_native(),
            )
        })
    }

    pub fn new_non_texture_image(&self) -> Option<Image> {
        Image::from_ptr(unsafe { sb::C_SkImage_makeNonTextureImage(self.native()) })
    }

    pub fn new_raster_image(&self) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_makeRasterImage(self.native(), CachingHint::Disallow)
        })
    }

    pub fn new_raster_image_with_caching_hint(&self, caching_hint: CachingHint) -> Option<Image> {
        Image::from_ptr(unsafe { sb::C_SkImage_makeRasterImage(self.native(), caching_hint) })
    }

    // TODO: rename to with_filter()?
    #[cfg(feature = "gpu")]
    pub fn new_with_filter(
        &self,
        mut context: Option<&mut gpu::RecordingContext>,
        filter: &ImageFilter,
        clip_bounds: impl Into<IRect>,
        subset: impl Into<IRect>,
    ) -> Option<(Image, IRect, IPoint)> {
        let mut out_subset = IRect::default();
        let mut offset = IPoint::default();

        Image::from_ptr(unsafe {
            sb::C_SkImage_makeWithFilter(
                self.native(),
                context.native_ptr_or_null_mut(),
                filter.native(),
                subset.into().native(),
                clip_bounds.into().native(),
                out_subset.native_mut(),
                offset.native_mut(),
            )
        })
        .map(|image| (image, out_subset, offset))
    }

    #[cfg(not(feature = "gpu"))]
    pub fn new_with_filter(
        &self,
        filter: &ImageFilter,
        clip_bounds: impl Into<IRect>,
        subset: impl Into<IRect>,
    ) -> Option<(Image, IRect, IPoint)> {
        let mut out_subset = IRect::default();
        let mut offset = IPoint::default();

        Image::from_ptr(unsafe {
            sb::C_SkImage_makeWithFilter(
                self.native(),
                std::ptr::null_mut(),
                filter.native(),
                subset.into().native(),
                clip_bounds.into().native(),
                out_subset.native_mut(),
                offset.native_mut(),
            )
        })
        .map(|image| (image, out_subset, offset))
    }

    // TODO: MakeBackendTextureFromSkImage()

    pub fn is_lazy_generated(&self) -> bool {
        unsafe { self.native().isLazyGenerated() }
    }

    pub fn new_color_space(&self, color_space: impl Into<Option<ColorSpace>>) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_makeColorSpace(
                self.native(),
                color_space.into().into_ptr_or_null(),
                ptr::null_mut(),
            )
        })
    }

    #[cfg(feature = "gpu")]
    pub fn new_color_space_with_context<'a>(
        &self,
        color_space: impl Into<Option<ColorSpace>>,
        direct: impl Into<Option<&'a mut gpu::DirectContext>>,
    ) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_makeColorSpace(
                self.native(),
                color_space.into().into_ptr_or_null(),
                direct.into().native_ptr_or_null_mut(),
            )
        })
    }

    pub fn reinterpret_color_space(&self, new_color_space: impl Into<ColorSpace>) -> Option<Image> {
        Image::from_ptr(unsafe {
            sb::C_SkImage_reinterpretColorSpace(self.native(), new_color_space.into().into_ptr())
        })
    }
}
