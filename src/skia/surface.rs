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

#[derive(RCCloneDrop)]
pub struct Surface(pub(crate) *mut SkSurface);

impl RefCounted for Surface {
    fn _ref(&self) {
        unsafe { (*self.0)._base._base.ref_() }
    }

    fn _unref(&self) {
        unsafe { (*self.0)._base._base.unref() }
    }
}

impl Surface {

    pub fn new_raster_n32_premul(width: u32, height: u32) -> Option<Surface> {
        unsafe { rust_skia::C_SkSurface_MakeRasterN32Premul(width as i32, height as i32, ptr::null()) }
            .to_option()
            .map(Surface)
    }

    pub fn new_from_backend_texture(
        context: Context,
        backend_texture: &BackendTexture,
        origin: GrSurfaceOrigin,
        sample_count: u32,
        color_type: SkColorType) -> Option<Surface> {
        unsafe { rust_skia::C_SkSurface_MakeFromBackendTexture(context.0, &backend_texture.0, origin, sample_count as i32, color_type) }
            .to_option()
            .map(Surface)
    }

    pub fn canvas(&self) -> Canvas {
        Canvas {
            native: unsafe { (*self.0).getCanvas() },
            owner: Some((*self).clone())
        }
    }

    pub fn make_image_snapshot(&mut self) -> Image {
        Image(unsafe { rust_skia::C_SkSurface_makeImageSnapshot(self.0) })
    }

    pub fn flush(&mut self) {
        unsafe { (*self.0).flush(); }
    }

    pub fn get_backend_texture(&mut self, handle_access: SkSurface_BackendHandleAccess) -> Option<BackendTexture> {
        unsafe {
            let mut backend_texture = GrBackendTexture::new();
            rust_skia::C_SkSurface_getBackendTexture(
                self.0,
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
    unsafe { assert_eq!(1, (*surface.0).ref_cnt()) }
}