use rust_skia::*;
use std::ptr;
use std::slice;
use std::marker::PhantomData;

pub struct Surface {
    native: *mut SkSurface
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { (*self.native)._base._base.unref() }
    }
}

impl Surface {

    // tbd: use Option as a return type.
    pub fn new_raster_n32_premul(width: i32, height: i32) -> Surface {
        Surface {
            native: unsafe { C_SkSurface_MakeRasterN32Premul(width, height, ptr::null()) }
        }
    }

    pub fn canvas(&self) -> Canvas
    {
        Canvas {
            native: unsafe { (*self.native).getCanvas() },
            phantom: PhantomData
        }
    }

    pub fn make_image_snapshot(&mut self) -> Image {
        Image { native: unsafe { C_SkSurface_makeImageSnapshot(self.native) } }
    }
}


pub struct Image {
    native: *mut SkImage
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { (*self.native)._base._base.unref() }
    }
}

impl Image {

    pub fn encode_to_data(&self) -> Data {
        Data { native: unsafe { C_SkImage_encodeToData(self.native) } }
    }
}


pub struct Data {
    native: *mut SkData
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe { C_SkData_unref(&*self.native) }
    }
}

impl Data {
    pub fn bytes(&self) -> &[u8] {
        unsafe {
            let bytes = (*self.native).bytes();
            slice::from_raw_parts(bytes, self.size())
        }
    }

    pub fn size(&self) -> usize {
        unsafe { (*self.native).size() }
    }
}


pub struct Canvas<'a> {
    native: *mut SkCanvas,
    phantom: PhantomData<&'a SkCanvas>
}

impl<'a> Canvas<'a> {

    #[inline]
    pub fn clear(&mut self, color: SkColor) {
        unsafe { (*self.native).clear(color) }
    }

    pub fn save(&mut self) -> i32 {
        unsafe { (*self.native).save() }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        unsafe { (*self.native).translate(dx, dy) }
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        unsafe { (*self.native).scale(sx, sy) }
    }

    pub fn draw_path(&mut self, path: &Path, paint: &Paint) {
        unsafe { (*self.native).drawPath(&path.native, &paint.native) }
    }
}

pub struct Path {
    native: SkPath
}

impl Path {
    pub fn new() -> Path {
        Path { native: unsafe { SkPath::new() } }
    }

    pub fn move_to(&mut self, x: f32, y: f32) -> &Path {
        unsafe { self.native.moveTo(x, y); }
        self
    }

    pub fn line_to(&mut self, x: f32, y: f32) -> &Path {
        unsafe { self.native.lineTo(x, y); }
        self
    }

    pub fn quad_to(&mut self, x: f32, y: f32, x2: f32, y2: f32) -> &Path {
        unsafe { self.native.quadTo(x, y, x2, y2); }
        self
    }

    pub fn cubic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> &Path {
        unsafe { self.native.cubicTo(x1, y1, x2, y2, x3, y3); }
        self
    }

    pub fn close(&mut self) -> &Path {
        unsafe { self.native.close(); }
        self
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { C_SkPath_destruct(&self.native) }
    }
}

pub struct Paint {
    native: SkPaint
}

impl Paint {
    pub fn new() -> Paint {
        Paint { native: unsafe { SkPaint::new() }}
    }

    pub fn set_color(&mut self, color: SkColor) {
        unsafe { self.native.setColor(color) }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) {
        unsafe { self.native.setAntiAlias(anti_alias) }
    }

    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe { self.native.setStrokeWidth(width) }
    }

    pub fn set_style(&mut self, style: SkPaint_Style) {
        unsafe { self.native.setStyle(style) }
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        unsafe { C_SkPaint_destruct(&self.native) }
    }
}
