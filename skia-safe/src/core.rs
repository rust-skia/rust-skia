// Unsupported, not used in any public APIs.
// mod executor;
// Unsupported, because it's not used in publicly exposed APIs:
// mod font_lcd_config;

mod alpha_type;
mod annotation;
mod bbh_factory;
mod bitmap;
mod blend_mode;
mod blender;
mod blur_types;
pub mod canvas;
mod clip_op;
mod color;
pub mod color_filter;
mod color_space;
mod color_type;
pub mod contour_measure;
mod coverage_mode;
mod cubic_map;
mod data;
mod data_table;
mod deferred_display_list;
mod deferred_display_list_recorder;
pub mod document;
pub mod drawable;
mod flattenable;
pub mod font;
pub mod font_arguments;
pub mod font_metrics;
mod font_mgr;
pub mod font_parameters;
pub mod font_style;
mod font_types;
pub mod graphics;
pub mod image;
pub mod image_filter;
mod image_generator;
mod image_info;
mod m44;
mod mask_filter;
pub mod matrix;
mod mesh;
mod milestone;
pub mod paint;
pub mod path;
mod path_builder;
pub mod path_effect;
pub mod path_measure;
pub mod path_types;
pub mod path_utils;
mod picture;
pub mod picture_recorder;
mod pixel_ref;
mod pixmap;
mod point;
mod point3;
mod promise_image_texture;
mod raster_handle_allocator;
mod rect;
pub mod region;
pub mod rrect;
mod rsxform;
pub mod sampling_options;
mod scalar_;
pub mod shader;
mod size;
pub mod stroke_rec;
pub mod surface;
mod surface_characterization;
mod surface_props;
mod swizzle;
mod text_blob;
mod texture_compression_type;
mod tile_mode;
mod time;
mod trace_memory_dump;
pub mod typeface;
mod types;
mod un_pre_multiply;
pub mod vertices;
pub mod yuva_info;
pub mod yuva_pixmaps;

pub use alpha_type::*;
pub use annotation::annotate;
pub use bbh_factory::*;
pub use bitmap::*;
pub use blend_mode::*;
pub use blender::*;
pub use blur_types::*;
pub use canvas::{AutoCanvasRestore, Canvas, OwnedCanvas};
pub use clip_op::*;
pub use color::*;
pub use color_filter::{color_filters, ColorFilter};
pub use color_space::*;
pub use color_type::*;
pub use contour_measure::{ContourMeasure, ContourMeasureIter};
pub use coverage_mode::*;
pub use cubic_map::*;
pub use data::*;
pub use data_table::*;
pub use deferred_display_list::*;
pub use deferred_display_list_recorder::*;
pub use document::Document;
pub use drawable::Drawable;
pub use flattenable::*;
pub use font::Font;
pub use font_arguments::FontArguments;
pub use font_metrics::FontMetrics;
pub use font_mgr::*;
pub use font_style::FontStyle;
pub use font_types::*;
pub use image::{images, Image};
pub use image_filter::ImageFilter;
pub use image_generator::*;
pub use image_info::*;
pub use m44::*;
pub use mask_filter::*;
pub use matrix::Matrix;
pub use milestone::*;
pub use paint::Paint;
pub use tile_mode::*;
// We keep these around for the time being.
pub use paint::Cap as PaintCap;
pub use paint::Join as PaintJoin;
pub use paint::Style as PaintStyle;
pub use path::Path;
pub use path_builder::PathBuilder;
pub use path_effect::PathEffect;
pub use path_measure::PathMeasure;
pub use path_types::*;
pub use picture::*;
pub use picture_recorder::PictureRecorder;
pub use pixel_ref::*;
pub use pixmap::*;
pub use point::*;
pub use point3::*;
pub use promise_image_texture::*;
pub use raster_handle_allocator::*;
pub use rect::*;
pub use region::Region;
pub use rrect::RRect;
pub use rsxform::*;
#[allow(deprecated)]
pub use sampling_options::{
    CubicResampler, FilterMode, FilterOptions, MipmapMode, SamplingMode, SamplingOptions,
};
pub use scalar_::*;
pub use shader::{shaders, Shader};
pub use size::*;
pub use stroke_rec::StrokeRec;
pub use surface::Surface;
pub use surface_characterization::*;
pub use surface_props::*;
pub use swizzle::*;
pub use text_blob::*;
pub use texture_compression_type::*;
pub use time::*;
pub use trace_memory_dump::*;
pub use typeface::Typeface;
pub use types::*;
pub use un_pre_multiply::*;
pub use vertices::Vertices;
pub use yuva_info::YUVAInfo;
pub use yuva_pixmaps::{yuva_pixmap_info, YUVAPixmapInfo, YUVAPixmaps};

//
// Skia specific traits used for overloading functions.
//

pub trait Contains<T> {
    fn contains(&self, other: T) -> bool;
}

pub trait QuickReject<T> {
    fn quick_reject(&self, other: &T) -> bool;
}
