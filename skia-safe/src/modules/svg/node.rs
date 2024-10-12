use super::{
    element::Svg, fe, pattern::Pattern, Circle, ClipPath, ColorSpace, DebugAttributes, Defs,
    Display, Ellipse, Fill, FillRule, Filter, FontFamily, FontSize, FontStyle, FontWeight, Image,
    IriFunc, Length, Line, LineCap, LineJoin, LinearGradient, Mask, Paint, Path, Poly,
    RadialGradient, Rect, Stop, TSpan, Text, TextAnchor, TextLiteral, TextPath, Use, Visibility, G,
};
use crate::{prelude::*, scalar, Color};
use skia_bindings as sb;

#[derive(Debug)]
pub enum Node {
    Circle(Circle),
    ClipPath(ClipPath),
    Defs(Defs),
    Ellipse(Ellipse),
    FeBlend(fe::Blend),
    FeColorMatrix(fe::ColorMatrix),
    FeComponentTransfer(fe::ComponentTransfer),
    FeComposite(fe::Composite),
    FeDiffuseLighting(fe::DiffuseLighting),
    FeDisplacementMap(fe::DisplacementMap),
    FeDistantLight(fe::DistantLight),
    FeFlood(fe::Flood),
    FeFuncA(fe::Func),
    FeFuncR(fe::Func),
    FeFuncG(fe::Func),
    FeFuncB(fe::Func),
    FeGaussianBlur(fe::GaussianBlur),
    FeImage(fe::Image),
    FeMerge(fe::Merge),
    FeMergeNode(fe::MergeNode),
    FeMorphology(fe::Morphology),
    FeOffset(fe::Offset),
    FePointLight(fe::PointLight),
    FeSpecularLighting(fe::SpecularLighting),
    FeSpotLight(fe::SpotLight),
    FeTurbulence(fe::Turbulence),
    Filter(Filter),
    G(G),
    Image(Image),
    Line(Line),
    LinearGradient(LinearGradient),
    Mask(Mask),
    Path(Path),
    Pattern(Pattern),
    Polygon(Poly),
    Polyline(Poly),
    RadialGradient(RadialGradient),
    Rect(Rect),
    Stop(Stop),
    Svg(Svg),
    Text(Text),
    TextLiteral(TextLiteral),
    TextPath(TextPath),
    TSpan(TSpan),
    Use(Use),
}

