use crate::prelude::NativeTransmutable;
use skia_bindings::{GrGLenum, C_GrGLTextureInfo_Equals, GrGLTextureInfo, GrGLFramebufferInfo, GrGLuint};

pub type Enum = GrGLenum;
pub type UInt = GrGLuint;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TextureInfo {
    pub target: Enum,
    pub id: Enum,
    pub format: Enum
}

impl NativeTransmutable<GrGLTextureInfo> for TextureInfo {}
#[test] fn test_texture_info_layout() { TextureInfo::test_layout() }

impl PartialEq for TextureInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            C_GrGLTextureInfo_Equals(self.native(), other.native())
        }
    }
}

// TODO: does this make sense?
impl Default for TextureInfo {
    fn default() -> Self {
        TextureInfo {
            target: 0,
            id: 0,
            format: 0
        }
    }
}

impl TextureInfo {
    pub fn from_target_and_id(target: Enum, id: Enum) -> Self {
        Self {
            target,
            id,
            format: 0
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct FramebufferInfo {
    pub fboid: UInt,
    pub format: Enum
}

impl NativeTransmutable<GrGLFramebufferInfo> for FramebufferInfo {}
#[test] fn test_framebuffer_info_layout() { FramebufferInfo::test_layout() }

impl Default for FramebufferInfo {
    fn default() -> Self {
        FramebufferInfo { fboid: 0, format: 0 }
    }
}

impl FramebufferInfo {
    pub fn from_fboid(fboid: UInt) -> Self {
        Self { fboid, format: 0 }
    }
}
