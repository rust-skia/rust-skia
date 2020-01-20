use crate::prelude::NativeTransmutable;
use skia_bindings as sb;
use skia_bindings::{GrGLFramebufferInfo, GrGLTextureInfo};

pub use skia_bindings::GrGLStandard as Standard;
#[test]
fn test_standard_naming() {
    let _ = Standard::GLES;
}

pub use skia_bindings::GrGLFormat as Format;
#[test]
fn test_format_naming() {
    let _ = Format::COMPRESSED_ETC1_RGB8;
}

pub use skia_bindings::GrGLenum as Enum;
pub use skia_bindings::GrGLuint as UInt;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TextureInfo {
    pub target: Enum,
    pub id: Enum,
    pub format: Enum,
}

impl NativeTransmutable<GrGLTextureInfo> for TextureInfo {}
#[test]
fn test_texture_info_layout() {
    TextureInfo::test_layout()
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FramebufferInfo {
    pub fboid: UInt,
    pub format: Enum,
}

impl NativeTransmutable<GrGLFramebufferInfo> for FramebufferInfo {}
#[test]
fn test_framebuffer_info_layout() {
    FramebufferInfo::test_layout()
}

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
        const PATH_RENDERING = sb::GrGLBackendState_kPathRendering_GrGLBackendState as _;
    }
}

// TODO: BackendState::ALL
