use std::env;
use std::path::PathBuf;

extern crate skia_safe;

mod skcanvas_overview;
mod skpath_overview;

pub(crate) mod artifact {
    use skia_safe::skia::{Canvas, EncodedImageFormat, Surface};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    pub fn draw_canvas_256<F>(path: &PathBuf, name: &str, func: F)
        where F: Fn(&mut Canvas) -> () {
        let mut surface = Surface::new_raster_n32_premul((512, 512)).unwrap();
        let mut canvas = surface.canvas();
        canvas.scale(2.0, 2.0);
        func(&mut canvas);
        let image = surface.make_image_snapshot();
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

fn main() {
    let args : Vec<String> = env::args().collect();

    let out_path : PathBuf = match args.len() {
        1 => PathBuf::from("."),
        2 => PathBuf::from(args[1].clone()),
        _ => {
            println!("use skia-org [OUT_PATH]");
            return
        }
    };

    skcanvas_overview::draw(&out_path);
    skpath_overview::draw(&out_path);
}