use super::{
    circle::SvgCircle,
    color::SvgColor,
    element::Svg,
    ellipse::SvgEllipse,
    font::{SvgFontFamily, SvgFontSize, SvgFontStyle, SvgFontWeight},
    image::SvgImage,
    iri::SvgIriFunc,
    line::SvgLine,
    paint::SvgPaint,
    path::SvgPath,
    rect::SvgRect,
    text::SvgTextLiteral,
    using::SvgUse,
};
use crate::{prelude::*, scalar, Color};
use skia_bindings as sb;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

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
/// TODO (Aiving): Implement bindings for the remaining classes that inherit SkSVGNode
pub enum Node {
    Circle(SvgCircle),
    ClipPath,
    Defs,
    Ellipse(SvgEllipse),
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeFlood,
    FeFuncA,
    FeFuncR,
    FeFuncG,
    FeFuncB,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTurbulence,
    Filter,
    G,
    Image(SvgImage),
    Line(SvgLine),
    LinearGradient,
    Mask,
    Path(SvgPath),
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect(SvgRect),
    Stop,
    Svg(Svg),
    Text,
    TextLiteral(SvgTextLiteral),
    TextPath,
    TSpan,
    Use(SvgUse),
}

impl Node {
    pub fn from_unshared_ptr(ptr: *mut sb::SkSVGNode) -> Option<Self> {
        let tag = unsafe { sb::C_SkSVGNode_tag(ptr as *const _) };

        Some(match tag {
            NodeTag::Circle => Self::Circle(SvgNode::from_unshared_ptr(ptr)?),
            NodeTag::ClipPath => Self::ClipPath,
            NodeTag::Defs => Self::Defs,
            NodeTag::Ellipse => Self::Ellipse(SvgNode::from_unshared_ptr(ptr)?),
            NodeTag::FeBlend => Self::FeBlend,
            NodeTag::FeColorMatrix => Self::FeColorMatrix,
            NodeTag::FeComponentTransfer => Self::FeComponentTransfer,
            NodeTag::FeComposite => Self::FeComposite,
            NodeTag::FeDiffuseLighting => Self::FeDiffuseLighting,
            NodeTag::FeDisplacementMap => Self::FeDisplacementMap,
            NodeTag::FeDistantLight => Self::FeDistantLight,
            NodeTag::FeFlood => Self::FeFlood,
            NodeTag::FeFuncA => Self::FeFuncA,
            NodeTag::FeFuncR => Self::FeFuncR,
            NodeTag::FeFuncG => Self::FeFuncG,
            NodeTag::FeFuncB => Self::FeFuncB,
            NodeTag::FeGaussianBlur => Self::FeGaussianBlur,
            NodeTag::FeImage => Self::FeImage,
            NodeTag::FeMerge => Self::FeMerge,
            NodeTag::FeMergeNode => Self::FeMergeNode,
            NodeTag::FeMorphology => Self::FeMorphology,
            NodeTag::FeOffset => Self::FeOffset,
            NodeTag::FePointLight => Self::FePointLight,
            NodeTag::FeSpecularLighting => Self::FeSpecularLighting,
            NodeTag::FeSpotLight => Self::FeSpotLight,
            NodeTag::FeTurbulence => Self::FeTurbulence,
            NodeTag::Filter => Self::Filter,
            NodeTag::G => Self::G,
            NodeTag::Image => Self::Image(SvgNode::from_unshared_ptr(ptr)?),
            NodeTag::Line => Self::Line(SvgNode::from_unshared_ptr(ptr)?),
            NodeTag::LinearGradient => Self::LinearGradient,
            NodeTag::Mask => Self::Mask,
            NodeTag::Path => Self::Path(SvgNode::from_unshared_ptr(ptr)?),
            NodeTag::Pattern => Self::Pattern,
            NodeTag::Polygon => Self::Polygon,
            NodeTag::Polyline => Self::Polyline,
            NodeTag::RadialGradient => Self::RadialGradient,
            NodeTag::Rect => Self::Rect(SvgNode::from_unshared_ptr(ptr)?),
            NodeTag::Stop => Self::Stop,
            NodeTag::Svg => Self::Svg(SvgNode::from_unshared_ptr(ptr)?),
            NodeTag::Text => Self::Text,
            NodeTag::TextLiteral => Self::TextLiteral(SvgNode::from_unshared_ptr(ptr)?),
            NodeTag::TextPath => Self::TextPath,
            NodeTag::TSpan => Self::TSpan,
            NodeTag::Use => Self::Use(SvgNode::from_unshared_ptr(ptr)?),
        })
    }
}

pub type NodeTag = sb::SkSVGTag;

type SkSvgNode = RCHandle<sb::SkSVGNode>;

impl NativeRefCountedBase for sb::SkSVGNode {
    type Base = sb::SkRefCntBase;
}

pub struct SvgNode<N: Tagged + NativeRefCounted> {
    node: SkSvgNode,
    tag: NodeTag,
    data: RCHandle<N>,
}

impl<T: Tagged + NativeRefCounted> Deref for SvgNode<T> {
    type Target = RCHandle<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Tagged + NativeRefCounted> DerefMut for SvgNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Tagged + NativeRefCounted> fmt::Debug for SvgNode<T>
where
    Self: TaggedDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = if self.tag == NodeTag::Svg {
            "Svg".into()
        } else {
            format!("Svg{:?}", self.tag)
        };

        let mut builder = f.debug_struct(&name);

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

        self._dbg(&mut builder);

        builder.finish()
    }
}

impl<T: Tagged + NativeRefCounted> SvgNode<T> {
    pub fn from_ptr(ptr: *mut sb::SkSVGNode) -> Option<Self> {
        let node = SkSvgNode::from_ptr(ptr)?;
        let tag = unsafe { sb::C_SkSVGNode_tag(ptr as *const _) };

        if tag == T::TAG {
            Some(Self {
                node,
                tag,
                data: RCHandle::<T>::from_ptr(ptr as *mut _)?,
            })
        } else {
            None
        }
    }

    pub fn from_unshared_ptr(ptr: *mut sb::SkSVGNode) -> Option<Self> {
        let node = SkSvgNode::from_unshared_ptr(ptr)?;
        let tag = unsafe { sb::C_SkSVGNode_tag(ptr as *const _) };

        if tag == T::TAG {
            Some(Self {
                node,
                tag,
                data: RCHandle::<T>::from_unshared_ptr(ptr as *mut _)?,
            })
        } else {
            None
        }
    }

    pub fn native(&self) -> &T {
        self.data.native()
    }

    pub fn native_node(&self) -> &sb::SkSVGNode {
        self.node.native()
    }

    pub fn native_mut(&mut self) -> &mut T {
        self.data.native_mut()
    }

    pub fn native_node_mut(&mut self) -> &mut sb::SkSVGNode {
        self.node.native_mut()
    }

    pub fn into_ptr(self) -> *mut T {
        self.data.into_ptr()
    }

    pub fn into_node_ptr(self) -> *mut sb::SkSVGNode {
        self.node.into_ptr()
    }

    pub fn tag(&self) -> NodeTag {
        self.tag
    }

    skia_macros::attrs! {
        SkSVGNode[native_node, native_node_mut] => {
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

pub trait TaggedDebug {
    fn _dbg(&self, f: &mut fmt::DebugStruct);
}
