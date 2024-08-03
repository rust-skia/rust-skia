#ifndef SK_VULKAN
    #define SK_VULKAN
#endif

#include "include/gpu/ganesh/vk/GrBackendDrawableInfo.h"
#include "include/gpu/GrBackendSurface.h"
#include "include/gpu/GrDirectContext.h"
#include "include/gpu/MutableTextureState.h"
#include "include/gpu/ganesh/vk/GrVkBackendSurface.h"
#include "include/gpu/ganesh/vk/GrVkDirectContext.h"
#include "include/gpu/vk/GrVkTypes.h"
#include "include/gpu/vk/VulkanBackendContext.h"
#include "include/gpu/vk/VulkanExtensions.h"
#include "include/gpu/vk/VulkanMutableTextureState.h"

// Additional types not yet referenced.
extern "C" void C_GrVkTypes(GrVkSurfaceInfo *) {};

extern "C" void C_GrBackendFormat_ConstructVk(GrBackendFormat* uninitialized, VkFormat format, bool willUseDRMFormatModifiers) {
    new(uninitialized)GrBackendFormat(GrBackendFormats::MakeVk(format, willUseDRMFormatModifiers));
}

extern "C" void C_GrBackendFormat_ConstructVk2(GrBackendFormat* uninitialized, const skgpu::VulkanYcbcrConversionInfo* ycbcrInfo,  bool willUseDRMFormatModifiers) {
    new(uninitialized)GrBackendFormat(GrBackendFormats::MakeVk(*ycbcrInfo, willUseDRMFormatModifiers));
}

extern "C" GrBackendTexture* C_GrBackendTexture_newVk(
    int width, int height,
    const GrVkImageInfo* vkInfo,
    const char* label,
    size_t labelCount) {
    return new GrBackendTexture(GrBackendTextures::MakeVk(width, height, *vkInfo, std::string_view(label, labelCount)));
}

extern "C" void C_GrBackendRenderTargets_ConstructVk(GrBackendRenderTarget* uninitialized, int width, int height, const GrVkImageInfo* vkInfo) {
    new (uninitialized) GrBackendRenderTarget(GrBackendRenderTargets::MakeVk(width, height, *vkInfo));
}

extern "C" bool C_GrBackendDrawableInfo_getVkDrawableInfo(const GrBackendDrawableInfo* self, GrVkDrawableInfo* info) {
    return self->getVkDrawableInfo(info);
}

extern "C" void C_GPU_VK_Types(VkBuffer *) {}

typedef PFN_vkVoidFunction (*GetProcFn)(const char* name, VkInstance instance, VkDevice device);
typedef const void* (*GetProcFnVoidPtr)(const char* name, VkInstance instance, VkDevice device);

extern "C" void *C_VulkanBackendContext_new(
    void *instance,
    void *physicalDevice,
    void *device,
    void *queue,
    uint32_t graphicsQueueIndex,

    /* PFN_vkVoidFunction makes us trouble on the Rust side */
    GetProcFnVoidPtr getProc,
    const char *const *instanceExtensions, size_t instanceExtensionCount,
    const char *const *deviceExtensions, size_t deviceExtensionCount)
{
    auto vkInstance = static_cast<VkInstance>(instance);
    auto vkPhysicalDevice = static_cast<VkPhysicalDevice>(physicalDevice);
    auto vkDevice = static_cast<VkDevice>(device);
    auto vkGetProc = *(reinterpret_cast<GetProcFn *>(&getProc));

    auto &extensions = *new skgpu::VulkanExtensions();
    extensions.init(vkGetProc, vkInstance, vkPhysicalDevice, instanceExtensionCount, instanceExtensions, deviceExtensionCount, deviceExtensions);
    auto &context = *new skgpu::VulkanBackendContext();
    context.fInstance = vkInstance;
    context.fPhysicalDevice = vkPhysicalDevice;
    context.fDevice = vkDevice;
    context.fQueue = static_cast<VkQueue>(queue);
    context.fGraphicsQueueIndex = graphicsQueueIndex;
    context.fVkExtensions = &extensions;
    context.fGetProc = vkGetProc;
    return &context;
}

