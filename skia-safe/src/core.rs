mod annotation;
pub use annotation::annotate;

mod bbh_factory;
pub use bbh_factory::*;

mod bitmap;
pub use bitmap::*;

mod blend_mode;
pub use blend_mode::*;

mod blur_types;
pub use blur_types::*;

pub mod canvas;
#[deprecated(since = "0.12.0", note = "use canvas::lattice::RectType")]
pub use canvas::lattice::RectType as CanvasLatticeRectType;
#[deprecated(since = "0.12.0", note = "use canvas::Lattice")]
pub use canvas::Lattice as CanvasLattice;
#[deprecated(since = "0.12.0", note = "use canvas::PointMode")]
pub use canvas::PointMode as CanvasPointMode;
#[deprecated(since = "0.12.0", note = "use canvas::SaveLayerFlags")]
pub use canvas::SaveLayerFlags;
#[deprecated(since = "0.12.0", note = "use canvas::SaveLayerRec")]
pub use canvas::SaveLayerRec;
#[deprecated(since = "0.12.0", note = "use canvas::SrcRectConstraint")]
pub use canvas::SrcRectConstraint;
#[deprecated(since = "0.12.0", note = "use canvas::TopLayerPixels")]
pub use canvas::TopLayerPixels as CanvasTopLayerPixels;
pub use canvas::{AutoCanvasRestore, Canvas, OwnedCanvas};

mod clip_op;
pub use clip_op::*;

mod color;
pub use color::*;

pub mod color_filter;
#[deprecated(since = "0.12.0", note = "use ColorFilter::Flags")]
pub use color_filter::Flags as ColorFilterFlags;
pub use color_filter::{color_filters, ColorFilter};
#[deprecated(since = "0.12.0", note = "use color_filters")]
pub use color_filters as ColorFilters;

mod color_space;
pub use color_space::*;
#[deprecated(since = "0.12.0", note = "use named_transfer_fn")]
pub use named_transfer_fn as NamedTransferFn;

pub mod contour_measure;
pub use contour_measure::{ContourMeasure, ContourMeasureIter};

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

pub mod document;
pub use document::Document;

pub mod draw_looper;
#[deprecated(since = "0.12.0", note = "use draw_looper::BlurShadowRec")]
pub use draw_looper::BlurShadowRec as DrawLooperBlurShadowRec;
pub use draw_looper::DrawLooper;

pub mod drawable;
pub use drawable::Drawable;

mod encoded_image_format;
pub use encoded_image_format::*;

// unsupported, because it's used in experimental APIs only.
// mod executor;

mod filter_quality;
pub use filter_quality::*;

mod flattenable;
pub use flattenable::*;

pub mod font;
#[deprecated(since = "0.12.0", note = "use font::Edging")]
pub use font::Edging as FontEdging;
pub use font::Font;

pub mod font_arguments;
#[deprecated(
    since = "0.12.0",
    note = "use font_arguments::variation_position::Coordinate"
)]
pub use font_arguments::variation_position::Coordinate as FontArgumentsVariationPositionCoordinate;
pub use font_arguments::FontArguments;
#[deprecated(since = "0.12.0", note = "use font_arguments::VariationPosition")]
pub use font_arguments::VariationPosition as FontArgumentsVariationPosition;

// unsupported, because it's not used in publicly exposed APIs:
// mod font_lcd_config;

pub mod font_metrics;
#[deprecated(since = "0.12.0", note = "use font_metrics::Flags")]
pub use font_metrics::Flags as FontMetricsFlags;
pub use font_metrics::FontMetrics;

mod font_mgr;
pub use font_mgr::*;

pub mod font_parameters;

pub mod font_style;
pub use font_style::FontStyle;
#[deprecated(since = "0.12.0", note = "use font_style::Slant")]
pub use font_style::Slant as FontStyleSlant;
#[deprecated(since = "0.12.0", note = "use font_style::Weight")]
pub use font_style::Weight as FontStyleWeight;
#[deprecated(since = "0.12.0", note = "use font_style::Width")]
pub use font_style::Width as FontStyleWidth;

