use clap::ArgEnum;
use skia_safe::Canvas;
use std::{fmt::Display, path::Path, str::FromStr};

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

pub use cpu::Cpu;
#[cfg(feature = "d3d")]
pub use d3d::D3D;
#[cfg(feature = "gl")]
pub use gl::OpenGl;
#[cfg(feature = "metal")]
pub use metal::Metal;
pub use pdf::Pdf;
pub use svg::Svg;
#[cfg(feature = "svg")]
pub use svg_render::SvgRender;
#[cfg(feature = "vulkan")]
pub use vulkan::Vulkan;

pub trait DrawingDriver {
    const DRIVER: Driver;

    fn new() -> Self;

    fn draw_image(&mut self, size: (i32, i32), path: &Path, name: &str, func: impl Fn(&mut Canvas));

    fn draw_image_256(&mut self, path: &Path, name: &str, func: impl Fn(&mut Canvas)) {
        self.draw_image((256, 256), path, name, func)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, ArgEnum)]
pub enum Driver {
    Cpu,
    Pdf,
    Svg,
    #[cfg(feature = "gl")]
    OpenGl,
    #[cfg(feature = "gl")]
    OpenGlEs,
    #[cfg(feature = "vulkan")]
    Vulkan,
    #[cfg(feature = "metal")]
    Metal,
    #[cfg(feature = "d3d")]
    D3d,
}

impl FromStr for Driver {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Driver::*;
        Ok(match s {
            "cpu" => Cpu,
            "pdf" => Pdf,
            "svg" => Svg,
            #[cfg(feature = "gl")]
            "opengl" => OpenGl,
            #[cfg(feature = "gl")]
            "opengl-es" => OpenGlEs,
            #[cfg(feature = "vulkan")]
            "vulkan" => Vulkan,
            #[cfg(feature = "metal")]
            "metal" => Metal,
            #[cfg(feature = "d3d")]
            "d3d" => D3d,
            _ => return Err("Unknown driver"),
        })
    }
}

impl Display for Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Driver::*;
        let name = match self {
            Cpu => "cpu",
            Pdf => "pdf",
            Svg => "svg",
            #[cfg(feature = "gl")]
            OpenGl => "opengl",
            #[cfg(feature = "gl")]
            OpenGlEs => "opengl-es",
            #[cfg(feature = "vulkan")]
            Vulkan => "vulkan",
            #[cfg(feature = "metal")]
            Metal => "metal",
            #[cfg(feature = "d3d")]
            D3d => "d3d",
        };
        f.write_str(name)
    }
}
