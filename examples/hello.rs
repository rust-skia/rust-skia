extern crate rust_skia;

use std::io::Write;
use std::fs::File;

use rust_skia::Canvas;

fn main() {
  let mut canvas = Canvas::new(2560, 1280);
  canvas.scale(1.2, 1.2);
  canvas.move_to(36.0, 48.0);
  canvas.quad_to(660.0, 880.0, 1200.0, 360.0);
  canvas.translate(10.0, 10.0);
  canvas.stroke();
  canvas.save();
  let d = canvas.data();
  let mut file = File::create("test.png").unwrap();
  file.write_all(d).unwrap();
}
