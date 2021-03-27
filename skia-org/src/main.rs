use clap::{App, Arg};
#[cfg(feature = "gl")]
use offscreen_gl_context::{GLContext, GLVersion, NativeGLContext};
use std::path::{Path, PathBuf};

use crate::drivers::DrawingDriver;

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

fn main() {
    const OUT_PATH: &str = "OUT_PATH";
    const DRIVER: &str = "driver";

    let matches = App::new("skia-org examples")
        .about("Renders examples from skia.org with rust-skia")
        .arg(
            Arg::with_name(OUT_PATH)
                .help("The output path to render into.")
                .default_value(".")
                .required(true),
        )
        .arg(
            Arg::with_name(DRIVER)
                .long(DRIVER)
                .takes_value(true)
                .possible_values(get_available_drivers().as_slice())
                .multiple(true)
                .help("In addition to the CPU, render with the given driver."),
        )
        .get_matches();

    let out_path = PathBuf::from(matches.value_of(OUT_PATH).unwrap());

    let drivers = {
        let drivers = matches
            .values_of(DRIVER)
            .unwrap_or_default()
            .collect::<Vec<&str>>();
        if drivers.is_empty() {
            vec!["cpu"]
        } else {
            drivers
        }
    };

    if drivers.contains(&drivers::Cpu::NAME) {
        draw_all(&mut drivers::Cpu::new(), &out_path);
    }

    if drivers.contains(&drivers::Pdf::NAME) {
        draw_all(&mut drivers::Pdf::new(), &out_path);
    }

    if drivers.contains(&drivers::Svg::NAME) {
        draw_all(&mut drivers::Svg::new(), &out_path);
    }

    #[cfg(feature = "gl")]
    {
        if drivers.contains(&drivers::OpenGl::NAME) {
            let context = GLContext::<NativeGLContext>::create(
                sparkle::gl::GlType::Gl,
                GLVersion::MajorMinor(3, 3),
                None,
            )
            .unwrap();

            context.make_current().unwrap();
            draw_all(&mut drivers::OpenGl::new(), &out_path);
        }

        if drivers.contains(&"opengl-es") {
            let context = GLContext::<NativeGLContext>::create(
                sparkle::gl::GlType::Gles,
                GLVersion::MajorMinor(3, 3),
                None,
            )
            .unwrap();

            context.make_current().unwrap();
            draw_all(&mut drivers::OpenGl::new(), &out_path);
        }
    }

    #[cfg(feature = "vulkan")]
    {
        use drivers::vulkan::AshGraphics;
        use drivers::Vulkan;

        if drivers.contains(&Vulkan::NAME) {
            match AshGraphics::vulkan_version() {
                Some((major, minor, patch)) => {
                    println!("Detected Vulkan version {}.{}.{}", major, minor, patch)
                }
                None => println!("Failed to detect Vulkan version, falling back to 1.0.0"),
            }

            draw_all(&mut drivers::Vulkan::new(), &out_path)
        }
    }

    #[cfg(feature = "metal")]
    {
        use drivers::metal::Metal;

        if drivers.contains(&Metal::NAME) {
            draw_all(&mut Metal::new(), &out_path)
        }
    }

    #[cfg(feature = "d3d")]
    {
        use drivers::d3d::D3D;

        if drivers.contains(&D3D::NAME) {
            draw_all(&mut D3D::new(), &out_path)
        }
    }

    fn draw_all<Driver: DrawingDriver>(driver: &mut Driver, out_path: &Path) {
        let out_path = out_path.join(Driver::NAME);

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

fn get_available_drivers() -> Vec<&'static str> {
    let mut drivers = vec!["cpu", "pdf", "svg"];
    if cfg!(feature = "gl") {
        drivers.extend(vec!["opengl", "opengl-es"]);
    }
    if cfg!(feature = "vulkan") {
        drivers.push("vulkan")
    }
    if cfg!(feature = "metal") {
        drivers.push("metal")
    }
    if cfg!(feature = "d3d") {
        drivers.push("d3d")
    }
    drivers
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
