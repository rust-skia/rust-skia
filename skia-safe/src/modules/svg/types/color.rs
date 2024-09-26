use std::fmt;

use crate::{prelude::*, Color};
use skia_bindings as sb;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SvgColorKind {
    CurrentColor,
    Color,
}

pub type SvgColor = Handle<sb::SkSVGColor>;

unsafe_send_sync!(SvgColor);

impl NativeDrop for sb::SkSVGColor {
    fn drop(&mut self) {
        unsafe { sb::C_SkSVGColor_destruct(self) }
    }
}

impl fmt::Debug for SvgColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgColor")
            .field("kind", &self.kind())
            .field("color", &self.color())
            .finish()
    }
}

impl SvgColor {
    pub fn color(&self) -> Option<Color> {
        let color = self.native();

        if matches!(color.fType, sb::SkSVGColor_Type::Color) {
            Some(Color::new(color.fColor))
        } else {
            None
        }
    }

    pub fn kind(&self) -> SvgColorKind {
        match &self.native().fType {
            sb::SkSVGColor_Type::Color => SvgColorKind::Color,
            sb::SkSVGColor_Type::CurrentColor | sb::SkSVGColor_Type::ICCColor => {
                SvgColorKind::CurrentColor
            }
        }
    }

    pub fn current_color() -> Self {
        Self::construct(|uninitialized| unsafe { sb::C_SkSVGColor_CurrentColor(uninitialized) })
    }

    pub fn from_color(color: Color) -> Self {
        Self::construct(|uninitialized| unsafe {
            sb::C_SkSVGColor_Color(uninitialized, color.into_native())
        })
    }
}
