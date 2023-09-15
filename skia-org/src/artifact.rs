use skia_safe::{Canvas, EncodedImageFormat, Surface};
use std::{fs, io::Write, path::Path};

pub fn draw_image_on_surface(
    surface: &mut Surface,
    path: &Path,
    name: &str,
    func: impl Fn(&Canvas),
) {
    let canvas = surface.canvas();

    canvas.scale((2.0, 2.0));
    func(canvas);
    let image = surface.image_snapshot();
    let data = image
        .encode(&mut surface.direct_context(), EncodedImageFormat::PNG, None)
        .unwrap();
    write_file(data.as_bytes(), path, name, "png");
}

pub fn write_file(bytes: &[u8], path: &Path, name: &str, ext: &str) {
    fs::create_dir_all(path).expect("failed to create directory");

    let mut file_path = path.join(name);
    file_path.set_extension(ext);

    let mut file = fs::File::create(file_path).expect("failed to create file");
    file.write_all(bytes).expect("failed to write to file");
}
