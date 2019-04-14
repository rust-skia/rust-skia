use std::path::PathBuf;
use crate::artifact::DrawingDriver;
use clap::{App, Arg};
use offscreen_gl_context::{GLContext, NativeGLContext, GLVersion};
use gleam;

extern crate skia_safe;

#[cfg(feature="vulkan")]
use crate::artifact::{Vulkan};
#[cfg(feature="vulkan")]
use crate::drivers::skia_ash::AshGraphics;

#[cfg(feature="vulkan")]
#[macro_use]
extern crate ash;

// TODO: think about making the examples more Rust-idiomatic, by using method chaining for Paint / Paths, for example.

mod drivers;
mod skcanvas_overview;
mod skpaint_overview;
mod skpath_overview;

pub(crate) mod artifact {
    use skia_safe::{Canvas, EncodedImageFormat, Surface, Budgeted, ImageInfo};
    use skia_safe::gpu;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    #[cfg(feature="vulkan")]
    use std::ptr;
    #[cfg(feature="vulkan")]
    use crate::drivers::skia_ash::AshGraphics;
    #[cfg(feature="vulkan")]
    use ash::version::InstanceV1_0;
    #[cfg(feature="vulkan")]
    use ash::vk::Handle;

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
    pub enum PDF {}

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

            let mut context = gpu::Context::new_gl(None).unwrap();

            let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
            let mut surface = Surface::new_render_target(
                &mut context,
                Budgeted::YES,
                &image_info, None, gpu::SurfaceOrigin::TopLeft, None, false).unwrap();

            draw_image_on_surface(&mut surface, path, name, func);
        }
    }

    #[cfg(feature="vulkan")]
    impl DrawingDriver for Vulkan {

        const NAME: &'static str = "vulkan";

        fn draw_image<F>((width, height): (i32, i32), path: &PathBuf, name: &str, func: F)
            where F: Fn(&mut Canvas) -> () {


            let ash_graphics = unsafe { AshGraphics::new("skia-org") };

            let get_proc = |of| unsafe {
                match ash_graphics.get_proc(of) {
                    Some(f) => f as _,
                    None => {
                        println!("resolve of {} failed", of.name().to_str().unwrap());
                        ptr::null()
                    }
                }
            };

            let backend_context = unsafe {
                gpu::vk::BackendContext::new(
                    ash_graphics.instance.handle().as_raw() as _,
                    ash_graphics.physical_device.as_raw() as _,
                    ash_graphics.device.handle().as_raw() as _,
                    (ash_graphics.queue_and_index.0.as_raw() as _, ash_graphics.queue_and_index.1),
                    &get_proc)
            };

            let mut context = gpu::Context::new_vulkan(&backend_context).unwrap();

            let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
            let mut surface = Surface::new_render_target(
                &mut context,
                Budgeted::YES,
                &image_info, None, gpu::SurfaceOrigin::TopLeft, None, false).unwrap();

            draw_image_on_surface(&mut surface, path, name, func);
        }
    }

    impl DrawingDriver for PDF {
        const NAME: &'static str = "pdf";

        fn draw_image<F>(size: (i32, i32), path: &PathBuf, name: &str, func: F) -> () where F: Fn(&mut Canvas) -> () {
            let mut document = skia_safe::pdf::new_document(None).begin_page((size.0 as _, size.1 as _), None);
            func(document.canvas());
            let data = document.end_page().close();
            write_file(data.bytes(), path, name, "pdf");
        }
    }

    fn draw_image_on_surface<F>(surface: &mut Surface, path: &PathBuf, name: &str, func: F)
        where F: Fn(&mut Canvas) -> () {

        let mut canvas = surface.canvas();

        canvas.scale((2.0, 2.0));
        func(&mut canvas);
        let image = surface.image_snapshot();
        let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
        write_file(data.bytes(), path, name, "png");
    }

    fn write_file(bytes: &[u8], path: &PathBuf, name: &str, ext: &str) {
        fs::create_dir_all(&path)
            .expect("failed to create directory");

        let mut file_path = path.join(name);
        file_path.set_extension(ext);

        let mut file = fs::File::create(file_path).expect("failed to create file");
        file.write_all(bytes).expect("failed to write to file");
    }
}

pub (crate) mod resources {

    use skia_safe::{Image, Data};

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

    if drivers.contains(&artifact::CPU::NAME) {
        draw_all::<artifact::CPU>(&out_path);
    }

    if drivers.contains(&artifact::PDF::NAME) {
        draw_all::<artifact::PDF>(&out_path);
    }

    if drivers.contains(&artifact::OpenGL::NAME) {
        let context =
            GLContext::<NativeGLContext>::create(
                gleam::gl::GlType::default(),
                GLVersion::Major(4),
                None).unwrap();

        context.make_current().unwrap();
        draw_all::<artifact::OpenGL>(&out_path);
    }

    #[cfg(feature = "vulkan")]
    {
        if drivers.contains(&Vulkan::NAME) {
            match AshGraphics::vulkan_version() {
                Some((major, minor, patch)) => {
                    println!("Detected Vulkan version {}.{}.{}", major, minor, patch)
                },
                None => {
                    println!("Failed to detect Vulkan version, falling back to 1.0.0")
                }
            }

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
    ["cpu", "pdf", "opengl"].as_ref()
}

#[cfg(feature = "vulkan")]
fn get_possible_drivers() -> &'static [&'static str] {
    ["cpu", "pdf", "opengl", "vulkan"].as_ref()
}

