use skia_safe::Canvas;
use std::path::Path;

pub mod cpu;
pub use cpu::Cpu;
#[cfg(feature = "gl")]
pub mod gl;
#[cfg(feature = "gl")]
pub use gl::OpenGl;
#[cfg(feature = "metal")]
pub mod metal;
pub mod pdf;
pub use pdf::Pdf;
pub mod svg;
pub use svg::Svg;
#[cfg(feature = "vulkan")]
pub mod vulkan;
#[cfg(feature = "vulkan")]
pub use vulkan::Vulkan;
#[cfg(feature = "d3d")]
pub mod d3d;
#[cfg(feature = "d3d")]
pub use d3d::D3D;

pub trait DrawingDriver {
    const NAME: &'static str;

    fn new() -> Self;

    fn draw_image(&mut self, size: (i32, i32), path: &Path, name: &str, func: impl Fn(&mut Canvas));

    fn draw_image_256(&mut self, path: &Path, name: &str, func: impl Fn(&mut Canvas)) {
        self.draw_image((256, 256), path, name, func)
    }
}
