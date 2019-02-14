use std::ptr;
use std::marker::PhantomData;
use rust_skia::{
    SkSurface, SkColorType, GrSurfaceOrigin,
    C_SkSurface_MakeRasterN32Premul,
    C_SkSurface_MakeRenderTarget,
    C_SkSurface_MakeFromBackendTexture,
    C_SkSurface_makeImageSnapshot,
    SkSurface_BackendHandleAccess,
    C_SkSurface_getBackendTexture,
    GrBackendTexture
};
use super::image::Image;
use super::canvas::Canvas;
use crate::graphics::{Context, BackendTexture};
use crate::prelude::*;

#[derive(Clone)]
pub struct Surface {
    inner: RefCounted<Inner>
}

impl Native<*mut SkSurface> for Surface {
    fn native(&self) -> *mut SkSurface {
        self.inner.0
    }
}

#[derive(Clone)]
struct Inner(*mut SkSurface);

impl RefCount for Inner {
    fn refer(&self) {
        unsafe { (*self.0)._base._base.unref() }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe { (*self.0)._base._base.unref() }
    }
}

impl Surface {

    pub fn new_raster_n32_premul(width: u32, height: u32) -> Option<Surface> {
        unsafe { C_SkSurface_MakeRasterN32Premul(width as i32, height as i32, ptr::null()) }
            .to_option()
            .map(|native| Surface { inner: Inner(native).into() })
    }

    pub fn new_from_backend_texture(
        context: Context,
        backend_texture: &BackendTexture,
        origin: GrSurfaceOrigin,
        sample_count: u32,
        color_type: SkColorType) -> Option<Surface> {
        unsafe { C_SkSurface_MakeFromBackendTexture(context.native(), &backend_texture.native, origin, sample_count as i32, color_type) }
            .to_option()
            .map(|native| Surface { inner: Inner(native).into() })
    }

    pub fn canvas(&self) -> Canvas {
        Canvas {
            native: unsafe { (*self.inner.0).getCanvas() },
            phantom: PhantomData
        }
    }

    pub fn make_image_snapshot(&mut self) -> Image {
        Image { native: unsafe { C_SkSurface_makeImageSnapshot(self.native()) } }
    }

    pub fn flush(&mut self) {
        unsafe {
            (*self.native()).flush();
        }
    }

    pub fn get_backend_texture(&mut self, handle_access: SkSurface_BackendHandleAccess) -> Option<BackendTexture> {
        unsafe {
            let mut backend_texture = GrBackendTexture::new();
            C_SkSurface_getBackendTexture(
                self.native(),
                handle_access,
                &mut backend_texture as _);

            BackendTexture::from_raw(backend_texture)
        }
    }
}
