extern crate image;
extern crate rust_skia;

use image::{save_buffer, ColorType as ImageColorType};
use rust_skia::Canvas;

fn main() {
  let mut canvas = Canvas::new(300, 300);
  canvas.scale(1.2, 1.2);
  canvas.begin_path();
  canvas.move_to(100.0, 100.0);
  canvas.line_to(300.0, 300.0);
  canvas.translate(100.0, 100.0);
  canvas.close_path();
  canvas.stroke();
  canvas.save();
  let d = canvas.data();
  save_buffer("./test.png", d, 300, 300, ImageColorType::RGB(8)).unwrap();
}