mod font_types;
pub use font_types::*;

pub mod graphics;

pub mod image;
#[deprecated(since = "0.12.0", note = "use image::BitDepth")]
pub use image::BitDepth as ImageBitDepth;
#[deprecated(since = "0.12.0", note = "use image::CachingHint")]
pub use image::CachingHint as ImageCachingHint;
#[deprecated(since = "0.12.0", note = "use image::CompressionType")]
pub use image::CompressionType as ImageCompressionType;
pub use image::Image;

mod image_encoder;
pub use image_encoder::*;

pub mod image_filter;
#[deprecated(since = "0.12.0", note = "use image_filter::crop_rect::CropEdge")]
pub use image_filter::crop_rect::CropEdge as ImageFilterCropRectCropEdge;
#[deprecated(since = "0.12.0", note = "use image_filter::CropRect")]
pub use image_filter::CropRect as ImageFilterCropRect;
pub use image_filter::ImageFilter;
#[deprecated(since = "0.12.0", note = "use image_filter::MapDirection")]
pub use image_filter::MapDirection as ImageFilterMapDirection;

mod image_generator;
pub use image_generator::*;

mod image_info;
pub use image_info::*;

mod mask_filter;
pub use mask_filter::*;

pub mod matrix;
#[deprecated(since = "0.12.0", note = "use matrix::AffineMember")]
pub use matrix::AffineMember as AffineMatrixMember;
pub use matrix::Matrix;
#[deprecated(since = "0.12.0", note = "use matrix::Member")]
pub use matrix::Member as MatrixMember;
#[deprecated(since = "0.12.0", note = "use matrix::ScaleToFit")]
pub use matrix::ScaleToFit as MatrixScaletoFit;
#[deprecated(since = "0.12.0", note = "use matrix::TypeMask")]
pub use matrix::TypeMask as MatrixTypeMask;

pub mod matrix44;
pub use matrix44::{Matrix44, Vector4};

mod milestone;
pub use milestone::*;

pub mod paint;
pub use paint::Paint;
// We keep these around for the time being.
pub use paint::Cap as PaintCap;
pub use paint::Join as PaintJoin;
pub use paint::Style as PaintStyle;

pub mod path;
#[deprecated(since = "0.12.0", note = "use path::AddPathMode")]
pub use path::AddPathMode;
#[deprecated(since = "0.12.0", note = "use path::ArcSize")]
pub use path::ArcSize as PathArcSize;
pub use path::Path;

pub mod path_effect;
#[deprecated(since = "0.12.0", note = "use path_effect::point_data::PointFlags")]
pub use path_effect::point_data::PointFlags as PointDataPointFlags;
#[deprecated(since = "0.12.0", note = "use path_effect::DashInfo")]
pub use path_effect::DashInfo as PathEffectDashInfo;
pub use path_effect::PathEffect;
#[deprecated(since = "0.12.0", note = "use path_effect::PointData")]
pub use path_effect::PointData as PathEffectPointData;

pub mod path_measure;
pub use path_measure::PathMeasure;

pub mod path_types;
pub use path_types::*;

mod picture;
pub use picture::*;

pub mod picture_recorder;
pub use picture_recorder::PictureRecorder;
#[deprecated(since = "0.12.0", note = "use picture_recorder::RecordFlags")]
pub use picture_recorder::RecordFlags as PictureRecorderRecordFlags;

mod pixel_ref;
pub use pixel_ref::*;

mod pixmap;
pub use pixmap::*;

mod point;
pub use point::*;

mod point3;
pub use point3::*;

mod promise_image_texture;
pub use promise_image_texture::*;

mod raster_handle_allocator;
pub use raster_handle_allocator::*;

mod rect;
pub use rect::*;

pub mod region;
pub use region::Region;

