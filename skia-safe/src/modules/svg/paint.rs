use super::color::SvgColor;
use crate::{prelude::*, Color};
use skia_bindings as sb;
use std::fmt;

pub type SvgPaint = Handle<sb::SkSVGPaint>;

unsafe_send_sync!(SvgPaint);

impl NativeDrop for sb::SkSVGPaint {
    fn drop(&mut self) {
        unsafe { sb::C_SkSVGPaint_destruct(self) }
    }
}

impl fmt::Debug for SvgPaint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_color() {
            f.debug_tuple("SvgPaint::Color")
                .field(&self.color().unwrap())
                .finish()
        } else if self.is_current_color() {
            f.debug_tuple("SvgPaint::CurrentColor").finish()
        } else {
            f.debug_tuple("SvgPaint::None").finish()
        }
    }
}

impl SvgPaint {
    pub fn color(&self) -> Option<Color> {
        let paint = self.native();

        if self.is_color() {
            Some(Color::new(paint.fColor.fColor))
        } else {
            None
        }
    }

    pub fn is_current_color(&self) -> bool {
        matches!(self.native().fType, sb::SkSVGPaint_Type::Color)
            && matches!(
                self.native().fColor.fType,
                sb::SkSVGColor_Type::CurrentColor
            )
    }

    pub fn is_color(&self) -> bool {
        matches!(self.native().fType, sb::SkSVGPaint_Type::Color)
            && matches!(self.native().fColor.fType, sb::SkSVGColor_Type::Color)
    }

    pub fn is_none(&self) -> bool {
        matches!(self.native().fType, sb::SkSVGPaint_Type::None)
    }

    pub fn from_color(color: Color) -> Self {
        Self::construct(|uninitialized| unsafe {
            let color = SvgColor::from_color(color);

            sb::C_SkSVGPaint_Color(uninitialized, color.native())
        })
    }

    pub fn current_color() -> Self {
        Self::construct(|uninitialized| unsafe {
            let color = SvgColor::current_color();

            sb::C_SkSVGPaint_Color(uninitialized, color.native())
        })
    }

    pub fn none() -> Self {
        Self::construct(|uninitialized| unsafe { sb::C_SkSVGPaint_None(uninitialized) })
    }
}
