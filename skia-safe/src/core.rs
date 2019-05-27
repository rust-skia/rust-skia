#[allow(non_camel_case_types)]
pub type scalar = skia_bindings::SkScalar;

pub trait Scalar : Copy {
    const NEARLY_ZERO: Self;
    const ONE: Self;
    const HALF: Self;
}

impl Scalar for scalar {
    const NEARLY_ZERO: Self = 1.0 / ((1 << 12) as Self);
    const ONE: Self = 1.0;
    const HALF: Self = 0.5;
}

pub type FontTableTag = skia_bindings::SkFontTableTag;

mod annotation;
pub use annotation::*;

mod bbh_factory;
pub use bbh_factory::*;

mod bitmap;
pub use bitmap::*;

mod blend_mode;
pub use blend_mode::*;

mod blur_types;
pub use blur_types::*;

mod canvas;
pub use canvas::*;

mod clip_op;
pub use clip_op::*;

mod color;
pub use color::*;

mod color_filter;
pub use color_filter::*;

mod color_space;
pub use color_space::*;

mod color_space_xform_canvas;
pub use color_space_xform_canvas::*;

pub mod contour_measure;
pub use contour_measure::*;

mod coverage_mode;
pub use coverage_mode::*;

mod cubic_map;
pub use cubic_map::*;

mod data;
pub use data::*;

mod data_table;
pub use data_table::*;

mod deferred_display_list_recorder;
pub use deferred_display_list_recorder::*;

pub(crate) mod document;
pub use document::*;

mod draw_looper;
pub use draw_looper::*;

mod drawable;
pub use drawable::*;

mod encoded_image_format;
pub use encoded_image_format::*;

// unsupported, because it's used in experimental APIs only.
// mod executor;

mod filter_quality;
pub use filter_quality::*;

mod flattenable;
pub use flattenable:: *;

mod font;
pub use font::*;

pub mod font_arguments;
pub use font_arguments::FontArguments;
#[deprecated(since = "0.11.0", note = "use font_arguments::VariationPosition instead")]
pub use font_arguments::VariationPosition as FontArgumentsVariationPosition;
#[deprecated(since = "0.11.0", note = "use font_arguments::variation_position::Coordinate instead")]
pub use font_arguments::variation_position::Coordinate as FontArgumentsVariationPositionCoordinate;

// unsupported, because it's not used in publicly exposed APIs:
// mod font_lcd_config;

pub mod font_metrics;
pub use font_metrics::FontMetrics;
#[deprecated(since = "0.11.0", note = "use font_metrics::Flags instead")]
pub use font_metrics::Flags as FontMetricsFlags;

mod font_mgr;
pub use font_mgr::*;

pub mod font_parameters;

pub mod font_style;
pub use font_style::FontStyle;
#[deprecated(since = "0.11.0", note = "use font_style::Weight")]
pub use font_style::Weight as FontStyleWeight;
#[deprecated(since = "0.11.0", note = "use font_style::Width")]
pub use font_style::Width as FontStyleWidth;
#[deprecated(since = "0.11.0", note = "use font_style::Slant")]
pub use font_style::Slant as FontStyleSlant;

mod font_types;
pub use font_types::*;

pub mod graphics;

pub mod image;
pub use image::Image;
#[deprecated(since = "0.11.0", note = "use image::BitDepth instead")]
pub use image::BitDepth as ImageBitDepth;
#[deprecated(since = "0.11.0", note = "use image::CachingHint instead")]
pub use image::CachingHint as ImageCachingHint;
#[deprecated(since = "0.11.0", note = "use image::CompressionType instead")]
pub use image::CompressionType as ImageCompressionType;

mod image_encoder;
pub use image_encoder::*;

