use crate::prelude::NativeTransmutable;
use skia_bindings as sb;
use skia_bindings::{
    GrGLFormat, GrGLFramebufferInfo, GrGLStandard, GrGLTextureInfo, GrGLenum, GrGLuint,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Standard {
    None = GrGLStandard::kNone_GrGLStandard as _,
    GL = GrGLStandard::kGL_GrGLStandard as _,
    GLES = GrGLStandard::kGLES_GrGLStandard as _,
    WebGL = GrGLStandard::kWebGL_GrGLStandard as _,
}

impl NativeTransmutable<GrGLStandard> for Standard {}
#[test]
fn test_standard_layout() {
    Standard::test_layout()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum Format {
    Unknown = GrGLFormat::kUnknown as _,
    RGBA8 = GrGLFormat::kRGBA8 as _,
    R8 = GrGLFormat::kR8 as _,
    ALPHA8 = GrGLFormat::kALPHA8 as _,
    LUMINANCE8 = GrGLFormat::kLUMINANCE8 as _,
    BGRA8 = GrGLFormat::kBGRA8 as _,
    RGB565 = GrGLFormat::kRGB565 as _,
    RGBA16F = GrGLFormat::kRGBA16F as _,
    R16F = GrGLFormat::kR16F as _,
    RGB8 = GrGLFormat::kRGB8 as _,
    RG8 = GrGLFormat::kRG8 as _,
    RGB10_A2 = GrGLFormat::kRGB10_A2 as _,
    RGBA4 = GrGLFormat::kRGBA4 as _,
    RGBA32F = GrGLFormat::kRGBA32F as _,
    SRGB8_ALPHA8 = GrGLFormat::kSRGB8_ALPHA8 as _,
    COMPRESSED_RGB8_ETC2 = GrGLFormat::kCOMPRESSED_RGB8_ETC2 as _,
    COMPRESSED_ETC1_RGB8 = GrGLFormat::kCOMPRESSED_ETC1_RGB8 as _,
    R16 = GrGLFormat::kR16 as _,
    RG16 = GrGLFormat::kRG16 as _,
    RGBA16 = GrGLFormat::kRGBA16 as _,
    RG16F = GrGLFormat::kRG16F as _,
    LUMINANCE16F = GrGLFormat::kLUMINANCE16F as _,
}

impl NativeTransmutable<GrGLFormat> for Format {}
#[test]
fn test_format_layout() {
    Format::test_layout()
}

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
