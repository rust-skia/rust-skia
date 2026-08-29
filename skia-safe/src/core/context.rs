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
        f.debug_struct("ContextOptions").finish()
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
