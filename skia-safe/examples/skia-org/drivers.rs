use skia_safe::Canvas;
use std::path::Path;

pub mod cpu;
pub use cpu::CPU;
pub mod opengl;
pub use opengl::OpenGL;
pub mod pdf;
pub use pdf::PDF;
#[cfg(feature = "svg")]
pub mod svg;
#[cfg(feature = "svg")]
pub use svg::SVG;
#[cfg(feature = "vulkan")]
pub mod vulkan;
#[cfg(feature = "vulkan")]
pub use vulkan::Vulkan;

pub trait DrawingDriver {
    const NAME: &'static str;

    fn draw_image(size: (i32, i32), path: &Path, name: &str, func: impl Fn(&mut Canvas));

    fn draw_image_256(path: &Path, name: &str, func: impl Fn(&mut Canvas)) {
        Self::draw_image((256, 256), path, name, func)
    }
}
