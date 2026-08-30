use std::fmt;

use skia_bindings::{self as sb, GrBackendFormat, GrBackendRenderTarget, GrBackendTexture};

use crate::gpu;
use crate::{ISize, interop::AsStr, prelude::*};
#[cfg(feature = "d3d")]
use gpu::d3d;
#[cfg(feature = "gl")]
use gpu::gl;
#[cfg(feature = "metal")]
use gpu::mtl;
#[cfg(feature = "vulkan")]
use gpu::vk;
use gpu::{BackendAPI, Mipmapped, MutableTextureState};

pub type BackendFormat = Handle<GrBackendFormat>;
unsafe_send_sync!(BackendFormat);

impl NativeDrop for GrBackendFormat {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendFormat_destruct(self) }
    }
}

impl NativeClone for GrBackendFormat {
    fn clone(&self) -> Self {
        unsafe { GrBackendFormat::new1(self) }
    }
}

impl fmt::Debug for BackendFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("BackendFormat");
        d.field("backend", &self.backend());
        d.field("channel_mask", &self.channel_mask());
        #[cfg(feature = "gl")]
        d.field("gl_format", &self.as_gl_format());
        #[cfg(feature = "vulkan")]
        d.field("vk_format", &self.as_vk_format());
        #[cfg(feature = "metal")]
        d.field("mtl_format", &self.as_mtl_format());
        #[cfg(feature = "d3d")]
        d.field("dxgi_format", &self.as_dxgi_format());
        d.finish()
    }
}

impl BackendFormat {
    pub(crate) fn new_invalid() -> Self {
        Self::construct(|bf| unsafe { sb::C_GrBackendFormat_Construct(bf) })
    }

    #[cfg(feature = "gl")]
    pub fn new_gl(format: gl::Enum, target: gl::Enum) -> Self {
        Self::construct(|bf| unsafe { sb::C_GrBackendFormats_ConstructGL(bf, format, target) })
            .assert_valid()
    }

    pub fn backend(&self) -> BackendAPI {
        self.native().fBackend
    }

    pub fn channel_mask(&self) -> u32 {
        unsafe { self.native().channelMask() }
    }

    // m117: Even though Skia did, we won't deprecate these functions here for convenience.

    #[cfg(feature = "gl")]
    pub fn as_gl_format(&self) -> gl::Format {
        gpu::backend_formats::as_gl_format(self)
    }

    #[cfg(feature = "gl")]
    pub fn as_gl_format_enum(&self) -> gl::Enum {
        gpu::backend_formats::as_gl_format_enum(self)
    }

    // Deprecated in Skia
    #[cfg(feature = "vulkan")]
    pub fn as_vk_format(&self) -> Option<vk::Format> {
        gpu::backend_formats::as_vk_format(self)
    }

    #[cfg(feature = "metal")]
    pub fn as_mtl_format(&self) -> Option<mtl::PixelFormat> {
        gpu::backend_formats::as_mtl_format(self)
    }

    #[cfg(feature = "d3d")]
    pub fn as_dxgi_format(&self) -> Option<d3d::DXGI_FORMAT> {
        gpu::backend_formats::as_dxgi_format(self)
    }

    #[must_use]
    pub fn to_texture_2d(&self) -> Self {
        let mut new = Self::new_invalid();
        unsafe { sb::C_GrBackendFormat_makeTexture2D(self.native(), new.native_mut()) };
        assert!(Self::native_is_valid(new.native()));
        new
    }

    pub(crate) fn native_is_valid(format: &GrBackendFormat) -> bool {
        format.fValid
    }

    pub(crate) fn assert_valid(self) -> Self {
        assert!(Self::native_is_valid(self.native()));
        self
    }
}

// GrBackendTexture contains a string `fLabel`, and with SSO on some platforms, it can't be moved.
// See <https://github.com/rust-skia/rust-skia/issues/750>.
pub type BackendTexture = RefHandle<GrBackendTexture>;
unsafe_send_sync!(BackendTexture);

impl NativeDrop for GrBackendTexture {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendTexture_delete(self) }
    }
}

impl Clone for BackendTexture {
    fn clone(&self) -> Self {
        unsafe { Self::from_ptr(sb::C_GrBackendTexture_Clone(self.native())) }.unwrap()
    }
}

