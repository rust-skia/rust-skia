use std::fmt;

use skia_bindings::{self as sb, GrBackendFormat, GrBackendRenderTarget, GrBackendTexture};

use super::types::BackendApi;
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
use gpu::{Mipmapped, MutableTextureState};

/// Describes a texture or render target format for a specific backend API.
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

    /// Creates a [`BackendFormat`] for a GL format and texture target.
    #[cfg(feature = "gl")]
    pub fn new_gl(format: gl::Enum, target: gl::Enum) -> Self {
        Self::construct(|bf| unsafe { sb::C_GrBackendFormats_ConstructGL(bf, format, target) })
            .assert_valid()
    }

    /// The [`crate::gpu::ganesh::BackendApi`] this format is used with.
    pub fn backend(&self) -> BackendApi {
        self.native().fBackend
    }

    /// Gets the channels present in the format as a bitfield of
    /// [`crate::ColorChannelFlag`] values. Luminance channels are reported as
    /// [`crate::ColorChannelFlag::GRAY`].
    pub fn channel_mask(&self) -> u32 {
        unsafe { self.native().channelMask() }
    }

    // m117: Even though Skia did, we won't deprecate these functions here for convenience.

    /// The GL format of this [`BackendFormat`], or `None` if this format is not GL.
    #[cfg(feature = "gl")]
    pub fn as_gl_format(&self) -> gl::Format {
        gpu::backend_formats::as_gl_format(self)
    }

    /// The GL format enum of this [`BackendFormat`], or `None` if this format is not GL.
    #[cfg(feature = "gl")]
    pub fn as_gl_format_enum(&self) -> gl::Enum {
        gpu::backend_formats::as_gl_format_enum(self)
    }

    // Deprecated in Skia
    /// The Vulkan format of this [`BackendFormat`], or `None` if this format is not Vulkan.
    #[cfg(feature = "vulkan")]
    pub fn as_vk_format(&self) -> Option<vk::Format> {
        gpu::backend_formats::as_vk_format(self)
    }

    /// The Metal pixel format of this [`BackendFormat`], or `None` if this format is not
    /// Metal.
    #[cfg(feature = "metal")]
    pub fn as_mtl_format(&self) -> Option<mtl::PixelFormat> {
        gpu::backend_formats::as_mtl_format(self)
    }

    /// The DXGI format of this [`BackendFormat`], or `None` if this format is not D3D.
    #[cfg(feature = "d3d")]
    pub fn as_dxgi_format(&self) -> Option<d3d::DXGI_FORMAT> {
        gpu::backend_formats::as_dxgi_format(self)
    }

    /// If possible, copies the [`BackendFormat`] and forces the texture type to be Texture2D.
    /// If the [`BackendFormat`] was for Vulkan and it originally had a
    /// skgpu::VulkanYcbcrConversionInfo, the conversion is removed and the format is set to be
    /// VK_FORMAT_R8G8B8A8_UNORM.
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
/// Describes a texture on a specific GPU backend. Its contents may be initialized or not.
pub type BackendTexture = RefHandle<GrBackendTexture>;
unsafe_send_sync!(BackendTexture);

impl NativeDelete for GrBackendTexture {
    unsafe fn delete(ptr: *mut Self) {
        unsafe { sb::C_GrBackendTexture_delete(ptr) }
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

    /// The dimensions of the texture in pixels.
    pub fn dimensions(&self) -> ISize {
        ISize::new(self.width(), self.height())
    }

    /// The width of the texture in pixels.
    pub fn width(&self) -> i32 {
        self.native().fWidth
    }

    /// The height of the texture in pixels.
    pub fn height(&self) -> i32 {
        self.native().fHeight
    }

    /// The optional label assigned to this texture.
    pub fn label(&self) -> &str {
        self.native().fLabel.as_str()
    }

    /// Whether the texture is mipmapped.
    pub fn mipmapped(&self) -> Mipmapped {
        self.native().fMipmapped
    }

    /// Returns true if the texture is mipmapped.
    pub fn has_mipmaps(&self) -> bool {
        self.native().fMipmapped == Mipmapped::Yes
    }

    /// The [`crate::gpu::ganesh::BackendApi`] this texture is used with.
    pub fn backend(&self) -> BackendApi {
        self.native().fBackend
    }

    // Deprecated in Skia
    /// The GL texture info, or `None` if this texture is not GL.
    #[cfg(feature = "gl")]
    pub fn gl_texture_info(&self) -> Option<gl::TextureInfo> {
        gpu::backend_textures::get_gl_texture_info(self)
    }

    // Deprecated in Skia
    /// Marks that the client has modified the GL texture parameters and Skia should re-read
    /// them.
    #[cfg(feature = "gl")]
    pub fn gl_texture_parameters_modified(&mut self) {
        gpu::backend_textures::gl_texture_parameters_modified(self)
    }

    // Deprecated in Skia
    /// The Vulkan image info, or `None` if this texture is not Vulkan.
    #[cfg(feature = "vulkan")]
    pub fn vulkan_image_info(&self) -> Option<vk::ImageInfo> {
        gpu::backend_textures::get_vk_image_info(self)
    }

    // Deprecated in Skia
    /// Sets the Vulkan image layout. This should be called when the client changes the image
    /// layout outside of Skia.
    #[cfg(feature = "vulkan")]
    pub fn set_vulkan_image_layout(&mut self, layout: vk::ImageLayout) -> &mut Self {
        gpu::backend_textures::set_vk_image_layout(self, layout)
    }

    /// The Metal texture info, or `None` if this texture is not Metal.
    #[cfg(feature = "metal")]
    pub fn metal_texture_info(&self) -> Option<mtl::TextureInfo> {
        gpu::backend_textures::get_mtl_texture_info(self)
    }

    /// The D3D texture resource info, or `None` if this texture is not D3D.
    #[cfg(feature = "d3d")]
    pub fn d3d_texture_resource_info(&self) -> Option<d3d::TextureResourceInfo> {
        gpu::backend_textures::get_d3d_texture_resource_info(self)
    }

    /// Sets the D3D resource state, informing Skia that the client has changed it outside of
    /// Skia.
    #[cfg(feature = "d3d")]
    pub fn set_d3d_resource_state(&mut self, resource_state: d3d::ResourceStateEnum) -> &mut Self {
        gpu::backend_textures::set_d3d_resource_state(self, resource_state)
    }

    /// Get the [`BackendFormat`] for this texture (or an invalid format if this is not valid).
    pub fn backend_format(&self) -> BackendFormat {
        let mut format = BackendFormat::new_invalid();
        unsafe { sb::C_GrBackendTexture_getBackendFormat(self.native(), format.native_mut()) };
        assert!(BackendFormat::native_is_valid(format.native()));
        format
    }

    /// If the client changes any of the mutable state of the [`BackendTexture`] they should
    /// call this function to inform Skia that those values have changed. The backend API
    /// specific state that can be set from this function are:
    ///
    /// Vulkan: VkImageLayout and QueueFamilyIndex
    pub fn set_mutable_state(&mut self, state: &MutableTextureState) {
        unsafe { sb::GrBackendTexture_setMutableState(self.native_ptr(), state.native()) }
    }

    /// Returns true if we are working with protected content.
    pub fn is_protected(&self) -> bool {
        unsafe { self.native().isProtected() }
    }

    pub(crate) unsafe fn native_is_valid(texture: *const GrBackendTexture) -> bool {
        unsafe { (*texture).fIsValid }
    }

    /// Returns true if both textures are valid and refer to the same API texture.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_same_texture(&mut self, texture: &BackendTexture) -> bool {
        unsafe { sb::GrBackendTexture_isSameTexture(self.native_ptr(), texture.native()) }
    }
}

