use std::fmt;

use skia_bindings::{self as sb, SkContext, SkContextOptions};

use crate::prelude::*;

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
            .field("typeface_cache_count_limit", &self.typeface_cache_count_limit())
            .field("resource_cache_total_byte_limit", &self.resource_cache_total_byte_limit())
            .field(
                "resource_cache_single_allocation_byte_limit",
                &self.resource_cache_single_allocation_byte_limit(),
            )
            .field("font_cache_count_limit", &self.font_cache_count_limit())
            .field("font_cache_limit", &self.font_cache_limit())
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

impl NativeDrop for SkContext {
    fn drop(&mut self) {
        unsafe { sb::C_SkContext_delete(self) }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context").finish()
    }
}

impl Context {
    /// Creates a context that uses software rasterization only.
    pub fn new_raster(options: &ContextOptions) -> Option<Self> {
        Self::from_ptr(unsafe { sb::C_SkContexts_MakeRaster(options.native()) })
    }
}
