use skia_safe::{color_filters, BlendMode, Color, ColorFilter, ConditionallySend, Sendable};

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
        font_parameters, image::CubicResampler, image_filter, path, path_effect, region, typeface,
        vertices, Bitmap, Canvas, Color, ColorFilter, ColorInfo, ColorSpace, ContourMeasure,
        ContourMeasureIter, CubicMap, Data, DataTable, DeferredDisplayList,
        DeferredDisplayListRecorder, Document, Drawable, FilterOptions, Font, FontArguments,
        FontMetrics, FontMgr, FontStyle, FontStyleSet, Image, ImageFilter, ImageGenerator,
        ImageInfo, MaskFilter, Matrix, OwnedCanvas, Paint, Path, PathBuilder, PathEffect,
        PathMeasure, Picture, PictureRecorder, PixelRef, Pixmap, RRect, RSXform, Region, Shader,
        Surface, SurfaceCharacterization, SurfaceProps, TextBlob, TextBlobBuilder, TextBlobIter,
        TextBlobRun, Typeface, Vertices, YUVAIndex, YUVASizeInfo, M44,
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
    assert_impl_all!(CubicResampler: Send, Sync);
    assert_impl_all!(Data: Send, Sync);
    assert_impl_all!(DataTable: Send, Sync);
    assert_impl_all!(DeferredDisplayList: Send, Sync);
    assert_not_impl_any!(DeferredDisplayListRecorder: Send, Sync);
    assert_not_impl_any!(Document: Send, Sync);
    assert_not_impl_any!(Drawable: Send, Sync);
    assert_impl_all!(FilterOptions: Send, Sync);
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
    assert_impl_all!(PathBuilder: Send, Sync);
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
    assert_not_impl_any!(pdf::StructureElementNode: Send, Sync);
    assert_not_impl_any!(pdf::Metadata: Send, Sync);
}

mod effects {
    use skia_safe::{runtime_effect, RuntimeEffect};
    use static_assertions::*;

    assert_impl_all!(runtime_effect::Uniform: Send, Sync);
    assert_not_impl_any!(RuntimeEffect: Send, Sync);
}

#[cfg(feature = "gpu")]
mod gpu {
    use skia_safe::gpu::{
        BackendFormat, BackendRenderTarget, BackendSurfaceMutableState, BackendTexture, Context,
        ContextOptions, DirectContext, DriverBugWorkarounds, RecordingContext,
    };
    use static_assertions::*;
    assert_impl_all!(BackendFormat: Send, Sync);
    assert_impl_all!(BackendTexture: Send, Sync);
    assert_impl_all!(BackendRenderTarget: Send, Sync);
    assert_impl_all!(BackendSurfaceMutableState: Send, Sync);
    assert_impl_all!(ContextOptions: Send, Sync);
    assert_impl_all!(DriverBugWorkarounds: Send, Sync);
    // The Context implementation checks for single ownership before mutation, so
    // no Send and Sync can be supported.
    // If RC is 1, it can be sent to other threads with `Sendable` / `ConditionallySend`.
    assert_not_impl_any!(Context: Send, Sync);
    assert_not_impl_any!(DirectContext: Send, Sync);
    assert_not_impl_any!(RecordingContext: Send, Sync);

    #[cfg(feature = "gl")]
    mod gl {
        use skia_safe::gpu::gl::{Extensions, FramebufferInfo, Interface, TextureInfo};
        use static_assertions::*;
        assert_impl_all!(Extensions: Send, Sync);
        // RC & mutable (extensions_mut() ... we could make this function unsafe)
        assert_not_impl_any!(Interface: Send, Sync);
        assert_impl_all!(TextureInfo: Send, Sync);
        assert_impl_all!(FramebufferInfo: Send, Sync);
    }

    #[cfg(feature = "metal")]
    mod mtl {
        use skia_safe::gpu::mtl::TextureInfo;
        use static_assertions::*;
        assert_impl_all!(TextureInfo: Send, Sync);
    }

