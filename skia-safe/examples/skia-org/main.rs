use std::path::PathBuf;
use crate::artifact::DrawingDriver;
use clap::{App, Arg};
use offscreen_gl_context::{GLContext, NativeGLContext, GLVersion};
use gleam;


extern crate skia_safe;

// TODO: think about making the examples more Rust-idiomatic, by using method chaining for Paint / Paths, for example.

mod skcanvas_overview;
mod skpaint_overview;
mod skpath_overview;

pub(crate) mod artifact {
    use skia_safe::skia::{Canvas, EncodedImageFormat, Surface, Budgeted, ImageInfo};
    use skia_safe::graphics;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

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

            let mut context = graphics::Context::new_gl(None).unwrap();

            let image_info = ImageInfo::new_n32_premul((width * 2, height * 2), None);
            let mut surface = Surface::new_render_target(
                &mut context,
                Budgeted::YES,
                &image_info, None, graphics::SurfaceOrigin::TopLeft, None, false).unwrap();

            draw_image_on_surface(&mut surface, path, name, func);
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
    const POSSIBLE_DRIVERS : &[&str; 1] = &["opengl"];

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
                .possible_values(POSSIBLE_DRIVERS)
                .multiple(true)
                .help("In addition to the CPU, render with the given driver.")
            )
            .get_matches();

    let out_path : PathBuf =
        PathBuf::from(matches.value_of(OUT_PATH).unwrap());

    fn draw_all<Driver: DrawingDriver>(out_path: &PathBuf) {

        let out_path = out_path.join(Driver::NAME);

        skcanvas_overview::draw::<Driver>(&out_path);
        skpath_overview::draw::<Driver>(&out_path);
        skpaint_overview::draw::<Driver>(&out_path);
    }

    draw_all::<artifact::CPU>(&out_path);

    let drivers = matches.values_of(DRIVER).unwrap_or_default();


    if drivers.into_iter().any(|v| v == "opengl") {

        let context =
            GLContext::<NativeGLContext>::create(
                gleam::gl::GlType::default(),
                GLVersion::Major(4),
                None).unwrap();

        context.make_current().unwrap();

        draw_all::<artifact::OpenGL>(&out_path);
    }
}
