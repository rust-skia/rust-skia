use crate::drivers::DrawingDriver;
use clap::Parser;
use std::path::{Path, PathBuf};

#[cfg(feature = "gl")]
use offscreen_gl_context::{GLContext, GLVersion, NativeGLContext};

#[cfg(feature = "vulkan")]
extern crate ash;

#[cfg(feature = "metal")]
#[macro_use]
extern crate objc;

// TODO: think about making the examples more Rust-idiomatic, by using method chaining for Paint / Paths, for example.

mod artifact;
mod drivers;
mod skcanvas_overview;
mod skpaint_overview;
#[cfg(feature = "textlayout")]
mod skparagraph_example;
mod skpath_overview;
#[cfg(feature = "textlayout")]
mod skshaper_example;

pub use drivers::Driver;

#[derive(Parser)]
#[clap(about)]
struct Arguments {
    #[clap(default_value = ".", help = "The output path to render into.")]
    out_path: PathBuf,
    #[clap(
        long,
        arg_enum,
        help = "In addition with the CPU, render with the given driver."
    )]
    driver: Vec<Driver>,
}

fn main() {
    let args = Arguments::parse();

    let out_path = args.out_path;
    let drivers = args.driver;
    let drivers = if drivers.is_empty() {
        vec![Driver::Cpu]
    } else {
        drivers
    };

    if drivers.contains(&Driver::Cpu) {
        draw_all(&mut drivers::Cpu::new(), &out_path);
    }

    if drivers.contains(&Driver::Pdf) {
        draw_all(&mut drivers::Pdf::new(), &out_path);
    }

    if drivers.contains(&Driver::Svg) {
        draw_all(&mut drivers::Svg::new(), &out_path);
    }

    #[cfg(feature = "svg")]
    {
        use drivers::render_svg::*;
        if drivers.contains(&Driver::RenderSvg) {
            draw_all(&mut RenderSvg::new(), &out_path);
        }
    }

    #[cfg(feature = "gl")]
    {
        use drivers::gl::*;
        if drivers.contains(&Driver::OpenGl) {
            let context = GLContext::<NativeGLContext>::create(
                sparkle::gl::GlType::Gl,
                GLVersion::MajorMinor(3, 3),
                None,
            )
            .unwrap();

            context.make_current().unwrap();
            draw_all(&mut OpenGl::new(), &out_path);
        }

        if drivers.contains(&Driver::OpenGlEs) {
            let context = GLContext::<NativeGLContext>::create(
                sparkle::gl::GlType::Gles,
                GLVersion::MajorMinor(3, 3),
                None,
            )
            .unwrap();

            context.make_current().unwrap();
            draw_all(&mut OpenGl::new(), &out_path);
        }
    }

    #[cfg(feature = "vulkan")]
    {
        use drivers::vulkan::{AshGraphics, Vulkan};

        if drivers.contains(&Driver::Vulkan) {
            match AshGraphics::vulkan_version() {
                Some((major, minor, patch)) => {
                    println!("Detected Vulkan version {}.{}.{}", major, minor, patch)
                }
                None => println!("Failed to detect Vulkan version, falling back to 1.0.0"),
            }

            draw_all(&mut Vulkan::new(), &out_path)
        }
    }

    #[cfg(feature = "metal")]
    {
        use drivers::metal::Metal;

        if drivers.contains(&Driver::Metal) {
            draw_all(&mut Metal::new(), &out_path)
        }
    }

    #[cfg(feature = "d3d")]
    {
        use drivers::d3d::D3D;

        if drivers.contains(&Driver::D3d) {
            draw_all(&mut D3D::new(), &out_path)
        }
    }

    fn draw_all<Driver: DrawingDriver>(driver: &mut Driver, out_path: &Path) {
        let out_path = out_path.join(Driver::DRIVER.to_string());

        skcanvas_overview::draw(driver, &out_path);
        skpath_overview::draw(driver, &out_path);
        skpaint_overview::draw(driver, &out_path);

        #[cfg(feature = "textlayout")]
        {
            skshaper_example::draw(driver, &out_path);
            skparagraph_example::draw(driver, &out_path);
        }
    }
}

pub(crate) mod resources {

    use skia_safe::{Data, Image};

    pub fn color_wheel() -> Image {
        let bytes = include_bytes!("resources/color_wheel.png");
        let data = Data::new_copy(bytes);
        Image::from_encoded(data).unwrap()
    }

    pub fn mandrill() -> Image {
        let bytes = include_bytes!("resources/mandrill_512.png");
        let data = Data::new_copy(bytes);
        Image::from_encoded(data).unwrap()
    }
}
