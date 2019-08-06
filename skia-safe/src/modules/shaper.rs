use crate::prelude::*;
use crate::shaper::run_handler::RunInfo;
use crate::{scalar, Font, FontMgr, FourByteTag, Point, TextBlob};
pub use run_handler::RunHandler;
use skia_bindings::{
    C_RustRunHandler_construct, C_SkShaper_BiDiRunIterator_currentLevel,
    C_SkShaper_FontRunIterator_currentFont, C_SkShaper_LanguageRunIterator_currentLanguage,
    C_SkShaper_Make, C_SkShaper_MakeFontMgrRunIterator, C_SkShaper_MakeHbIcuScriptRunIterator,
    C_SkShaper_MakeIcuBidiRunIterator, C_SkShaper_MakePrimitive, C_SkShaper_MakeShapeThenWrap,
    C_SkShaper_MakeShaperDrivenWrapper, C_SkShaper_MakeStdLanguageRunIterator,
    C_SkShaper_RunIterator_atEnd, C_SkShaper_RunIterator_consume, C_SkShaper_RunIterator_delete,
    C_SkShaper_RunIterator_endOfCurrentRun, C_SkShaper_ScriptRunIterator_currentScript,
    C_SkShaper_delete, C_SkShaper_shape, C_SkTextBlobBuilderRunHandler_construct,
    C_SkTextBlobBuilderRunHandler_endPoint, C_SkTextBlobBuilderRunHandler_makeBlob,
    RustRunHandler_Param, SkShaper, SkShaper_BiDiRunIterator, SkShaper_FontRunIterator,
    SkShaper_LanguageRunIterator, SkShaper_RunHandler_Buffer, SkShaper_RunHandler_RunInfo,
    SkShaper_RunIterator, SkShaper_ScriptRunIterator, SkTextBlobBuilderRunHandler, TraitObject,
};
use std::ffi::{CStr, CString};
use std::mem;
use std::pin::Pin;

pub struct Shaper(*mut SkShaper);

impl NativeAccess<SkShaper> for Shaper {
    fn native(&self) -> &SkShaper {
        unsafe { &*self.0 }
    }

    fn native_mut(&mut self) -> &mut SkShaper {
        unsafe { &mut *self.0 }
    }
}

impl Drop for Shaper {
    fn drop(&mut self) {
        unsafe { C_SkShaper_delete(self.0) }
    }
}

impl Default for Shaper {
    fn default() -> Self {
        Self::new()
    }
}

impl Shaper {
    pub fn new_primitive() -> Self {
        unsafe { C_SkShaper_MakePrimitive() }
            .to_option()
            .map(Shaper)
            .unwrap()
    }

    pub fn new_shaper_driven_wrapper() -> Option<Self> {
        unsafe { C_SkShaper_MakeShaperDrivenWrapper() }
            .to_option()
            .map(Shaper)
    }

    pub fn new_shape_then_wrap() -> Option<Self> {
        unsafe { C_SkShaper_MakeShapeThenWrap() }
            .to_option()
            .map(Shaper)
    }

    pub fn new() -> Self {
        unsafe { C_SkShaper_Make() }
            .to_option()
            .map(Shaper)
            .unwrap()
    }
}

pub trait RunIteratorNativeAccess {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator;
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator;
}

pub trait RunIterator {
    fn consume(&mut self);
    fn end_of_current_run(&self) -> usize;
    fn at_end(&self) -> bool;
}

impl<T> RunIterator for T
where
    T: RunIteratorNativeAccess,
{
    fn consume(&mut self) {
        unsafe { C_SkShaper_RunIterator_consume(self.access_run_iterator_mut()) }
    }

    fn end_of_current_run(&self) -> usize {
        unsafe { C_SkShaper_RunIterator_endOfCurrentRun(self.access_run_iterator()) }
    }

    fn at_end(&self) -> bool {
        unsafe { C_SkShaper_RunIterator_atEnd(self.access_run_iterator()) }
    }
}

pub struct FontRunIterator(*mut SkShaper_FontRunIterator);

impl Drop for FontRunIterator {
    fn drop(&mut self) {
        unsafe { C_SkShaper_RunIterator_delete(self.access_run_iterator_mut()) }
    }
}

