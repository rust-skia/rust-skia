//! Ports the examples of Skia's SkBlendMode overview page
//! (`site/docs/user/api/SkBlendMode_Overview.md` in the Skia repository).
//!
//! Each example draws a destination glyph "&" and then an orange source triangle over it with a
//! blend mode, and labels the result with the blend mode's name. The source is either a bitmap that
//! holds the triangle or the triangle's geometry. The page shows only images, so the code was taken
//! from the fiddles behind them and translated from Skia's pre-2020 API.

use std::path;

use skia_safe::{
    BlendMode, Canvas, Color, Font, FontMgr, FontStyle, Image, Paint, Path, Point, scalar,
    surfaces, utils::text_utils::Align,
};

use crate::{DrawingDriver, helper::default_typeface};

pub fn draw(driver: &mut impl DrawingDriver, path: &path::Path) {
    let path = &path.join("SkBlendMode-Overview");

    driver.draw_image((480, 330), path, "01-porter-duff-bitmap", |canvas| {
        draw_modes(canvas, &PORTER_DUFF_MODES, Source::Bitmap, 80.0)
    });
    driver.draw_image((480, 330), path, "02-porter-duff-geometry", |canvas| {
        draw_modes(canvas, &PORTER_DUFF_MODES, Source::Geometry, 80.0)
    });
    driver.draw_image((480, 330), path, "03-lighten-darken", |canvas| {
        draw_modes(canvas, &LIGHTEN_DARKEN_MODES, Source::Geometry, 90.0)
    });
    driver.draw_image((480, 110), path, "04-modulate", draw_modulate);
    driver.draw_image((480, 110), path, "05-color-blends", |canvas| {
        draw_modes(canvas, &COLOR_BLEND_MODES, Source::Geometry, 90.0)
    });
}

const PORTER_DUFF_MODES: [BlendMode; 12] = [
    BlendMode::Src,
    BlendMode::SrcATop,
    BlendMode::SrcOver,
    BlendMode::SrcIn,
    BlendMode::SrcOut,
    BlendMode::Dst,
    BlendMode::DstATop,
    BlendMode::DstOver,
    BlendMode::DstIn,
    BlendMode::DstOut,
    BlendMode::Clear,
    BlendMode::Xor,
];

const LIGHTEN_DARKEN_MODES: [BlendMode; 12] = [
    BlendMode::Plus,
    BlendMode::Screen,
    BlendMode::Overlay,
    BlendMode::Darken,
    BlendMode::Lighten,
    BlendMode::ColorDodge,
    BlendMode::ColorBurn,
    BlendMode::HardLight,
    BlendMode::SoftLight,
    BlendMode::Difference,
    BlendMode::Exclusion,
    BlendMode::Multiply,
];

const COLOR_BLEND_MODES: [BlendMode; 4] = [
    BlendMode::Hue,
    BlendMode::Saturation,
    BlendMode::Color,
    BlendMode::Luminosity,
];

/// The number of blend mode cells that one row of an example holds.
const CELLS_PER_ROW: usize = 5;

/// The vertical distance between two rows of cells.
const ROW_HEIGHT: scalar = 100.0;

/// How the source triangle is drawn over the destination glyph.
#[derive(Copy, Clone)]
enum Source {
    /// Draws an image that holds the triangle, so the transparent pixels around the triangle are
    /// blended, too.
    Bitmap,
    /// Draws the triangle's path, so only the pixels inside the triangle are blended.
    Geometry,
}

/// Draws one cell per blend mode, `cell_width` apart, and starts a new row after every
/// [`CELLS_PER_ROW`] cells.
fn draw_modes(canvas: &Canvas, modes: &[BlendMode], source: Source, cell_width: scalar) {
    let scene = Scene::new();
    canvas.draw_color(Color::TRANSPARENT, BlendMode::Clear);
    for (i, &mode) in modes.iter().enumerate() {
        let column = (i % CELLS_PER_ROW) as scalar;
        let row = (i / CELLS_PER_ROW) as scalar;
        canvas.save();
        canvas.translate((column * cell_width, row * ROW_HEIGHT));
        scene.draw_destination(canvas);
        scene.draw_source(canvas, source, mode);
        scene.draw_label(canvas, mode.name(), 100.0);
        canvas.restore();
    }
}

/// Draws the modulate blend mode once with the bitmap source and once with the geometry source,
/// each with a heading that names the source.
fn draw_modulate(canvas: &Canvas) {
    let scene = Scene::new();
    canvas.draw_color(Color::TRANSPARENT, BlendMode::Clear);
    for (source, heading) in [(Source::Bitmap, "Bitmap"), (Source::Geometry, "Geometry")] {
        scene.draw_destination(canvas);
        scene.draw_source(canvas, source, BlendMode::Modulate);
        scene.draw_label(canvas, heading, 18.0);
        scene.draw_label(canvas, BlendMode::Modulate.name(), 100.0);
        canvas.translate((120.0, 0.0));
    }
}

/// The paints, fonts, and shapes that every example cell draws with.
struct Scene {
    source_paint: Paint,
    source_path: Path,
    source_image: Image,
    destination_font: Font,
    destination_paint: Paint,
    label_font: Font,
    label_paint: Paint,
}

impl Scene {
    fn new() -> Self {
        let mut source_paint = Paint::default();
        source_paint
            .set_anti_alias(true)
            .set_color(Color::from(0xFFCC6633));

        let points: [Point; 3] = [
            (20.0, 20.0).into(),
            (80.0, 45.0).into(),
            (45.0, 80.0).into(),
        ];
        let source_path = Path::polygon(&points, true, None, None);

        let mut source_surface = surfaces::raster_n32_premul((80, 84)).unwrap();
        let source_canvas = source_surface.canvas();
        source_canvas.clear(Color::TRANSPARENT);
        source_canvas.draw_path(&source_path, &source_paint);
        let source_image = source_surface.image_snapshot();

        // The fiddles ask for Roboto Bold, which is not installed everywhere. The default family
        // in bold takes its place.
        let bold = FontMgr::new()
            .legacy_make_typeface(None, FontStyle::bold())
            .unwrap();
        let destination_font = Font::from_typeface(bold, 80.0);
        let mut destination_paint = Paint::default();
        destination_paint
            .set_anti_alias(true)
            .set_color(Color::from(0xFF606080));

        let label_font = Font::from_typeface(default_typeface(), 16.0);
        let mut label_paint = Paint::default();
        label_paint.set_anti_alias(true);

        Self {
            source_paint,
            source_path,
            source_image,
            destination_font,
            destination_paint,
            label_font,
            label_paint,
        }
    }

    fn draw_destination(&self, canvas: &Canvas) {
        canvas.draw_str_align(
            "&",
            (50.0, 80.0),
            &self.destination_font,
            &self.destination_paint,
            Align::Center,
        );
    }

    fn draw_source(&self, canvas: &Canvas, source: Source, mode: BlendMode) {
        let mut paint = self.source_paint.clone();
        paint.set_blend_mode(mode);
        match source {
            Source::Bitmap => canvas.draw_image(&self.source_image, (0.0, 0.0), Some(&paint)),
            Source::Geometry => canvas.draw_path(&self.source_path, &paint),
        };
    }

    /// Draws `text` centered on the cell, with its baseline at `baseline`.
    fn draw_label(&self, canvas: &Canvas, text: &str, baseline: scalar) {
        canvas.draw_str_align(
            text,
            (50.0, baseline),
            &self.label_font,
            &self.label_paint,
            Align::Center,
        );
    }
}
