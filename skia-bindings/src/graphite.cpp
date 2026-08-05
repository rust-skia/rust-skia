#include "bindings.h"

#include <algorithm>
#include <cstring>
#include <memory>

#include "include/core/SkCanvas.h"
#include "include/core/SkColorSpace.h"
#include "include/core/SkImage.h"
#include "include/core/SkRect.h"
#include "include/core/SkSize.h"
#include "include/core/SkSurface.h"
#include "include/gpu/GpuTypes.h"
#include "include/gpu/graphite/BackendTexture.h"
#include "include/gpu/graphite/Context.h"
#include "include/gpu/graphite/ContextOptions.h"
#include "include/gpu/graphite/GraphiteTypes.h"
#include "include/gpu/graphite/Image.h"
#include "include/gpu/graphite/Recorder.h"
#include "include/gpu/graphite/Surface.h"
#include "include/gpu/graphite/TextureInfo.h"

#ifdef SK_METAL
#include "include/gpu/graphite/mtl/MtlBackendContext.h"
#include "include/gpu/graphite/mtl/MtlGraphiteTypes_cpp.h"
#endif

#ifdef SK_VULKAN
#include "include/gpu/vk/VulkanBackendContext.h"
#include "include/gpu/graphite/vk/VulkanGraphiteContext.h"
#endif

// Forward declaration to avoid including Recording.h which exposes std::unordered_set
namespace skgpu::graphite {
    class Recording;
}

extern "C" void C_GraphiteUnreferencedTypes(skgpu::Budgeted *, skgpu::Mipmapped *, skgpu::Budgeted *) {}

//
// gpu/graphite/BackendTexture.h
//

extern "C" void C_BackendTexture_Construct(skgpu::graphite::BackendTexture* uninitialized) {
    new(uninitialized) skgpu::graphite::BackendTexture();
}

extern "C" void C_BackendTexture_CopyConstruct(skgpu::graphite::BackendTexture* uninitialized, const skgpu::graphite::BackendTexture* backendTexture) {
    new(uninitialized) skgpu::graphite::BackendTexture(*backendTexture);
}

extern "C" void C_BackendTexture_destruct(skgpu::graphite::BackendTexture* self) {
    self->~BackendTexture();
}

extern "C" bool C_BackendTexture_isValid(const skgpu::graphite::BackendTexture* self) {
    return self->isValid();
}

extern "C" skgpu::BackendApi C_BackendTexture_backend(const skgpu::graphite::BackendTexture* self) {
    return self->backend();
}

extern "C" void C_BackendTexture_dimensions(const skgpu::graphite::BackendTexture* self, SkISize* dimensions) {
    *dimensions = self->dimensions();
}

extern "C" void C_BackendTexture_info(const skgpu::graphite::BackendTexture* self, skgpu::graphite::TextureInfo* info) {
    *info = self->info();
}

//
// gpu/graphite/TextureInfo.h
//

extern "C" void C_TextureInfo_Construct(skgpu::graphite::TextureInfo* uninitialized) {
    new(uninitialized) skgpu::graphite::TextureInfo();
}

extern "C" void C_TextureInfo_destruct(skgpu::graphite::TextureInfo* self) {
    self->~TextureInfo();
}

extern "C" bool C_TextureInfo_isValid(const skgpu::graphite::TextureInfo* self) {
    return self->isValid();
}

extern "C" skgpu::BackendApi C_TextureInfo_backend(const skgpu::graphite::TextureInfo* self) {
    return self->backend();
}

extern "C" bool C_TextureInfo_Equals(const skgpu::graphite::TextureInfo* lhs, const skgpu::graphite::TextureInfo* rhs) {
    return *lhs == *rhs;
}

//
// gpu/graphite/Context.h
//

extern "C" skgpu::graphite::Recorder* C_Context_makeRecorder(skgpu::graphite::Context* self, const skgpu::graphite::RecorderOptions* options) {
    return self->makeRecorder(*options).release();
}

