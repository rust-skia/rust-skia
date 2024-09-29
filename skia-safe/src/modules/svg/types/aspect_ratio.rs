use skia_bindings as sb;

pub type SvgAlign = sb::SkSVGPreserveAspectRatio_Align;
pub type SvgScale = sb::SkSVGPreserveAspectRatio_Scale;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SvgPreserveAspectRatio {
    pub align: SvgAlign,
    pub scale: SvgScale,
}

impl SvgPreserveAspectRatio {
    pub fn new(align: SvgAlign, scale: SvgScale) -> Self {
        Self { align, scale }
    }
}

native_transmutable!(
    sb::SkSVGPreserveAspectRatio,
    SvgPreserveAspectRatio,
    svg_preserve_aspect_ratio_layout
);
