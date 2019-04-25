use crate::prelude::*;
use crate::core::u8cpu;
use std::ops::{Index, Mul, IndexMut, BitOr, BitAnd};
use skia_bindings::{
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
#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[repr(transparent)]
pub struct Color(SkColor);

impl NativeTransmutable<SkColor> for Color {}
#[test]
fn test_color_layout() {
    Color::test_layout();
}

impl From<u32> for Color {
    fn from(c: u32) -> Self {
        Color(c)
    }
}

impl From<RGB> for Color {
    fn from(rgb: RGB) -> Self {
        Color::from_rgb(rgb.r, rgb.g, rgb.b)
    }
}

//
// Bitwise operators.
//

impl BitOr for Color {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Color::from_native(self.native() | rhs.native())
    }
}

impl BitAnd for Color {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Color::from_native(self.native() & rhs.native())
    }
}

impl BitOr<u32> for Color {
    type Output = Self;

    fn bitor(self, rhs: u32) -> Self::Output {
        self | (Color::from_native(rhs))
    }
}

impl BitAnd<u32> for Color {
    type Output = Self;

    fn bitand(self, rhs: u32) -> Self::Output {
        self & (Color::from_native(rhs))
    }
}

impl Color {

    // note: we don't use the u8cpu type here, because we trust the rust
    // compiler to optimize the storage type.

    #[inline]
    pub fn from_argb(a: u8, r: u8, g: u8, b:u8) -> Color {
        Self(
            (u8cpu::from(a) << 24) |
            (u8cpu::from(r) << 16) |
            (u8cpu::from(g) << 8) |
            (u8cpu::from(b))
        )
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Self::from_argb(0xff, r, g, b)
    }

    #[inline]
    pub fn a(self) -> u8 {
        (self.into_native() >> 24) as _
    }

    #[inline]
    pub fn r(self) -> u8 {
        (self.into_native() >> 16) as _
    }

    #[inline]
    pub fn g(self) -> u8 {
        (self.into_native() >> 8) as _
    }

    #[inline]
    pub fn b(self) -> u8 {
        self.into_native() as _
    }

    #[inline]
    #[must_use]
    pub fn with_a(self, a: u8) -> Self {
        Self::from_argb(a, self.r(), self.g(), self.b())
    }

    pub const TRANSPARENT: Self = Self(skia_bindings::SK_ColorTRANSPARENT);
    pub const BLACK: Self = Self(skia_bindings::SK_ColorBLACK);
    pub const DARK_GRAY: Self = Self(skia_bindings::SK_ColorDKGRAY);
    pub const GRAY: Self = Self(skia_bindings::SK_ColorLTGRAY);
    pub const LIGHT_GRAY: Self = Self(skia_bindings::SK_ColorLTGRAY);
    pub const WHITE: Self = Self(skia_bindings::SK_ColorWHITE);
    pub const RED: Self = Self(skia_bindings::SK_ColorRED);
    pub const GREEN: Self = Self(skia_bindings::SK_ColorGREEN);
    pub const BLUE: Self = Self(skia_bindings::SK_ColorBLUE);
    pub const YELLOW: Self = Self(skia_bindings::SK_ColorYELLOW);
    pub const CYAN: Self = Self(skia_bindings::SK_ColorCYAN);
    pub const MAGENTA: Self = Self(skia_bindings::SK_ColorMAGENTA);

    pub fn to_rgb(self) -> RGB {
        (self.r(), self.g(), self.b()).into()
    }

    pub fn to_hsv(self) -> HSV {
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
    pub fn to_hsv(self) -> HSV {
        let mut hsv: [f32; 3] = Default::default();
        unsafe {
            SkRGBToHSV(
                self.r.into(),
                self.g.into(),
                self.b.into(),
                hsv.as_mut_ptr());
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
            SkHSVToColor(alpha.into(), [self.h, self.s, self.v].as_ptr())
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
#[test]
fn test_color4f_layout() {
    Color4f::test_layout();
}

impl AsRef<Color4f> for Color4f {
    fn as_ref(&self) -> &Self {
        self
    }
}

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
            (f32::from(c)) * (1.0 / 255.0)
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
        unsafe { transmute_ref(self) }
    }

    pub fn vec_mut(&mut self) -> &mut[f32; 4] {
        unsafe { transmute_ref_mut(self) }
    }

    #[allow(clippy::float_cmp)]
    pub fn is_opaque(&self) -> bool {
        self.a == 1.0
    }

    // TODO: this is the copied implementation, it would probably be better to call the Skia function.
    pub fn fits_in_bytes(&self) -> bool {
        debug_assert!(self.a >= 0.0 && self.a <= 1.0);
        self.r >= 0.0 && self.r <= 1.0 &&
        self.g >= 0.0 && self.g <= 1.0 &&
        self.b >= 0.0 && self.b <= 1.0
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
    assert_eq!(c, c2);
}
