#ifndef SK_VULKAN
    #define SK_VULKAN
#endif

#include "include/gpu/GrBackendDrawableInfo.h"
#include "include/gpu/GrBackendSurface.h"
#include "include/gpu/GrContext.h"
#include "include/gpu/GrDirectContext.h"
#include "include/gpu/vk/GrVkVulkan.h"
#include "include/gpu/vk/GrVkTypes.h"
#include "include/gpu/vk/GrVkBackendContext.h"
#include "include/gpu/vk/GrVkExtensions.h"

extern "C" void C_GrBackendFormat_ConstructVk(GrBackendFormat* uninitialized, VkFormat format) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeVk(format));
}

extern "C" void C_GrBackendFormat_ConstructVk2(GrBackendFormat* uninitialized, const GrVkYcbcrConversionInfo* ycbcrInfo) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeVk(*ycbcrInfo));
}

extern "C" void C_GrBackendTexture_ConstructVk(GrBackendTexture* uninitialized, int width, int height, const GrVkImageInfo* vkInfo) {
    new(uninitialized)GrBackendTexture(width, height, *vkInfo);
}

extern "C" void C_GrBackendRenderTarget_ConstructVk(GrBackendRenderTarget* uninitialized, int width, int height, int sampleCnt, const GrVkImageInfo* vkInfo) {
    new(uninitialized)GrBackendRenderTarget(width, height, sampleCnt, *vkInfo);
}

extern "C" bool C_GrBackendDrawableInfo_getVkDrawableInfo(const GrBackendDrawableInfo* self, GrVkDrawableInfo* info) {
    return self->getVkDrawableInfo(info);
}

extern "C" void C_GPU_VK_Types(GrVkExtensionFlags *, GrVkFeatureFlags *, VkBuffer *) {}

// The GrVkBackendContext struct binding's length is too short
// because of the std::function that is used in it.
// TODO: verify if this is actually true for the latest bindings (it doesn't seem so, because all skia-bindings testcases work and 
// GrVkBackendContext seems to be generated).

typedef PFN_vkVoidFunction (*GetProcFn)(const char* name, VkInstance instance, VkDevice device);
typedef const void* (*GetProcFnVoidPtr)(const char* name, VkInstance instance, VkDevice device);

extern "C" void *C_GrVkBackendContext_New(
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

extern "C" void C_GrVkBackendContext_Delete(void* vkBackendContext) {
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

extern "C" GrDirectContext* C_GrDirectContext_MakeVulkan(
    const GrVkBackendContext* vkBackendContext,
    const GrContextOptions* options) {
    if (options) {
        return GrDirectContext::MakeVulkan(*vkBackendContext, *options).release();
    }
    return GrDirectContext::MakeVulkan(*vkBackendContext).release();
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

//
// gpu/GrBackendSurfaceMutableState.h
//

extern "C" void C_GrBackendSurfaceMutableState_ConstructVK(GrBackendSurfaceMutableState* uninitialized, VkImageLayout layout, uint32_t queueFamilyIndex) {
    new(uninitialized)GrBackendSurfaceMutableState(layout, queueFamilyIndex);
}
