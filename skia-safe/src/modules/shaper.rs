//! Script-aware text shaping via Skia's `SkShaper`, producing [`crate::TextBlob`]s (see [`crate::Shaper`]).
use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::os::raw;

use skia_bindings::{
    self as sb, SkShaper, SkShaper_BiDiRunIterator, SkShaper_FontRunIterator,
    SkShaper_LanguageRunIterator, SkShaper_RunHandler, SkShaper_RunIterator,
    SkShaper_ScriptRunIterator, SkTextBlobBuilderRunHandler,
};

use crate::prelude::*;
use crate::{Font, FontMgr, FourByteTag, Point, TextBlob, scalar};

// The following three are re-exported in `modules.rs` via `mod shapers {}`.
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "visionos"))]
pub(crate) mod core_text;
pub(crate) mod harfbuzz;
pub(crate) mod unicode;

pub use run_handler::RunHandler;

pub type Shaper = RefHandle<SkShaper>;
unsafe_send_sync!(Shaper);

impl NativeDelete for SkShaper {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_SkShaper_delete(ptr) }
    }
}

impl Default for Shaper {
    fn default() -> Self {
        Self::new(None)
    }
}

impl fmt::Debug for Shaper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Shaper").finish()
    }
}

impl Shaper {
    #[deprecated(since = "0.74.0", note = "use shapers::primitive::primitive_text()")]
    pub fn new_primitive() -> Self {
        crate::shapers::primitive::primitive_text()
    }

    pub fn new_shaper_driven_wrapper(
        fallback_font_mgr: impl Into<Option<FontMgr>>,
    ) -> Option<Self> {
        crate::shapers::hb::shaper_driven_wrapper(fallback_font_mgr)
    }

    pub fn new_shape_then_wrap(fallback_font_mgr: impl Into<Option<FontMgr>>) -> Option<Self> {
        crate::shapers::hb::shape_then_wrap(fallback_font_mgr)
    }

    #[deprecated(
        since = "0.74.0",
        note = "use shapers::hb::shape_dont_wrap_or_reorder()"
    )]
    pub fn new_shape_dont_wrap_or_reorder(
        fallback_font_mgr: impl Into<Option<FontMgr>>,
    ) -> Option<Self> {
        crate::shapers::hb::shape_dont_wrap_or_reorder(fallback_font_mgr)
    }

    pub fn purge_harf_buzz_cache() {
        unsafe { sb::SkShaper_PurgeHarfBuzzCache() }
    }

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "visionos"))]
    pub fn new_core_text(line_break_mode: crate::shapers::ct::LineBreakMode) -> Self {
        #[cfg(feature = "embed-icudtl")]
        crate::icu::init();

        Self::from_ptr(unsafe { sb::C_SkShaper_MakeCoreText(line_break_mode) }).unwrap()
    }

    pub fn new(font_mgr: impl Into<Option<FontMgr>>) -> Self {
        #[cfg(feature = "embed-icudtl")]
        crate::icu::init();

        Self::from_ptr(unsafe { sb::C_SkShaper_Make(font_mgr.into().into_ptr_or_null()) }).unwrap()
    }

    pub fn purge_caches() {
        unsafe { sb::SkShaper_PurgeCaches() }
    }
}

pub use skia_bindings::SkShaper_Feature as Feature;

pub trait RunIterator {
    /// Set state to that of current run and move iterator to end of that run.
    fn consume(&mut self);
    /// Offset to one past the last (utf8) element in the current run.
    fn end_of_current_run(&self) -> usize;
    /// Return true if [`RunIterator::consume()`] should no longer be called.
    fn at_end(&self) -> bool;
}

impl<T> RunIterator for RefHandle<T>
where
    T: NativeDelete,
    T: NativeBase<SkShaper_RunIterator>,
{
    fn consume(&mut self) {
        unsafe { sb::C_SkShaper_RunIterator_consume(self.native_ptr().cast()) }
    }

    fn end_of_current_run(&self) -> usize {
        unsafe { sb::C_SkShaper_RunIterator_endOfCurrentRun(self.native().base()) }
    }

    fn at_end(&self) -> bool {
        unsafe { sb::C_SkShaper_RunIterator_atEnd(self.native().base()) }
    }
}

pub type FontRunIterator = RefHandle<SkShaper_FontRunIterator>;

