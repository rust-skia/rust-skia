//! [`Context`], Skia's central object for shared resources and caches, and the options it is created
//! with.

use std::fmt;

use skia_bindings::{self as sb, SkContext, SkContextOptions};

use crate::{OwnedRecorder, prelude::*};

/// Options shared by Skia's raster and GPU context factories.
pub type ContextOptions = Handle<SkContextOptions>;
unsafe_send_sync!(ContextOptions);

impl NativeDrop for SkContextOptions {
    fn drop(&mut self) {
        unsafe { sb::C_SkContextOptions_destruct(self) }
    }
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self::construct(|options| unsafe { sb::C_SkContextOptions_Construct(options) })
    }
}

impl fmt::Debug for ContextOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContextOptions")
            .field(
                "typeface_cache_count_limit",
                &self.typeface_cache_count_limit(),
            )
            .field(
                "resource_cache_total_byte_limit",
                &self.resource_cache_total_byte_limit(),
            )
            .field(
                "resource_cache_single_allocation_byte_limit",
                &self.resource_cache_single_allocation_byte_limit(),
            )
            .field("font_cache_count_limit", &self.font_cache_count_limit())
            .field("font_cache_limit", &self.font_cache_limit())
            .field(
                "thread_safe_shared_resources",
                &self.thread_safe_shared_resources(),
            )
            .finish()
    }
}

impl ContextOptions {
    /// Maximum number of entries in the typeface cache. Each entry corresponds to a cached
    /// [`Typeface`](crate::Typeface). (1024 is the historical default value.)
    pub fn typeface_cache_count_limit(&self) -> i32 {
        self.native().fTypefaceCacheCountLimit
    }

    /// Maximum total memory (in bytes) for the CPU resource cache, used for temporary bitmaps,
    /// scaled images, and other decoded resources. Entries are purged from the cache when memory
    /// usage exceeds this limit.
    pub fn resource_cache_total_byte_limit(&self) -> usize {
        self.native().fResourceCacheTotalByteLimit
    }

    /// Maximum size (in bytes) for a single allocation in the CPU resource cache. When a cacheable
    /// entry is very large (e.g. a large scaled bitmap), adding it to the cache can cause most or
    /// all existing entries to be purged. If an entry's size exceeds this limit, it is not cached
    /// at all.
    ///
    /// `0` means no separate single-allocation cap beyond
    /// [`resource_cache_total_byte_limit()`][`Self::resource_cache_total_byte_limit`].
    pub fn resource_cache_single_allocation_byte_limit(&self) -> usize {
        self.native().fResourceCacheSingleAllocationByteLimit
    }

    /// Maximum number of entries (strikes) in the font cache. A cache entry is associated with each
    /// unique combination of typeface, point size, and matrix.
    pub fn font_cache_count_limit(&self) -> i32 {
        self.native().fFontCacheCountLimit
    }

    /// Maximum total memory (in bytes) used by the font/strike cache for glyph metrics, masks, and
    /// paths. If the cache needs to allocate more, it will purge previous entries.
    pub fn font_cache_limit(&self) -> usize {
        self.native().fFontCacheLimit
    }

    /// When `true`, shared caches (such as the strike cache, the resource cache, and the typeface
    /// cache) are guarded by internal mutexes so they can safely be accessed concurrently across
    /// threads. (`true` is the default value.)
    pub fn thread_safe_shared_resources(&self) -> bool {
        self.native().fThreadSafeSharedResources
    }

    /// Sets the maximum number of entries in the typeface cache.
    pub fn set_typeface_cache_count_limit(&mut self, limit: i32) -> &mut Self {
        self.native_mut().fTypefaceCacheCountLimit = limit;
        self
    }

    /// Sets the maximum total memory (in bytes) for the CPU resource cache.
    pub fn set_resource_cache_total_byte_limit(&mut self, limit: usize) -> &mut Self {
        self.native_mut().fResourceCacheTotalByteLimit = limit;
        self
    }

    /// Sets the maximum size (in bytes) for a single allocation in the CPU resource cache.
    /// `0` disables the separate single-allocation cap.
    pub fn set_resource_cache_single_allocation_byte_limit(&mut self, limit: usize) -> &mut Self {
        self.native_mut().fResourceCacheSingleAllocationByteLimit = limit;
        self
    }

