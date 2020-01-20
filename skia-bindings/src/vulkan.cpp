#include "include/gpu/GrBackendDrawableInfo.h"
#include "include/gpu/GrBackendSurface.h"
#include "include/gpu/GrContext.h"
#include "include/gpu/vk/GrVkVulkan.h"
#include "include/gpu/vk/GrVkTypes.h"
#include "include/gpu/vk/GrVkBackendContext.h"

extern "C" void C_GrBackendFormat_ConstructVk(GrBackendFormat* uninitialized, VkFormat format) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeVk(format));
}

extern "C" void C_GrBackendFormat_ConstructVk2(GrBackendFormat* uninitialized, const GrVkYcbcrConversionInfo* ycbcrInfo) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeVk(*ycbcrInfo));
}

extern "C" bool C_GrBackendDrawableInfo_getVkDrawableInfo(const GrBackendDrawableInfo* self, GrVkDrawableInfo* info) {
    return self->getVkDrawableInfo(info);
}

extern "C" void C_GPU_VK_Types(GrVkExtensionFlags *, GrVkFeatureFlags *) {}

// The GrVkBackendContext struct binding's length is too short
// because of the std::function that is used in it.

typedef PFN_vkVoidFunction (*GetProcFn)(const char* name, VkInstance instance, VkDevice device);
typedef const void* (*GetProcFnVoidPtr)(const char* name, VkInstance instance, VkDevice device);

extern "C" void* C_GrVkBackendContext_New(
        void* instance,
        void* physicalDevice,
        void* device,
        void* queue,
        uint32_t graphicsQueueIndex,

        /* PFN_vkVoidFunction makes us trouble on the Rust side */
        GetProcFnVoidPtr getProc) {

    auto& context = *new GrVkBackendContext();
    context.fInstance = static_cast<VkInstance>(instance);
    context.fPhysicalDevice = static_cast<VkPhysicalDevice>(physicalDevice);
    context.fDevice = static_cast<VkDevice>(device);
    context.fQueue = static_cast<VkQueue>(queue);
    context.fGraphicsQueueIndex = graphicsQueueIndex;

    context.fGetProc = *(reinterpret_cast<GetProcFn*>(&getProc));
    return &context;
}

extern "C" void C_GrVkBackendContext_Delete(void* vkBackendContext) {
    delete static_cast<GrVkBackendContext*>(vkBackendContext);
}

extern "C" GrContext* C_GrContext_MakeVulkan(const GrVkBackendContext* vkBackendContext) {
    return GrContext::MakeVulkan(*vkBackendContext).release();
}

//
// GrVkTypes.h
//

extern "C" void C_GrVkAlloc_Construct(GrVkAlloc* uninitialized, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, uint32_t flags) {
    new (uninitialized) GrVkAlloc(memory, offset, size, flags);
}

extern "C" bool C_GrVkAlloc_Equals(const GrVkAlloc* lhs, const GrVkAlloc* rhs) {
    return *lhs == *rhs;
}

extern "C" bool C_GrVkYcbcrConversionInfo_Equals(const GrVkYcbcrConversionInfo* lhs, const GrVkYcbcrConversionInfo* rhs) {
    return *lhs == *rhs;
}

extern "C" void C_GrVkImageInfo_updateImageLayout(GrVkImageInfo* self, VkImageLayout layout) {
    self->updateImageLayout(layout);
}

extern "C" bool C_GrVkImageInfo_Equals(const GrVkImageInfo* lhs, const GrVkImageInfo* rhs) {
    return *lhs == *rhs;
}
