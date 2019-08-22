use skia_bindings as sb;
use std::ptr;

mod backend_context;
pub use backend_context::*;

mod types;
pub use types::*;

//
// Additional Vulkan re-exports and definitions.
//

pub use sb::VkBool32 as Bool32;
pub use sb::VkBuffer as Buffer;
pub use sb::VkChromaLocation as ChromaLocation;
pub use sb::VkCommandBuffer as CommandBuffer;
pub use sb::VkDevice as Device;
pub use sb::VkDeviceMemory as DeviceMemory;
pub use sb::VkDeviceSize as DeviceSize;
pub use sb::VkExtent2D as Extent2D;
pub use sb::VkFilter as Filter;
pub use sb::VkFlags as Flags;
pub use sb::VkFormat as Format;
pub use sb::VkFormatFeatureFlags as FormatFeatureFlags;
pub use sb::VkImage as Image;
pub use sb::VkImageLayout as ImageLayout;
pub use sb::VkImageTiling as ImageTiling;
pub use sb::VkInstance as Instance;
pub use sb::VkOffset2D as Offset2D;
pub use sb::VkPhysicalDevice as PhysicalDevice;
pub use sb::VkPhysicalDeviceFeatures as PhysicalDeviceFeatures;
pub use sb::VkPhysicalDeviceFeatures2 as PhysicalDeviceFeatures2;
pub use sb::VkQueue as Queue;
pub use sb::VkRect2D as Rect2D;
pub use sb::VkRenderPass as RenderPass;
pub use sb::VkSamplerYcbcrModelConversion as SamplerYcbcrModelConversion;
pub use sb::VkSamplerYcbcrRange as SamplerYcbcrRange;
pub use sb::VkStructureType as StructureType;

pub const QUEUE_FAMILY_IGNORED: u32 = !0;

//
// VK_NULL_HANDLE and conversions.
//

pub struct NullHandle;
pub const NULL_HANDLE: NullHandle = NullHandle;

impl From<NullHandle> for Buffer {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}

impl From<NullHandle> for CommandBuffer {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}

impl From<NullHandle> for Device {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}

impl From<NullHandle> for DeviceMemory {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}

impl From<NullHandle> for Instance {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}

impl From<NullHandle> for PhysicalDevice {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}

impl From<NullHandle> for Image {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}

impl From<NullHandle> for Queue {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}

impl From<NullHandle> for RenderPass {
    fn from(_: NullHandle) -> Self {
        ptr::null_mut()
    }
}
