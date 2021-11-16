use skia_safe::Canvas;
use std::path::Path;

pub mod cpu;
#[cfg(feature = "d3d")]
pub mod d3d;
#[cfg(feature = "gl")]
pub mod gl;
#[cfg(feature = "metal")]
pub mod metal;
pub mod pdf;
pub mod svg;
#[cfg(feature = "svg")]
pub mod render_svg;
#[cfg(feature = "vulkan")]
pub mod vulkan;

pub trait DrawingDriver {
    const NAME: &'static str;

    fn new() -> Self;

    fn draw_image(&mut self, size: (i32, i32), path: &Path, name: &str, func: impl Fn(&mut Canvas));

    fn draw_image_256(&mut self, path: &Path, name: &str, func: impl Fn(&mut Canvas)) {
        self.draw_image((256, 256), path, name, func)
    }
}