    /// Sets the maximum number of entries (strikes) in the font cache.
    pub fn set_font_cache_count_limit(&mut self, limit: i32) -> &mut Self {
        self.native_mut().fFontCacheCountLimit = limit;
        self
    }

    /// Sets the maximum total memory (in bytes) used by the font/strike cache.
    pub fn set_font_cache_limit(&mut self, limit: usize) -> &mut Self {
        self.native_mut().fFontCacheLimit = limit;
        self
    }
}

/// Skia's central context object for shared resources and internal caches.
pub type Context = RefHandle<SkContext>;

impl NativeDelete for SkContext {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_SkContext_delete(ptr) }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("font_cache_limit", &self.font_cache_limit())
            .field("font_cache_used", &self.font_cache_used())
            .field("font_cache_count_limit", &self.font_cache_count_limit())
            .field("font_cache_count_used", &self.font_cache_count_used())
            .field(
                "resource_cache_total_bytes_used",
                &self.resource_cache_total_bytes_used(),
            )
            .field(
                "resource_cache_total_byte_limit",
                &self.resource_cache_total_byte_limit(),
            )
            .field(
                "resource_cache_single_allocation_byte_limit",
                &self.resource_cache_single_allocation_byte_limit(),
            )
            .field(
                "typeface_cache_count_limit",
                &self.typeface_cache_count_limit(),
            )
            .finish()
    }
}

