use skia_safe::{color_filters, BlendMode, Color, ColorFilter, Mutable, Sendable};

/// Test if RCHandle<> types can be wrapped into a Sendable and unwrapped.
#[test]
fn sendable() {
    let color = Color::CYAN;
    let mode = BlendMode::ColorBurn;
    let cf = color_filters::blend(color, mode).unwrap();
    let sendable = cf.wrap_send().ok().unwrap();
    let _unwrapped = sendable.unwrap();
}

/// Test if Sendable<> is actually sendable for RCHandle types.
#[test]
fn sendable_implements_send() {
    assert::send::<Sendable<ColorFilter>>();
}

mod codec {
    use skia_safe::Codec;
    use static_assertions::*;

    // Codec seems to call into SkPngChunkReader*
    assert_not_impl_any!(Codec: Send, Sync);
}

mod core {
    use skia_safe::{
        font_parameters, image_filter, path, path_effect, region, typeface, vertices, Bitmap,
        Canvas, Color, ColorFilter, ColorInfo, ColorSpace, ContourMeasure, ContourMeasureIter,
        CubicMap, Data, DataTable, DeferredDisplayList, DeferredDisplayListRecorder, Document,
        Drawable, Font, FontArguments, FontMetrics, FontMgr, FontStyle, FontStyleSet, Image,
        ImageFilter, ImageGenerator, ImageInfo, MaskFilter, Matrix, OwnedCanvas, Paint, Path,
        PathEffect, PathMeasure, Picture, PictureRecorder, PixelRef, Pixmap, RRect, RSXform,
        Region, Shader, Surface, SurfaceCharacterization, SurfaceProps, TextBlob, TextBlobBuilder,
        TextBlobIter, TextBlobRun, Typeface, Vertices, YUVAIndex, YUVASizeInfo, M44,
    };
    use static_assertions::*;

    // SkBitmap is not thread safe. Each thread must have its own copy of SkBitmap fields,
    // although threads may share the underlying pixel array.
    assert_not_impl_any!(Bitmap: Send, Sync);
    assert_not_impl_any!(Canvas: Send, Sync);
    assert_not_impl_any!(OwnedCanvas: Send, Sync);
    assert_impl_all!(Color: Send, Sync);
    assert_impl_all!(ColorFilter: Send, Sync);
    assert_impl_all!(ColorSpace: Send, Sync);
    assert_impl_all!(ContourMeasure: Send, Sync);
    assert_impl_all!(ContourMeasureIter: Send, Sync);
    assert_impl_all!(CubicMap: Send, Sync);
    assert_impl_all!(Data: Send, Sync);
    assert_impl_all!(DataTable: Send, Sync);
    // SkSurface::draw function needs to mutate it
    assert_not_impl_any!(DeferredDisplayList: Send, Sync);
    assert_not_impl_any!(DeferredDisplayListRecorder: Send, Sync);
    assert_not_impl_any!(Document: Send, Sync);
    assert_not_impl_any!(Drawable: Send, Sync);
    assert_impl_all!(Font: Send, Sync);
    assert_not_impl_any!(FontArguments: Send, Sync);
    assert_impl_all!(FontMetrics: Send, Sync);
    assert_not_impl_any!(FontStyleSet: Send, Sync);
    assert_not_impl_any!(FontMgr: Send, Sync);
    assert_impl_all!(font_parameters::variation::Axis: Send, Sync);
    assert_impl_all!(FontStyle: Send, Sync);
    // SkImage cannot be modified after it is created. SkImage may allocate additional
    // storage as needed; for instance, an encoded SkImage may decode when drawn.
    // > So far the implementatio seems to handle the "allocate additional storage as needed"
    // > in a thread safe way.
    assert_impl_all!(Image: Send, Sync);
    assert_impl_all!(image_filter::CropRect: Send, Sync);
    assert_impl_all!(ImageFilter: Send, Sync);
    assert_impl_all!(ImageGenerator: Send, Sync);
    assert_impl_all!(ColorInfo: Send, Sync);
    assert_impl_all!(ImageInfo: Send, Sync);
    assert_impl_all!(M44: Send, Sync);
    assert_impl_all!(MaskFilter: Send, Sync);
    assert_impl_all!(Matrix: Send, Sync);
    assert_impl_all!(Paint: Send, Sync);
    assert_not_impl_any!(path::Iter: Send, Sync);
    assert_impl_all!(Path: Send, Sync);
    assert_impl_all!(path_effect::PointData: Send, Sync);
    assert_impl_all!(path_effect::DashInfo: Send, Sync);
    assert_impl_all!(PathEffect: Send, Sync);
    assert_not_impl_any!(PathMeasure: Send, Sync);
    assert_impl_all!(Picture: Send, Sync);
    assert_not_impl_any!(PictureRecorder: Send, Sync);
    assert_impl_all!(PixelRef: Send, Sync);
    assert_impl_all!(Pixmap: Send, Sync);
    assert_impl_all!(Region: Send, Sync);
    assert_not_impl_any!(region::Iterator: Send, Sync);
    assert_not_impl_any!(region::Cliperator: Send, Sync);
    assert_not_impl_any!(region::Spanerator: Send, Sync);
    assert_impl_all!(RRect: Send, Sync);
    assert_impl_all!(RSXform: Send, Sync);
    assert_impl_all!(Shader: Send, Sync);
    assert_not_impl_any!(Surface: Send, Sync);
    assert_impl_all!(SurfaceCharacterization: Send, Sync);
    assert_impl_all!(SurfaceProps: Send, Sync);
    assert_impl_all!(TextBlob: Send, Sync);
    assert_impl_all!(TextBlobBuilder: Send, Sync);
    assert_not_impl_any!(TextBlobIter: Send, Sync);
    assert_not_impl_any!(TextBlobRun: Send, Sync);
    assert_impl_all!(typeface::LocalizedString: Send, Sync);
    assert_impl_all!(Typeface: Send, Sync);
    assert_not_impl_any!(typeface::LocalizedStringsIter: Send, Sync);
    assert_not_impl_any!(vertices::Attribute: Send, Sync);
    assert_impl_all!(Vertices: Send, Sync);
    assert_impl_all!(vertices::Builder: Send, Sync);
    assert_impl_all!(YUVAIndex: Send, Sync);
    assert_impl_all!(YUVASizeInfo: Send, Sync);
}

mod docs {
    use skia_safe::pdf;
    use static_assertions::*;
    assert_impl_all!(pdf::AttributeList: Send, Sync);
}

pub mod assert {
    pub fn send<T: Send>() {}
    pub fn sync<T: Sync>() {}
}
