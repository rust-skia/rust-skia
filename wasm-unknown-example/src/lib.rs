use skia_safe::{AlphaType, Canvas, Color, ColorType, ImageInfo, Paint, PaintStyle, Rect};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_scene(width: u32, height: u32) -> Vec<u8> {
    const MAX_DIMENSION: u32 = 4096;
    if width == 0 || height == 0 || width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Vec::new();
    }

    let width_i32 = match i32::try_from(width) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    let height_i32 = match i32::try_from(height) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };

    let row_bytes = match usize::try_from(width).ok().and_then(|w| w.checked_mul(4)) {
        Some(value) => value,
        None => return Vec::new(),
    };
    let pixel_len = match usize::try_from(height)
        .ok()
        .and_then(|h| row_bytes.checked_mul(h))
    {
        Some(value) => value,
        None => return Vec::new(),
    };

    let mut pixels = vec![0; pixel_len];
    let image_info = ImageInfo::new((width_i32, height_i32), ColorType::RGBA8888, AlphaType::Premul, None);
    if row_bytes < image_info.min_row_bytes() {
        return Vec::new();
    }

    let Some(canvas) = Canvas::from_raster_direct(
        &image_info,
        pixels.as_mut_slice(),
        Some(row_bytes),
        None,
    ) else {
        return pixels;
    };
    {
        let canvas = canvas;
        canvas.clear(Color::WHITE);

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);

        paint.set_color(Color::from_rgb(0x28, 0x65, 0xff));
        canvas.draw_rect(
            Rect::from_xywh(
                24.0,
                24.0,
                (width as f32 * 0.56).max(32.0),
                (height as f32 * 0.36).max(24.0),
            ),
            &paint,
        );

        paint.set_color(Color::from_rgb(0xff, 0x8a, 0x00));
        let radius = (width.min(height) as f32 * 0.2).max(18.0);
        canvas.draw_circle((width as f32 * 0.7, height as f32 * 0.68), radius, &paint);

        paint.set_style(PaintStyle::Stroke);
        paint.set_color(Color::from_rgb(0x1a, 0x2b, 0x48));
        paint.set_stroke_width(6.0);
        canvas.draw_circle((width as f32 * 0.35, height as f32 * 0.68), radius * 0.72, &paint);
    }
    pixels
}
