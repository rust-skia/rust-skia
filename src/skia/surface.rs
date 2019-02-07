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

pub struct Surface {
    native: *mut SkSurface
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { (*self.native)._base._base.unref() }
    }
}

impl Surface {

    pub fn new_raster_n32_premul(width: i32, height: i32) -> Option<Surface> {
        let native = unsafe { C_SkSurface_MakeRasterN32Premul(width, height, ptr::null()) };
        if native.is_null()
            { None }
        else
            { Some (Surface { native }) }
    }

    pub fn new_from_backend_texture(
        context: &mut Context,
        backend_texture: &BackendTexture,
        origin: GrSurfaceOrigin,
        sample_count: i32,
        color_type: SkColorType) -> Surface {
        Surface {
            native:
                unsafe { C_SkSurface_MakeFromBackendTexture(context.native, &backend_texture.native, origin, sample_count, color_type) }
        }
    }

    pub fn canvas(&self) -> Canvas
    {
        Canvas {
            native: unsafe { (*self.native).getCanvas() },
            phantom: PhantomData
        }
    }

    pub fn make_image_snapshot(&mut self) -> Image {
        Image { native: unsafe { C_SkSurface_makeImageSnapshot(self.native) } }
    }
}