impl NativeBase<SkShaper_RunIterator> for SkShaper_FontRunIterator {}

impl NativeDelete for SkShaper_FontRunIterator {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_SkShaper_RunIterator_delete(ptr.cast()) }
    }
}

impl fmt::Debug for FontRunIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FontRunIterator")
            .field("current_font", &self.current_font())
            .finish()
    }
}

impl FontRunIterator {
    pub fn current_font(&self) -> &Font {
        Font::from_native_ref(unsafe {
            &*sb::C_SkShaper_FontRunIterator_currentFont(self.native())
        })
    }
}

impl Shaper {
    pub fn new_font_mgr_run_iterator<'a>(
        utf8: &'a str,
        font: &Font,
        fallback: impl Into<Option<FontMgr>>,
    ) -> Borrows<'a, FontRunIterator> {
        let bytes = utf8.as_bytes();
        FontRunIterator::from_ptr(unsafe {
            sb::C_SkShaper_MakeFontMgrRunIterator(
                bytes.as_ptr() as _,
                bytes.len(),
                font.native(),
                fallback.into().into_ptr_or_null(),
            )
        })
        .unwrap()
        .borrows(utf8)
    }

    // TODO: m79: wrap MakeFontMgrRunIterator with requestName (borrowed), requestStyle and
    //       a LanguageRunIterator.

    pub fn new_trivial_font_run_iterator(font: &Font, utf8_bytes: usize) -> FontRunIterator {
        FontRunIterator::from_ptr(unsafe {
            sb::C_SkShaper_TrivialFontRunIterator_new(font.native(), utf8_bytes)
        })
        .unwrap()
    }
}

pub type BiDiRunIterator = RefHandle<SkShaper_BiDiRunIterator>;

impl NativeBase<SkShaper_RunIterator> for SkShaper_BiDiRunIterator {}

impl NativeDelete for SkShaper_BiDiRunIterator {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_SkShaper_RunIterator_delete(ptr.cast()) }
    }
}

impl fmt::Debug for BiDiRunIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BiDiRunIterator")
            .field("current_level", &self.current_level())
            .finish()
    }
}

impl BiDiRunIterator {
    /// The unicode bidi embedding level (even ltr, odd rtl).
    pub fn current_level(&self) -> u8 {
        unsafe { sb::C_SkShaper_BiDiRunIterator_currentLevel(self.native()) }
    }
}

impl Shaper {
    pub fn new_bidi_run_iterator(utf8: &str, bidi_level: u8) -> Option<Borrows<BiDiRunIterator>> {
        let bytes = utf8.as_bytes();
        BiDiRunIterator::from_ptr(unsafe {
            sb::C_SkShaper_MakeBidiRunIterator(bytes.as_ptr() as _, bytes.len(), bidi_level)
        })
        .map(|i| i.borrows(utf8))
    }

    pub fn new_icu_bidi_run_iterator(utf8: &str, level: u8) -> Option<Borrows<BiDiRunIterator>> {
        let bytes = utf8.as_bytes();
        BiDiRunIterator::from_ptr(unsafe {
            sb::C_SkShaper_MakeIcuBidiRunIterator(bytes.as_ptr() as _, bytes.len(), level)
        })
        .map(|i| i.borrows(utf8))
    }

    #[deprecated(
        since = "0.74.0",
        note = "use shapers::primitive::trivial_bidi_run_iterator()"
    )]
    pub fn new_trivial_bidi_run_iterator(bidi_level: u8, utf8_bytes: usize) -> BiDiRunIterator {
        shapers::primitive::trivial_bidi_run_iterator(bidi_level, utf8_bytes)
    }
}

pub type ScriptRunIterator = RefHandle<SkShaper_ScriptRunIterator>;

impl NativeBase<SkShaper_RunIterator> for SkShaper_ScriptRunIterator {}

impl NativeDelete for SkShaper_ScriptRunIterator {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_SkShaper_RunIterator_delete(ptr.cast()) }
    }
}

impl fmt::Debug for ScriptRunIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScriptRunIterator")
            .field("current_script", &self.current_script())
            .finish()
    }
}

impl ScriptRunIterator {
    /// Should be ISO 15924 codes.
    pub fn current_script(&self) -> FourByteTag {
        FourByteTag::from_native_c(unsafe {
            sb::C_SkShaper_ScriptRunIterator_currentScript(self.native())
        })
    }
}

