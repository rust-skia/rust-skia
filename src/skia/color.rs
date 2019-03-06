use crate::prelude::*;
use crate::skia::u8cpu;
use std::mem;
use std::ops::{Index,Mul,IndexMut};
use rust_skia::{
    SkColor,
    SkRGBToHSV,
    SkHSVToColor,
    SkColor4f
};

// TODO: What should we do with SkAlpha?
// It does not seem to be used, but if we want to export it, we'd
// like to define Alpha::TRANSPARENT and Alpha::OPAQUE.
// pub type Alpha = u8;

// note: SkColor _is_ a u32, and therefore its components are
// endian dependent, so we can't expose it as (transmuted) fields.
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Color(SkColor);

impl NativeTransmutable<SkColor> for Color {}

#[test]
fn test_layout() {
    Color::test_layout();
}

impl From<RGB> for Color {
    fn from(rgb: RGB) -> Self {
        Color::from_rgb(rgb.r, rgb.g, rgb.b)
    }
}

impl Color {

    // note: we don't use the u8cpu type here, because we trust the rust
    // compiler to optimize the storage type.

    #[inline]
    pub fn from_argb(a: u8, r: u8, g: u8, b:u8) -> Color {
        Self(((a as u8cpu) << 24) | ((r as u8cpu) << 16) | ((g as u8cpu) << 8) | (b as u8cpu))
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Self::from_argb(0xff, r, g, b)
    }

    #[inline]
    pub fn a(&self) -> u8 {
        (self.into_native() >> 24) as _
    }

    #[inline]
    pub fn r(&self) -> u8 {
        (self.into_native() >> 16) as _
    }

    #[inline]
    pub fn g(&self) -> u8 {
        (self.into_native() >> 8) as _
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.into_native() as _
    }

    #[inline]
    #[warn(unused)]
    pub fn with_a(&self, a: u8) -> Self {
        Self::from_argb(a, self.r(), self.g(), self.b())
    }

    pub const BLACK: Self = Self(rust_skia::SK_ColorBLACK);
    pub const DARK_GRAY: Self = Self(rust_skia::SK_ColorDKGRAY);
    pub const GRAY: Self = Self(rust_skia::SK_ColorLTGRAY);
    pub const LIGHT_GRAY: Self = Self(rust_skia::SK_ColorLTGRAY);
    pub const WHITE: Self = Self(rust_skia::SK_ColorWHITE);
    pub const RED: Self = Self(rust_skia::SK_ColorRED);
    pub const GREEN: Self = Self(rust_skia::SK_ColorGREEN);
    pub const BLUE: Self = Self(rust_skia::SK_ColorBLUE);
    pub const YELLOW: Self = Self(rust_skia::SK_ColorYELLOW);
    pub const CYAN: Self = Self(rust_skia::SK_ColorCYAN);
    pub const MAGENTA: Self = Self(rust_skia::SK_ColorMAGENTA);

    pub fn to_rgb(&self) -> RGB {
        (self.r(), self.g(), self.b()).into()
    }

    pub fn to_hsv(&self) -> HSV {
        self.to_rgb().to_hsv()
    }

}

#[derive(Copy, Clone, PartialEq)]
pub struct RGB { pub r: u8, pub g: u8, pub b: u8 }

impl From<(u8, u8, u8)> for RGB {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self{ r, g, b }
    }
}

impl RGB {
    pub fn to_hsv(&self) -> HSV {
        let mut hsv: [f32; 3] = Default::default();
        unsafe {
            SkRGBToHSV(self.r as u8cpu, self.g as u8cpu, self.b as u8cpu, hsv.as_mut_ptr());
        }
        HSV { h: hsv[0], s: hsv[1], v: hsv[2] }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct HSV { pub h: f32, pub s: f32, pub v: f32 }

impl From<(f32, f32, f32)> for HSV {
    fn from((h, s, v): (f32, f32, f32)) -> Self {
        Self{ h, s, v }
    }
}

impl HSV {
    pub fn to_color(&self, alpha: u8) -> Color {
        Color::from_native(unsafe {
            SkHSVToColor(alpha as u8cpu, [self.h, self.s, self.v].as_ptr())
        })
    }
}

// TODO: What should we do about PMColor, is it needed?
// pub struct PMColor(SkPMColor);

// decided not to directly support SkRGBA4f for now because of the
// lack of const generics.

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Color4f { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }

impl NativeTransmutable<SkColor4f> for Color4f {}

impl Mul<f32> for Color4f {
    type Output = Color4f;

    fn mul(self, scale: f32) -> Self::Output {
        let r = self.r * scale;
        let g = self.g * scale;
        let b = self.b * scale;
        let a = self.a * scale;
        Color4f { r, g, b, a }
    }
}

impl Index<usize> for Color4f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec()[index]
    }
}

impl IndexMut<usize> for Color4f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec_mut()[index]
    }
}

impl From<Color> for Color4f {
    fn from(color: Color) -> Self {
        fn c(c: u8) -> f32 {
            (c as f32) * (1.0 / 255.0)
        }
        let r = c(color.r());
        let g = c(color.g());
        let b = c(color.b());
        let a = c(color.a());
        Color4f { r, g, b, a }
    }
}

impl Color4f {

    pub fn vec(&self) -> &[f32; 4] {
        unsafe {
            mem::transmute::<&Self, &[f32; 4]>(self)
        }
    }

    pub fn vec_mut(&mut self) -> &mut[f32; 4] {
        unsafe { mem::transmute::<&mut Self, &mut [f32; 4]>(self) }
    }

    pub fn is_opaque(&self) -> bool {
        self.a == 1.0
    }

    pub fn to_color(&self) -> Color {
        fn c(f: f32) -> u8 {
            (f.max(0.0).min(1.0) * 255.0) as u8
        }
        let a = c(self.a);
        let r = c(self.r);
        let g = c(self.g);
        let b = c(self.b);
        Color::from_argb(a, r, g, b)
    }

    pub fn make_opaque(&self) -> Self {
        Self { a: 1.0, .. *self }
    }
}

#[test]
pub fn color4f_array_access() {
    let mut color = Color4f { r: 0.1, g: 0.2, b: 0.3, a: 0.4 };
    color[1] = 0.5;
    assert_eq!(0.5, color.g);
}

#[test]
pub fn color_color4f_conversion() {
    let c = Color::from_argb(1, 2, 3, 4);
    let cf = Color4f::from(c);
    let c2 = cf.to_color();
    assert!(c == c2);
}