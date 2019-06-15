use crate::prelude::NativeTransmutable;
use skia_bindings::{
    C_GrGLTextureInfo_Equals, GrGLFramebufferInfo, GrGLTextureInfo, GrGLenum, GrGLuint,
};

pub type Enum = GrGLenum;
pub type UInt = GrGLuint;

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
        unsafe { C_GrGLTextureInfo_Equals(self.native(), other.native()) }
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
        const RENDER_TARGET = skia_bindings::GrGLBackendState_kRenderTarget_GrGLBackendState as _;
        const TEXTURE_BINDING = skia_bindings::GrGLBackendState_kTextureBinding_GrGLBackendState as _;
        const VIEW = skia_bindings::GrGLBackendState_kView_GrGLBackendState as _;
        const BLEND = skia_bindings::GrGLBackendState_kBlend_GrGLBackendState as _;
        const MSAA_ENABLE = skia_bindings::GrGLBackendState_kMSAAEnable_GrGLBackendState as _;
        const VERTEX = skia_bindings::GrGLBackendState_kVertex_GrGLBackendState as _;
        const STENCIL = skia_bindings::GrGLBackendState_kStencil_GrGLBackendState as _;
        const PIXEL_STORE = skia_bindings::GrGLBackendState_kPixelStore_GrGLBackendState as _;
        const PROGRAM = skia_bindings::GrGLBackendState_kProgram_GrGLBackendState as _;
        const FIXED_FUNCTION = skia_bindings::GrGLBackendState_kFixedFunction_GrGLBackendState as _;
        const MISC = skia_bindings::GrGLBackendState_kMisc_GrGLBackendState as _;
        const PATH_RENDERING = skia_bindings::GrGLBackendState_kPathRendering_GrGLBackendState as _;
    }
}

// TODO: BackendState::ALL
