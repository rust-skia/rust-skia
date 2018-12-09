mod raw {
  #![allow(non_upper_case_globals)]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  include!("./bindings.rs");
}

use std::mem;
use std::slice;

use self::raw::*;

struct ReleaseSurface(unsafe extern "C" fn());

pub struct Canvas {
  sk_canvas: &'static mut SkCanvas,
  sk_path: SkPath,
  sk_rect: SkRect,
  sk_paint: SkPaint,
  sk_image_info: &'static mut SkImageInfo,
  release_surface: ReleaseSurface,
  row_bytes: usize,
  size: usize,
  width: i32,
  height: i32,
  data_ptr: *mut u8,
}

#[inline]
pub fn set_a_rgb(a: U8CPU, r: U8CPU, g: U8CPU, b: U8CPU) -> SkColor {
  debug_assert!(a <= 255);
  debug_assert!(r <= 255);
  debug_assert!(g <= 255);
  debug_assert!(b <= 255);
  (a << 24) | (r << 16) | (g << 8) | (b << 0)
}

impl Canvas {
  pub fn new(width: i32, height: i32) -> Canvas {
    unsafe {
      let sk_canvas_bindings = SkiaCreateCanvas(width, height);
      let sk_canvas = mem::transmute::<*mut SkCanvas, &mut SkCanvas>(sk_canvas_bindings.canvas);
      let sk_image_info =
        mem::transmute::<*mut SkImageInfo, &mut SkImageInfo>(sk_canvas_bindings.info);
      let sk_path = SkPath::new();
      let sk_rect = SkiaCreateRect(width as f32, height as f32);
      let mut sk_paint = SkPaint::new();
      sk_paint.setARGB(255, 0, 0, 0);
      sk_paint.setStrokeWidth(1.0);
      SkiaClearCanvas(sk_canvas as *mut _, set_a_rgb(255, 255, 255, 255));
      Canvas {
        sk_canvas,
        sk_path,
        sk_rect,
        sk_paint,
        sk_image_info,
        release_surface: ReleaseSurface(sk_canvas_bindings.release_fn.unwrap()),
        row_bytes: sk_canvas_bindings.rowBytes,
        size: sk_canvas_bindings.size,
        data_ptr: sk_canvas_bindings.data_ptr as *mut u8,
        width,
        height,
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
  pub fn begin_path(&mut self) {
    unsafe {
      let new_path = SkPath::new();
      self.sk_canvas.drawPath(
        &mut self.sk_path as *const _,
        &mut self.sk_paint as *const _,
      );
      mem::replace(&mut self.sk_path, new_path);
    }
  }

  #[inline]
  pub fn stroke(&mut self) {
    unsafe {
      self.sk_paint.setStyle(SkPaint_Style_kStroke_Style);
      self.sk_canvas.drawPath(
        &mut self.sk_path as *const _,
        &mut self.sk_paint as *const _,
      );
    };
  }

  #[inline]
  pub fn set_line_width(&mut self, width: f32) {
    unsafe {
      self.sk_paint.setStrokeWidth(width);
    };
  }

  #[inline]
  pub fn data(&mut self) -> &'static [u8] {
    unsafe { slice::from_raw_parts(self.data_ptr, self.row_bytes * self.width as usize) }
  }
}

impl Drop for Canvas {
  fn drop(&mut self) {
    unsafe {
      self.release_surface.0();
    };
  }
}
