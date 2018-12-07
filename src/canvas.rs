mod raw {
  #![allow(non_upper_case_globals)]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  include!("./bindings.rs");
}

use std::ptr;
use std::mem;
use std::slice;
use std::os::raw::c_void;

use self::raw::*;

struct ReleaseSurface(unsafe extern "C" fn());

pub struct Canvas{
  sk_canvas: &'static mut SkCanvas,
  sk_path: SkPath,
  sk_image_info: &'static mut SkImageInfo,
  release_surface: ReleaseSurface,
  row_bytes: usize,
  size: usize,
  width: i32,
  height: i32,
  data_ptr: *mut u8,
}

impl Canvas {
  pub fn new(width: i32, height: i32) -> Canvas {
    unsafe {
      let sk_canvas_bindings = SkiaCreateCanvas(width, height);
      let sk_canvas = mem::transmute::<*mut SkCanvas, &mut SkCanvas>(sk_canvas_bindings.canvas);
      let sk_image_info = mem::transmute::<*mut SkImageInfo, &mut SkImageInfo>(sk_canvas_bindings.info);
      let sk_path = SkPath::new();
      Canvas {
        sk_canvas,
        sk_path,
        sk_image_info,
        release_surface: ReleaseSurface(sk_canvas_bindings.release_fn.unwrap()),
        row_bytes: sk_canvas_bindings.rowBytes,
        size: sk_canvas_bindings.size,
        data_ptr: sk_canvas_bindings.data_ptr as *mut u8,
        width, height,
      }
    }
  }

  #[inline]
  pub fn save(&mut self) {
    unsafe {
      self.sk_canvas.save();
    }
  }

  #[inline]
  pub fn translate(&mut self, dx: f32, dy: f32) {
    unsafe {
      self.sk_canvas.translate(dx, dy);
    }
  }

  #[inline]
  pub fn scale(&mut self, sx: f32, sy: f32) {
    unsafe {
      self.sk_canvas.scale(sx, sy);
    }
  }

  #[inline]
  pub fn move_to(&mut self, x: f32, y: f32) {
    unsafe {
      self.sk_path.moveTo(x, y);
    }
  }

  #[inline]
  pub fn line_to(&mut self, x: f32, y: f32) {
    unsafe {
      self.sk_path.lineTo(x, y);
    }
  }

  #[inline]
  pub fn close_path(&mut self) {
    unsafe {
      self.sk_path.close();
    }
  }

  #[inline]
  pub fn data(&mut self) -> &'static [u8] {
    unsafe {
      slice::from_raw_parts(self.data_ptr, self.row_bytes * self.width as usize)
    }
  }
}

impl Drop for Canvas {
  fn drop(&mut self) {
    unsafe {
      self.release_surface.0();
    };
  }
}
