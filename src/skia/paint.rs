use crate::prelude::*;
use crate::skia::Color;
use rust_skia::{
    SkPaint_Cap,
    SkPaint,
    C_SkPaint_destruct,
    SkPaint_Style,
    SkPaint_Flags,
    SkPaint_Join
};
use rust_skia::C_SkPaint_Equals;
use std::hash::{
    Hash,
    Hasher
};

pub type PaintFlags = EnumHandle<SkPaint_Flags>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPaint_Flags> {
    pub const AntiAlias: Self = Self(SkPaint_Flags::kAntiAlias_Flag);
    pub const Dither: Self = Self(SkPaint_Flags::kDither_Flag);
    pub const FakeBoldText: Self = Self(SkPaint_Flags::kFakeBoldText_Flag);
    pub const LinearText: Self = Self(SkPaint_Flags::kLinearText_Flag);
    pub const SubpixelText: Self = Self(SkPaint_Flags::kSubpixelText_Flag);
    pub const LCDRenderText: Self = Self(SkPaint_Flags::kLCDRenderText_Flag);
    pub const EmbeddedBitmapText: Self = Self(SkPaint_Flags::kEmbeddedBitmapText_Flag);
    pub const AutoHinting: Self = Self(SkPaint_Flags::kAutoHinting_Flag);
}

pub type PaintStyle = EnumHandle<SkPaint_Style>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPaint_Style> {
    pub const Stroke: Self = Self(SkPaint_Style::kStroke_Style);
    pub const Fill: Self = Self(SkPaint_Style::kFill_Style);
    pub const StrokeAndFill: Self = Self(SkPaint_Style::kStrokeAndFill_Style);
}

pub type PaintCap = EnumHandle<SkPaint_Cap>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPaint_Cap> {
    pub const Butt: Self = Self(SkPaint_Cap::kButt_Cap);
    pub const Round: Self = Self(SkPaint_Cap::kRound_Cap);
    pub const Square: Self = Self(SkPaint_Cap::kSquare_Cap);
}

impl Default for EnumHandle<SkPaint_Cap> {
    fn default() -> Self {
        Self(SkPaint_Cap::kDefault_Cap)
    }
}

pub type PaintJoin = EnumHandle<SkPaint_Join>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkPaint_Join> {
    pub const Miter: Self = Self(SkPaint_Join::kMiter_Join);
    pub const Round: Self = Self(SkPaint_Join::kRound_Join);
    pub const Bevel: Self = Self(SkPaint_Join::kBevel_Join);
}

impl Default for EnumHandle<SkPaint_Join> {
    fn default() -> Self {
        Self(SkPaint_Join::kDefault_Join)
    }
}

pub type Paint = Handle<SkPaint>;

impl NativeDrop for SkPaint {
    fn drop(&mut self) {
        unsafe { C_SkPaint_destruct(self) }
    }
}

impl NativeClone for SkPaint {
    fn clone(&self) -> Self {
        unsafe { SkPaint::new1(self) }
    }
}

impl NativePartialEq for SkPaint {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkPaint_Equals(self, rhs) }
    }
}

impl NativeHash for SkPaint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { self.getHash() }.hash(state)
    }
}

impl Default for Handle<SkPaint> {
    fn default() -> Self {
        Paint::from_native(unsafe { SkPaint::new() })
    }
}

impl Handle<SkPaint> {

    pub fn set_color(&mut self, color: Color) {
        unsafe { self.native_mut().setColor(color.into_native()) }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) {
        unsafe { self.native_mut().setAntiAlias(anti_alias) }
    }

    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe { self.native_mut().setStrokeWidth(width) }
    }

    pub fn set_style(&mut self, style: PaintStyle) {
        unsafe { self.native_mut().setStyle(style.0) }
    }
}