impl Node {
    pub fn from_unshared_ptr(ptr: *mut sb::SkSVGNode) -> Option<Self> {
        let tag = unsafe { sb::C_SkSVGNode_tag(ptr as *const _) };

        Some(match tag {
            NodeTag::Circle => Self::Circle(Circle::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::ClipPath => Self::ClipPath(ClipPath::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Defs => Self::Defs(Defs::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Ellipse => Self::Ellipse(Ellipse::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeBlend => Self::FeBlend(fe::Blend::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeColorMatrix => {
                Self::FeColorMatrix(fe::ColorMatrix::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeComponentTransfer => {
                Self::FeComponentTransfer(fe::ComponentTransfer::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeComposite => {
                Self::FeComposite(fe::Composite::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeDiffuseLighting => {
                Self::FeDiffuseLighting(fe::DiffuseLighting::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeDisplacementMap => {
                Self::FeDisplacementMap(fe::DisplacementMap::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeDistantLight => {
                Self::FeDistantLight(fe::DistantLight::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeFlood => Self::FeFlood(fe::Flood::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeFuncA => Self::FeFuncA(fe::Func::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeFuncR => Self::FeFuncR(fe::Func::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeFuncG => Self::FeFuncG(fe::Func::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeFuncB => Self::FeFuncB(fe::Func::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeGaussianBlur => {
                Self::FeGaussianBlur(fe::GaussianBlur::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeImage => Self::FeImage(fe::Image::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeMerge => Self::FeMerge(fe::Merge::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeMergeNode => {
                Self::FeMergeNode(fe::MergeNode::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeMorphology => {
                Self::FeMorphology(fe::Morphology::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeOffset => Self::FeOffset(fe::Offset::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FePointLight => {
                Self::FePointLight(fe::PointLight::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeSpecularLighting => {
                Self::FeSpecularLighting(fe::SpecularLighting::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeSpotLight => {
                Self::FeSpotLight(fe::SpotLight::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeTurbulence => {
                Self::FeTurbulence(fe::Turbulence::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::Filter => Self::Filter(Filter::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::G => Self::G(G::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Image => Self::Image(Image::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Line => Self::Line(Line::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::LinearGradient => {
                Self::LinearGradient(LinearGradient::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::Mask => Self::Mask(Mask::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Path => Self::Path(Path::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Pattern => Self::Pattern(Pattern::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Polygon => Self::Polygon(Poly::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Polyline => Self::Polyline(Poly::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::RadialGradient => {
                Self::RadialGradient(RadialGradient::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::Rect => Self::Rect(Rect::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Stop => Self::Stop(Stop::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Svg => Self::Svg(Svg::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Text => Self::Text(Text::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::TextLiteral => {
                Self::TextLiteral(TextLiteral::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::TextPath => Self::TextPath(TextPath::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::TSpan => Self::TSpan(TSpan::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Use => Self::Use(Use::from_unshared_ptr(ptr as *mut _)?),
        })
    }
}

pub type NodeTag = sb::SkSVGTag;

pub type SvgNode = RCHandle<sb::SkSVGNode>;

impl DebugAttributes for SvgNode {
    const NAME: &'static str = "Node";

    fn _dbg(&self, builder: &mut std::fmt::DebugStruct) {
        builder
            .field("clip_rule", &self.get_clip_rule())
            .field("color_interpolation", &self.get_color_interpolation())
            .field(
                "color_interpolation_filters",
                &self.get_color_interpolation_filters(),
            )
            .field("color", &self.get_color())
            .field("fill_rule", &self.get_fill_rule())
            .field("fill", &self.get_fill())
            .field("fill_opacity", &self.get_fill_opacity())
            .field("font_family", &self.get_font_family())
            .field("font_size", &self.get_font_size())
            .field("font_style", &self.get_font_style())
            .field("font_weight", &self.get_font_weight())
            .field("stroke", &self.get_stroke())
            .field("stroke_line_cap", &self.get_stroke_line_cap())
            .field("stroke_line_join", &self.get_stroke_line_join())
            .field("stroke_miter_limit", &self.get_stroke_miter_limit())
            .field("stroke_opacity", &self.get_stroke_opacity())
            .field("stroke_width", &self.get_stroke_width())
            .field("text_anchor", &self.get_text_anchor())
            .field("visibility", &self.get_visibility())
            .field("clip_path", &self.get_clip_path())
            .field("display", &self.get_display())
            .field("mask", &self.get_mask())
            .field("filter", &self.get_filter())
            .field("opacity", &self.get_opacity())
            .field("stop_color", &self.get_stop_color())
            .field("stop_opacity", &self.get_stop_opacity())
            .field("flood_color", &self.get_flood_color())
            .field("flood_opacity", &self.get_flood_opacity())
            .field("lighting_color", &self.get_lighting_color());
    }
}

impl NativeRefCountedBase for sb::SkSVGNode {
    type Base = sb::SkRefCntBase;
}

impl SvgNode {
    pub fn tag(&self) -> NodeTag {
        unsafe { sb::C_SkSVGNode_tag(self.native()) }
    }

    skia_macros::attrs! {
        SkSVGNode => {
            // inherited
            clip_rule?: FillRule [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGFillRule { fType: value }],
            color_interpolation?: ColorSpace [get(value) => value, set(value) => value],
            color_interpolation_filters?: ColorSpace [get(value) => value, set(value) => value],
            color?: Color [get(value) => value.map(Color::from_native_ref), set(value) => value.into_native()],
            fill_rule?: FillRule [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGFillRule { fType: value }],
            fill?: Paint [get(value) => value.map(Paint::from_native_ref), set(value) => value.native()],
            *fill_opacity?: scalar [get(value) => value, set(value) => value],
            font_family?: FontFamily [get(value) => value.map(FontFamily::from_native_ref), set(value) => value.into_native()],
            font_size?: FontSize [get(value) => value.map(FontSize::from_native_ref), set(value) => value.into_native()],
            font_style?: FontStyle [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGFontStyle { fType: value }],
            font_weight?: FontWeight [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGFontWeight { fType: value }],
            stroke?: Paint [get(value) => value.map(Paint::from_native_ref), set(value) => value.native()],
            stroke_line_cap?: LineCap [get(value) => value, set(value) => value],
            stroke_line_join?: LineJoin [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGLineJoin { fType: value }],
            *stroke_miter_limit?: scalar [get(value) => value, set(value) => value],
            *stroke_opacity?: scalar [get(value) => value, set(value) => value],
            stroke_width?: Length [get(value) => value.map(Length::from_native_ref), set(value) => value.into_native()],
            text_anchor?: TextAnchor [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGTextAnchor { fType: value }],
            visibility?: Visibility [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGVisibility { fType: value }],

            // not inherited
            clip_path?: IriFunc [get(value) => value.map(IriFunc::from_native_ref), set(value) => value.native()],
            display?: Display [get(value) => value, set(value) => value],
            mask?: IriFunc [get(value) => value.map(IriFunc::from_native_ref), set(value) => value.native()],
            filter?: IriFunc [get(value) => value.map(IriFunc::from_native_ref), set(value) => value.native()],
            *opacity?: scalar [get(value) => value, set(value) => value],
            stop_color?: Fill [get(value) => value.map(Fill::from_native_ref), set(value) => value.native()],
            *stop_opacity?: scalar [get(value) => value, set(value) => value],
            flood_color?: Fill [get(value) => value.map(Fill::from_native_ref), set(value) => value.native()],
            *flood_opacity?: scalar [get(value) => value, set(value) => value],
            lighting_color?: Fill [get(value) => value.map(Fill::from_native_ref), set(value) => value.native()]
        }
    }
}
