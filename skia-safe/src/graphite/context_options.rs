use std::fmt;

use skia_bindings as sb;

use crate::prelude::*;

pub type ContextOptions = Handle<sb::skgpu_graphite_ContextOptions>;

impl NativeDrop for sb::skgpu_graphite_ContextOptions {
    fn drop(&mut self) {
        unsafe { sb::C_ContextOptions_destruct(self) }
    }
}

impl fmt::Debug for ContextOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContextOptions").finish()
    }
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextOptions {
    /// Create new ContextOptions with default settings
    pub fn new() -> Self {
        Self::construct(|options| unsafe { sb::C_ContextOptions_Construct(options) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_options_creation() {
        let options = ContextOptions::new();
        let _default_options = ContextOptions::default();
        // Should not panic
        let _ = format!("{:?}", options);
    }

    #[test]
    fn test_context_options_copy() {
        let options = ContextOptions::new();
        // Context options don't support clone, but we can create new ones
        let _other_options = ContextOptions::new();
        // Should not panic
        let _ = format!("{:?}", options);
    }
}
