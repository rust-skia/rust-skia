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

impl RefCounted for SkSurface {
    fn _ref(&self) {
        unsafe { self._base._base.ref_() }
    }

    fn _unref(&self) {
        unsafe { self._base._base.unref() }
    }
}

impl Surface {

    pub fn new_raster_n32_premul(width: u32, height: u32) -> Option<Surface> {
        Surface::from_ptr(unsafe {
            rust_skia::C_SkSurface_MakeRasterN32Premul(width as i32, height as i32, ptr::null())
        })
    }

    pub fn new_from_backend_texture(
        context: &mut Context,
        backend_texture: &BackendTexture,
        origin: GrSurfaceOrigin,
        sample_count: u32,
        color_type: SkColorType) -> Option<Surface> {
        Surface::from_ptr(unsafe {
            rust_skia::C_SkSurface_MakeFromBackendTexture(context.native_mut(), &backend_texture.0, origin, sample_count as i32, color_type)
        })
    }

    pub fn canvas(&mut self) -> Canvas {
        Canvas {
            native: unsafe { self.native_mut().getCanvas() },
            owner: Some((*self).clone())
        }
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
    unsafe { assert_eq!(1, surface.native().ref_cnt()) }
}