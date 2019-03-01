use std::ptr;
use super::{Image, Canvas};
use crate::graphics::{Context, BackendTexture};
use crate::prelude::*;
use rust_skia::{
    SkColorType,
    SkSurface,
    GrSurfaceOrigin,
    SkSurface_BackendHandleAccess,
    GrBackendTexture
};

pub type Surface = RCHandle<SkSurface>;

impl NativeRefCounted for SkSurface {
    fn _ref(&self) {
        unsafe { self._base._base.ref_() }
    }

    fn _unref(&self) {
        unsafe { self._base._base.unref() }
    }
}

impl RCHandle<SkSurface> {

    pub fn new_raster_n32_premul(width: i32, height: i32) -> Option<Self> {
        Self::from_ptr(unsafe {
            rust_skia::C_SkSurface_MakeRasterN32Premul(width, height, ptr::null())
        })
    }

    pub fn new_from_backend_texture(
        context: &mut Context,
        backend_texture: &BackendTexture,
        origin: GrSurfaceOrigin,
        sample_count: usize,
        color_type: SkColorType) -> Option<Self> {
        Self::from_ptr(unsafe {
            rust_skia::C_SkSurface_MakeFromBackendTexture(
                context.native_mut(),
                backend_texture.native(),
                origin,
                sample_count.try_into().unwrap(),
                color_type)
        })
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        Canvas::borrow_from_surface(self)
    }

    pub fn make_image_snapshot(&mut self) -> Image {
        Image::from_ptr(unsafe {
            rust_skia::C_SkSurface_makeImageSnapshot(self.native_mut())
        }).unwrap()
    }

    pub fn flush(&mut self) {
        unsafe { self.native_mut().flush(); }
    }

    pub fn get_backend_texture(&mut self, handle_access: SkSurface_BackendHandleAccess) -> Option<BackendTexture> {
        unsafe {
            let mut backend_texture = GrBackendTexture::new();
            rust_skia::C_SkSurface_getBackendTexture(
                self.native_mut(),
                handle_access,
                &mut backend_texture as _);

            BackendTexture::from_raw(backend_texture)
        }
    }
}

#[test]
fn create() {
    assert!(Surface::new_raster_n32_premul(0, 0).is_none());
    let surface = Surface::new_raster_n32_premul(1, 1).unwrap();
    assert_eq!(1, surface.native().ref_cnt())
}