    #[cfg(feature = "vulkan")]
    mod vulkan {
        use skia_safe::gpu::vk::{
            Alloc, BackendContext, DrawableInfo, GetProcOf, ImageInfo, YcbcrConversionInfo,
        };
        use skia_safe::gpu::BackendDrawableInfo;
        use static_assertions::*;
        // TODO: BackendContext is referencing get_proc and is used only temporarily for building
        //       the context.
        assert_not_impl_any!(BackendContext: Send, Sync);
        // already Copy & Clone, and highly unsafe.
        assert_impl_all!(Alloc: Send, Sync);
        assert_impl_all!(YcbcrConversionInfo: Send, Sync);
        assert_impl_all!(ImageInfo: Send, Sync);
        // GetProc could be Send & Sync , but does it make sense (it's already Copy & Clone)
        assert_not_impl_any!(GetProcOf: Send, Sync);
        assert_impl_all!(DrawableInfo: Send, Sync);
        assert_impl_all!(BackendDrawableInfo: Send, Sync);
        // Note that we can't make most of vk.rs re-export of native Vulkan types Send nor Sync,
        // because they are just re-exports of simple pointers, which already implements a negative
        // Send & Sync that can not be overriden...
    }

    #[cfg(feature = "d3d")]
    mod d3d {
        use skia_safe::gpu::d3d::{BackendContext, FenceInfo, TextureResourceInfo};
        use static_assertions::*;
        // not sure if BackendContext is Sync, so we'd set it to Send only for now.
        assert_impl_all!(BackendContext: Send);
        assert_not_impl_any!(BackendContext: Sync);
        assert_impl_all!(TextureResourceInfo: Send, Sync);
        assert_impl_all!(FenceInfo: Send, Sync);
    }
}

#[cfg(feature = "textlayout")]
mod textlayout {
    use skia_safe::textlayout::{
        Block, Decoration, FontCollection, FontFamilies, FontFeature, Paragraph, ParagraphBuilder,
        ParagraphCache, Placeholder, PlaceholderStyle, StrutStyle, TextShadow, TextStyle,
        TypefaceFontProvider, TypefaceFontStyleSet,
    };
    use static_assertions::*;

    // RC _and_ mutable, forbid shared mutability.
    assert_not_impl_any!(FontCollection: Send, Sync);
    // ParagraphCache seems to be fully thread safe, but I don't think it is itself meant to be shared between threads.
    assert_not_impl_any!(ParagraphCache: Send, Sync);
    assert_impl_all!(Paragraph: Send, Sync);
    assert_impl_all!(ParagraphBuilder: Send, Sync);
    assert_impl_all!(StrutStyle: Send, Sync);
    assert_impl_all!(TextShadow: Send, Sync);
    assert_impl_all!(Decoration: Send, Sync);
    assert_impl_all!(FontFeature: Send, Sync);
    assert_impl_all!(PlaceholderStyle: Send, Sync);
    assert_impl_all!(TextStyle: Send, Sync);
    assert_impl_all!(Block: Send, Sync);
    assert_impl_all!(Placeholder: Send, Sync);
    assert_not_impl_any!(TypefaceFontStyleSet: Send, Sync);
    assert_not_impl_any!(TypefaceFontProvider: Send, Sync);
    assert_not_impl_any!(FontFamilies: Send, Sync);
}

#[cfg(feature = "textlayout")]
mod shaper {
    use skia_safe::shaper::{
        BiDiRunIterator, FontRunIterator, LanguageRunIterator, ScriptRunIterator,
        TextBlobBuilderRunHandler,
    };
    use skia_safe::Shaper;
    use static_assertions::*;
    assert_impl_all!(Shaper: Send, Sync);
    assert_not_impl_any!(FontRunIterator: Send, Sync);
    assert_not_impl_any!(BiDiRunIterator: Send, Sync);
    assert_not_impl_any!(ScriptRunIterator: Send, Sync);
    assert_not_impl_any!(LanguageRunIterator: Send, Sync);
    assert_not_impl_any!(TextBlobBuilderRunHandler: Send, Sync);
}

mod pathops {
    use skia_safe::OpBuilder;
    use static_assertions::*;
    assert_impl_all!(OpBuilder: Send, Sync);
}

mod svg {
    use skia_safe::svg::Canvas;
    use static_assertions::*;
    assert_not_impl_any!(Canvas: Send, Sync);
}

mod utils {
    use skia_safe::utils::{interpolator::TimeToT, CustomTypefaceBuilder, Interpolator};
    use static_assertions::*;
    assert_impl_all!(CustomTypefaceBuilder: Send, Sync);
    assert_impl_all!(Interpolator: Send, Sync);
    assert_impl_all!(TimeToT: Send, Sync);
}

pub mod assert {
    pub fn send<T: Send>() {}
    pub fn sync<T: Sync>() {}
}
