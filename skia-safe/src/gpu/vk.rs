use skia_bindings::{
    VkBool32, VkChromaLocation, VkCommandBuffer, VkDevice, VkDeviceMemory, VkDeviceSize, VkFilter,
    VkFormat, VkFormatFeatureFlags, VkImage, VkImageLayout, VkImageTiling, VkInstance,
    VkPhysicalDevice, VkQueue, VkRect2D, VkRenderPass, VkSamplerYcbcrModelConversion,
    VkSamplerYcbcrRange,
};
use std::ptr;

mod backend_context;
pub use self::backend_context::*;

mod types;
pub use self::types::*;

//
// Additional Vulkan re-exports and definitions.
//

pub type Device = VkDevice;
pub type PhysicalDevice = VkPhysicalDevice;
pub type Instance = VkInstance;
pub type Queue = VkQueue;
pub type DeviceMemory = VkDeviceMemory;
pub type DeviceSize = VkDeviceSize;
pub type Image = VkImage;
pub type ImageTiling = VkImageTiling;
pub type ImageLayout = VkImageLayout;
pub type SamplerYcbcrModelConversion = VkSamplerYcbcrModelConversion;
pub type SamplerYcbcrRange = VkSamplerYcbcrRange;
pub type ChromaLocation = VkChromaLocation;
pub type Filter = VkFilter;
pub type Bool32 = VkBool32;
pub type FormatFeatureFlags = VkFormatFeatureFlags;
pub type Format = VkFormat;
pub type CommandBuffer = VkCommandBuffer;
pub type RenderPass = VkRenderPass;
pub type Rect2D = VkRect2D;

pub const QUEUE_FAMILY_IGNORED: u32 = !0;

//
// VK_NULL_HANDLE and conversions.
//

pub struct NullHandle;
pub const NULL_HANDLE: NullHandle = NullHandle;

impl From<NullHandle> for VkDeviceMemory {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}

impl From<NullHandle> for VkImage {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}
