use crate::gpu::{BackendRenderTarget, BackendTexture, Context, SurfaceOrigin};
use crate::prelude::*;
use crate::{
    Bitmap, Budgeted, Canvas, ColorSpace, ColorType, DeferredDisplayList, IPoint, IRect, ISize,
    Image, ImageInfo, Paint, Pixmap, Size, SurfaceCharacterization, SurfaceProps,
};
use skia_bindings::{
    C_SkSurface_makeSurface, GrBackendRenderTarget, GrBackendTexture, SkRefCntBase, SkSurface,
    SkSurface_BackendHandleAccess, SkSurface_ContentChangeMode,
};
use std::ptr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ContentChangeMode {
    Discard = SkSurface_ContentChangeMode::kDiscard_ContentChangeMode as _,
    Retain = SkSurface_ContentChangeMode::kRetain_ContentChangeMode as _,
}

impl NativeTransmutable<SkSurface_ContentChangeMode> for ContentChangeMode {}
#[test]
fn test_surface_content_change_mode() {
    ContentChangeMode::test_layout()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum BackendHandleAccess {
    FlushRead = SkSurface_BackendHandleAccess::kFlushRead_BackendHandleAccess as _,
    FlushWrite = SkSurface_BackendHandleAccess::kFlushWrite_BackendHandleAccess as _,
    DiscardWrite = SkSurface_BackendHandleAccess::kDiscardWrite_BackendHandleAccess as _,
}

impl NativeTransmutable<SkSurface_BackendHandleAccess> for BackendHandleAccess {}
#[test]
fn test_surface_backend_handle_access_layout() {
    BackendHandleAccess::test_layout()
}

pub type Surface = RCHandle<SkSurface>;

impl NativeRefCountedBase for SkSurface {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl RCHandle<SkSurface> {
    pub fn new_raster_direct<'pixels>(
        image_info: &ImageInfo,
        pixels: &'pixels mut [u8],
        row_bytes: impl Into<Option<usize>>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Borrows<'pixels, Surface>> {
        let row_bytes = row_bytes
            .into()
            .unwrap_or_else(|| image_info.min_row_bytes());

        if pixels.len() < image_info.compute_byte_size(row_bytes) {
            return None;
        };

        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeRasterDirect(
                image_info.native(),
                pixels.as_mut_ptr() as _,
                row_bytes,
                surface_props.native_ptr_or_null(),
            )
        })
        .map(move |surface| surface.borrows(pixels))
    }

    // TODO: MakeRasterDirectReleaseProc()?

    pub fn new_raster(
        image_info: &ImageInfo,
        row_bytes: impl Into<Option<usize>>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Self> {
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeRaster(
                image_info.native(),
                row_bytes.into().unwrap_or_default(),
                surface_props.native_ptr_or_null(),
            )
        })
    }

    pub fn new_raster_n32_premul(size: impl Into<ISize>) -> Option<Self> {
        let size = size.into();
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeRasterN32Premul(size.width, size.height, ptr::null())
        })
    }

    pub fn from_backend_texture(
        context: &mut Context,
        backend_texture: &BackendTexture,
        origin: SurfaceOrigin,
        sample_count: impl Into<Option<usize>>,
        color_type: ColorType,
        color_space: Option<&ColorSpace>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Self> {
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeFromBackendTexture(
                context.native_mut(),
                backend_texture.native(),
                origin.into_native(),
                sample_count.into().unwrap_or(0).try_into().unwrap(),
                color_type.into_native(),
                color_space.shared_ptr(),
                surface_props.native_ptr_or_null(),
            )
        })
    }

    pub fn from_backend_render_target(
        context: &mut Context,
        backend_render_target: &BackendRenderTarget,
        origin: SurfaceOrigin,
        color_type: ColorType,
        color_space: Option<&ColorSpace>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Self> {
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeFromBackendRenderTarget(
                context.native_mut(),
                backend_render_target.native(),
                origin.into_native(),
                color_type.into_native(),
                color_space.shared_ptr(),
                surface_props.native_ptr_or_null(),
            )
        })
    }

    pub fn from_backend_texture_as_render_target(
        context: &mut Context,
        backend_texture: &BackendTexture,
        origin: SurfaceOrigin,
        sample_count: impl Into<Option<usize>>,
        color_type: ColorType,
        color_space: Option<&ColorSpace>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Self> {
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeFromBackendTextureAsRenderTarget(
                context.native_mut(),
                backend_texture.native(),
                origin.into_native(),
                sample_count.into().unwrap_or(0).try_into().unwrap(),
                color_type.into_native(),
                color_space.shared_ptr(),
                surface_props.native_ptr_or_null(),
            )
        })
    }

    pub fn new_render_target(
        context: &mut Context,
        budgeted: Budgeted,
        image_info: &ImageInfo,
        sample_count: impl Into<Option<usize>>,
        // not optional, because with vulkan, there is no clear default anymore.
        surface_origin: SurfaceOrigin,
        surface_props: Option<&SurfaceProps>,
        should_create_with_mips: impl Into<Option<bool>>,
    ) -> Option<Self> {
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeRenderTarget(
                context.native_mut(),
                budgeted.into_native(),
                image_info.native(),
                sample_count.into().unwrap_or(0).try_into().unwrap(),
                surface_origin.into_native(),
                surface_props.native_ptr_or_null(),
                should_create_with_mips.into().unwrap_or_default(),
            )
        })
    }

    pub fn new_render_target_with_characterization(
        context: &mut Context,
        characterization: &SurfaceCharacterization,
        budgeted: Budgeted,
    ) -> Option<Self> {
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeRenderTarget2(
                context.native_mut(),
                characterization.native(),
                budgeted.into_native(),
            )
        })
    }

    pub fn new_null(size: impl Into<ISize>) -> Option<Self> {
        let size = size.into();
        Self::from_ptr(unsafe { skia_bindings::C_SkSurface_MakeNull(size.width, size.height) })
    }

    pub fn width(&self) -> i32 {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { self.native().height() }
    }

    pub fn generation_id(&mut self) -> u32 {
        unsafe { self.native_mut().generationID() }
    }

    pub fn notify_content_will_change(&mut self, mode: ContentChangeMode) -> &mut Self {
        unsafe {
            self.native_mut()
                .notifyContentWillChange(mode.into_native())
        }
        self
    }

    #[deprecated(note = "use get_backend_texture()")]
    pub fn backend_texture(
        &mut self,
        handle_access: BackendHandleAccess,
    ) -> Option<BackendTexture> {
        self.get_backend_texture(handle_access)
    }

    pub fn get_backend_texture(
        &mut self,
        handle_access: BackendHandleAccess,
    ) -> Option<BackendTexture> {
        unsafe {
            let mut backend_texture = GrBackendTexture::new();
            skia_bindings::C_SkSurface_getBackendTexture(
                self.native_mut(),
                handle_access.into_native(),
                &mut backend_texture as _,
            );

            BackendTexture::from_native_if_valid(backend_texture)
        }
    }

    #[deprecated(note = "use get_backend_render_target()")]
    pub fn backend_render_target(
        &mut self,
        handle_access: BackendHandleAccess,
    ) -> Option<BackendRenderTarget> {
        self.get_backend_render_target(handle_access)
    }

    pub fn get_backend_render_target(
        &mut self,
        handle_access: BackendHandleAccess,
    ) -> Option<BackendRenderTarget> {
        unsafe {
            let mut backend_render_target = GrBackendRenderTarget::new();
            skia_bindings::C_SkSurface_getBackendRenderTarget(
                self.native_mut(),
                handle_access.into_native(),
                &mut backend_render_target as _,
            );

            BackendRenderTarget::from_native_if_valid(backend_render_target)
        }
    }

    // TODO: support variant with TextureReleaseProc and ReleaseContext
    pub fn replace_backend_texture(
        &mut self,
        backend_texture: &BackendTexture,
        origin: SurfaceOrigin,
    ) -> bool {
        unsafe {
            self.native_mut().replaceBackendTexture(
                backend_texture.native(),
                origin.into_native(),
                None,
                ptr::null_mut(),
            )
        }
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        let canvas_ref = unsafe { &mut *self.native_mut().getCanvas() };
        Canvas::borrow_from_native(canvas_ref)
    }

    #[deprecated(note = "use Surface::new_surface")]
    pub fn new_compatible(&mut self, info: &ImageInfo) -> Option<Surface> {
        self.new_surface(info)
    }

    // TODO: why is self mutable here?
    pub fn new_surface(&mut self, info: &ImageInfo) -> Option<Surface> {
        Surface::from_ptr(unsafe { C_SkSurface_makeSurface(self.native_mut(), info.native()) })
    }

    pub fn image_snapshot(&mut self) -> Image {
        Image::from_ptr(unsafe {
            skia_bindings::C_SkSurface_makeImageSnapshot(self.native_mut(), ptr::null())
        })
        .unwrap()
    }

    // TODO: combine this function with image_snapshot and make bounds optional()?
    pub fn image_snapshot_with_bounds(&mut self, bounds: impl AsRef<IRect>) -> Option<Image> {
        Image::from_ptr(unsafe {
            skia_bindings::C_SkSurface_makeImageSnapshot(
                self.native_mut(),
                bounds.as_ref().native(),
            )
        })
    }

    // TODO: why is self mutable here?
    #[deprecated(note = "use Surface::draw()")]
    pub fn draw_to_canvas(
        &mut self,
        canvas: impl AsMut<Canvas>,
        size: impl Into<Size>,
        paint: Option<&Paint>,
    ) {
        self.draw(canvas, size, paint)
    }

    pub fn draw(
        &mut self,
        mut canvas: impl AsMut<Canvas>,
        size: impl Into<Size>,
        paint: Option<&Paint>,
    ) {
        let size = size.into();
        unsafe {
            self.native_mut().draw(
                canvas.as_mut().native_mut(),
                size.width,
                size.height,
                paint.native_ptr_or_null(),
            )
        }
    }

    pub fn peek_pixels(&mut self) -> Option<Borrows<Pixmap>> {
        let mut pm = Pixmap::default();
        unsafe { self.native_mut().peekPixels(pm.native_mut()) }
            .if_true_then_some(move || pm.borrows(self))
    }

    // TODO: why is self mut?
    pub fn read_pixels_to_pixmap(&mut self, dst: &Pixmap, src: impl Into<IPoint>) -> bool {
        let src = src.into();
        unsafe { self.native_mut().readPixels(dst.native(), src.x, src.y) }
    }

    pub fn read_pixels(
        &mut self,
        dst_info: &ImageInfo,
        dst_pixels: &mut [u8],
        dst_row_bytes: usize,
        src: impl Into<IPoint>,
    ) -> bool {
        if dst_row_bytes < dst_info.min_row_bytes() {
            return false;
        };
        let height: usize = dst_info.height().try_into().unwrap();
        if dst_pixels.len() < dst_row_bytes * height {
            return false;
        };
        let src = src.into();
        unsafe {
            self.native_mut().readPixels1(
                dst_info.native(),
                dst_pixels.as_mut_ptr() as _,
                dst_row_bytes,
                src.x,
                src.y,
            )
        }
    }

    // TODO: why is self mut?
    // TODO: why is Bitmap non-mutable.
    pub fn read_pixels_to_bitmap(&mut self, bitmap: &Bitmap, src: impl Into<IPoint>) -> bool {
        let src = src.into();
        unsafe { self.native_mut().readPixels2(bitmap.native(), src.x, src.y) }
    }

    // TODO: wrap asyncRescaleAndReadPixels (m76)

    pub fn write_pixels_from_pixmap(&mut self, src: &Pixmap, dst: impl Into<IPoint>) {
        let dst = dst.into();
        unsafe { self.native_mut().writePixels(src.native(), dst.x, dst.y) }
    }

    pub fn write_pixels_from_bitmap(&mut self, bitmap: &Bitmap, dst: impl Into<IPoint>) {
        let dst = dst.into();
        unsafe {
            self.native_mut()
                .writePixels1(bitmap.native(), dst.x, dst.y)
        }
    }

    pub fn props(&self) -> &SurfaceProps {
        SurfaceProps::from_native_ref(unsafe { &*self.native().props() })
    }

    pub fn flush(&mut self) {
        unsafe {
            self.native_mut().flush();
        }
    }

    // TODO: flush(access, FlushInfo)
    // TODO: flush(access, FlshFlags, semaphores)
    // TODO: wait()

    pub fn characterize(&self) -> Option<SurfaceCharacterization> {
        let mut sc = SurfaceCharacterization::default();
        unsafe { self.native().characterize(sc.native_mut()) }.if_true_some(sc)
    }

    pub fn draw_display_list(&mut self, deferred_display_list: &mut DeferredDisplayList) -> bool {
        unsafe { self.native_mut().draw1(deferred_display_list.native_mut()) }
    }
}

#[test]
fn create() {
    assert!(Surface::new_raster_n32_premul((0, 0)).is_none());
    let surface = Surface::new_raster_n32_premul((1, 1)).unwrap();
    assert_eq!(1, surface.native().ref_cnt())
}

#[test]
fn test_raster_direct() {
    let image_info = ImageInfo::new(
        (20, 20),
        ColorType::RGBA8888,
        crate::AlphaType::Unpremul,
        None,
    );
    let min_row_bytes = image_info.min_row_bytes();
    let mut pixels = vec![0u8; image_info.compute_byte_size(min_row_bytes)];
    let mut surface = Surface::new_raster_direct(
        &image_info,
        pixels.as_mut_slice(),
        Some(min_row_bytes),
        None,
    )
    .unwrap();
    let paint = Paint::default();
    surface.canvas().draw_circle((10, 10), 10.0, &paint);
}
