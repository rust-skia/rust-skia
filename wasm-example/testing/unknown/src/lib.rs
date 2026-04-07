use skia_safe::{surfaces, AlphaType, Color, ColorType, Data, ImageInfo, Paint, PaintStyle, Surface};
use wasm_bindgen::prelude::*;

fn test_surface_alloc_impl(width: i32, height: i32, count: u32) -> i32 {
    let info = ImageInfo::new(
        (width, height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );

    let mut surfaces: Vec<Surface> = Vec::with_capacity(count as usize);

    for i in 0..count {
        match surfaces::raster(&info, None, None) {
            Some(surface) => surfaces.push(surface),
            None => return i as i32,
        }
    }

    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::BLACK);
    paint.set_anti_alias(true);

    for surface in &mut surfaces {
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_circle((width as f32 / 2.0, height as f32 / 2.0), 10.0, &paint);
    }

    -1
}

fn test_surface_alloc_sequential_impl(width: i32, height: i32, count: u32) -> i32 {
    let info = ImageInfo::new(
        (width, height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );

    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::BLACK);
    paint.set_anti_alias(true);

    for i in 0..count {
        match surfaces::raster(&info, None, None) {
            Some(mut surface) => {
                let canvas = surface.canvas();
                canvas.clear(Color::WHITE);
                canvas.draw_circle((width as f32 / 2.0, height as f32 / 2.0), 10.0, &paint);
            }
            None => return i as i32,
        }
    }

    -1
}

fn test_plain_rust_alloc_impl(bytes: usize, rounds: u32) -> i32 {
    for i in 0..rounds {
        let mut buf = vec![0u8; bytes];
        if !buf.is_empty() {
            let last = buf.len() - 1;
            buf[0] = 0xAB;
            buf[last] = 0xCD;
            if buf[0] != 0xAB || buf[last] != 0xCD {
                return i as i32;
            }
        }
    }

    -1
}

fn test_sk_data_alloc_impl(bytes: usize, rounds: u32) -> i32 {
    let mut blocks: Vec<Data> = Vec::with_capacity(rounds as usize);

    for i in 0..rounds {
        let data = unsafe { Data::new_uninitialized(bytes) };
        if data.len() != bytes {
            return i as i32;
        }
        blocks.push(data);
    }

    -1
}


#[wasm_bindgen]
pub fn test_surface_alloc(width: i32, height: i32, count: u32) -> i32 {
    test_surface_alloc_impl(width, height, count)
}

#[wasm_bindgen]
pub fn test_surface_alloc_sequential(width: i32, height: i32, count: u32) -> i32 {
    test_surface_alloc_sequential_impl(width, height, count)
}

#[wasm_bindgen]
pub fn test_plain_rust_alloc(bytes: u32, rounds: u32) -> i32 {
    test_plain_rust_alloc_impl(bytes as usize, rounds)
}

#[wasm_bindgen]
pub fn test_sk_data_alloc(bytes: u32, rounds: u32) -> i32 {
    test_sk_data_alloc_impl(bytes as usize, rounds)
}
