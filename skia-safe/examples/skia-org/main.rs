use std::path::PathBuf;
use crate::artifact::{DrawingDriver, OpenGL, Vulkan, CPU};
use clap::{App, Arg};

extern crate skia_safe;
#[macro_use]
extern crate ash;


// TODO: think about making the examples more Rust-idiomatic, by using method chaining for Paint / Paths, for example.

mod drivers;
mod skcanvas_overview;
mod skpaint_overview;
mod skpath_overview;

pub(crate) mod artifact {
    use skia_safe::skia::{Canvas, EncodedImageFormat, Surface, Budgeted, ImageInfo};
    use skia_safe::graphics;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use glutin::{ContextBuilder, ContextTrait};
    use crate::drivers::skia_ash::AshGraphics;

    pub trait DrawingDriver {

        const NAME: &'static str;

        fn draw_image<F>(size: (i32, i32), path: &PathBuf, name: &str, func: F) -> ()
            where F: Fn(&mut Canvas) -> ();

        fn draw_image_256<F>(path: &PathBuf, name: &str, func: F)
            where F: Fn(&mut Canvas) -> () {
            Self::draw_image((256, 256), path, name, func)
        }
    }

    pub enum CPU {}
    pub enum OpenGL {}
    #[cfg(feature = "vulkan")]
    pub enum Vulkan {}

    impl DrawingDriver for CPU {

        const NAME : &'static str = "cpu";

        fn draw_image<F>((width, height): (i32, i32), path: &PathBuf, name: &str, func: F)
            where F: Fn(&mut Canvas) -> () {
            let mut surface = Surface::new_raster_n32_premul((width*2, height*2)).unwrap();
            draw_image_on_surface(&mut surface, path, name, func);
        }
    }

    impl DrawingDriver for OpenGL {

        const NAME: &'static str = "opengl";

        fn draw_image<F>((width, height): (i32, i32), path: &PathBuf, name: &str, func: F)
            where F: Fn(&mut Canvas) -> () {

            let events_loop = glutin::EventsLoop::new();
            let context = ContextBuilder::new()
                .build_headless(
                    &events_loop,
                    glutin::dpi::PhysicalSize::new(0.0, 0.0)
                )
                .unwrap();

            unsafe { context.make_current().unwrap(); }

            let mut context = graphics::Context::new_gl(None).unwrap();

            let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
            let mut surface = Surface::new_render_target(
                &mut context,
                Budgeted::YES,
                &image_info, None, graphics::SurfaceOrigin::TopLeft, None, false).unwrap();

            draw_image_on_surface(&mut surface, path, name, func);
        }
    }

    #[cfg(feature="vulkan")]
    impl DrawingDriver for Vulkan {

        const NAME: &'static str = "vulkan";

        fn draw_image<F>((width, height): (i32, i32), path: &PathBuf, name: &str, func: F)
            where F: Fn(&mut Canvas) -> () {


            let ash_graphics = unsafe { AshGraphics::new("skia-org") };


            /*
            let mut context = graphics::Context::new_vulkan(vulkan_context).unwrap();

            let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
            let mut surface = Surface::new_render_target(
                &mut context,
                Budgeted::YES,
                &image_info, None, graphics::SurfaceOrigin::TopLeft, None, false).unwrap();

            draw_image_on_surface(&mut surface, path, name, func);
            */
        }


    }

    fn draw_image_on_surface<F>(surface: &mut Surface, path: &PathBuf, name: &str, func: F)
        where F: Fn(&mut Canvas) -> () {

        let mut canvas = surface.canvas();

        canvas.scale((2.0, 2.0));
        func(&mut canvas);
        let image = surface.image_snapshot();
        let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();

        fs::create_dir_all(&path)
        .expect("failed to create directory");

        let mut file_path = path.join(name);
        file_path.set_extension("png");

        let mut file = fs::File::create(file_path).expect("failed to create file");
        let bytes = data.bytes();
        file.write_all(bytes).expect("failed to write to file");
    }
}

pub (crate) mod resources {

    use skia_safe::skia::{Image, Data};

    pub fn color_wheel() -> Image {
        let bytes = include_bytes!("resources/color_wheel.png");
        let data = Data::new_copy(bytes);
        Image::from_encoded(&data, None).unwrap()
    }

    pub fn mandrill() -> Image {
        let bytes = include_bytes!("resources/mandrill_512.png");
        let data = Data::new_copy(bytes);
        Image::from_encoded(&data, None).unwrap()
    }
}

fn main() {
    const OUT_PATH : &str = "OUT_PATH";
    const DRIVER : &str = "driver";

    let matches =
        App::new("skia-org examples")
            .about("Renders examples from skia.org with rust-skia")
            .arg(Arg::with_name(OUT_PATH)
                .help("The output path to render into.")
                .default_value(".")
                .required(true))
            .arg(Arg::with_name(DRIVER)
                .long(DRIVER)
                .takes_value(true)
                .possible_values(get_possible_drivers())
                .multiple(true)
                .help("In addition to the CPU, render with the given driver.")
            )
            .get_matches();

    let out_path : PathBuf =
        PathBuf::from(matches.value_of(OUT_PATH).unwrap());


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

    if drivers.contains(&CPU::NAME) {
        draw_all::<artifact::CPU>(&out_path);
    }

    if drivers.contains(&OpenGL::NAME) {
        draw_all::<artifact::OpenGL>(&out_path);
    }

    #[cfg(feature = "vulkan")]
    {
        if drivers.contains(&Vulkan::NAME) {
            draw_all::<artifact::Vulkan>(&out_path)
        }
    }

    fn draw_all<Driver: DrawingDriver>(out_path: &PathBuf) {

        let out_path = out_path.join(Driver::NAME);

        skcanvas_overview::draw::<Driver>(&out_path);
        skpath_overview::draw::<Driver>(&out_path);
        skpaint_overview::draw::<Driver>(&out_path);
    }
}

#[cfg(not(feature = "vulkan"))]
fn get_possible_drivers() -> &'static [&'static str] {
    ["cpu", "opengl"].as_ref()
}

#[cfg(feature = "vulkan")]
fn get_possible_drivers() -> &'static [&'static str] {
    ["cpu", "opengl", "vulkan"].as_ref()
}
