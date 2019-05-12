use crate::prelude::*;
use crate::{Canvas, Font, Paint, Path, Point, TextEncoding};
use skia_bindings::{SkTextUtils, SkTextUtils_Align};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Align {
    Left = SkTextUtils_Align::kLeft_Align as _,
    Center = SkTextUtils_Align::kCenter_Align as _,
    Right = SkTextUtils_Align::kRight_Align as _,
}

impl NativeTransmutable<SkTextUtils_Align> for Align {}
#[test]
fn test_align_layout() {
    Align::test_layout()
}

pub fn draw_str(
    mut canvas: impl AsMut<Canvas>,
    text: impl AsRef<str>,
    p: impl Into<Point>,
    font: &Font,
    paint: &Paint,
    align: Align,
) {
    let text = text.as_ref().as_bytes();
    let p = p.into();
    unsafe {
        SkTextUtils::Draw(
            canvas.as_mut().native_mut(),
            text.as_ptr() as _,
            text.len(),
            TextEncoding::UTF8.into_native(),
            p.x,
            p.y,
            font.native(),
            paint.native(),
            align.into_native(),
        )
    }
}

impl Canvas {
    pub fn draw_str_align(
        &mut self,
        text: impl AsRef<str>,
        p: impl Into<Point>,
        font: &Font,
        paint: &Paint,
        align: Align,
    ) -> &mut Self {
        draw_str(self.as_mut(), text, p, font, paint, align);
        self
    }
}

pub fn get_path(text: impl AsRef<str>, p: impl Into<Point>, font: &Font) -> Path {
    let text = text.as_ref().as_bytes();
    let p = p.into();
    let mut path = Path::default();
    unsafe {
        SkTextUtils::GetPath(
            text.as_ptr() as _,
            text.len(),
            TextEncoding::UTF8.into_native(),
            p.x,
            p.y,
            font.native(),
            path.native_mut(),
        )
    }
    path
}

impl Path {
    pub fn from_str(text: impl AsRef<str>, p: impl Into<Point>, font: &Font) -> Self {
        get_path(text, p, font)
    }
}