extern "C" skgpu::graphite::InsertStatus::V C_Context_insertRecording(skgpu::graphite::Context* self, const skgpu::graphite::InsertRecordingInfo* info) {
    return self->insertRecording(*info);
}

extern "C" bool C_Context_submit(skgpu::graphite::Context* self, const skgpu::graphite::SubmitInfo* submitInfo) {
    return self->submit(submitInfo ? *submitInfo : skgpu::graphite::SubmitInfo{});
}

extern "C" void C_Context_checkAsyncWorkCompletion(skgpu::graphite::Context* self) {
    self->checkAsyncWorkCompletion();
}

extern "C" void C_Context_deleteBackendTexture(skgpu::graphite::Context* self, const skgpu::graphite::BackendTexture* backendTexture) {
    self->deleteBackendTexture(*backendTexture);
}

extern "C" bool C_Context_isDeviceLost(const skgpu::graphite::Context* self) {
    return self->isDeviceLost();
}

// skgpu::graphite::Context is owned via std::unique_ptr (Context::MakeMetal etc.
// return unique_ptr and the shim releases it). It is NOT ref-counted, so the
// Rust wrapper must `delete` it rather than unref a (non-existent) SkRefCntBase.
extern "C" void C_Context_delete(skgpu::graphite::Context* self) {
    delete self;
}

// Synchronous pixel readback for a Graphite-backed surface.
//
// Graphite is a deferred backend: SkSurface::readPixels() does not work, so the
// only supported readback is Context::asyncRescaleAndReadPixels() driven to
// completion by a synchronous submit. This shim hides the async callback dance
// behind a blocking call that copies one plane into the caller's buffer, which
// is what a screenshot / golden-image path needs.
namespace {
struct SyncReadContext {
    bool fCalled = false;
    bool fSuccess = false;
    void* fDst = nullptr;
    size_t fDstRowBytes = 0;
    size_t fMinRowBytes = 0;
    int fHeight = 0;
};

void sync_read_callback(
    SkImage::ReadPixelsContext context,
    std::unique_ptr<const SkImage::AsyncReadResult> result) {
    auto* c = static_cast<SyncReadContext*>(context);
    c->fCalled = true;
    if (!result || result->count() != 1) {
        c->fSuccess = false;
        return;
    }
    const char* src = static_cast<const char*>(result->data(0));
    size_t srcRowBytes = result->rowBytes(0);
    size_t copyBytes = std::min(c->fMinRowBytes, c->fDstRowBytes);
    for (int y = 0; y < c->fHeight; ++y) {
        memcpy(static_cast<char*>(c->fDst) + static_cast<size_t>(y) * c->fDstRowBytes,
               src + static_cast<size_t>(y) * srcRowBytes,
               copyBytes);
    }
    c->fSuccess = true;
}
}  // namespace

extern "C" bool C_Context_readPixels(
    skgpu::graphite::Context* self,
    SkSurface* surface,
    const SkImageInfo* dstInfo,
    void* dstPixels,
    size_t dstRowBytes,
    int srcX,
    int srcY) {
    SyncReadContext ctx;
    ctx.fDst = dstPixels;
    ctx.fDstRowBytes = dstRowBytes;
    ctx.fMinRowBytes = dstInfo->minRowBytes();
    ctx.fHeight = dstInfo->height();

    SkIRect srcRect = SkIRect::MakeXYWH(srcX, srcY, dstInfo->width(), dstInfo->height());
    self->asyncRescaleAndReadPixels(
        surface,
        *dstInfo,
        srcRect,
        SkImage::RescaleGamma::kSrc,
        SkImage::RescaleMode::kNearest,
        &sync_read_callback,
        &ctx);

    // Force submission and block until the GPU work — and therefore the async
    // readback copy — has completed.
    self->submit(skgpu::graphite::SubmitInfo(skgpu::graphite::SyncToCpu::kYes));

    // After a synchronous submit the finished procs run, but pump
    // checkAsyncWorkCompletion() defensively until the callback fires. Bounded so
    // an unsupported / failed readback returns false instead of hanging.
    int guard = 0;
    while (!ctx.fCalled && guard++ < 10000) {
        self->checkAsyncWorkCompletion();
    }
    return ctx.fSuccess;
}

