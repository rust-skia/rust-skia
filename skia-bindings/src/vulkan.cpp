#ifndef SK_VULKAN
    #define SK_VULKAN
#endif

#include "include/gpu/GrBackendDrawableInfo.h"
#include "include/gpu/GrBackendSurface.h"
#include "include/gpu/GrDirectContext.h"
#include "include/gpu/MutableTextureState.h"
#include "include/gpu/ganesh/vk/GrVkBackendSurface.h"
#include "include/gpu/ganesh/vk/GrVkDirectContext.h"
#include "include/gpu/vk/GrVkTypes.h"
#include "include/gpu/vk/GrVkBackendContext.h"
#include "include/gpu/vk/GrVkExtensions.h"
#include "include/gpu/vk/VulkanMutableTextureState.h"

// Additional types not yet referenced.
extern "C" void C_GrVkTypes(GrVkSurfaceInfo *) {};

extern "C" void C_GrBackendFormat_ConstructVk(GrBackendFormat* uninitialized, VkFormat format, bool willUseDRMFormatModifiers) {
    new(uninitialized)GrBackendFormat(GrBackendFormats::MakeVk(format, willUseDRMFormatModifiers));
}

extern "C" void C_GrBackendFormat_ConstructVk2(GrBackendFormat* uninitialized, const GrVkYcbcrConversionInfo* ycbcrInfo,  bool willUseDRMFormatModifiers) {
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

extern "C" void C_GPU_VK_Types(GrVkExtensionFlags *, GrVkFeatureFlags *, VkBuffer *) {}

typedef PFN_vkVoidFunction (*GetProcFn)(const char* name, VkInstance instance, VkDevice device);
typedef const void* (*GetProcFnVoidPtr)(const char* name, VkInstance instance, VkDevice device);

extern "C" void *C_GrVkBackendContext_new(
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

    auto &extensions = *new GrVkExtensions();
    extensions.init(vkGetProc, vkInstance, vkPhysicalDevice, instanceExtensionCount, instanceExtensions, deviceExtensionCount, deviceExtensions);
    auto &context = *new GrVkBackendContext();
    context.fInstance = vkInstance;
    context.fPhysicalDevice = vkPhysicalDevice;
    context.fDevice = vkDevice;
    context.fQueue = static_cast<VkQueue>(queue);
    context.fGraphicsQueueIndex = graphicsQueueIndex;
    context.fVkExtensions = &extensions;
    context.fGetProc = vkGetProc;
    return &context;
}

extern "C" void C_GrVkBackendContext_delete(void* vkBackendContext) {
    auto bc = static_cast<GrVkBackendContext*>(vkBackendContext);
    if (bc) {
        delete bc->fVkExtensions;
    }
    delete bc;
}

extern "C" void C_GrVkBackendContext_setProtectedContext(GrVkBackendContext *self, GrProtected protectedContext) {
    self->fProtectedContext = protectedContext;
}

extern "C" void C_GrVkBackendContext_setMaxAPIVersion(GrVkBackendContext *self, uint32_t maxAPIVersion) {
    self->fMaxAPIVersion = maxAPIVersion;
}

//
// GrVkTypes.h
//

extern "C" bool C_GrVkAlloc_Equals(const GrVkAlloc* lhs, const GrVkAlloc* rhs) {
    return *lhs == *rhs;
}

extern "C" bool C_GrVkYcbcrConversionInfo_Equals(const GrVkYcbcrConversionInfo* lhs, const GrVkYcbcrConversionInfo* rhs) {
    return *lhs == *rhs;
}

//
// gpu/ganesh/vk
//

extern "C" bool C_GrBackendFormats_AsVkFormat(const GrBackendFormat* format, VkFormat* vkFormat) {
    return GrBackendFormats::AsVkFormat(*format, vkFormat);
}

extern "C" const GrVkYcbcrConversionInfo* C_GrBackendFormats_GetVkYcbcrConversionInfo(const GrBackendFormat* format) {
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
    const GrVkBackendContext* vkBackendContext,
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