impl NativeAccess<SkShaper_FontRunIterator> for FontRunIterator {
    fn native(&self) -> &SkShaper_FontRunIterator {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut SkShaper_FontRunIterator {
        unsafe { &mut *self.0 }
    }
}

impl RunIteratorNativeAccess for FontRunIterator {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator {
        &self.native()._base
    }
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator {
        &mut self.native_mut()._base
    }
}

impl FontRunIterator {
    pub fn current_font(&self) -> &Font {
        Font::from_native_ref(unsafe { &*C_SkShaper_FontRunIterator_currentFont(self.native()) })
    }
}

impl Shaper {
    pub fn new_font_mgr_run_iterator(
        utf8: &str,
        font: &Font,
        mut fallback: Option<&mut FontMgr>,
    ) -> FontRunIterator {
        let bytes = utf8.as_bytes();
        FontRunIterator(unsafe {
            C_SkShaper_MakeFontMgrRunIterator(
                bytes.as_ptr() as _,
                bytes.len(),
                font.native(),
                fallback.shared_ptr_mut(),
            )
        })
    }
}

pub struct BiDiRunIterator(*mut SkShaper_BiDiRunIterator);

impl Drop for BiDiRunIterator {
    fn drop(&mut self) {
        unsafe { C_SkShaper_RunIterator_delete(self.access_run_iterator_mut()) }
    }
}

impl NativeAccess<SkShaper_BiDiRunIterator> for BiDiRunIterator {
    fn native(&self) -> &SkShaper_BiDiRunIterator {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut SkShaper_BiDiRunIterator {
        unsafe { &mut *self.0 }
    }
}

impl RunIteratorNativeAccess for BiDiRunIterator {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator {
        &self.native()._base
    }
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator {
        &mut self.native_mut()._base
    }
}

impl BiDiRunIterator {
    pub fn current_level(&self) -> u8 {
        unsafe { C_SkShaper_BiDiRunIterator_currentLevel(self.native()) }
    }
}

impl Shaper {
    pub fn new_icu_bidi_run_iterator(utf8: impl AsRef<str>, level: u8) -> Option<BiDiRunIterator> {
        let bytes = utf8.as_ref().as_bytes();
        unsafe { C_SkShaper_MakeIcuBidiRunIterator(bytes.as_ptr() as _, bytes.len(), level) }
            .to_option()
            .map(BiDiRunIterator)
    }
}

pub struct ScriptRunIterator(*mut SkShaper_ScriptRunIterator);

impl Drop for ScriptRunIterator {
    fn drop(&mut self) {
        unsafe { C_SkShaper_RunIterator_delete(self.access_run_iterator_mut()) }
    }
}

impl NativeAccess<SkShaper_ScriptRunIterator> for ScriptRunIterator {
    fn native(&self) -> &SkShaper_ScriptRunIterator {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut SkShaper_ScriptRunIterator {
        unsafe { &mut *self.0 }
    }
}

impl RunIteratorNativeAccess for ScriptRunIterator {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator {
        &self.native()._base
    }
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator {
        &mut self.native_mut()._base
    }
}

impl ScriptRunIterator {
    pub fn current_script(&self) -> FourByteTag {
        FourByteTag::from_native(unsafe {
            C_SkShaper_ScriptRunIterator_currentScript(self.native())
        })
    }
}

impl Shaper {
    pub fn new_hb_icu_script_run_iterator(utf8: impl AsRef<str>) -> ScriptRunIterator {
        let bytes = utf8.as_ref().as_bytes();
        unsafe { C_SkShaper_MakeHbIcuScriptRunIterator(bytes.as_ptr() as _, bytes.len()) }
            .to_option()
            .map(ScriptRunIterator)
            .unwrap()
    }
}

pub struct LanguageRunIterator(*mut SkShaper_LanguageRunIterator);

impl Drop for LanguageRunIterator {
    fn drop(&mut self) {
        unsafe { C_SkShaper_RunIterator_delete(self.access_run_iterator_mut()) }
    }
}

impl NativeAccess<SkShaper_LanguageRunIterator> for LanguageRunIterator {
    fn native(&self) -> &SkShaper_LanguageRunIterator {
        unsafe { &*self.0 }
    }
    fn native_mut(&mut self) -> &mut SkShaper_LanguageRunIterator {
        unsafe { &mut *self.0 }
    }
}

impl RunIteratorNativeAccess for LanguageRunIterator {
    fn access_run_iterator(&self) -> &SkShaper_RunIterator {
        &self.native()._base
    }
    fn access_run_iterator_mut(&mut self) -> &mut SkShaper_RunIterator {
        &mut self.native_mut()._base
    }
}

impl LanguageRunIterator {
    pub fn current_language(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(C_SkShaper_LanguageRunIterator_currentLanguage(
                self.native(),
            ))
        }
    }
}

impl Shaper {
    pub fn new_std_language_run_iterator(utf8: impl AsRef<str>) -> Option<LanguageRunIterator> {
        let bytes = utf8.as_ref().as_bytes();
        unsafe { C_SkShaper_MakeStdLanguageRunIterator(bytes.as_ptr() as _, bytes.len()) }
            .to_option()
            .map(LanguageRunIterator)
    }
}

mod run_handler {
    use crate::prelude::*;
    use crate::{Font, GlyphId, Point, Vector};
    use skia_bindings::{SkShaper_RunHandler_Buffer, SkShaper_RunHandler_RunInfo};
    use std::ops::Range;

