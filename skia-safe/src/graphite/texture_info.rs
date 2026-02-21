use crate::graphite::types::BackendApi;

use skia_bindings as sb;
use std::fmt;

pub struct TextureInfo {
    inner: sb::skgpu_graphite_TextureInfo,
}

impl fmt::Debug for TextureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextureInfo")
            .field("is_valid", &self.is_valid())
            .field("backend", &self.backend())
            .finish()
    }
}

impl Default for TextureInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TextureInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sb::C_TextureInfo_Equals(self.native(), other.native()) }
    }
}

impl Eq for TextureInfo {}

impl Drop for TextureInfo {
    fn drop(&mut self) {
        unsafe { sb::C_TextureInfo_destruct(&mut self.inner) }
    }
}

impl TextureInfo {
    /// Create a new TextureInfo with default settings
    pub fn new() -> Self {
        let mut inner = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        unsafe {
            sb::C_TextureInfo_Construct(&mut inner);
        }
        Self { inner }
    }

    /// Check if this TextureInfo is valid
    ///
    /// # Returns
    /// `true` if the texture info is valid and can be used
    pub fn is_valid(&self) -> bool {
        unsafe { sb::C_TextureInfo_isValid(self.native()) }
    }

    /// Get the backend API for this texture
    ///
    /// # Returns
    /// The backend API (Vulkan, Metal, etc.)
    pub fn backend(&self) -> BackendApi {
        unsafe { sb::C_TextureInfo_backend(self.native()) }
    }

    pub(crate) fn native(&self) -> &sb::skgpu_graphite_TextureInfo {
        &self.inner
    }

    pub(crate) fn native_mut(&mut self) -> &mut sb::skgpu_graphite_TextureInfo {
        &mut self.inner
    }
}

native_transmutable!(sb::skgpu_graphite_TextureInfo, TextureInfo);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_info_creation() {
        let info = TextureInfo::new();
        let _default_info = TextureInfo::default();
        // Should not panic
        let _ = format!("{:?}", info);
    }

    #[test]
    fn test_texture_info_copy() {
        let info = TextureInfo::new();
        // TextureInfo doesn't support Clone, but we can create new ones
        let other_info = TextureInfo::new();
        // Both should have same validity status for default construction
        assert_eq!(info.is_valid(), other_info.is_valid());
    }

    #[test]
    fn test_texture_info_validity() {
        let info = TextureInfo::new();
        // Default texture info may or may not be valid depending on implementation
        let _ = info.is_valid();
    }
}
