use super::{
    element::Svg, pattern::SvgPattern, DebugAttributes, SvgCircle, SvgClipPath, SvgColor,
    SvgColorSpace, SvgDefs, SvgDisplay, SvgEllipse, SvgFeBlend, SvgFeColorMatrix,
    SvgFeComponentTransfer, SvgFeComposite, SvgFeDiffuseLighting, SvgFeDisplacementMap,
    SvgFeDistantLight, SvgFeFlood, SvgFeFunc, SvgFeGaussianBlur, SvgFeImage, SvgFeMerge,
    SvgFeMergeNode, SvgFeMorphology, SvgFeOffset, SvgFePointLight, SvgFeSpecularLighting,
    SvgFeSpotLight, SvgFeTurbulence, SvgFillRule, SvgFilter, SvgFontFamily, SvgFontSize,
    SvgFontStyle, SvgFontWeight, SvgG, SvgImage, SvgIriFunc, SvgLength, SvgLine, SvgLineCap,
    SvgLineJoin, SvgLinearGradient, SvgMask, SvgPaint, SvgPath, SvgPoly, SvgRadialGradient,
    SvgRect, SvgStop, SvgTSpan, SvgText, SvgTextAnchor, SvgTextLiteral, SvgTextPath, SvgUse,
    SvgVisibility,
};
use crate::{prelude::*, scalar, Color};
use skia_bindings as sb;

#[derive(Debug)]
pub enum Node {
    Circle(SvgCircle),
    ClipPath(SvgClipPath),
    Defs(SvgDefs),
    Ellipse(SvgEllipse),
    FeBlend(SvgFeBlend),
    FeColorMatrix(SvgFeColorMatrix),
    FeComponentTransfer(SvgFeComponentTransfer),
    FeComposite(SvgFeComposite),
    FeDiffuseLighting(SvgFeDiffuseLighting),
    FeDisplacementMap(SvgFeDisplacementMap),
    FeDistantLight(SvgFeDistantLight),
    FeFlood(SvgFeFlood),
    FeFuncA(SvgFeFunc),
    FeFuncR(SvgFeFunc),
    FeFuncG(SvgFeFunc),
    FeFuncB(SvgFeFunc),
    FeGaussianBlur(SvgFeGaussianBlur),
    FeImage(SvgFeImage),
    FeMerge(SvgFeMerge),
    FeMergeNode(SvgFeMergeNode),
    FeMorphology(SvgFeMorphology),
    FeOffset(SvgFeOffset),
    FePointLight(SvgFePointLight),
    FeSpecularLighting(SvgFeSpecularLighting),
    FeSpotLight(SvgFeSpotLight),
    FeTurbulence(SvgFeTurbulence),
    Filter(SvgFilter),
    G(SvgG),
    Image(SvgImage),
    Line(SvgLine),
    LinearGradient(SvgLinearGradient),
    Mask(SvgMask),
    Path(SvgPath),
    Pattern(SvgPattern),
    Polygon(SvgPoly),
    Polyline(SvgPoly),
    RadialGradient(SvgRadialGradient),
    Rect(SvgRect),
    Stop(SvgStop),
    Svg(Svg),
    Text(SvgText),
    TextLiteral(SvgTextLiteral),
    TextPath(SvgTextPath),
    TSpan(SvgTSpan),
    Use(SvgUse),
}

