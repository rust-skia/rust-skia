use std::ptr;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use crate::prelude::*;
use crate::gpu::{Context, BackendTexture, BackendRenderTarget, SurfaceOrigin};
use crate::core::{ImageInfo, SurfaceProps, Paint, ColorSpace, Budgeted, IRect, Size, IPoint, Bitmap, Image, Canvas, ISize, ColorType };
use skia_bindings::{SkSurface, SkSurface_BackendHandleAccess, GrBackendTexture, SkRefCntBase, SkSurface_ContentChangeMode, GrBackendRenderTarget, C_SkSurface_makeSurface};
#[cfg(test)]
use crate::core::AlphaType;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum SurfaceContentChangeMode {
    Discard = SkSurface_ContentChangeMode::kDiscard_ContentChangeMode as _,
    Retain = SkSurface_ContentChangeMode::kRetain_ContentChangeMode as _
}

impl NativeTransmutable<SkSurface_ContentChangeMode> for SurfaceContentChangeMode {}
#[test] fn test_surface_content_change_mode() { SurfaceContentChangeMode::test_layout() }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum SurfaceBackendHandleAccess {
    FlushRead = SkSurface_BackendHandleAccess::kFlushRead_BackendHandleAccess as _,
    FlushWrite = SkSurface_BackendHandleAccess::kFlushWrite_BackendHandleAccess as _,
    DiscardWrite = SkSurface_BackendHandleAccess::kDiscardWrite_BackendHandleAccess as _
}

impl NativeTransmutable<SkSurface_BackendHandleAccess> for SurfaceBackendHandleAccess {}
#[test] fn test_surface_backend_handle_access() { SurfaceBackendHandleAccess::test_layout() }

