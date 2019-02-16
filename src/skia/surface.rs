use std::ptr;
use rust_skia::{
    SkSurface, SkColorType, GrSurfaceOrigin,
    C_SkSurface_MakeRasterN32Premul,
    // C_SkSurface_MakeRenderTarget,
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

pub struct Surface {
    pub(crate) native: *mut SkSurface
}

impl Clone for Surface {
    fn clone(&self) -> Self {
        unsafe { (*self.native)._base._base.ref_() }
        Surface { native: self.native }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { (*self.native)._base._base.unref() }
    }
}

impl Surface {

    pub fn new_raster_n32_premul(width: u32, height: u32) -> Option<Surface> {
        unsafe { C_SkSurface_MakeRasterN32Premul(width as i32, height as i32, ptr::null()) }
            .to_option()
            .map(|native| Surface { native })
    }

    pub fn new_from_backend_texture(
        context: Context,
        backend_texture: &BackendTexture,
        origin: GrSurfaceOrigin,
        sample_count: u32,
        color_type: SkColorType) -> Option<Surface> {
        unsafe { C_SkSurface_MakeFromBackendTexture(context.native, &backend_texture.native, origin, sample_count as i32, color_type) }
            .to_option()
            .map(|native| Surface { native })
    }

    pub fn canvas(&self) -> Canvas {
        Canvas {
            native: unsafe { (*self.native).getCanvas() },
            owner: Some(self.clone())
        }
    }

    pub fn make_image_snapshot(&mut self) -> Image {
        Image { native: unsafe { C_SkSurface_makeImageSnapshot(self.native) } }
    }

    pub fn flush(&mut self) {
        unsafe {
            (*self.native).flush();
        }
    }

    pub fn get_backend_texture(&mut self, handle_access: SkSurface_BackendHandleAccess) -> Option<BackendTexture> {
        unsafe {
            let mut backend_texture = GrBackendTexture::new();
            C_SkSurface_getBackendTexture(
                self.native,
                handle_access,
                &mut backend_texture as _);

            BackendTexture::from_raw(backend_texture)
        }
    }
}
