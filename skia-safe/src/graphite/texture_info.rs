use std::fmt;

use skia_bindings as sb;

use crate::{graphite::types::BackendApi, prelude::*};

pub type TextureInfo = Handle<sb::skgpu_graphite_TextureInfo>;

impl NativeDrop for sb::skgpu_graphite_TextureInfo {
    fn drop(&mut self) {
        unsafe { sb::C_TextureInfo_destruct(self) }
    }
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

impl TextureInfo {
    /// Create a new TextureInfo with default settings
    pub fn new() -> Self {
        Self::construct(|texture_info| unsafe { sb::C_TextureInfo_Construct(texture_info) })
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
}