// TODO: complete the implementation.
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
        row_bytes: Option<usize>,
        surface_props: Option<&SurfaceProps>
    ) -> Option<OwnedSurface<'pixels>> {

        let row_bytes = row_bytes.unwrap_or(image_info.min_row_bytes());

        if pixels.len() < image_info.compute_byte_size(row_bytes) {
            return None
        };

        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeRasterDirect(
                image_info.native(),
                pixels.as_mut_ptr() as _,
                row_bytes,
                surface_props.native_ptr_or_null())
        }).map(|surface| surface.into())
    }

    pub fn new_raster(
        image_info: &ImageInfo,
        row_bytes: Option<usize>,
        surface_props: Option<&SurfaceProps>
    ) -> Option<Self> {
        let row_bytes = row_bytes.unwrap_or_default();
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeRaster(
                image_info.native(),
                row_bytes,
                surface_props.native_ptr_or_null())
        })
    }

    pub fn new_raster_n32_premul<IS: Into<ISize>>(size: IS) -> Option<Self> {
        let size = size.into();
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeRasterN32Premul(size.width, size.height, ptr::null())
        })
    }

    pub fn from_backend_texture<SC: Into<Option<usize>>>(
        context: &mut Context,
        backend_texture: &BackendTexture,
        origin: SurfaceOrigin,
        sample_count: SC,
        color_type: ColorType,
        color_space: Option<&ColorSpace>,
        surface_props: Option<&SurfaceProps>
    ) -> Option<Self> {
        let sample_count = sample_count.into().unwrap_or(0);
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeFromBackendTexture(
                context.native_mut(),
                backend_texture.native(),
                origin.into_native(),
                sample_count.try_into().unwrap(),
                color_type.into_native(),
                color_space.shared_ptr(),
                surface_props.native_ptr_or_null())
        })
    }

    pub fn from_backend_render_target(
        context: &mut Context,
        backend_render_target: &BackendRenderTarget,
        origin: SurfaceOrigin,
        color_type: ColorType,
        color_space: Option<&ColorSpace>,
        surface_props: Option<&SurfaceProps>
    ) -> Option<Self> {
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeFromBackendRenderTarget(
                context.native_mut(),
                backend_render_target.native(),
                origin.into_native(),
                color_type.into_native(),
                color_space.shared_ptr(),
                surface_props.native_ptr_or_null()
            )
        })
    }

    pub fn from_backend_texture_as_render_target<SC: Into<Option<usize>>>(
        context: &mut Context,
        backend_texture: &BackendTexture,
        origin: SurfaceOrigin,
        sample_count: SC,
        color_type: ColorType,
        color_space: Option<&ColorSpace>,
        surface_props: Option<&SurfaceProps>
    ) -> Option<Self> {
        let sample_count = sample_count.into().unwrap_or(0);
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeFromBackendTextureAsRenderTarget(
                context.native_mut(),
                backend_texture.native(),
                origin.into_native(),
                sample_count.try_into().unwrap(),
                color_type.into_native(),
                color_space.shared_ptr(),
                surface_props.native_ptr_or_null())
        })
    }

    pub fn new_render_target<SC: Into<Option<usize>>>(
        context: &mut Context,
        budgeted: Budgeted,
        image_info: &ImageInfo,
        sample_count: SC,
        surface_origin: SurfaceOrigin,
        surface_props: Option<&SurfaceProps>,
        should_create_with_mips: bool
    ) -> Option<Self> {
        let sample_count = sample_count.into().unwrap_or(0);
        Self::from_ptr(unsafe {
            skia_bindings::C_SkSurface_MakeRenderTarget(
                context.native_mut(),
                budgeted.into_native(),
                image_info.native(),
                sample_count.try_into().unwrap(),
                surface_origin.into_native(),
                surface_props.native_ptr_or_null(),
                should_create_with_mips
            )
        })
    }

    pub fn width(&self) -> i32 {
        unsafe {
            self.native().width()
        }
    }

    pub fn height(&self) -> i32 {
        unsafe {
            self.native().height()
        }
    }

    pub fn generation_id(&mut self) -> u32 {
        unsafe {
            self.native_mut().generationID()
        }
    }

    pub fn notify_content_will_change(&mut self, mode: SurfaceContentChangeMode) -> &mut Self {
        unsafe {
            self.native_mut().notifyContentWillChange(mode.into_native())
        }
        self
    }

    pub fn backend_texture(&mut self, handle_access: SurfaceBackendHandleAccess) -> Option<BackendTexture> {
        unsafe {
            let mut backend_texture = GrBackendTexture::new();
            skia_bindings::C_SkSurface_getBackendTexture(
                self.native_mut(),
                handle_access.into_native(),
                &mut backend_texture as _);

            BackendTexture::from_native_if_valid(backend_texture)
        }
    }

    pub fn backend_render_target(&mut self, handle_access: SurfaceBackendHandleAccess) -> Option<BackendRenderTarget> {
        unsafe {
            let mut backend_render_target = GrBackendRenderTarget::new();
            skia_bindings::C_SkSurface_getBackendRenderTarget(
                self.native_mut(),
                handle_access.into_native(),
                &mut backend_render_target as _);

            BackendRenderTarget::from_native_if_valid(backend_render_target)
        }
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        let canvas_ref = unsafe { &mut *self.native_mut().getCanvas() };
        Canvas::borrow_from_native(canvas_ref)
    }

    // TODO: why is self mutable here?
    pub fn new_compatible(&mut self, info: &ImageInfo) -> Option<Surface> {
        Surface::from_ptr(unsafe {
            C_SkSurface_makeSurface(self.native_mut(), info.native())
        })
    }

    pub fn image_snapshot(&mut self) -> Image {
        Image::from_ptr(unsafe {
            skia_bindings::C_SkSurface_makeImageSnapshot(
                self.native_mut(), ptr::null())
        }).unwrap()
    }

    pub fn image_snapshot_with_bounds(&mut self, bounds: IRect) -> Option<Image> {
        Image::from_ptr(unsafe {
            skia_bindings::C_SkSurface_makeImageSnapshot(
                self.native_mut(), bounds.native())
        })
    }

    // TODO: why is self mutable here?
    pub fn draw_to_canvas<S: Into<Size>>(&mut self, mut canvas: impl AsMut<Canvas>, size: S, paint: Option<&Paint>) {
        let size = size.into();
        unsafe {
            self.native_mut().draw(
                canvas.as_mut().native_mut(),
                size.width,
                size.height,
                paint.native_ptr_or_null())
        }
    }

    // TODO: support Pixmap peekPixels
    // TODO: support Pixmap readPixels

    pub fn read_pixels<IP: Into<IPoint>>(
        &mut self,
        dst_info: &ImageInfo,
        dst_pixels: &mut [u8], dst_row_bytes: usize, src: IP) -> bool {
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
                src.x, src.y)
        }
    }

    // TODO: why is self mutable here?
    // TODO: why is Bitmap non-mutable here.
    pub fn read_pixels_to_bitmap<IP: Into<IPoint>>(
        &mut self,
        bitmap: &Bitmap,
        src: IP
    ) -> bool {
        let src = src.into();
        unsafe {
            self.native_mut().readPixels2(bitmap.native(), src.x, src.y)
        }
    }

    pub fn write_pixels_from_bitmap<IP: Into<IPoint>>(&mut self, bitmap: &Bitmap, dst: IP) {
        let dst = dst.into();
        unsafe {
            self.native_mut().writePixels1(bitmap.native(), dst.x, dst.y)
        }
    }

    pub fn props(&self) -> &SurfaceProps {
        SurfaceProps::from_native_ref(unsafe {
            &*self.native().props()
        })
    }

    pub fn flush(&mut self) {
        unsafe { self.native_mut().flush(); }
    }

    // TODO: flushAndSignalSemaphores()
    // TODO: wait()
    // TODO: characterize()
    // TODO: draw()

}

// A lifetime bound surface.
pub struct OwnedSurface<'a> {
    surface: Surface,
    phantom: PhantomData<& 'a()>
}

impl<'a> Deref for OwnedSurface<'a> {
    type Target = Surface;
    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}

impl<'a> DerefMut for OwnedSurface<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.surface
    }
}

impl<'a> From<Surface> for OwnedSurface<'a> {
    fn from(surface: RCHandle<SkSurface>) -> Self {
        OwnedSurface {
            surface,
            phantom: PhantomData
        }
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
    let image_info = ImageInfo::new((20, 20), ColorType::RGBA8888, AlphaType::Unpremul, None);
    let min_row_bytes = image_info.min_row_bytes();
    let mut pixels = vec![0u8; image_info.compute_byte_size(min_row_bytes)];
    let mut surface = Surface::new_raster_direct(&image_info, pixels.as_mut_slice(), Some(min_row_bytes), None).unwrap();
    let paint = Paint::default();
    surface.canvas().draw_circle((10, 10), 10.0, &paint);
}