    pub trait RunHandler {
        fn begin_line(&mut self);
        fn run_info(&mut self, info: &RunInfo);
        fn commit_run_info(&mut self);
        fn run_buffer<'a>(&'a mut self, info: &RunInfo) -> Buffer<'a>;
        fn commit_run_buffer(&mut self, info: &RunInfo);
        fn commit_line(&mut self);
    }

    pub struct RunInfo<'a> {
        pub font: &'a Font,
        pub bidi_level: u8,
        pub advance: Vector,
        pub glyph_count: usize,
        pub utf8_range: Range<usize>,
    }

    impl<'a> RunInfo<'a> {
        pub(crate) fn from_native(ri: &SkShaper_RunHandler_RunInfo) -> Self {
            // TODO: should we avoid that copy and wrap RunInfo with functions?
            let utf8_range = ri.utf8Range;
            RunInfo {
                font: Font::from_native_ref(unsafe { &*ri.fFont }),
                bidi_level: ri.fBidiLevel,
                advance: Vector::from_native(ri.fAdvance),
                glyph_count: ri.glyphCount,
                utf8_range: utf8_range.fBegin..utf8_range.fBegin + utf8_range.fSize,
            }
        }
    }

    #[derive(Debug)]
    pub struct Buffer<'a> {
        pub glyphs: &'a mut [GlyphId],
        pub positions: &'a mut [Point],
        pub offsets: Option<&'a mut [Point]>,
        pub clusters: Option<&'a mut [u32]>,
        pub point: Point,
    }

    impl<'a> Buffer<'a> {
        pub fn new(
            glyphs: &'a mut [GlyphId],
            positions: &'a mut [Point],
            point: impl Into<Option<Point>>,
        ) -> Self {
            Buffer {
                glyphs,
                positions,
                offsets: None,
                clusters: None,
                point: point.into().unwrap_or_default(),
            }
        }

        pub(crate) fn native_buffer_mut(
            &mut self,
            glyph_count: usize,
        ) -> SkShaper_RunHandler_Buffer {
            assert_eq!(self.glyphs.len(), glyph_count);
            assert_eq!(self.positions.len(), glyph_count);
            if let Some(offsets) = &self.offsets {
                assert_eq!(offsets.len(), glyph_count)
            }
            if let Some(clusters) = &self.clusters {
                assert_eq!(clusters.len(), glyph_count)
            }
            SkShaper_RunHandler_Buffer {
                glyphs: self.glyphs.as_mut_ptr(),
                positions: self.positions.native_mut().as_mut_ptr(),
                offsets: self.offsets.native_mut().as_ptr_or_null_mut(),
                clusters: self.clusters.as_ptr_or_null_mut(),
                point: self.point.into_native(),
            }
        }
    }
}

impl Shaper {
    // TODO: SkShaper::shape() with non-standard run iterators.

    pub fn shape(
        &self,
        utf8: impl AsRef<str>,
        font: &Font,
        left_to_right: bool,
        width: scalar,
        run_handler: &mut dyn RunHandler,
    ) {
        let param = RustRunHandler_Param {
            trait_: unsafe { mem::transmute(run_handler) },
            beginLine: Some(begin_line),
            runInfo: Some(run_info),
            commitRunInfo: Some(commit_run_info),
            runBuffer: Some(run_buffer),
            commitRunBuffer: Some(commit_run_buffer),
            commitLine: Some(commit_line),
        };

        extern "C" fn begin_line(to: TraitObject) {
            to_run_handler(to).begin_line()
        }

        extern "C" fn run_info(to: TraitObject, ri: *const SkShaper_RunHandler_RunInfo) {
            to_run_handler(to).run_info(&RunInfo::from_native(unsafe { &*ri }));
        }

        extern "C" fn commit_run_info(to: TraitObject) {
            to_run_handler(to).commit_run_info()
        }

        extern "C" fn run_buffer(
            to: TraitObject,
            ri: *const SkShaper_RunHandler_RunInfo,
        ) -> SkShaper_RunHandler_Buffer {
            let ri = unsafe { &*ri };
            to_run_handler(to)
                .run_buffer(&RunInfo::from_native(ri))
                .native_buffer_mut(ri.glyphCount)
        }

        extern "C" fn commit_run_buffer(to: TraitObject, ri: *const SkShaper_RunHandler_RunInfo) {
            to_run_handler(to).commit_run_buffer(&RunInfo::from_native(unsafe { &*ri }))
        }

        extern "C" fn commit_line(to: TraitObject) {
            to_run_handler(to).commit_line()
        }

        fn to_run_handler<'a>(to: TraitObject) -> &'a mut dyn RunHandler {
            unsafe { mem::transmute(to) }
        }

        let bytes = utf8.as_ref().as_bytes();

        unsafe {
            let mut run_handler = construct(|rh| C_RustRunHandler_construct(rh, &param));

            C_SkShaper_shape(
                self.native(),
                bytes.as_ptr() as _,
                bytes.len(),
                font.native(),
                left_to_right,
                width,
                &mut run_handler._base,
            )
        }
    }
}