extern "C" void C_VulkanBackendContext_delete(void* vkBackendContext) {
    auto bc = static_cast<skgpu::VulkanBackendContext*>(vkBackendContext);
    if (bc) {
        delete bc->fVkExtensions;
    }
    delete bc;
}

extern "C" void C_VulkanBackendContext_setProtectedContext(skgpu::VulkanBackendContext *self, GrProtected protectedContext) {
    self->fProtectedContext = protectedContext;
}

extern "C" void C_VulkanBackendContext_setMaxAPIVersion(skgpu::VulkanBackendContext *self, uint32_t maxAPIVersion) {
    self->fMaxAPIVersion = maxAPIVersion;
}

//
// VulkanTypes.h
//

extern "C" bool C_VulkanAlloc_Equals(const skgpu::VulkanAlloc* lhs, const skgpu::VulkanAlloc* rhs) {
    return *lhs == *rhs;
}

extern "C" bool C_VulkanYcbcrConversionInfo_Equals(const skgpu::VulkanYcbcrConversionInfo* lhs, const skgpu::VulkanYcbcrConversionInfo* rhs) {
    return *lhs == *rhs;
}

//
// gpu/ganesh/vk
//

extern "C" bool C_GrBackendFormats_AsVkFormat(const GrBackendFormat* format, VkFormat* vkFormat) {
    return GrBackendFormats::AsVkFormat(*format, vkFormat);
}

extern "C" const skgpu::VulkanYcbcrConversionInfo* C_GrBackendFormats_GetVkYcbcrConversionInfo(const GrBackendFormat* format) {
    return GrBackendFormats::GetVkYcbcrConversionInfo(*format);
}

extern "C" bool C_GrBackendTextures_GetVkImageInfo(const GrBackendTexture* texture, GrVkImageInfo* imageInfo) {
    return GrBackendTextures::GetVkImageInfo(*texture, imageInfo);
}

extern "C" void C_GrBackendTextures_SetVkImageLayout(GrBackendTexture* texture, VkImageLayout imageLayout) {
    GrBackendTextures::SetVkImageLayout(texture, imageLayout);
}

extern "C" bool C_GrBackendRenderTargets_GetVkImageInfo(const GrBackendRenderTarget* renderTarget, GrVkImageInfo* imageInfo) {
    return GrBackendRenderTargets::GetVkImageInfo(*renderTarget, imageInfo);
}

extern "C" void C_GrBackendRenderTargets_SetVkImageLayout(GrBackendRenderTarget* renderTarget, VkImageLayout imageLayout) {
    GrBackendRenderTargets::SetVkImageLayout(renderTarget, imageLayout);
}

extern "C" GrDirectContext* C_GrDirectContexts_MakeVulkan(
    const skgpu::VulkanBackendContext* vkBackendContext,
    const GrContextOptions* options) {
    if (options) {
        return GrDirectContexts::MakeVulkan(*vkBackendContext, *options).release();
    }
    return GrDirectContexts::MakeVulkan(*vkBackendContext).release();
}

// MutableTextureState.h

extern "C" skgpu::MutableTextureState* C_MutableTextureStates_ConstructVulkan(VkImageLayout layout, uint32_t queueFamilyIndex) {
    return new skgpu::MutableTextureState(skgpu::MutableTextureStates::MakeVulkan(layout, queueFamilyIndex));
}

extern "C" VkImageLayout C_MutableTextureStates_getVkImageLayout(const skgpu::MutableTextureState* self) {
    return skgpu::MutableTextureStates::GetVkImageLayout(self);
}

extern "C" uint32_t C_MutableTextureStates_getVkQueueFamilyIndex(const skgpu::MutableTextureState* self) {
    return skgpu::MutableTextureStates::GetVkQueueFamilyIndex(self);
}
