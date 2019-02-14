use rust_skia::*;

#[derive(Copy, Clone)]
pub struct Color(pub(crate) SkColor);

impl Color {

    #[inline]
    pub fn from_argb(a: u8, r: u8, g: u8, b:u8) -> Color {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Self::from_argb(0xff, r, g, b)
    }

    #[inline]
    pub fn a(&self) -> u8 {
        (self.0 >> 24) as _
    }

    #[inline]
    pub fn r(&self) -> u8 {
        (self.0 >> 16) as _
    }

    #[inline]
    pub fn g(&self) -> u8 {
        (self.0 >> 8) as _
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.0 as _
    }

    #[inline]
    pub fn set_a(&self, a: u8) -> Self {
        Self::from_argb(a, self.r(), self.g(), self.b())
    }

    pub const TRANSPARENT: Color = Color(SK_ColorTRANSPARENT);
    pub const BLACK: Color = Color(SK_ColorBLACK);
    pub const DARK_GRAY: Color = Color(SK_ColorDKGRAY);
    pub const GRAY: Color = Color(SK_ColorLTGRAY);
    pub const LIGHT_GRAY: Color = Color(SK_ColorLTGRAY);
    pub const WHITE: Color = Color(SK_ColorWHITE);
    pub const RED: Color = Color(SK_ColorRED);
    pub const GREEN: Color = Color(SK_ColorGREEN);
    pub const BLUE: Color = Color(SK_ColorBLUE);
    pub const YELLOW: Color = Color(SK_ColorYELLOW);
    pub const CYAN: Color = Color(SK_ColorCYAN);
    pub const MAGENTA: Color = Color(SK_ColorMAGENTA);
}
