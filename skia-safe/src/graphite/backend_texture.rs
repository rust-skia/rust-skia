use std::fmt;

use skia_bindings as sb;

use crate::graphite::TextureInfo;
use crate::graphite::types::BackendApi;
use crate::{ISize, prelude::*};

pub type BackendTexture = Handle<sb::skgpu_graphite_BackendTexture>;

impl NativeDrop for sb::skgpu_graphite_BackendTexture {
    fn drop(&mut self) {
        unsafe { sb::C_BackendTexture_destruct(self) }
    }
}

impl fmt::Debug for BackendTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackendTexture")
            .field("is_valid", &self.is_valid())
            .field("backend", &self.backend())
            .field("dimensions", &self.dimensions())
            .finish()
    }
}

impl Default for BackendTexture {
    fn default() -> Self {
        Self::new()
    }
}

impl BackendTexture {
    /// Create a new BackendTexture with default settings
    pub fn new() -> Self {
        BackendTexture::construct(|texture| unsafe { sb::C_BackendTexture_Construct(texture) })
    }

    /// Create a BackendTexture by copying from another
    pub fn from_backend_texture(other: &BackendTexture) -> Self {
        BackendTexture::construct(|texture| unsafe {
            sb::C_BackendTexture_CopyConstruct(texture, other.native())
        })
    }

    /// Check if this BackendTexture is valid
    ///
    /// # Returns
    /// `true` if the backend texture is valid and can be used
    pub fn is_valid(&self) -> bool {
        unsafe { sb::C_BackendTexture_isValid(self.native()) }
    }

    /// Get the backend API for this texture
    ///
    /// # Returns
    /// The backend API (Vulkan, Metal, etc.)
    pub fn backend(&self) -> BackendApi {
        unsafe { sb::C_BackendTexture_backend(self.native()) }
    }

    /// Get the dimensions of this texture
    ///
    /// # Returns
    /// The width and height of the texture
    pub fn dimensions(&self) -> ISize {
        let mut dimensions = ISize::default();
        unsafe {
            sb::C_BackendTexture_dimensions(self.native(), dimensions.native_mut());
        }
        dimensions
    }

    /// Get the TextureInfo for this backend texture
    ///
    /// # Returns
    /// TextureInfo describing the format and properties of this texture
    pub fn info(&self) -> TextureInfo {
        let mut info = TextureInfo::new();
        unsafe {
            sb::C_BackendTexture_info(self.native(), info.native_mut());
        }
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_texture_creation() {
        let texture = BackendTexture::new();
        let _default_texture = BackendTexture::default();
        // Should not panic
        let _ = format!("{:?}", texture);
    }

    #[test]
    fn test_backend_texture_copy() {
        let texture = BackendTexture::new();
        let copied = BackendTexture::from_backend_texture(&texture);
        // Should not panic
        let _ = format!("{:?}", copied);
    }

    #[test]
    fn test_backend_texture_copy_constructor() {
        let texture = BackendTexture::new();
        let copied = BackendTexture::from_backend_texture(&texture);
        // Should not panic
        let _ = format!("{:?}", copied);
    }

    #[test]
    fn test_backend_texture_properties() {
        let texture = BackendTexture::new();
        // Default texture may or may not be valid depending on implementation
        let _ = texture.is_valid();
        let _ = texture.dimensions();
        let _ = texture.info();
    }
}
