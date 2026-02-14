use skia_bindings as sb;
use std::fmt;

pub struct ContextOptions {
    inner: sb::skgpu_graphite_ContextOptions,
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
        let mut inner = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        unsafe {
            sb::C_ContextOptions_Construct(&mut inner);
        }
        Self { inner }
    }

    pub(crate) fn native(&self) -> &sb::skgpu_graphite_ContextOptions {
        &self.inner
    }

    #[allow(dead_code)]
    pub(crate) fn native_mut(&mut self) -> &mut sb::skgpu_graphite_ContextOptions {
        &mut self.inner
    }
}

native_transmutable!(sb::skgpu_graphite_ContextOptions, ContextOptions);

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
