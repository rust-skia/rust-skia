use super::{gl, BackendAPI, MipMapped};
use crate::prelude::*;
use skia_bindings::{
    C_GrBackendFormat_destruct, C_GrBackendRenderTarget_backend, C_GrBackendRenderTarget_destruct,
    C_GrBackendTexture_destruct, GrBackendFormat, GrBackendRenderTarget, GrBackendTexture,
};

#[cfg(feature = "vulkan")]
use super::vk;

pub type BackendFormat = Handle<GrBackendFormat>;

impl NativeDrop for GrBackendFormat {
    fn drop(&mut self) {
        unsafe { C_GrBackendFormat_destruct(self) }
    }
}

impl Handle<GrBackendFormat> {
    pub fn new_gl(format: gl::Enum, target: gl::Enum) -> Self {
        Self::from_native(unsafe { GrBackendFormat::MakeGL(format, target) })
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(format: vk::Format) -> Self {
        Self::from_native(unsafe { GrBackendFormat::MakeVk(format) })
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan_ycbcr(conversion_info: &vk::YcbcrConversionInfo) -> Self {
        Self::from_native(unsafe { GrBackendFormat::MakeVk1(conversion_info.native()) })
    }

    pub fn backend_api(&self) -> BackendAPI {
        BackendAPI::from_native(unsafe { self.native().backend() })
    }

    pub fn gl_format(&self) -> Option<gl::Enum> {
        unsafe {
            #[allow(clippy::map_clone)]
            self.native()
                .getGLFormat()
                .to_option()
                .map(|format| *format)
        }
    }

    pub fn gl_target(&self) -> Option<gl::Enum> {
        unsafe {
            #[allow(clippy::map_clone)]
            self.native()
                .getGLTarget()
                .to_option()
                .map(|target| *target)
        }
    }

    #[cfg(feature = "vulkan")]
    pub fn vulkan_format(&self) -> Option<vk::Format> {
        unsafe {
            self.native()
                .getVkFormat()
                .to_option()
                .map(|format| *format)
        }
    }

    pub fn to_texture_2d(&self) -> Option<Self> {
        let new = Self::from_native(unsafe { self.native().makeTexture2D() });

        new.is_valid().if_true_some(new)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }
}

pub type BackendTexture = Handle<GrBackendTexture>;

impl NativeDrop for GrBackendTexture {
    fn drop(&mut self) {
        unsafe { C_GrBackendTexture_destruct(self) }
    }
}

impl NativeClone for GrBackendTexture {
    fn clone(&self) -> Self {
        unsafe { GrBackendTexture::new4(self) }
    }
}

impl Handle<GrBackendTexture> {
    pub unsafe fn new_gl(
        (width, height): (i32, i32),
        mip_mapped: MipMapped,
        gl_info: gl::TextureInfo,
    ) -> BackendTexture {
        Self::from_native_if_valid(GrBackendTexture::new1(
            width,
            height,
            mip_mapped.into_native(),
            gl_info.native(),
        ))
        .unwrap()
    }

    #[cfg(feature = "vulkan")]
    pub unsafe fn new_vulkan(
        (width, height): (i32, i32),
        vk_info: &vk::ImageInfo,
    ) -> BackendTexture {
        Self::from_native_if_valid(GrBackendTexture::new2(width, height, vk_info.native())).unwrap()
    }

    pub(crate) unsafe fn from_native_if_valid(
        backend_texture: GrBackendTexture,
    ) -> Option<BackendTexture> {
        backend_texture
            .fIsValid
            .if_true_then_some(|| BackendTexture::from_native(backend_texture))
    }

    pub fn width(&self) -> i32 {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { self.native().height() }
    }

    pub fn has_mip_maps(&self) -> bool {
        unsafe { self.native().hasMipMaps() }
    }

    pub fn backend(&self) -> BackendAPI {
        BackendAPI::from_native(unsafe { self.native().backend() })
    }

    pub fn gl_texture_info(&self) -> Option<gl::TextureInfo> {
        unsafe {
            let mut texture_info = gl::TextureInfo::default();
            self.native()
                .getGLTextureInfo(texture_info.native_mut())
                .if_true_some(texture_info)
        }
    }

    #[cfg(feature = "vulkan")]
    pub fn vulkan_image_info(&self) -> Option<vk::ImageInfo> {
        unsafe {
            // constructor not available.
            let mut image_info = vk::ImageInfo::default();
            self.native()
                .getVkImageInfo(image_info.native_mut())
                .if_true_some(image_info)
        }
    }

    #[cfg(feature = "vulkan")]
    pub fn set_vulkan_image_layout(&mut self, layout: vk::ImageLayout) -> &mut Self {
        unsafe { self.native_mut().setVkImageLayout(layout) }
        self
    }

    pub fn backend_format(&self) -> Option<BackendFormat> {
        let format = BackendFormat::from_native(unsafe { self.native().getBackendFormat() });

        format.is_valid().if_true_some(format)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }
}

pub type BackendRenderTarget = Handle<GrBackendRenderTarget>;

impl NativeDrop for GrBackendRenderTarget {
    fn drop(&mut self) {
        // does not link:
        // unsafe { GrBackendRenderTarget::destruct(self) }
        unsafe { C_GrBackendRenderTarget_destruct(self) }
    }
}

impl NativeClone for GrBackendRenderTarget {
    fn clone(&self) -> Self {
        unsafe { GrBackendRenderTarget::new5(self) }
    }
}

impl Handle<GrBackendRenderTarget> {
    pub fn new_gl(
        (width, height): (i32, i32),
        sample_count: impl Into<Option<usize>>,
        stencil_bits: usize,
        info: gl::FramebufferInfo,
    ) -> BackendRenderTarget {
        Self::from_native(unsafe {
            GrBackendRenderTarget::new1(
                width,
                height,
                sample_count.into().unwrap_or(0).try_into().unwrap(),
                stencil_bits.try_into().unwrap(),
                info.native(),
            )
        })
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(
        (width, height): (i32, i32),
        sample_count: impl Into<Option<usize>>,
        info: &vk::ImageInfo,
    ) -> BackendRenderTarget {
        BackendRenderTarget::from_native(unsafe {
            GrBackendRenderTarget::new3(
                width,
                height,
                sample_count.into().unwrap_or(0).try_into().unwrap(),
                info.native(),
            )
        })
    }

    pub(crate) fn from_native_if_valid(
        native: GrBackendRenderTarget,
    ) -> Option<BackendRenderTarget> {
        let backend_render_target = BackendRenderTarget::from_native(native);
        backend_render_target
            .is_valid()
            .if_true_some(backend_render_target)
    }

    pub fn width(&self) -> i32 {
        unsafe { self.native().width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { self.native().height() }
    }

    pub fn sample_count(&self) -> usize {
        unsafe { self.native().sampleCnt().try_into().unwrap() }
    }

    pub fn stencil_bits(&self) -> usize {
        unsafe { self.native().stencilBits().try_into().unwrap() }
    }

    pub fn backend(&self) -> BackendAPI {
        /* does not link
        BackendAPI::from_native(unsafe {
            self.native().backend()
        }) */

        BackendAPI::from_native(unsafe { C_GrBackendRenderTarget_backend(self.native()) })
    }

    pub fn gl_framebuffer_info(&self) -> Option<gl::FramebufferInfo> {
        let mut info = gl::FramebufferInfo::default();
        unsafe { self.native().getGLFramebufferInfo(info.native_mut()) }.if_true_some(info)
    }

    #[cfg(feature = "vulkan")]
    pub fn vulkan_image_info(&self) -> Option<vk::ImageInfo> {
        let mut info = vk::ImageInfo::default();
        unsafe { self.native().getVkImageInfo(info.native_mut()) }.if_true_some(info)
    }

    #[cfg(feature = "vulkan")]
    pub fn set_vulkan_image_layout(&mut self, layout: vk::ImageLayout) -> &mut Self {
        unsafe { self.native_mut().setVkImageLayout(layout) }
        self
    }

    pub fn is_valid(&self) -> bool {
        unsafe { self.native().isValid() }
    }
}