impl Node {
    pub fn from_unshared_ptr(ptr: *mut sb::SkSVGNode) -> Option<Self> {
        let tag = unsafe { sb::C_SkSVGNode_tag(ptr as *const _) };

        Some(match tag {
            NodeTag::Circle => Self::Circle(SvgCircle::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::ClipPath => Self::ClipPath(SvgClipPath::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Defs => Self::Defs(SvgDefs::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Ellipse => Self::Ellipse(SvgEllipse::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeBlend => Self::FeBlend(SvgFeBlend::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeColorMatrix => {
                Self::FeColorMatrix(SvgFeColorMatrix::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeComponentTransfer => {
                Self::FeComponentTransfer(SvgFeComponentTransfer::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeComposite => {
                Self::FeComposite(SvgFeComposite::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeDiffuseLighting => {
                Self::FeDiffuseLighting(SvgFeDiffuseLighting::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeDisplacementMap => {
                Self::FeDisplacementMap(SvgFeDisplacementMap::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeDistantLight => {
                Self::FeDistantLight(SvgFeDistantLight::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeFlood => Self::FeFlood(SvgFeFlood::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeFuncA => Self::FeFuncA(SvgFeFunc::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeFuncR => Self::FeFuncR(SvgFeFunc::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeFuncG => Self::FeFuncG(SvgFeFunc::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeFuncB => Self::FeFuncB(SvgFeFunc::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeGaussianBlur => {
                Self::FeGaussianBlur(SvgFeGaussianBlur::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeImage => Self::FeImage(SvgFeImage::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeMerge => Self::FeMerge(SvgFeMerge::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FeMergeNode => {
                Self::FeMergeNode(SvgFeMergeNode::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeMorphology => {
                Self::FeMorphology(SvgFeMorphology::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeOffset => Self::FeOffset(SvgFeOffset::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::FePointLight => {
                Self::FePointLight(SvgFePointLight::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeSpecularLighting => {
                Self::FeSpecularLighting(SvgFeSpecularLighting::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeSpotLight => {
                Self::FeSpotLight(SvgFeSpotLight::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::FeTurbulence => {
                Self::FeTurbulence(SvgFeTurbulence::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::Filter => Self::Filter(SvgFilter::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::G => Self::G(SvgG::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Image => Self::Image(SvgImage::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Line => Self::Line(SvgLine::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::LinearGradient => {
                Self::LinearGradient(SvgLinearGradient::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::Mask => Self::Mask(SvgMask::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Path => Self::Path(SvgPath::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Pattern => Self::Pattern(SvgPattern::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Polygon => Self::Polygon(SvgPoly::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Polyline => Self::Polyline(SvgPoly::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::RadialGradient => {
                Self::RadialGradient(SvgRadialGradient::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::Rect => Self::Rect(SvgRect::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Stop => Self::Stop(SvgStop::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Svg => Self::Svg(Svg::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Text => Self::Text(SvgText::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::TextLiteral => {
                Self::TextLiteral(SvgTextLiteral::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::TextPath => Self::TextPath(SvgTextPath::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::TSpan => Self::TSpan(SvgTSpan::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Use => Self::Use(SvgUse::from_unshared_ptr(ptr as *mut _)?),
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
        SkSVGNode[native, native_mut] => {
            // inherited
            clip_rule?: SvgFillRule [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGFillRule { fType: value }],
            color_interpolation?: SvgColorSpace [get(value) => value, set(value) => value],
            color_interpolation_filters?: SvgColorSpace [get(value) => value, set(value) => value],
            color?: Color [get(value) => value.map(Color::from_native_ref), set(value) => value.into_native()],
            fill_rule?: SvgFillRule [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGFillRule { fType: value }],
            fill?: SvgPaint [get(value) => value.map(SvgPaint::from_native_ref), set(value) => value.native()],
            *fill_opacity?: scalar [get(value) => value, set(value) => value],
            font_family?: SvgFontFamily [get(value) => value.map(SvgFontFamily::from_native_ref), set(value) => value.into_native()],
            font_size?: SvgFontSize [get(value) => value.map(SvgFontSize::from_native_ref), set(value) => value.into_native()],
            font_style?: SvgFontStyle [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGFontStyle { fType: value }],
            font_weight?: SvgFontWeight [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGFontWeight { fType: value }],
            stroke?: SvgPaint [get(value) => value.map(SvgPaint::from_native_ref), set(value) => value.native()],
            stroke_line_cap?: SvgLineCap [get(value) => value, set(value) => value],
            stroke_line_join?: SvgLineJoin [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGLineJoin { fType: value }],
            *stroke_miter_limit?: scalar [get(value) => value, set(value) => value],
            *stroke_opacity?: scalar [get(value) => value, set(value) => value],
            stroke_width?: SvgLength [get(value) => value.map(SvgLength::from_native_ref), set(value) => value.into_native()],
            text_anchor?: SvgTextAnchor [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGTextAnchor { fType: value }],
            visibility?: SvgVisibility [get(value) => value.map(|value| &value.fType), set(value) => sb::SkSVGVisibility { fType: value }],

            // not inherited
            clip_path?: SvgIriFunc [get(value) => value.map(SvgIriFunc::from_native_ref), set(value) => value.native()],
            display?: SvgDisplay [get(value) => value, set(value) => value],
            mask?: SvgIriFunc [get(value) => value.map(SvgIriFunc::from_native_ref), set(value) => value.native()],
            filter?: SvgIriFunc [get(value) => value.map(SvgIriFunc::from_native_ref), set(value) => value.native()],
            *opacity?: scalar [get(value) => value, set(value) => value],
            stop_color?: SvgColor [get(value) => value.map(SvgColor::from_native_ref), set(value) => value.native()],
            *stop_opacity?: scalar [get(value) => value, set(value) => value],
            flood_color?: SvgColor [get(value) => value.map(SvgColor::from_native_ref), set(value) => value.native()],
            *flood_opacity?: scalar [get(value) => value, set(value) => value],
            lighting_color?: SvgColor [get(value) => value.map(SvgColor::from_native_ref), set(value) => value.native()]
        }
    }
}
