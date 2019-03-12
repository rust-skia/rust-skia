extern crate skia_safe;

mod skcanvas_overview;
mod skpath_overview;

pub(crate) mod artifact {
    use skia_safe::skia::{Canvas, EncodedImageFormat, Surface};
    use std::fs;
    use std::io::Write;

    pub fn draw_canvas_256<F>(name: &str, func: F)
        where F: Fn(&mut Canvas) -> () {
        let mut surface = Surface::new_raster_n32_premul((512, 512)).unwrap();
        let mut canvas = surface.canvas();
        canvas.scale(2.0, 2.0);
        func(&mut canvas);
        let image = surface.make_image_snapshot();
        let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();

        let mut file = fs::File::create(format!("{}.png", name)).unwrap();
        let bytes = data.bytes();
        file.write_all(bytes).unwrap();
    }
}

fn main() {
    skcanvas_overview::draw();
    skpath_overview::draw();
}