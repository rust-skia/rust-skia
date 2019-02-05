use std::ptr;
use std::marker::PhantomData;
use rust_skia::{SkSurface, C_SkSurface_MakeRasterN32Premul, C_SkSurface_makeImageSnapshot};
use super::image::Image;
use super::canvas::Canvas;

pub struct Surface {
    native: *mut SkSurface
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { (*self.native)._base._base.unref() }
    }
}

impl Surface {

    // tbd: use Option as a return type.
    pub fn new_raster_n32_premul(width: i32, height: i32) -> Surface {
        Surface {
            native: unsafe { C_SkSurface_MakeRasterN32Premul(width, height, ptr::null()) }
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
