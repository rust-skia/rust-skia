use std::ptr;
use std::marker::PhantomData;
use rust_skia::{
    SkSurface, SkColorType, GrSurfaceOrigin,
    C_SkSurface_MakeRasterN32Premul,
    C_SkSurface_MakeRenderTarget,
    C_SkSurface_MakeFromBackendTexture,
    C_SkSurface_makeImageSnapshot,
};
use super::image::Image;
use super::canvas::Canvas;
use crate::graphics::{Context, BackendTexture};
use crate::prelude::*;

#[derive(Debug)]
pub struct Surface {
    native: *mut SkSurface
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
        context: &mut Context,
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
            phantom: PhantomData
        }
    }

    pub fn make_image_snapshot(&mut self) -> Image {
        Image { native: unsafe { C_SkSurface_makeImageSnapshot(self.native) } }
    }
}