impl Shaper {
    pub fn new_script_run_iterator(utf8: &str, script: FourByteTag) -> Borrows<ScriptRunIterator> {
        let bytes = utf8.as_bytes();
        ScriptRunIterator::from_ptr(unsafe {
            sb::C_SkShaper_MakeScriptRunIterator(
                bytes.as_ptr() as _,
                bytes.len(),
                script.into_native(),
            )
        })
        .unwrap()
        .borrows(utf8)
    }

    // TODO: wrap MakeSkUnicodeHbScriptRunIterator (m88: uses type SkUnicode defined in src/).

    pub fn new_hb_icu_script_run_iterator(utf8: &str) -> Borrows<ScriptRunIterator> {
        let bytes = utf8.as_bytes();
        ScriptRunIterator::from_ptr(unsafe {
            sb::C_SkShaper_MakeHbIcuScriptRunIterator(bytes.as_ptr() as _, bytes.len())
        })
        .unwrap()
        .borrows(utf8)
    }

    #[deprecated(
        since = "0.74.0",
        note = "use shapers::primitive::trivial_script_run_iterator"
    )]
    pub fn new_trivial_script_run_iterator(bidi_level: u8, utf8_bytes: usize) -> ScriptRunIterator {
        shapers::primitive::trivial_script_run_iterator(bidi_level, utf8_bytes)
    }
}

pub type LanguageRunIterator = RefHandle<SkShaper_LanguageRunIterator>;

impl NativeBase<SkShaper_RunIterator> for SkShaper_LanguageRunIterator {}

impl NativeDelete for SkShaper_LanguageRunIterator {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_SkShaper_RunIterator_delete(ptr.cast()) }
    }
}

impl fmt::Debug for LanguageRunIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LanguageRunIterator")
            .field("current_language", &self.current_language())
            .finish()
    }
}

impl LanguageRunIterator {
    /// Should be BCP-47; C locale names may also work.
    pub fn current_language(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(sb::C_SkShaper_LanguageRunIterator_currentLanguage(
                self.native(),
            ))
        }
    }
}

impl Shaper {
    pub fn new_std_language_run_iterator(utf8: &str) -> Option<LanguageRunIterator> {
        // a LanguageRunIterator never accesses the UTF8 string, so it's safe to
        // not borrow the string.
        let bytes = utf8.as_bytes();
        LanguageRunIterator::from_ptr(unsafe {
            sb::C_SkShaper_MakeStdLanguageRunIterator(bytes.as_ptr() as _, bytes.len())
        })
    }

    pub fn new_trivial_language_run_iterator(
        language: impl AsRef<str>,
        utf8_bytes: usize,
    ) -> LanguageRunIterator {
        let language = CString::new(language.as_ref()).unwrap();
        LanguageRunIterator::from_ptr(unsafe {
            sb::C_SkShaper_TrivialLanguageRunIterator_new(language.as_ptr(), utf8_bytes)
        })
        .unwrap()
    }
}

pub mod run_handler {
    //! Callback interface that receives shaped runs (glyphs, positions, and advances) as text is shaped.
    use std::{ffi::CStr, ops::Range, slice};

    use skia_bindings::{SkShaper_RunHandler_Buffer, SkShaper_RunHandler_RunInfo};

    use crate::{Font, FourByteTag, GlyphId, Point, Vector, prelude::*};

    pub trait RunHandler {
        /// Called when beginning a line.
        fn begin_line(&mut self);
        /// Called once for each run in a line. Can compute baselines and offsets.
        fn run_info(&mut self, info: &RunInfo);
        /// Called after all [`RunHandler::run_info()`] calls for a line.
        fn commit_run_info(&mut self);
        /// Called for each run in a line after [`RunHandler::commit_run_info()`]. The buffer will be filled out.
        fn run_buffer(&mut self, info: &RunInfo) -> Buffer;
        /// Called after each [`RunHandler::run_buffer()`] is filled out.
        fn commit_run_buffer(&mut self, info: &RunInfo);
        /// Called when ending a line.
        fn commit_line(&mut self);
    }

