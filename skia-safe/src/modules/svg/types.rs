mod aspect_ratio;
mod color;
mod fe;
mod font;
mod iri;
mod length;
mod paint;

pub use self::{aspect_ratio::*, color::*, fe::*, font::*, iri::*, length::*, paint::*};

use skia_bindings as sb;

pub type SvgFillRule = sb::SkSVGFillRule_Type;
pub type SvgColorSpace = sb::SkSVGColorspace;
pub type SvgDisplay = sb::SkSVGDisplay;
pub type SvgLineCap = sb::SkSVGLineCap;
pub type SvgVisibility = sb::SkSVGVisibility_Type;
pub type SvgLineJoin = sb::SkSVGLineJoin_Type;
pub type SvgTextAnchor = sb::SkSVGTextAnchor_Type;
pub type SvgBoundingBoxUnits = sb::SkSVGObjectBoundingBoxUnits_Type;
pub type SvgSpreadMethod = sb::SkSVGSpreadMethod_Type;