// The methods take `&self` even where the C++ method is not `const`, so that recorders can keep
// borrowing the context while it is used. They write only to the context's shared caches and to
// the recorders they create, never to `SkContext` itself, and `Context` is not `Sync`.
impl Context {
    /// Creates a context that uses software rasterization only.
    pub fn new_raster(options: &ContextOptions) -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_SkContexts_MakeRaster(options.native()) })
    }

    // TODO: Wrap `startCapture()` and `endCapture()` once `SkCapture` is public. Today it is
    // defined only in a private header, so Rust could not read or serialize a capture.

    /// Creates and returns a new recorder for this context based on the requested backend.
    ///
    /// Returns `None` for [`BackendSelection::GPU`] when the context has no GPU backend.
    // TODO: Wrap the Graphite recorder options, and a way to turn a Graphite recorder into a
    // `graphite::Recorder`, so that its recording can be snapped and submitted.
    pub fn make_recorder(&self, backend: BackendSelection) -> Option<OwnedRecorder<'_>> {
        OwnedRecorder::from_ptr(
            unsafe { sb::C_SkContext_makeRecorder(self.native_mut_force(), backend) },
            self,
        )
    }

    /// Returns the maximum number of bytes that the font cache uses. If the cache needs to
    /// allocate more, it purges previous entries.
    pub fn font_cache_limit(&self) -> usize {
        unsafe { self.native().fontCacheLimit() }
    }

    /// Sets the maximum number of bytes that the font cache uses, and returns the previous limit.
    /// If the cache needs to allocate more, it purges previous entries.
    pub fn set_font_cache_limit(&self, bytes: usize) -> usize {
        unsafe { sb::SkContext_setFontCacheLimit(self.native_mut_force(), bytes) }
    }

    /// Returns the number of bytes that the font cache currently uses.
    pub fn font_cache_used(&self) -> usize {
        unsafe { self.native().fontCacheUsed() }
    }

    /// Returns the limit of the number of entries in the font cache. A cache entry is associated
    /// with each typeface + point size + matrix.
    pub fn font_cache_count_limit(&self) -> i32 {
        unsafe { self.native().fontCacheCountLimit() }
    }

    /// Sets the limit of the number of entries in the font cache, and returns the previous limit.
    /// If the new limit is lower than the previous one, the cache tries to purge entries to meet
    /// it.
    pub fn set_font_cache_count_limit(&self, count: i32) -> i32 {
        unsafe { sb::SkContext_setFontCacheCountLimit(self.native_mut_force(), count) }
    }

    /// Returns the number of entries in the font cache.
    pub fn font_cache_count_used(&self) -> i32 {
        unsafe { self.native().fontCacheCountUsed() }
    }

    /// For debugging purposes, attempts to purge the font cache. It does not change the limit,
    /// but causes subsequent font measures and draws to be recreated.
    pub fn purge_font_cache(&self) {
        unsafe { sb::SkContext_purgeFontCache(self.native_mut_force()) }
    }

    /// If the strike cache is above the cache limit, attempts to purge strikes with pinners. Call
    /// it after releasing locks on pinned strikes.
    pub fn purge_pinned_font_cache(&self) {
        unsafe { sb::SkContext_purgePinnedFontCache(self.native_mut_force()) }
    }

    /// Returns the memory used for temporary images and other resources.
    pub fn resource_cache_total_bytes_used(&self) -> usize {
        unsafe { self.native().resourceCacheTotalBytesUsed() }
    }

    /// Returns the memory usage limit of the resource cache, used for temporary bitmaps and other
    /// resources. Entries are purged from the cache when the memory usage exceeds this limit.
    pub fn resource_cache_total_byte_limit(&self) -> usize {
        unsafe { self.native().resourceCacheTotalByteLimit() }
    }

    /// Sets the memory usage limit of the resource cache, and returns the previous limit.
    pub fn set_resource_cache_total_byte_limit(&self, new_limit: usize) -> usize {
        unsafe { sb::SkContext_setResourceCacheTotalByteLimit(self.native_mut_force(), new_limit) }
    }

    /// Returns the size limit of a single entry in the resource cache. An entry larger than the
    /// limit is not cached at all, so that it does not purge most of the other entries.
    ///
    /// `None` means no limit, so Skia always attempts to cache entries.
    pub fn resource_cache_single_allocation_byte_limit(&self) -> Option<usize> {
        let size = unsafe { self.native().resourceCacheSingleAllocationByteLimit() };
        (size != 0).then_some(size)
    }

    /// Sets the size limit of a single entry in the resource cache, and returns the previous
    /// limit. `None` means no limit.
    pub fn set_resource_cache_single_allocation_byte_limit(
        &self,
        new_limit: Option<usize>,
    ) -> Option<usize> {
        let size = unsafe {
            sb::SkContext_setResourceCacheSingleAllocationByteLimit(
                self.native_mut_force(),
                new_limit.unwrap_or_default(),
            )
        };
        (size != 0).then_some(size)
    }

    /// For debugging purposes, attempts to purge the resource cache. It does not change the limit.
    pub fn purge_resource_cache(&self) {
        unsafe { sb::SkContext_purgeResourceCache(self.native_mut_force()) }
    }

    /// Returns the limit of the number of entries in the typeface cache. A cache entry is
    /// associated with each typeface.
    pub fn typeface_cache_count_limit(&self) -> i32 {
        unsafe { self.native().typefaceCacheCountLimit() }
    }

    /// Sets the limit of the number of entries in the typeface cache, and returns the previous
    /// limit. A change takes effect the next time each cache object is modified.
    pub fn set_typeface_cache_count_limit(&self, count: i32) -> i32 {
        unsafe { sb::SkContext_setTypefaceCacheCountLimit(self.native_mut_force(), count) }
    }

    // TODO: Wrap `dumpMemoryStatistics()`, which needs a wrapper for `SkTraceMemoryDump`.
}

/// Selects the backend of the recorder that [`Context::make_recorder`] creates.
pub use sb::SkContext_BackendSelection as BackendSelection;
variant_name!(BackendSelection::Automatic);

#[cfg(test)]
mod tests {
    use super::{BackendSelection, Context, ContextOptions};
    use crate::{Recorder, recorder::Type};

    #[test]
    fn raster_context_makes_only_cpu_recorders() {
        let context = Context::new_raster(&ContextOptions::default()).unwrap();
        let recorder = |backend| context.make_recorder(backend).map(|r| r.ty());
        assert_eq!(recorder(BackendSelection::Automatic), Some(Type::CPU));
        assert_eq!(recorder(BackendSelection::CPU), Some(Type::CPU));
        assert_eq!(recorder(BackendSelection::GPU), None);
    }
}
