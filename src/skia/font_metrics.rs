use crate::prelude::*;
use rust_skia::{
    SkFontMetrics,
    SkFontMetrics_FontMetricsFlags
};

bitflags! {
    struct FontMetricsFlags: u32 {
        const UnderlineThicknessIsValid = SkFontMetrics_FontMetricsFlags::kUnderlineThicknessIsValid_Flag as _;
        const UnderlinePositionIsValid = SkFontMetrics_FontMetricsFlags::kUnderlinePositionIsValid_Flag as _;
        const StrikeoutThicknessIsValid = SkFontMetrics_FontMetricsFlags::kStrikeoutThicknessIsValid_Flag as _;
        const StrikeoutPositionIsValid = SkFontMetrics_FontMetricsFlags::kStrikeoutPositionIsValid_Flag as _;
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FontMetrics {
    flags: FontMetricsFlags,
    pub top: f32,
    pub ascent: f32,
    pub descent: f32,
    pub bottom: f32,
    pub leading: f32,
    pub avg_char_width: f32,
    pub max_char_width: f32,
    pub x_min: f32,
    pub x_max: f32,
    pub x_height: f32,
    pub cap_height: f32,
    underline_thickness: f32,
    underline_position: f32,
    strikeout_thickness: f32,
    strikeout_position: f32
}

impl NativeTransmutable<SkFontMetrics> for FontMetrics {}

impl FontMetrics {

    // the following 4 functions do not link.

    pub fn underline_thickness(&self) -> Option<f32> {
        self.if_valid(
            FontMetricsFlags::UnderlineThicknessIsValid,
            self.underline_thickness)
    }

    pub fn underline_position(&self) -> Option<f32> {
        self.if_valid(
            FontMetricsFlags::UnderlinePositionIsValid,
            self.underline_position)
    }

    pub fn strikeout_thickness(&self) -> Option<f32> {
        self.if_valid(
            FontMetricsFlags::StrikeoutThicknessIsValid,
            self.strikeout_thickness)
    }

    pub fn strikeout_position(&self) -> Option<f32> {
        self.if_valid(
            FontMetricsFlags::StrikeoutPositionIsValid,
            self.strikeout_position)
    }

    fn if_valid(&self, flag: FontMetricsFlags, value: f32) -> Option<f32> {
        if self.flags.contains(flag) {
            Some(value)
        } else {
            None
        }
    }
}

#[test]
fn test_layout() {
    FontMetrics::test_layout();
}