/// Describes a render target on a specific GPU backend.
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

    /// The dimensions of the render target in pixels.
    pub fn dimensions(&self) -> ISize {
        ISize::new(self.width(), self.height())
    }

    /// The width of the render target in pixels.
    pub fn width(&self) -> i32 {
        self.native().fWidth
    }

    /// The height of the render target in pixels.
    pub fn height(&self) -> i32 {
        self.native().fHeight
    }

    /// The number of samples per pixel.
    pub fn sample_count(&self) -> usize {
        self.native().fSampleCnt.try_into().unwrap()
    }

    /// The number of stencil bits.
    pub fn stencil_bits(&self) -> usize {
        self.native().fStencilBits.try_into().unwrap()
    }

    /// The [`crate::gpu::ganesh::BackendApi`] this render target is used with.
    pub fn backend(&self) -> BackendApi {
        self.native().fBackend
    }

    /// Whether this render target is only usable as a framebuffer.
    pub fn is_framebuffer_only(&self) -> bool {
        self.native().fFramebufferOnly
    }

    // Deprecated in Skia
    /// The GL framebuffer info, or `None` if this render target is not GL.
    #[cfg(feature = "gl")]
    pub fn gl_framebuffer_info(&self) -> Option<gl::FramebufferInfo> {
        gpu::backend_render_targets::get_gl_framebuffer_info(self)
    }

    // Deprecated in Skia
    /// The Vulkan image info, or `None` if this render target is not Vulkan.
    #[cfg(feature = "vulkan")]
    pub fn vulkan_image_info(&self) -> Option<vk::ImageInfo> {
        gpu::backend_render_targets::get_vk_image_info(self)
    }

    // Deprecated in Skia
    /// Sets the Vulkan image layout, informing Skia that the client changed it outside of Skia.
    #[cfg(feature = "vulkan")]
    pub fn set_vulkan_image_layout(&mut self, layout: vk::ImageLayout) -> &mut Self {
        gpu::backend_render_targets::set_vk_image_layout(self, layout)
    }

    /// The Metal texture info, or `None` if this render target is not Metal.
    #[cfg(feature = "metal")]
    pub fn metal_texture_info(&self) -> Option<mtl::TextureInfo> {
        gpu::backend_render_targets::get_mtl_texture_info(self)
    }

    /// The D3D texture resource info, or `None` if this render target is not D3D.
    #[cfg(feature = "d3d")]
    pub fn d3d_texture_resource_info(&self) -> Option<d3d::TextureResourceInfo> {
        gpu::backend_render_targets::get_d3d_texture_resource_info(self)
    }

    /// Sets the D3D resource state, informing Skia that the client changed it outside of Skia.
    #[cfg(feature = "d3d")]
    pub fn set_d3d_resource_state(&mut self, resource_state: d3d::ResourceStateEnum) -> &mut Self {
        gpu::backend_render_targets::set_d3d_resource_state(self, resource_state)
    }

    /// Get the [`BackendFormat`] for this render target (or an invalid format if this is not
    /// valid).
    pub fn backend_format(&self) -> BackendFormat {
        BackendFormat::construct(|format| unsafe {
            sb::C_GrBackendRenderTarget_getBackendFormat(self.native(), format)
        })
    }

    /// If the client changes any of the mutable state of the [`BackendRenderTarget`] they
    /// should call this function to inform Skia that those values have changed. The backend
    /// API specific state that can be set from this function are:
    ///
    /// Vulkan: VkImageLayout and QueueFamilyIndex
    pub fn set_mutable_state(&mut self, state: &MutableTextureState) {
        unsafe { self.native_mut().setMutableState(state.native()) }
    }

    /// Returns true if we are working with protected content.
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
