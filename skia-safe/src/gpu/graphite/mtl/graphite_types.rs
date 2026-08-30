use skia_bindings as sb;

use super::super::BackendTexture;

pub mod backend_textures {
    use super::*;

    /// # Safety
    ///
    /// `texture` must point to a valid Metal texture that remains alive while
    /// the returned backend texture is in use.
    pub unsafe fn make_metal(
        dimensions: impl Into<crate::ISize>,
        texture: *mut std::ffi::c_void,
    ) -> BackendTexture {
        let dimensions = dimensions.into();
        BackendTexture::construct(|backend_texture| unsafe {
            sb::C_BackendTextures_MakeMetal(
                backend_texture,
                dimensions.width,
                dimensions.height,
                texture,
            )
        })
    }
}
