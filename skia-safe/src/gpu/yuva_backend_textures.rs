use crate::prelude::*;
use skia_bindings as sb;
use skia_bindings::GrYUVABackendTextureInfo;

pub type YUVABackendTextureInfo = Handle<GrYUVABackendTextureInfo>;

impl NativeDrop for GrYUVABackendTextureInfo {
    fn drop(&mut self) {
        unsafe { sb::C_GrYUVABackendTextureInfo_destruct(self) }
    }
}
