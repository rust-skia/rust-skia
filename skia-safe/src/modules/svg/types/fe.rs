use crate::interop::AsStr;
use skia_bindings as sb;
use std::fmt;

pub type FeInputType = sb::SkSVGFeInputType_Type;

#[repr(C)]
pub struct FeInput {
    kind: FeInputType,
    id: sb::SkString,
}

native_transmutable!(sb::SkSVGFeInputType, FeInput, svg_fe_input_layout);

impl fmt::Debug for FeInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgFeInput")
            .field("kind", &self.kind())
            .field("id", &self.id())
            .finish()
    }
}

impl FeInput {
    pub fn kind(&self) -> FeInputType {
        self.kind
    }

    pub fn id(&self) -> Option<&str> {
        if self.kind == FeInputType::FilterPrimitiveReference {
            Some(self.id.as_str())
        } else {
            None
        }
    }

    pub fn source_graphic() -> Self {
        Self {
            kind: FeInputType::SourceGraphic,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn source_alpha() -> Self {
        Self {
            kind: FeInputType::SourceAlpha,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn background_image() -> Self {
        Self {
            kind: FeInputType::BackgroundImage,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn background_alpha() -> Self {
        Self {
            kind: FeInputType::BackgroundAlpha,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn fill_paint() -> Self {
        Self {
            kind: FeInputType::FillPaint,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn stroke_paint() -> Self {
        Self {
            kind: FeInputType::StrokePaint,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn unspecified() -> Self {
        Self {
            kind: FeInputType::Unspecified,
            id: crate::interop::String::default().into_native(),
        }
    }

    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self {
            kind: FeInputType::FilterPrimitiveReference,
            id: crate::interop::String::from_str(id.as_ref()).into_native(),
        }
    }
}