impl fmt::Debug for BackendTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("BackendTexture");
        d.field("dimensions", &self.dimensions());
        d.field("label", &self.label());
        d.field("mipmapped", &self.mipmapped());
        d.field("backend", &self.backend());
        #[cfg(feature = "gl")]
        d.field("gl_texture_info", &self.gl_texture_info());
        #[cfg(feature = "vulkan")]
        d.field("vulkan_image_info", &self.vulkan_image_info());
        #[cfg(feature = "metal")]
        d.field("metal_texture_info", &self.metal_texture_info());
        #[cfg(feature = "d3d")]
        d.field(
            "d3d_texture_resource_info",
            &self.d3d_texture_resource_info(),
        );
        d.field("backend_format", &self.backend_format());
        d.field("is_protected", &self.is_protected());
        d.finish()
    }
}

impl BackendTexture {
    pub(crate) fn new_invalid() -> Self {
        Self::from_ptr(unsafe { sb::C_GrBackendTexture_new() }).unwrap()
    }

    pub(crate) unsafe fn from_native_if_valid(
        backend_texture: *mut GrBackendTexture,
    ) -> Option<BackendTexture> {
        unsafe { Self::native_is_valid(backend_texture) }
            .then(|| BackendTexture::from_ptr(backend_texture).unwrap())
    }

    pub fn dimensions(&self) -> ISize {
        ISize::new(self.width(), self.height())
    }

    pub fn width(&self) -> i32 {
        self.native().fWidth
    }

    pub fn height(&self) -> i32 {
        self.native().fHeight
    }

    pub fn label(&self) -> &str {
        self.native().fLabel.as_str()
    }

    pub fn mipmapped(&self) -> Mipmapped {
        self.native().fMipmapped
    }

    pub fn has_mipmaps(&self) -> bool {
        self.native().fMipmapped == Mipmapped::Yes
    }

    pub fn backend(&self) -> BackendAPI {
        self.native().fBackend
    }

    // Deprecated in Skia
    #[cfg(feature = "gl")]
    pub fn gl_texture_info(&self) -> Option<gl::TextureInfo> {
        gpu::backend_textures::get_gl_texture_info(self)
    }

    // Deprecated in Skia
    #[cfg(feature = "gl")]
    pub fn gl_texture_parameters_modified(&mut self) {
        gpu::backend_textures::gl_texture_parameters_modified(self)
    }

    // Deprecated in Skia
    #[cfg(feature = "vulkan")]
    pub fn vulkan_image_info(&self) -> Option<vk::ImageInfo> {
        gpu::backend_textures::get_vk_image_info(self)
    }

    // Deprecated in Skia
    #[cfg(feature = "vulkan")]
    pub fn set_vulkan_image_layout(&mut self, layout: vk::ImageLayout) -> &mut Self {
        gpu::backend_textures::set_vk_image_layout(self, layout)
    }

    #[cfg(feature = "metal")]
    pub fn metal_texture_info(&self) -> Option<mtl::TextureInfo> {
        gpu::backend_textures::get_mtl_texture_info(self)
    }

    #[cfg(feature = "d3d")]
    pub fn d3d_texture_resource_info(&self) -> Option<d3d::TextureResourceInfo> {
        gpu::backend_textures::get_d3d_texture_resource_info(self)
    }

    #[cfg(feature = "d3d")]
    pub fn set_d3d_resource_state(&mut self, resource_state: d3d::ResourceStateEnum) -> &mut Self {
        gpu::backend_textures::set_d3d_resource_state(self, resource_state)
    }

    pub fn backend_format(&self) -> BackendFormat {
        let mut format = BackendFormat::new_invalid();
        unsafe { sb::C_GrBackendTexture_getBackendFormat(self.native(), format.native_mut()) };
        assert!(BackendFormat::native_is_valid(format.native()));
        format
    }

    pub fn set_mutable_state(&mut self, state: &MutableTextureState) {
        unsafe { self.native_mut().setMutableState(state.native()) }
    }

    pub fn is_protected(&self) -> bool {
        unsafe { self.native().isProtected() }
    }

    pub(crate) unsafe fn native_is_valid(texture: *const GrBackendTexture) -> bool {
        unsafe { (*texture).fIsValid }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_same_texture(&mut self, texture: &BackendTexture) -> bool {
        unsafe { self.native_mut().isSameTexture(texture.native()) }
    }
}

pub type BackendRenderTarget = Handle<GrBackendRenderTarget>;
unsafe_send_sync!(BackendRenderTarget);

impl NativeDrop for GrBackendRenderTarget {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendRenderTarget_destruct(self) }
    }
}

impl NativeClone for GrBackendRenderTarget {
    fn clone(&self) -> Self {
        construct(|render_target| unsafe {
            sb::C_GrBackendRenderTarget_CopyConstruct(render_target, self)
        })
    }
}

