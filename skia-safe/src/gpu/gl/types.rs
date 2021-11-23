use crate::prelude::*;
use skia_bindings::{self as sb, GrGLFramebufferInfo, GrGLTextureInfo};

pub use skia_bindings::GrGLFormat as Format;
variant_name!(Format::ALPHA8, format_naming);
pub use skia_bindings::GrGLStandard as Standard;
variant_name!(Standard::GLES, standard_naming);
pub use skia_bindings::GrGLenum as Enum;
pub use skia_bindings::GrGLuint as UInt;

#[derive(Copy, Clone, Eq, Debug)]
#[repr(C)]
pub struct TextureInfo {
    pub target: Enum,
    pub id: Enum,
    pub format: Enum,
}

native_transmutable!(GrGLTextureInfo, TextureInfo, text_info_layout);

impl PartialEq for TextureInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sb::C_GrGLTextureInfo_Equals(self.native(), other.native()) }
    }
}

// TODO: does this make sense?
impl Default for TextureInfo {
    fn default() -> Self {
        TextureInfo {
            target: 0,
            id: 0,
            format: 0,
        }
    }
}

impl TextureInfo {
    pub fn from_target_and_id(target: Enum, id: Enum) -> Self {
        Self {
            target,
            id,
            format: 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct FramebufferInfo {
    pub fboid: UInt,
    pub format: Enum,
}

native_transmutable!(
    GrGLFramebufferInfo,
    FramebufferInfo,
    framebuffer_info_layout
);

impl Default for FramebufferInfo {
    fn default() -> Self {
        FramebufferInfo {
            fboid: 0,
            format: 0,
        }
    }
}

impl FramebufferInfo {
    pub fn from_fboid(fboid: UInt) -> Self {
        Self { fboid, format: 0 }
    }
}

bitflags! {
    pub struct BackendState: u32 {
        const RENDER_TARGET = sb::GrGLBackendState_kRenderTarget_GrGLBackendState as _;
        const TEXTURE_BINDING = sb::GrGLBackendState_kTextureBinding_GrGLBackendState as _;
        const VIEW = sb::GrGLBackendState_kView_GrGLBackendState as _;
        const BLEND = sb::GrGLBackendState_kBlend_GrGLBackendState as _;
        const MSAA_ENABLE = sb::GrGLBackendState_kMSAAEnable_GrGLBackendState as _;
        const VERTEX = sb::GrGLBackendState_kVertex_GrGLBackendState as _;
        const STENCIL = sb::GrGLBackendState_kStencil_GrGLBackendState as _;
        const PIXEL_STORE = sb::GrGLBackendState_kPixelStore_GrGLBackendState as _;
        const PROGRAM = sb::GrGLBackendState_kProgram_GrGLBackendState as _;
        const FIXED_FUNCTION = sb::GrGLBackendState_kFixedFunction_GrGLBackendState as _;
        const MISC = sb::GrGLBackendState_kMisc_GrGLBackendState as _;
    }
}

// TODO: BackendState::ALL

#[cfg(test)]
mod tests {
    use super::{Enum, Format};

    #[test]
    fn test_support_from_format_to_enum_and_back() {
        let e: Enum = Format::ALPHA8.into();
        let f: Format = e.into();
        assert_eq!(f, Format::ALPHA8);
    }

    #[test]
    fn test_all_formats_exhaustive() {
        let x = Format::ALPHA8;
        // !!!!!
        // IF this match is not exhaustive anymore, the implementations of the format conversions
        // need to be updated in `skia-bindings/src/gl.cpp`, too.
        match x {
            Format::Unknown => {}
            Format::RGBA8 => {}
            Format::R8 => {}
            Format::ALPHA8 => {}
            Format::LUMINANCE8 => {}
            Format::LUMINANCE8_ALPHA8 => {}
            Format::BGRA8 => {}
            Format::RGB565 => {}
            Format::RGBA16F => {}
            Format::R16F => {}
            Format::RGB8 => {}
            Format::RG8 => {}
            Format::RGB10_A2 => {}
            Format::RGBA4 => {}
            Format::SRGB8_ALPHA8 => {}
            Format::COMPRESSED_ETC1_RGB8 => {}
            Format::COMPRESSED_RGB8_ETC2 => {}
            Format::COMPRESSED_RGB8_BC1 => {}
            Format::COMPRESSED_RGBA8_BC1 => {}
            Format::R16 => {}
            Format::RG16 => {}
            Format::RGBA16 => {}
            Format::RG16F => {}
            Format::LUMINANCE16F => {}
            Format::STENCIL_INDEX8 => {}
            Format::STENCIL_INDEX16 => {}
            Format::DEPTH24_STENCIL8 => {}
        }
    }

    #[test]
    fn test_format_last_color_and_last_exists() {
        let _ = Format::Last;
        let _ = Format::LastColorFormat;
    }
}