//
// gpu/graphite/GraphiteTypes.h
//

// InsertRecordingInfo is not POD: fSimulatedStatus is an InsertStatus, which
// holds a std::string. It must be heap-allocated (new/delete) rather than
// zero-initialized or placement-constructed into Rust-owned storage: a
// libstdc++ std::string in SSO state points into itself, so Rust moving the
// struct by value would leave that pointer dangling and the destructor would
// free() an invalid pointer. The heap object never moves.
extern "C" skgpu::graphite::InsertRecordingInfo* C_InsertRecordingInfo_new() {
    return new skgpu::graphite::InsertRecordingInfo();
}

extern "C" void C_InsertRecordingInfo_delete(skgpu::graphite::InsertRecordingInfo* self) {
    delete self;
}

//
// gpu/graphite/ContextOptions.h
//

extern "C" void C_ContextOptions_Construct(skgpu::graphite::ContextOptions* uninitialized) {
    new(uninitialized) skgpu::graphite::ContextOptions();
}

//
// gpu/graphite/Recorder.h
//

// RecorderOptions has a non-trivial constructor (e.g. fGpuBudgetInBytes defaults
// to 256 MiB, an sk_sp member, std::optional) and destructor, so it must be
// placement-constructed and destructed rather than zero-initialized.
extern "C" void C_RecorderOptions_Construct(skgpu::graphite::RecorderOptions* uninitialized) {
    new(uninitialized) skgpu::graphite::RecorderOptions();
}

extern "C" void C_RecorderOptions_destruct(skgpu::graphite::RecorderOptions* self) {
    self->~RecorderOptions();
}

extern "C" skgpu::graphite::Recording* C_Recorder_snap(skgpu::graphite::Recorder* self) {
    return self->snap().release();
}



extern "C" SkCanvas* C_Recorder_makeDeferredCanvas(skgpu::graphite::Recorder* self, const SkImageInfo* imageInfo, const skgpu::graphite::TextureInfo* textureInfo) {
    return self->makeDeferredCanvas(*imageInfo, *textureInfo);
}

extern "C" skgpu::BackendApi C_Recorder_backend(const skgpu::graphite::Recorder* self) {
    return self->backend();
}

// skgpu::graphite::Recorder is owned via std::unique_ptr (Context::makeRecorder
// returns unique_ptr and the shim releases it). It is NOT ref-counted, so the
// Rust wrapper must `delete` it.
extern "C" void C_Recorder_delete(skgpu::graphite::Recorder* self) {
    delete self;
}

//
// gpu/graphite/Recording.h
//

extern "C" void C_Recording_delete(const skgpu::graphite::Recording* self) {
    delete self;
}

//
// core/SkCanvas.h (Graphite-specific extensions)
//

extern "C" skgpu::graphite::Recorder* C_SkCanvas_recorder(const SkCanvas* self) {
    return self->recorder();
}

//
// gpu/graphite/Surface.h
//

extern "C" SkSurface* C_SkSurfaces_RenderTargetGraphite(
    skgpu::graphite::Recorder* recorder,
    const SkImageInfo* imageInfo,
    skgpu::Mipmapped mipmapped,
    const SkSurfaceProps* props,
    const char* label) {
    return SkSurfaces::RenderTarget(
            recorder,
            *imageInfo,
            mipmapped,
            props,
            label ? std::string_view(label) : std::string_view()).release();
}