// TODO: Is there a way around converting and storing the CString here?
#[repr(C)]
pub struct TextBlobBuilderRunHandler(Pin<CString>, SkTextBlobBuilderRunHandler);

impl NativeAccess<SkTextBlobBuilderRunHandler> for TextBlobBuilderRunHandler {
    fn native(&self) -> &SkTextBlobBuilderRunHandler {
        &self.1
    }
    fn native_mut(&mut self) -> &mut SkTextBlobBuilderRunHandler {
        &mut self.1
    }
}

impl TextBlobBuilderRunHandler {
    pub fn new(text: impl AsRef<str>, offset: impl Into<Point>) -> TextBlobBuilderRunHandler {
        let c_string = Pin::new(CString::new(text.as_ref()).unwrap());
        let ptr = c_string.as_ptr();
        /* does not link:
        TextBlobBuilderRunHandler(c_string, unsafe {
            SkTextBlobBuilderRunHandler::new(ptr, offset.into().into_native())
        }) */
        let run_handler = construct(|rh| unsafe {
            C_SkTextBlobBuilderRunHandler_construct(rh, ptr, offset.into().native())
        });
        TextBlobBuilderRunHandler(c_string, run_handler)
    }

    pub fn make_blob(&mut self) -> Option<TextBlob> {
        TextBlob::from_ptr(unsafe { C_SkTextBlobBuilderRunHandler_makeBlob(self.native_mut()) })
    }

    pub fn end_point(&mut self) -> Point {
        // .endPoint() does not link.
        Point::from_native(unsafe { C_SkTextBlobBuilderRunHandler_endPoint(self.native_mut()) })
    }
}

impl Shaper {
    pub fn shape_text_blob(
        &self,
        text: impl AsRef<str>,
        font: &Font,
        left_to_right: bool,
        width: scalar,
        offset: impl Into<Point>,
    ) -> Option<(TextBlob, Point)> {
        let text = text.as_ref();
        let bytes = text.as_bytes();
        let mut builder = TextBlobBuilderRunHandler::new(text, offset);
        unsafe {
            C_SkShaper_shape(
                self.native(),
                bytes.as_ptr() as _,
                bytes.len(),
                font.native(),
                left_to_right,
                width,
                &mut builder.native_mut()._base,
            )
        };
        builder.make_blob().map(|tb| (tb, builder.end_point()))
    }
}

pub mod icu {
    /// On Windows, this function writes the file `icudtl.dat` into the current's
    /// executable directory making sure that it's available when text shaping is used in Skia.
    ///
    /// If your executable directory can not be written to, make sure that `icudtl.dat` is
    /// available.
    ///
    /// It's currently not possible to load `icudtl.dat` from another location.
    pub fn init() {
        skia_bindings::icu::init()
    }
}

#[cfg(test)]
mod tests {
    use crate::shaper::run_handler::{Buffer, RunInfo};
    use crate::shaper::RunHandler;
    use crate::{Font, GlyphId, Point, Shaper};

    #[derive(Default, Debug)]
    pub struct DebugRunHandler {
        glyphs: Vec<GlyphId>,
        points: Vec<Point>,
    }

    impl RunHandler for DebugRunHandler {
        fn begin_line(&mut self) {
            println!("begin_line");
        }

        fn run_info(&mut self, info: &RunInfo) {
            println!("run_info: {:?} {:?}", info.advance, info.utf8_range);
        }

        fn commit_run_info(&mut self) {
            println!("commit_run_info");
        }

        fn run_buffer<'a>(&'a mut self, info: &RunInfo) -> Buffer {
            println!("run_buffer {}", info.glyph_count);
            let count = info.glyph_count;
            self.glyphs.resize(count, 0);
            self.points.resize(count, Point::default());
            Buffer::new(&mut self.glyphs, &mut self.points, None)
        }

        fn commit_run_buffer(&mut self, _info: &RunInfo) {
            println!("commit_run_buffer");
            println!("state: {:?}", self);
        }

        fn commit_line(&mut self) {
            println!("commit_line");
        }
    }

    #[test]
    fn test_rtl_text_shaping() {
        skia_bindings::icu::init();

        let shaper = Shaper::new();
        shaper.shape(
            "العربية",
            &Font::default(),
            false,
            10000.0,
            &mut DebugRunHandler::default(),
        );
    }
}
