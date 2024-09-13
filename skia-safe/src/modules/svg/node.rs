use super::{
    circle::SvgCircle,
    color::SvgColor,
    font::{SvgFontFamily, SvgFontSize, SvgFontStyle, SvgFontWeight},
    iri::SvgIriFunc,
    paint::SvgPaint,
    path::SvgPath,
    svg::Svg,
    text::SvgTextLiteral,
};
use crate::{prelude::*, scalar, Color};
use skia_bindings as sb;
use std::fmt;

pub type SvgFillRule = sb::SkSVGFillRule_Type;
pub type SvgColorSpace = sb::SkSVGColorspace;
pub type SvgDisplay = sb::SkSVGDisplay;
pub type SvgLineCap = sb::SkSVGLineCap;
pub type SvgVisibility = sb::SkSVGVisibility_Type;
pub type SvgLineJoin = sb::SkSVGLineJoin_Type;
pub type SvgTextAnchor = sb::SkSVGTextAnchor_Type;
pub type SvgUnit = sb::SkSVGLength_Unit;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SvgLength {
    pub value: scalar,
    pub unit: SvgUnit,
}

native_transmutable!(sb::SkSVGLength, SvgLength, svg_length_layout);

impl SvgLength {
    pub fn new(value: scalar, unit: SvgUnit) -> Self {
        Self { value, unit }
    }
}

#[derive(Debug)]
pub enum NodeAttributes {
    Circle(SvgCircle),
    Path(SvgPath),
    Svg(Svg),
    TextLiteral(SvgTextLiteral),
}

pub type NodeTag = sb::SkSVGTag;

type SkSvgNode = RCHandle<sb::SkSVGNode>;

require_base_type!(sb::SkSVGSVG, sb::SkSVGContainer);

impl NativeRefCountedBase for sb::SkSVGNode {
    type Base = sb::SkRefCntBase;
}

pub struct SvgNode {
    node: SkSvgNode,
    tag: NodeTag,
    attributes: NodeAttributes,
}

impl fmt::Debug for SvgNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgNode")
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
            .field("lighting_color", &self.get_lighting_color())
            .field("tag", &self.tag)
            .field("data", &self.attributes)
            .finish()
    }
}

/// TODO: Implement bindings for the remaining classes that inherit SkSVGNode
impl SvgNode {
    pub fn from_ptr(ptr: *mut sb::SkSVGNode) -> Option<Self> {
        let node = SkSvgNode::from_ptr(ptr)?;
        let tag = unsafe { sb::C_SkSVGNode_tag(ptr as *const _) };
        let attributes = match tag {
            NodeTag::Circle => NodeAttributes::Circle(SvgCircle::from_ptr(ptr as *mut _)?),
            NodeTag::ClipPath => todo!(),
            NodeTag::Defs => todo!(),
            NodeTag::Ellipse => todo!(),
            NodeTag::FeBlend => todo!(),
            NodeTag::FeColorMatrix => todo!(),
            NodeTag::FeComponentTransfer => todo!(),
            NodeTag::FeComposite => todo!(),
            NodeTag::FeDiffuseLighting => todo!(),
            NodeTag::FeDisplacementMap => todo!(),
            NodeTag::FeDistantLight => todo!(),
            NodeTag::FeFlood => todo!(),
            NodeTag::FeFuncA => todo!(),
            NodeTag::FeFuncR => todo!(),
            NodeTag::FeFuncG => todo!(),
            NodeTag::FeFuncB => todo!(),
            NodeTag::FeGaussianBlur => todo!(),
            NodeTag::FeImage => todo!(),
            NodeTag::FeMerge => todo!(),
            NodeTag::FeMergeNode => todo!(),
            NodeTag::FeMorphology => todo!(),
            NodeTag::FeOffset => todo!(),
            NodeTag::FePointLight => todo!(),
            NodeTag::FeSpecularLighting => todo!(),
            NodeTag::FeSpotLight => todo!(),
            NodeTag::FeTurbulence => todo!(),
            NodeTag::Filter => todo!(),
            NodeTag::G => todo!(),
            NodeTag::Image => todo!(),
            NodeTag::Line => todo!(),
            NodeTag::LinearGradient => todo!(),
            NodeTag::Mask => todo!(),
            NodeTag::Path => NodeAttributes::Path(SvgPath::from_ptr(ptr as *mut _)?),
            NodeTag::Pattern => todo!(),
            NodeTag::Polygon => todo!(),
            NodeTag::Polyline => todo!(),
            NodeTag::RadialGradient => todo!(),
            NodeTag::Rect => todo!(),
            NodeTag::Stop => todo!(),
            NodeTag::Svg => NodeAttributes::Svg(Svg::from_ptr(ptr as *mut _)?),
            NodeTag::Text => todo!(),
            NodeTag::TextLiteral => {
                NodeAttributes::TextLiteral(SvgTextLiteral::from_ptr(ptr as *mut _)?)
            }
            NodeTag::TextPath => todo!(),
            NodeTag::TSpan => todo!(),
            NodeTag::Use => todo!(),
        };

        Some(Self {
            node,
            tag,
            attributes,
        })
    }

