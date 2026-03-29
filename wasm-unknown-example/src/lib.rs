use skia_safe::{AlphaType, ColorType, Color, ImageInfo, Paint, PaintStyle};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub struct State {
    width: i32,
    height: i32,
    pixels: Vec<u8>,
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, context: CanvasRenderingContext2d) -> Self {
        let (width, height) = (width as i32, height as i32);
        Self {
            width,
            height,
            pixels: vec![0u8; (width * height * 4) as usize],
            context,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width as i32;
        self.height = height as i32;
        self.pixels.resize((self.width * self.height * 4) as usize, 0);
    }

    pub fn draw(&mut self, x: f32, y: f32) -> Result<(), JsValue> {
        let info = ImageInfo::new(
            (self.width, self.height),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );
        let row_bytes = self.width as usize * 4;
        let canvas = skia_safe::Canvas::from_raster_direct(&info, &mut self.pixels, Some(row_bytes), None)
            .ok_or_else(|| JsValue::from_str("failed to create raster canvas"))?;

        canvas.clear(Color::WHITE);
        let mut paint = Paint::default();
        paint.set_style(PaintStyle::Fill);
        paint.set_color(Color::BLACK);
        paint.set_anti_alias(true);
        canvas.draw_circle((x, y), 50.0, &paint);
        drop(canvas);

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&self.pixels),
            self.width as u32,
            self.height as u32,
        )?;
        self.context.put_image_data(&image_data, 0.0, 0.0)
    }
}