pub mod rrect;
#[deprecated(since = "0.12.0", note = "use rrect::Corner")]
pub use rrect::Corner as RRectCorner;
pub use rrect::RRect;
#[deprecated(since = "0.12.0", note = "use rrect::Type")]
pub use rrect::Type as RRectType;

mod rsxform;
pub use rsxform::*;

mod scalar_;
pub use scalar_::*;

pub mod shader;
#[deprecated(since = "0.12.0", note = "use shader::GradientInfo")]
pub use shader::GradientInfo as ShaderGradientInfo;
#[deprecated(since = "0.12.0", note = "use shader::GradientType")]
pub use shader::GradientType as ShaderGradientType;
pub use shader::{shaders, Shader};
#[deprecated(since = "0.12.0", note = "use shaders")]
pub use shaders as Shaders;
#[deprecated(since = "0.12.0", note = "use TileMode")]
pub use TileMode as ShaderTileMode;

mod size;
pub use size::*;

pub mod stroke_rec;
#[deprecated(since = "0.12.0", note = "use stroke_rec::InitStyle")]
pub use stroke_rec::InitStyle as StrokeRecInitStyle;
pub use stroke_rec::StrokeRec;
#[deprecated(since = "0.12.0", note = "use stroke_rec::Style")]
pub use stroke_rec::Style as StrokeRecStyle;

pub mod surface;
pub use surface::Surface;
#[deprecated(since = "0.12.0", note = "use Borrows<'a, Surface>")]
pub type OwnedSurface<'a> = crate::Borrows<'a, Surface>;
#[deprecated(since = "0.12.0", note = "use surface::BackendHandleAccess")]
pub use surface::BackendHandleAccess as SurfaceBackendHandleAccess;
#[deprecated(since = "0.12.0", note = "use surface::ContentChangeMode")]
pub use surface::ContentChangeMode as SurfaceContentChangeMode;

mod surface_characterization;
pub use surface_characterization::*;

mod surface_props;
pub use surface_props::*;

mod swizzle;
pub use swizzle::*;

mod text_blob;
pub use text_blob::*;

mod tile_mode;
pub use self::tile_mode::*;

mod time;
pub use time::*;

mod trace_memory_dump;
pub use trace_memory_dump::*;

pub mod typeface;
#[deprecated(since = "0.12.0", note = "use typeface::LocalizedString")]
pub use typeface::LocalizedString as TypefaceLocalizedString;
#[deprecated(since = "0.12.0", note = "use typeface::SerializeBehavior")]
pub use typeface::SerializeBehavior as TypefaceSerializeBehavior;
pub use typeface::Typeface;

mod types;
pub use types::*;

mod un_pre_multiply;
pub use un_pre_multiply::*;

pub mod vertices;
#[deprecated(since = "0.12.0", note = "use vertices::Bone")]
pub use vertices::Bone as VerticesBone;
#[deprecated(since = "0.12.0", note = "use vertices::BoneIndices")]
pub use vertices::BoneIndices;
#[deprecated(since = "0.12.0", note = "use vertices::BoneWeights")]
pub use vertices::BoneWeights;
#[deprecated(since = "0.12.0", note = "use vertices::Builder")]
pub use vertices::Builder as VerticesBuilder;
#[deprecated(since = "0.12.0", note = "use vertices::BuilderFlags")]
pub use vertices::BuilderFlags as VerticesBuilderFlags;
#[deprecated(since = "0.12.0", note = "use vertices::VertexMode")]
pub use vertices::VertexMode as VerticesVertexMode;
pub use vertices::Vertices;

pub mod yuva_index;
pub use yuva_index::YUVAIndex;

mod yuva_size_info;
pub use yuva_size_info::*;

//
// Skia specific traits used for overloading.
//

pub trait Contains<T> {
    fn contains(&self, other: T) -> bool;
}

pub trait QuickReject<T> {
    fn quick_reject(&self, other: &T) -> bool;
}
