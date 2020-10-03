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
pub use canvas::{AutoCanvasRestore, Canvas, OwnedCanvas};

mod clip_op;
pub use clip_op::*;

mod color;
pub use color::*;

pub mod color_filter;
pub use color_filter::{color_filters, ColorFilter};

mod color_space;
pub use color_space::*;

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

mod deferred_display_list;
pub use deferred_display_list::*;

mod deferred_display_list_recorder;
pub use deferred_display_list_recorder::*;

pub mod document;
pub use document::Document;

pub mod draw_looper;
#[allow(deprecated)]
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
pub use font::Font;

pub mod font_arguments;
pub use font_arguments::FontArguments;

// unsupported, because it's not used in publicly exposed APIs:
// mod font_lcd_config;

pub mod font_metrics;
pub use font_metrics::FontMetrics;

mod font_mgr;
pub use font_mgr::*;

pub mod font_parameters;

pub mod font_style;
pub use font_style::FontStyle;

mod font_types;
pub use font_types::*;

pub mod graphics;

pub mod image;
pub use image::{FilterOptions, Image, MipmapMode, SamplingMode};

mod image_encoder;
pub use image_encoder::*;

pub mod image_filter;
pub use image_filter::ImageFilter;

mod image_generator;
pub use image_generator::*;

mod image_info;
pub use image_info::*;

mod m44;
pub use m44::*;

mod mask_filter;
pub use mask_filter::*;

pub mod matrix;
pub use matrix::Matrix;

pub mod matrix44;
#[allow(deprecated)]
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
pub use path::Path;

mod path_builder;
pub use path_builder::PathBuilder;

pub mod path_effect;
pub use path_effect::PathEffect;

pub mod path_measure;
pub use path_measure::PathMeasure;

pub mod path_types;
pub use path_types::*;

mod picture;
pub use picture::*;

pub mod picture_recorder;
pub use picture_recorder::PictureRecorder;

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
pub use rrect::RRect;

mod rsxform;
pub use rsxform::*;

mod scalar_;
pub use scalar_::*;

pub mod shader;
pub use shader::{shaders, Shader};

mod size;
pub use size::*;

pub mod stroke_rec;
pub use stroke_rec::StrokeRec;

pub mod surface;
pub use surface::Surface;

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
pub use typeface::Typeface;

mod types;
pub use types::*;

mod un_pre_multiply;
pub use un_pre_multiply::*;

pub mod vertices;
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
