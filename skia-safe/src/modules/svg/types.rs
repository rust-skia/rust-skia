mod color;
mod font;
mod iri;
mod length;
mod paint;
pub mod preserve_aspect_ratio;

pub use self::{
    color::*, font::*, iri::*, length::*, paint::*, preserve_aspect_ratio::PreserveAspectRatio,
};

use skia_bindings as sb;

pub type FillRule = sb::SkSVGFillRule_Type;
pub type ColorSpace = sb::SkSVGColorspace;
pub type Display = sb::SkSVGDisplay;
pub type LineCap = sb::SkSVGLineCap;
pub type Visibility = sb::SkSVGVisibility_Type;
pub type LineJoin = sb::SkSVGLineJoin_Type;
pub type TextAnchor = sb::SkSVGTextAnchor_Type;
pub type BoundingBoxUnits = sb::SkSVGObjectBoundingBoxUnits_Type;
pub type SpreadMethod = sb::SkSVGSpreadMethod_Type;
pub type XmlSpace = sb::SkSVGXmlSpace;
