use rust_skia::{SkRect};

pub struct Rect {
    pub(crate) native: SkRect
}

impl Rect {

    pub fn new_wh(w: f32, h: f32) -> Rect {
        Rect::new_ltbr(0., 0., w, h)
    }

    pub fn new_iwh(w: i32, h: i32) -> Rect {
        Rect::new_wh(w as f32, h as f32)
    }

    pub fn new_empty() -> Rect {
        Rect::new_ltbr(0., 0., 0., 0.)
    }

    pub fn new_ltbr(l: f32, t: f32, r: f32, b: f32) -> Rect {
        Rect {
            native : SkRect {
                fLeft: l,
                fTop: t,
                fRight: r,
                fBottom: b
            }
        }
    }

    pub fn left(&self) -> f32 { self.native.fLeft }
    pub fn top(&self) -> f32 { self.native.fTop }
    pub fn right(&self) -> f32 { self.native.fRight }
    pub fn bottom(&self) -> f32 { self.native.fBottom }
}