pub mod image_filter;
pub use image_filter::ImageFilter;
#[deprecated(since = "0.11.0", note = "use image_filter::OutputProperties instead")]
pub use image_filter::OutputProperties as ImageFilterOutputProperties;
#[deprecated(since = "0.11.0", note = "use image_filter::Context instead")]
pub use image_filter::Context as ImageFilterContext;
#[deprecated(since = "0.11.0", note = "use image_filter::CropRect instead")]
pub use image_filter::CropRect as ImageFilterCropRect;
#[deprecated(since = "0.11.0", note = "use image_filter::crop_rect::CropEdge instead")]
pub use image_filter::crop_rect::CropEdge as ImageFilterCropRectCropEdge;
#[deprecated(since = "0.11.0", note = "use image_filter::TileUsage instead")]
pub use image_filter::TileUsage as ImageFilterTileUsage;
#[deprecated(since = "0.11.0", note = "use image_filter::MapDirection instead")]
pub use image_filter::MapDirection as ImageFilterMapDirection;

mod image_generator;
pub use image_generator::*;

mod image_info;
pub use image_info::*;

mod mask_filter;
pub use mask_filter::*;

pub mod matrix;
pub use matrix::Matrix;
#[deprecated(since = "0.11.0", note = "use matrix::TypeMask instead")]
pub use matrix::TypeMask as MatrixTypeMask;
#[deprecated(since = "0.11.0", note = "use matrix::ScaleToFit instead")]
pub use matrix::ScaleToFit as MatrixScaletoFit;
#[deprecated(since = "0.11.0", note = "use matrix::Member instead")]
pub use matrix::Member as MatrixMember;
#[deprecated(since = "0.11.0", note = "use matrix::AffineMember instead")]
pub use matrix::AffineMember as AffineMatrixMember;

pub mod matrix44;
pub use matrix44::{Vector4, Matrix44};

mod milestone;
pub use milestone::*;

mod multi_picture_draw;
pub use multi_picture_draw::*;

pub mod paint;
pub use paint::Paint;
// We keep these around for the time being.
pub use paint::Style as PaintStyle;
pub use paint::Cap as PaintCap;
pub use paint::Join as PaintJoin;

pub mod path;
pub use path::Path;
#[deprecated(since = "0.11.0", note = "use matrix::AffineMember instead")]
pub use path::Direction as PathDirection;
#[deprecated(since = "0.11.0", note = "use path::FillType instead")]
pub use path::FillType as PathFillType;
#[deprecated(since = "0.11.0", note = "use path::Convexity instead")]
pub use path::Convexity as PathConvexity;
#[deprecated(since = "0.11.0", note = "use path::AddPathMode instead")]
pub use path::AddPathMode as AddPathMode;
#[deprecated(since = "0.11.0", note = "use path::SegmentMask instead")]
pub use path::SegmentMask as PathSegmentMask;
#[deprecated(since = "0.11.0", note = "use path::ArcSize instead")]
pub use path::ArcSize as PathArcSize;

pub mod path_effect;
pub use path_effect::PathEffect;
#[deprecated(since = "0.11.0", note = "use path_effect::DashInfo instead")]
pub use path_effect::DashInfo as PathEffectDashInfo;
#[deprecated(since = "0.11.0", note = "use path_effect::PointData instead")]
pub use path_effect::PointData as PathEffectPointData;
#[deprecated(since = "0.11.0", note = "use path_effect::point_data::PointFlags instead")]
pub use path_effect::point_data::PointFlags as PointDataPointFlags;

pub mod path_measure;
pub use path_measure::PathMeasure;

mod picture;
pub use picture::*;

mod picture_recorder;
pub use picture_recorder::*;

mod point;
pub use point::*;

mod point3;
pub use point3::*;

mod rect;
pub use rect::*;

mod region;
pub use region::*;

mod rrect;
pub use rrect::*;

mod shader;
pub use shader::*;

mod size;
pub use size::*;

mod stroke_rec;
pub use stroke_rec::*;

mod surface;
pub use surface::*;

mod surface_characterization;
pub use surface_characterization::*;

mod surface_props;
pub use surface_props::*;

mod text_blob;
pub use text_blob::*;

mod time;
pub use time::*;

mod typeface;
pub use typeface::*;

mod types;
pub use types::*;

mod vertices;
pub use vertices::*;

mod yuva_index;
pub use yuva_index::*;

//
// Skia specific traits used for overloading.
//

pub trait Contains<T> {
    fn contains(&self, other: T) -> bool;
}

pub trait QuickReject<T> {
    fn quick_reject(&self, other: &T) -> bool;
}
