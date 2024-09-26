use crate::interop::AsStr;
use skia_bindings as sb;
use std::fmt;

pub type SvgFeInputType = sb::SkSVGFeInputType_Type;

#[repr(C)]
pub struct SvgFeInput {
    kind: SvgFeInputType,
    id: sb::SkString,
}

native_transmutable!(sb::SkSVGFeInputType, SvgFeInput, svg_fe_input_layout);

impl fmt::Debug for SvgFeInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgFeInput")
            .field("kind", &self.kind())
            .field("id", &self.id())
            .finish()
    }
}

impl SvgFeInput {
    pub fn kind(&self) -> SvgFeInputType {
        self.kind
    }

    pub fn id(&self) -> Option<&str> {
        if self.kind == SvgFeInputType::FilterPrimitiveReference {
            Some(self.id.as_str())
        } else {
            None
        }
    }

    pub fn source_graphic() -> Self {
        Self {
            kind: SvgFeInputType::SourceGraphic,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn source_alpha() -> Self {
        Self {
            kind: SvgFeInputType::SourceAlpha,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn background_image() -> Self {
        Self {
            kind: SvgFeInputType::BackgroundImage,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn background_alpha() -> Self {
        Self {
            kind: SvgFeInputType::BackgroundAlpha,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn fill_paint() -> Self {
        Self {
            kind: SvgFeInputType::FillPaint,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn stroke_paint() -> Self {
        Self {
            kind: SvgFeInputType::StrokePaint,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn unspecified() -> Self {
        Self {
            kind: SvgFeInputType::Unspecified,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self {
            kind: SvgFeInputType::FilterPrimitiveReference,
            id: crate::interop::String::from_str(id.as_ref()).into_native(),
        }
    }
}
