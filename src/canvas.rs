#[allow(dead_code)]
mod raw {
  #![allow(non_upper_case_globals)]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  include!("./bindings.rs");
}

use core::mem;
use core::slice;

use self::raw::*;

struct ReleaseSurface(unsafe extern "C" fn());

pub struct Canvas {
  sk_surface: &'static mut SkSurface,
  sk_canvas: &'static mut SkCanvas,
  sk_path: SkPath,
  sk_rect: SkRect,
  sk_paint: SkPaint,
  release_surface: ReleaseSurface,
  width: i32,
  height: i32,
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
      let sk_surface = mem::transmute::<*mut SkSurface, &mut SkSurface>(sk_canvas_bindings.surface);
      let sk_canvas = mem::transmute::<*mut SkCanvas, &mut SkCanvas>(sk_canvas_bindings.canvas);
      let sk_path = SkPath::new();
      let sk_rect = SkiaCreateRect(width as f32, height as f32);
      let mut sk_paint = SkPaint::new();
      sk_paint.setColor(SK_ColorBLACK);
      sk_paint.setAntiAlias(true);
      sk_paint.setStrokeWidth(1.0);
      SkiaClearCanvas(sk_canvas as *mut _, SK_ColorWHITE);
      Canvas {
        sk_canvas,
        sk_surface,
        sk_path,
        sk_rect,
        sk_paint,
        release_surface: ReleaseSurface(sk_canvas_bindings.release_fn.unwrap()),
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
  pub fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
    unsafe {
      self.sk_path.quadTo(cpx, cpy, x, y);
    }
  }

  #[inline]
  pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
    unsafe {
      self.sk_path.cubicTo(cp1x, cp1y, cp2x, cp2y, x, y);
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
    unsafe {
      let surface_data = SkiaGetSurfaceData(self.sk_surface as *mut SkSurface);
      slice::from_raw_parts(surface_data.data, surface_data.size)
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