    pub fn from_unshared_ptr(ptr: *mut sb::SkSVGNode) -> Option<Self> {
        let node = SkSvgNode::from_unshared_ptr(ptr)?;
        let tag = unsafe { sb::C_SkSVGNode_tag(ptr as *const _) };
        let attributes = match tag {
            NodeTag::Circle => NodeAttributes::Circle(SvgCircle::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::ClipPath => todo!(),
            NodeTag::Defs => todo!(),
            NodeTag::Ellipse => todo!(),
            NodeTag::FeBlend => todo!(),
            NodeTag::FeColorMatrix => todo!(),
            NodeTag::FeComponentTransfer => todo!(),
            NodeTag::FeComposite => todo!(),
            NodeTag::FeDiffuseLighting => todo!(),
            NodeTag::FeDisplacementMap => todo!(),
            NodeTag::FeDistantLight => todo!(),
            NodeTag::FeFlood => todo!(),
            NodeTag::FeFuncA => todo!(),
            NodeTag::FeFuncR => todo!(),
            NodeTag::FeFuncG => todo!(),
            NodeTag::FeFuncB => todo!(),
            NodeTag::FeGaussianBlur => todo!(),
            NodeTag::FeImage => todo!(),
            NodeTag::FeMerge => todo!(),
            NodeTag::FeMergeNode => todo!(),
            NodeTag::FeMorphology => todo!(),
            NodeTag::FeOffset => todo!(),
            NodeTag::FePointLight => todo!(),
            NodeTag::FeSpecularLighting => todo!(),
            NodeTag::FeSpotLight => todo!(),
            NodeTag::FeTurbulence => todo!(),
            NodeTag::Filter => todo!(),
            NodeTag::G => todo!(),
            NodeTag::Image => todo!(),
            NodeTag::Line => todo!(),
            NodeTag::LinearGradient => todo!(),
            NodeTag::Mask => todo!(),
            NodeTag::Path => NodeAttributes::Path(SvgPath::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Pattern => todo!(),
            NodeTag::Polygon => todo!(),
            NodeTag::Polyline => todo!(),
            NodeTag::RadialGradient => todo!(),
            NodeTag::Rect => todo!(),
            NodeTag::Stop => todo!(),
            NodeTag::Svg => NodeAttributes::Svg(Svg::from_unshared_ptr(ptr as *mut _)?),
            NodeTag::Text => todo!(),
            NodeTag::TextLiteral => {
                NodeAttributes::TextLiteral(SvgTextLiteral::from_unshared_ptr(ptr as *mut _)?)
            }
            NodeTag::TextPath => todo!(),
            NodeTag::TSpan => todo!(),
            NodeTag::Use => todo!(),
        };

        Some(Self {
            node,
            tag,
            attributes,
        })
    }

    pub fn native(&self) -> &sb::SkSVGNode {
        self.node.native()
    }

    pub fn native_mut(&mut self) -> &mut sb::SkSVGNode {
        self.node.native_mut()
    }

    pub fn into_ptr(self) -> *mut sb::SkSVGNode {
        self.node.into_ptr()
    }

    pub fn attributes(&self) -> &NodeAttributes {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut NodeAttributes {
        &mut self.attributes
    }

    pub fn tag(&self) -> NodeTag {
        self.tag
    }

    skia_macros::attrs! {
        SkSVGNode => {
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

/// TODO: It looks pretty dirty. Perhaps there are ways of a cleaner implementation?
pub struct SvgSpecNode<T: AsTag<N>, N> {
    node: SkSvgNode,
    tag: NodeTag,
    attributes: T,
    marker: std::marker::PhantomData<N>,
}

impl<N: NativeRefCounted, T: fmt::Debug + AsTag<N>> fmt::Debug for SvgSpecNode<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SvgNode")
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
            .field("lighting_color", &self.get_lighting_color())
            .field("tag", &self.tag)
            .field("data", &self.attributes)
            .finish()
    }
}

impl<N: NativeRefCounted, T: AsTag<N>> SvgSpecNode<T, N> {
    pub fn from_ptr(ptr: *mut sb::SkSVGNode) -> Option<Self> {
        let node = SkSvgNode::from_ptr(ptr)?;
        let tag = unsafe { sb::C_SkSVGNode_tag(ptr as *const _) };

        T::from_tag(ptr as *mut _, tag).map(|attributes| Self {
            node,
            tag,
            attributes,
            marker: std::marker::PhantomData,
        })
    }

    pub fn from_unshared_ptr(ptr: *mut sb::SkSVGNode) -> Option<Self> {
        let node = SkSvgNode::from_unshared_ptr(ptr)?;
        let tag = unsafe { sb::C_SkSVGNode_tag(ptr as *const _) };

        T::from_unshared_tag(ptr as *mut _, tag).map(|attributes| Self {
            node,
            tag,
            attributes,
            marker: std::marker::PhantomData,
        })
    }

    pub fn native(&self) -> &sb::SkSVGNode {
        self.node.native()
    }

    pub fn native_mut(&mut self) -> &mut sb::SkSVGNode {
        self.node.native_mut()
    }

    pub fn into_ptr(self) -> *mut sb::SkSVGNode {
        self.node.into_ptr()
    }

    pub fn tag(&self) -> NodeTag {
        self.tag
    }

    pub fn attributes(&self) -> &T {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut T {
        &mut self.attributes
    }

    skia_macros::attrs! {
        SkSVGNode => {
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

pub trait Tagged {
    const TAG: NodeTag;
}

pub trait AsTag<N>: Tagged + Sized {
    fn from_tag(ptr: *mut N, tag: NodeTag) -> Option<Self>;
    fn from_unshared_tag(ptr: *mut N, tag: NodeTag) -> Option<Self>;
    fn into_node(self) -> SvgSpecNode<Self, N>;
}

impl<N: NativeRefCounted> AsTag<N> for RCHandle<N>
where
    RCHandle<N>: Tagged,
{
    fn from_tag(ptr: *mut N, tag: NodeTag) -> Option<Self> {
        if tag == Self::TAG {
            Self::from_ptr(ptr)
        } else {
            None
        }
    }

    fn from_unshared_tag(ptr: *mut N, tag: NodeTag) -> Option<Self> {
        if tag == Self::TAG {
            Self::from_unshared_ptr(ptr)
        } else {
            None
        }
    }

    fn into_node(self) -> SvgSpecNode<Self, N> {
        let ptr = unsafe { self.native_mut_force() };

        SvgSpecNode {
            node: SkSvgNode::from_ptr(ptr as *mut _).unwrap(),
            tag: Self::TAG,
            attributes: self,
            marker: std::marker::PhantomData,
        }
    }
}
