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

mod bbh_factory;
pub use self::bbh_factory::*;

mod bitmap;
pub use self::bitmap::*;

mod blend_mode;
pub use self::blend_mode::*;

mod blur_types;
pub use self::blur_types::*;

mod canvas;
pub use self::canvas::*;

mod clip_op;
pub use self::clip_op::*;

mod color;
pub use self::color::*;

mod color_filter;
pub use self::color_filter::*;

mod color_space;
pub use self::color_space::*;

pub mod contour_measure;
pub use self::contour_measure::*;

mod coverage_mode;
pub use self::coverage_mode::*;

mod cubic_map;
pub use self::cubic_map::*;

mod data;
pub use self::data::*;

pub(crate) mod document;
pub use self::document::*;

mod draw_looper;
pub use self::draw_looper::*;

mod encoded_image_format;
pub use self::encoded_image_format::*;

mod filter_quality;
pub use self::filter_quality::*;

mod font;
pub use self::font::*;

mod font_arguments;
pub use self::font_arguments::*;

mod font_metrics;
pub use self::font_metrics::*;

mod font_mgr;
pub use self::font_mgr::*;

pub mod font_parameters;

mod font_style;
pub use self::font_style::*;

mod font_types;
pub use self::font_types::*;

mod image;
pub use self::image::*;

mod image_filter;
pub use self::image_filter::*;

mod image_info;
pub use self::image_info::*;

mod mask_filter;
pub use self::mask_filter::*;

mod matrix;
pub use self::matrix::*;

mod matrix44;
pub use self::matrix44::*;

mod matrix_typemask;
pub use self::matrix_typemask::*;

mod paint;
pub use self::paint::*;

mod path;
pub use self::path::*;

mod path_effect;
pub use self::path_effect::*;

pub mod path_measure;
pub use self::path_measure::*;

mod picture;
pub use self::picture::*;

mod picture_recorder;
pub use self::picture_recorder::*;

mod point;
pub use self::point::*;

mod point3;
pub use self::point3::*;

mod rect;
pub use self::rect::*;

mod region;
pub use self::region::*;

mod rrect;
pub use self::rrect::*;

mod shader;
pub use self::shader::*;

mod size;
pub use self::size::*;

mod stroke_rec;
pub use self::stroke_rec::*;

mod surface;
pub use self::surface::*;

mod surface_props;
pub use self::surface_props::*;

mod text_blob;
pub use self::text_blob::*;

mod time;
pub use self::time::*;

mod typeface;
pub use self::typeface::*;

mod types;
pub use self::types::*;

mod vector4;
pub use self::vector4::*;

mod vertices;
pub use self::vertices::*;

mod yuva_index;
pub use self::yuva_index::*;

//
// Skia specific traits used for overloading.
//

pub trait Contains<T> {
    fn contains(&self, other: T) -> bool;
}

pub trait QuickReject<T> {
    fn quick_reject(&self, other: &T) -> bool;
}
