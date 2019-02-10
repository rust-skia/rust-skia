use rust_skia::{SkPath, C_SkPath_destruct};

#[derive(Debug)]
pub struct Path {
    pub (crate) native: SkPath
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { C_SkPath_destruct(&self.native) }
    }
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