    #[derive(Debug)]
    pub struct RunInfo<'a> {
        pub font: &'a Font,
        pub bidi_level: u8,
        pub script: FourByteTag,
        pub language: Option<&'a str>,
        pub advance: Vector,
        pub glyph_count: usize,
        pub utf8_range: Range<usize>,
    }

    impl RunInfo<'_> {
        pub(crate) fn from_native(ri: &SkShaper_RunHandler_RunInfo) -> Self {
            let utf8_range = ri.utf8Range;
            RunInfo {
                font: Font::from_native_ref(unsafe { &*ri.fFont }),
                bidi_level: ri.fBidiLevel,
                script: ri.fScript.into(),
                language: unsafe {
                    if ri.fLanguage.is_null() {
                        None
                    } else {
                        CStr::from_ptr(ri.fLanguage).to_str().ok()
                    }
                },
                advance: Vector::from_native_c(ri.fAdvance),
                glyph_count: ri.glyphCount,
                utf8_range: utf8_range.fBegin..utf8_range.fBegin + utf8_range.fSize,
            }
        }
    }

    #[derive(Debug)]
    pub struct Buffer<'a> {
        /// Required glyphs.
        pub glyphs: &'a mut [GlyphId],
        /// Required positions. If [`Buffer::offsets`] is `None`, put `glyphs[i]` at `positions[i]`.
        pub positions: &'a mut [Point],
        /// Optional offsets. If present, put `glyphs[i]` at `positions[i] + offsets[i]`.
        pub offsets: Option<&'a mut [Point]>,
        /// Optional UTF-8 clusters. `clusters[i]` starts the run that produced `glyphs[i]`.
        pub clusters: Option<&'a mut [u32]>,
        /// Offset to add to all positions.
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

        #[allow(unused)]
        pub(crate) unsafe fn from_native(
            buffer: &SkShaper_RunHandler_Buffer,
            glyph_count: usize,
        ) -> Buffer {
            let offsets = buffer.offsets.into_non_null().map(|mut offsets| unsafe {
                slice::from_raw_parts_mut(Point::from_native_ref_mut(offsets.as_mut()), glyph_count)
            });

            let clusters = buffer.clusters.into_non_null().map(|clusters| unsafe {
                slice::from_raw_parts_mut(clusters.as_ptr(), glyph_count)
            });

            Buffer {
                glyphs: unsafe { safer::from_raw_parts_mut(buffer.glyphs, glyph_count) },
                positions: unsafe {
                    safer::from_raw_parts_mut(
                        Point::from_native_ptr_mut(buffer.positions),
                        glyph_count,
                    )
                },
                offsets,
                clusters,
                point: Point::from_native_c(buffer.point),
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

/// A run handler that the `shape*` functions of [`Shaper`] accept: any Rust [`RunHandler`], or a
/// [`TextBlobBuilderRunHandler`].
///
/// The trait is sealed, so code outside this crate cannot implement it. A forged implementation
/// could hand C++ a native handler with the wrong vtable without `unsafe`.
pub trait AsRunHandler: sealed::AsRunHandler {}

impl<T: sealed::AsRunHandler> AsRunHandler for T {}

/// The trait behind [`AsRunHandler`]. Code outside this crate can still reach its method through a
/// bound on [`AsRunHandler`], so the method also demands a [`Token`](sealed::Token) that only the
/// shaper module can create.
mod sealed {
    use skia_bindings::SkShaper_RunHandler;

    pub trait AsRunHandler {
        /// Calls `f` with the native run handler that C++ dispatches through. The handler only
        /// lives for the duration of the call, so it cannot outlive the borrow of `self`.
        fn with_native<F: FnOnce(&mut SkShaper_RunHandler)>(&mut self, token: Token, f: F);
    }

    /// Proves that the caller is the shaper module. Without it, outside code could nest two
    /// `with_native` calls and copy one native handler over the other.
    #[derive(Debug)]
    pub struct Token(pub(super) ());
}

impl<T: RunHandler> sealed::AsRunHandler for T {
    fn with_native<F: FnOnce(&mut SkShaper_RunHandler)>(&mut self, _: sealed::Token, f: F) {
        let param = unsafe { rust_run_handler::new_param(self) };
        let mut native = rust_run_handler::from_param(&param);
        f(native.base_mut())
    }
}

impl Shaper {
    pub fn shape(
        &self,
        utf8: &str,
        font: &Font,
        left_to_right: bool,
        width: scalar,
        run_handler: &mut impl AsRunHandler,
    ) {
        let bytes = utf8.as_bytes();
        run_handler.with_native(sealed::Token(()), |run_handler| unsafe {
            sb::C_SkShaper_shape(
                self.native(),
                bytes.as_ptr() as _,
                bytes.len(),
                font.native(),
                left_to_right,
                width,
                run_handler,
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn shape_with_iterators(
        &self,
        utf8: &str,
        font_run_iterator: &mut FontRunIterator,
        bidi_run_iterator: &mut BiDiRunIterator,
        script_run_iterator: &mut ScriptRunIterator,
        language_run_iterator: &mut LanguageRunIterator,
        width: scalar,
        run_handler: &mut impl AsRunHandler,
    ) {
        let bytes = utf8.as_bytes();
        run_handler.with_native(sealed::Token(()), |run_handler| unsafe {
            sb::C_SkShaper_shape2(
                self.native(),
                bytes.as_ptr() as _,
                bytes.len(),
                font_run_iterator.native_ptr(),
                bidi_run_iterator.native_ptr(),
                script_run_iterator.native_ptr(),
                language_run_iterator.native_ptr(),
                width,
                run_handler,
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn shape_with_iterators_and_features(
        &self,
        utf8: &str,
        font_run_iterator: &mut FontRunIterator,
        bidi_run_iterator: &mut BiDiRunIterator,
        script_run_iterator: &mut ScriptRunIterator,
        language_run_iterator: &mut LanguageRunIterator,
        features: &[Feature],
        width: scalar,
        run_handler: &mut impl AsRunHandler,
    ) {
        let bytes = utf8.as_bytes();
        run_handler.with_native(sealed::Token(()), |run_handler| unsafe {
            sb::C_SkShaper_shape3(
                self.native(),
                bytes.as_ptr() as _,
                bytes.len(),
                font_run_iterator.native_ptr(),
                bidi_run_iterator.native_ptr(),
                script_run_iterator.native_ptr(),
                language_run_iterator.native_ptr(),
                features.as_ptr(),
                features.len(),
                width,
                run_handler,
            )
        })
    }

    /// Shapes text using the new `SkShaper::Options`-based API, which supports
    /// glyph tracking (letter spacing) in addition to the width available for
    /// horizontal layout.
    #[allow(clippy::too_many_arguments)]
    pub fn shape_with_iterators_and_features_and_options(
        &self,
        utf8: &str,
        font_run_iterator: &mut FontRunIterator,
        bidi_run_iterator: &mut BiDiRunIterator,
        script_run_iterator: &mut ScriptRunIterator,
        language_run_iterator: &mut LanguageRunIterator,
        features: &[Feature],
        width: scalar,
        tracking: scalar,
        run_handler: &mut impl AsRunHandler,
    ) {
        let bytes = utf8.as_bytes();
        run_handler.with_native(sealed::Token(()), |run_handler| unsafe {
            sb::C_SkShaper_shape4(
                self.native(),
                bytes.as_ptr() as _,
                bytes.len(),
                font_run_iterator.native_ptr(),
                bidi_run_iterator.native_ptr(),
                script_run_iterator.native_ptr(),
                language_run_iterator.native_ptr(),
                features.as_ptr(),
                features.len(),
                width,
                tracking,
                run_handler,
            )
        })
    }
}

mod rust_run_handler {
    use std::ffi::c_void;

    use skia_bindings as sb;
    use skia_bindings::{
        RustRunHandler, RustRunHandler_Param, SkShaper_RunHandler, SkShaper_RunHandler_Buffer,
        SkShaper_RunHandler_RunInfo,
    };

    use crate::prelude::*;
    use crate::shaper::RunHandler;
    use crate::shaper::run_handler::RunInfo;

    impl NativeBase<SkShaper_RunHandler> for RustRunHandler {}

    pub unsafe fn new_param<T: RunHandler>(run_handler: &mut T) -> RustRunHandler_Param {
        RustRunHandler_Param {
            data: (run_handler as *mut T).cast(),
            beginLine: Some(begin_line::<T>),
            runInfo: Some(run_info::<T>),
            commitRunInfo: Some(commit_run_info::<T>),
            runBuffer: Some(run_buffer::<T>),
            commitRunBuffer: Some(commit_run_buffer::<T>),
            commitLine: Some(commit_line::<T>),
        }
    }

    pub fn from_param(param: &RustRunHandler_Param) -> RustRunHandler {
        construct(|rh| unsafe { sb::C_RustRunHandler_construct(rh, param) })
    }

    unsafe extern "C" fn begin_line<T: RunHandler>(data: *mut c_void) {
        unsafe { to_run_handler::<T>(data) }.begin_line()
    }

    unsafe extern "C" fn run_info<T: RunHandler>(
        data: *mut c_void,
        ri: *const SkShaper_RunHandler_RunInfo,
    ) {
        unsafe { to_run_handler::<T>(data).run_info(&RunInfo::from_native(&*ri)) }
    }

    unsafe extern "C" fn commit_run_info<T: RunHandler>(data: *mut c_void) {
        unsafe { to_run_handler::<T>(data) }.commit_run_info()
    }

    unsafe extern "C" fn run_buffer<T: RunHandler>(
        data: *mut c_void,
        ri: *const SkShaper_RunHandler_RunInfo,
    ) -> SkShaper_RunHandler_Buffer {
        let ri = unsafe { &*ri };
        unsafe { to_run_handler::<T>(data) }
            .run_buffer(&RunInfo::from_native(ri))
            .native_buffer_mut(ri.glyphCount)
    }

    unsafe extern "C" fn commit_run_buffer<T: RunHandler>(
        data: *mut c_void,
        ri: *const SkShaper_RunHandler_RunInfo,
    ) {
        unsafe { to_run_handler::<T>(data).commit_run_buffer(&RunInfo::from_native(&*ri)) }
    }

    unsafe extern "C" fn commit_line<T: RunHandler>(data: *mut c_void) {
        unsafe { to_run_handler::<T>(data) }.commit_line()
    }

    /// `data` is the `&mut T` erased in `new_param`, the only place that pairs it with the
    /// callbacks above.
    unsafe fn to_run_handler<'a, T>(data: *mut c_void) -> &'a mut T {
        unsafe { &mut *data.cast::<T>() }
    }
}

/// Helper for shaping text directly into a [`TextBlob`].
#[repr(transparent)]
#[derive(Debug)]
pub struct TextBlobBuilderRunHandler<'text>(SkTextBlobBuilderRunHandler, PhantomData<&'text str>);
impl NativeAccess for TextBlobBuilderRunHandler<'_> {
    type Native = SkTextBlobBuilderRunHandler;

    fn native(&self) -> &SkTextBlobBuilderRunHandler {
        &self.0
    }
}

impl NativeAccessMut for TextBlobBuilderRunHandler<'_> {
    fn native_mut(&mut self) -> &mut SkTextBlobBuilderRunHandler {
        &mut self.0
    }
}

impl NativeBase<SkShaper_RunHandler> for SkTextBlobBuilderRunHandler {}

impl TextBlobBuilderRunHandler<'_> {
    pub fn new(text: &str, offset: impl Into<Point>) -> TextBlobBuilderRunHandler {
        let ptr = text.as_ptr();
        // we can safely pass a ptr to the utf8 text string to the RunHandler, because it does not
        // expect it to be 0 terminated, but this introduces another problem because
        // we can never be sure that the RunHandler callbacks refer to that range. For
        // now we ensure that by not exposing the RunHandler of a TextBlobBuilder.
        let run_handler = construct(|rh| unsafe {
            sb::C_SkTextBlobBuilderRunHandler_construct(
                rh,
                ptr as *const raw::c_char,
                offset.into().native(),
            )
        });
        TextBlobBuilderRunHandler(run_handler, PhantomData)
    }

    pub fn make_blob(&mut self) -> Option<TextBlob> {
        TextBlob::from_ptr(unsafe { sb::C_SkTextBlobBuilderRunHandler_makeBlob(self.native_mut()) })
    }

    pub fn end_point(&mut self) -> Point {
        Point::from_native_c(unsafe {
            sb::C_SkTextBlobBuilderRunHandler_endPoint(self.native_mut())
        })
    }
}

impl sealed::AsRunHandler for TextBlobBuilderRunHandler<'_> {
    fn with_native<F: FnOnce(&mut SkShaper_RunHandler)>(&mut self, _: sealed::Token, f: F) {
        f(self.0.base_mut())
    }
}

impl Shaper {
    pub fn shape_text_blob(
        &self,
        text: &str,
        font: &Font,
        left_to_right: bool,
        width: scalar,
        offset: impl Into<Point>,
    ) -> Option<(TextBlob, Point)> {
        let bytes = text.as_bytes();
        let mut builder = TextBlobBuilderRunHandler::new(text, offset);
        unsafe {
            sb::C_SkShaper_shape(
                self.native(),
                bytes.as_ptr() as _,
                bytes.len(),
                font.native(),
                left_to_right,
                width,
                builder.native_mut().base_mut(),
            )
        };
        builder.make_blob().map(|tb| (tb, builder.end_point()))
    }
}

pub(crate) mod shapers {
    use super::{BiDiRunIterator, ScriptRunIterator, Shaper};

    #[deprecated(since = "0.74.0", note = "use shapers::primitive::primitive_text()")]
    pub fn primitive() -> Shaper {
        primitive::primitive_text()
    }

    #[deprecated(
        since = "0.74.0",
        note = "use shapers::primitive::trivial_bidi_run_iterator()"
    )]
    pub fn trivial_bidi_run_iterator(bidi_level: u8, utf8_bytes: usize) -> BiDiRunIterator {
        primitive::trivial_bidi_run_iterator(bidi_level, utf8_bytes)
    }

    #[deprecated(
        since = "0.74.0",
        note = "use shapers::primitive::trivial_script_run_iterator()"
    )]
    pub fn trivial_script_run_iterator(bidi_level: u8, utf8_bytes: usize) -> ScriptRunIterator {
        primitive::trivial_script_run_iterator(bidi_level, utf8_bytes)
    }

    pub mod primitive {
        //! A trivial text shaper and run iterators that do not depend on external shaping libraries.
        use skia_bindings as sb;

        use crate::shaper::{BiDiRunIterator, ScriptRunIterator, Shaper};

        pub fn primitive_text() -> Shaper {
            Shaper::from_ptr(unsafe { sb::C_SkShapers_Primitive_PrimitiveText() }).unwrap()
        }

        pub fn trivial_bidi_run_iterator(bidi_level: u8, utf8_bytes: usize) -> BiDiRunIterator {
            BiDiRunIterator::from_ptr(unsafe {
                sb::C_SkShapers_Primitive_TrivialBidiRunIterator_new(bidi_level, utf8_bytes)
            })
            .unwrap()
        }

        pub fn trivial_script_run_iterator(bidi_level: u8, utf8_bytes: usize) -> ScriptRunIterator {
            ScriptRunIterator::from_ptr(unsafe {
                sb::C_SkShapers_Primitive_TrivialScriptRunIterator_new(bidi_level, utf8_bytes)
            })
            .unwrap()
        }
    }
}

pub mod icu {
    //! Initialization support for International Components for Unicode (ICU), required by some text shapers.
    /// On Windows, and if the default feature "embed-icudtl" is _not_ set, this function writes the
    /// file `icudtl.dat` into the current executable's directory making sure that it's available
    /// when text shaping is used in Skia.
    ///
    /// If your executable directory can not be written to, make sure that `icudtl.dat` is
    /// available.
    ///
    /// Note that it is currently not possible to load `icudtl.dat` from another location.
    ///
    /// If the default feature "embed-icudtl" is set, the `icudtl.dat` file is directly used from
    /// memory, so no `icudtl.dat` file is needed.
    pub fn init() {
        skia_bindings::icu::init();
    }

    #[test]
    #[serial_test::serial]
    fn test_text_blob_builder_run_handler() {
        use crate::{Font, FontMgr, FontStyle};
        init();
        let str = "العربية";
        let mut text_blob_builder_run_handler =
            crate::shaper::TextBlobBuilderRunHandler::new(str, crate::Point::default());

        let font_mgr = FontMgr::new();
        let default_typeface = font_mgr
            .legacy_make_typeface(None, FontStyle::default())
            .unwrap();
        let default_font = Font::new(default_typeface, 10.0);
        let shaper = crate::Shaper::new(font_mgr);
        shaper.shape(
            str,
            &default_font,
            false,
            10000.0,
            &mut text_blob_builder_run_handler,
        );

        let blob = text_blob_builder_run_handler.make_blob().unwrap();
        let bounds = blob.bounds();
        assert!(bounds.width() > 0.0 && bounds.height() > 0.0);
    }

    #[test]
    #[serial_test::serial]
    fn icu_init_is_idempotent() {
        init();
        init();
    }
}