extern "C" SkSurface* C_SkSurfaces_WrapBackendTextureGraphite(
        skgpu::graphite::Recorder* recorder,
        const skgpu::graphite::BackendTexture* backendTexture,
        SkColorType colorType,
        SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps) {
    return SkSurfaces::WrapBackendTexture(
            recorder,
            *backendTexture,
            colorType,
            sp(colorSpace),
            surfaceProps).release();
}

extern "C" SkImage* C_SkSurfaces_AsImageGraphite(SkSurface* surface) {
    return SkSurfaces::AsImage(sp(surface)).release();
}

extern "C" SkImage* C_SkSurfaces_AsImageCopyGraphite(
    SkSurface* surface,
    const SkIRect* subset,
    skgpu::Mipmapped mipmapped) {
    return SkSurfaces::AsImageCopy(
            sp(surface),
            subset,
            mipmapped).release();
}

//
// gpu/graphite/Image.h
//

extern "C" SkImage* C_SkImages_WrapTextureGraphite(
        skgpu::graphite::Recorder* recorder,
        const skgpu::graphite::BackendTexture* backendTexture,
        SkColorType colorType,
        SkAlphaType alphaType,
        SkColorSpace* colorSpace) {
    return SkImages::WrapTexture(
            recorder,
            *backendTexture,
            colorType,
            alphaType,
            sp(colorSpace)).release();
}

extern "C" SkImage* C_SkImages_TextureFromImageGraphite(
        skgpu::graphite::Recorder* recorder,
        const SkImage* image) {
    return SkImages::TextureFromImage(
            recorder,
            image).release();
}

extern "C" SkImage* C_SkImages_SubsetTextureFromGraphite(
    skgpu::graphite::Recorder* recorder,
    const SkImage* image,
    const SkIRect* subset) {
    return SkImages::SubsetTextureFrom(
            recorder,
            image,
            *subset).release();
}

//
// gpu/graphite/mtl/MtlBackendContext.h
//

#ifdef SK_METAL
extern "C" void C_MtlBackendContext_Construct(
    skgpu::graphite::MtlBackendContext* uninitialized,
    const void* device, const void* queue) {
    new (uninitialized) skgpu::graphite::MtlBackendContext();
    uninitialized->fDevice.retain(static_cast<CFTypeRef>(const_cast<void*>(device)));
    uninitialized->fQueue.retain(static_cast<CFTypeRef>(const_cast<void*>(queue)));
}

extern "C" void C_MtlBackendContext_destruct(skgpu::graphite::MtlBackendContext* self) {
    self->~MtlBackendContext();
}

extern "C" skgpu::graphite::Context* C_ContextFactory_MakeMetal(
    const skgpu::graphite::MtlBackendContext* backendContext,
    const skgpu::graphite::ContextOptions* options) {
    return skgpu::graphite::ContextFactory::MakeMetal(*backendContext, *options).release();
}

extern "C" void C_BackendTextures_MakeMetal(
    skgpu::graphite::BackendTexture* uninitialized,
    int width,
    int height,
    const void* mtlTexture) {
    new(uninitialized) skgpu::graphite::BackendTexture(
        skgpu::graphite::BackendTextures::MakeMetal(
            SkISize::Make(width, height),
            static_cast<CFTypeRef>(const_cast<void*>(mtlTexture))
        )
    );
}
#endif

#ifdef SK_VULKAN
// skgpu::VulkanBackendContext is the backend-context type shared with Ganesh
// (already wrapped by skia_safe::gpu::vk::BackendContext). The graphite Vulkan
// context factory lives in the skgpu::graphite::ContextFactory namespace,
// mirroring MakeMetal above. The backend context is passed as an opaque pointer
// to match the existing gpu::vk binding, which keeps it as a heap void*.
extern "C" skgpu::graphite::Context* C_ContextFactory_MakeVulkan(
    const void* backendContext,
    const skgpu::graphite::ContextOptions* options) {
    return skgpu::graphite::ContextFactory::MakeVulkan(
        *static_cast<const skgpu::VulkanBackendContext*>(backendContext), *options).release();
}
#endif