impl fmt::Debug for BackendRenderTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("BackendRenderTarget");
        d.field("dimensions", &self.dimensions());
        d.field("sample_count", &self.sample_count());
        d.field("stencil_bits", &self.stencil_bits());
        d.field("backend", &self.backend());
        d.field("is_framebuffer_only", &self.is_framebuffer_only());
        #[cfg(feature = "gl")]
        d.field("gl_framebuffer_info", &self.gl_framebuffer_info());
        #[cfg(feature = "vulkan")]
        d.field("vulkan_image_info", &self.vulkan_image_info());
        #[cfg(feature = "metal")]
        d.field("metal_texture_info", &self.metal_texture_info());
        #[cfg(feature = "d3d")]
        d.field(
            "d3d_texture_resource_info",
            &self.d3d_texture_resource_info(),
        );
        d.field("backend_format", &self.backend_format());
        d.field("is_protected", &self.is_protected());
        d.finish()
    }
}

impl BackendRenderTarget {
    pub(crate) fn from_native_c_if_valid(
        native: GrBackendRenderTarget,
    ) -> Option<BackendRenderTarget> {
        let backend_render_target = BackendRenderTarget::from_native_c(native);
        Self::native_is_valid(backend_render_target.native()).then_some(backend_render_target)
    }

    pub fn dimensions(&self) -> ISize {
        ISize::new(self.width(), self.height())
    }

    pub fn width(&self) -> i32 {
        self.native().fWidth
    }

    pub fn height(&self) -> i32 {
        self.native().fHeight
    }

    pub fn sample_count(&self) -> usize {
        self.native().fSampleCnt.try_into().unwrap()
    }

    pub fn stencil_bits(&self) -> usize {
        self.native().fStencilBits.try_into().unwrap()
    }

    pub fn backend(&self) -> BackendAPI {
        self.native().fBackend
    }

    pub fn is_framebuffer_only(&self) -> bool {
        self.native().fFramebufferOnly
    }

    // Deprecated in Skia
    #[cfg(feature = "gl")]
    pub fn gl_framebuffer_info(&self) -> Option<gl::FramebufferInfo> {
        gpu::backend_render_targets::get_gl_framebuffer_info(self)
    }

    // Deprecated in Skia
    #[cfg(feature = "vulkan")]
    pub fn vulkan_image_info(&self) -> Option<vk::ImageInfo> {
        gpu::backend_render_targets::get_vk_image_info(self)
    }

    // Deprecated in Skia
    #[cfg(feature = "vulkan")]
    pub fn set_vulkan_image_layout(&mut self, layout: vk::ImageLayout) -> &mut Self {
        gpu::backend_render_targets::set_vk_image_layout(self, layout)
    }

    #[cfg(feature = "metal")]
    pub fn metal_texture_info(&self) -> Option<mtl::TextureInfo> {
        gpu::backend_render_targets::get_mtl_texture_info(self)
    }

    #[cfg(feature = "d3d")]
    pub fn d3d_texture_resource_info(&self) -> Option<d3d::TextureResourceInfo> {
        gpu::backend_render_targets::get_d3d_texture_resource_info(self)
    }

    #[cfg(feature = "d3d")]
    pub fn set_d3d_resource_state(&mut self, resource_state: d3d::ResourceStateEnum) -> &mut Self {
        gpu::backend_render_targets::set_d3d_resource_state(self, resource_state)
    }

    pub fn backend_format(&self) -> BackendFormat {
        BackendFormat::construct(|format| unsafe {
            sb::C_GrBackendRenderTarget_getBackendFormat(self.native(), format)
        })
    }

    pub fn set_mutable_state(&mut self, state: &MutableTextureState) {
        unsafe { self.native_mut().setMutableState(state.native()) }
    }

    pub fn is_protected(&self) -> bool {
        unsafe { self.native().isProtected() }
    }

    pub(crate) fn native_is_valid(rt: &GrBackendRenderTarget) -> bool {
        rt.fIsValid
    }
}

#[cfg(test)]
mod tests {
    use super::BackendTexture;
    use std::hint::black_box;

    // Regression test for <https://github.com/rust-skia/rust-skia/issues/750>
    #[test]
    fn create_move_and_drop_backend_texture() {
        let texture = force_move(BackendTexture::new_invalid());
        drop(texture);
    }

    fn force_move<V>(src: V) -> V {
        let src = black_box(src);
        *black_box(Box::new(src))
    }
}
