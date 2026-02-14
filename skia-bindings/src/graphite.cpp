#include "bindings.h"

#include "include/core/SkCanvas.h"
#include "include/core/SkColorSpace.h"
#include "include/core/SkImage.h"
#include "include/core/SkRect.h"
#include "include/core/SkSize.h"
#include "include/core/SkSpan.h"
#include "include/core/SkSurface.h"
#include "include/core/SkYUVAInfo.h"
#include "include/gpu/GpuTypes.h"
#include "include/gpu/graphite/BackendTexture.h"
#include "include/gpu/graphite/Context.h"
#include "include/gpu/graphite/ContextOptions.h"
#include "include/gpu/graphite/GraphiteTypes.h"
#include "include/gpu/graphite/Image.h"
#include "include/gpu/graphite/Recorder.h"

#include "include/gpu/graphite/Surface.h"
#include "include/gpu/graphite/TextureInfo.h"
#include "include/gpu/graphite/YUVABackendTextures.h"

#ifdef SK_METAL
#include "include/gpu/graphite/mtl/MtlBackendContext.h"
#include "include/gpu/graphite/mtl/MtlGraphiteTypes_cpp.h"
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

extern "C" int C_Context_insertRecording(skgpu::graphite::Context* self, const skgpu::graphite::InsertRecordingInfo* info) {
    auto status = self->insertRecording(*info);
    return static_cast<int>(static_cast<skgpu::graphite::InsertStatus::V>(status));
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

//
// gpu/graphite/ContextOptions.h
//

extern "C" void C_ContextOptions_Construct(skgpu::graphite::ContextOptions* uninitialized) {
    new(uninitialized) skgpu::graphite::ContextOptions();
}

//
// gpu/graphite/Recorder.h
//

extern "C" skgpu::graphite::Recording* C_Recorder_snap(skgpu::graphite::Recorder* self) {
    return self->snap().release();
}



extern "C" SkCanvas* C_Recorder_makeDeferredCanvas(skgpu::graphite::Recorder* self, const SkImageInfo* imageInfo, const skgpu::graphite::TextureInfo* textureInfo) {
    return self->makeDeferredCanvas(*imageInfo, *textureInfo);
}

extern "C" skgpu::BackendApi C_Recorder_backend(const skgpu::graphite::Recorder* self) {
    return self->backend();
}

//
// gpu/graphite/Recording.h
//

extern "C" void C_Recording_delete(const skgpu::graphite::Recording* self) {
    delete self;
}

//
// gpu/graphite/YUVABackendTextures.h
//

extern "C" void C_YUVABackendTextures_construct(
    skgpu::graphite::YUVABackendTextures* uninitialized,
    const SkYUVAInfo& yuvaInfo,
    const skgpu::graphite::BackendTexture* const *backend_textures
) {
    skgpu::graphite::BackendTexture textures[SkYUVAInfo::kMaxPlanes];
    for (int i = 0; i < SkYUVAInfo::kMaxPlanes; ++i) {
        textures[i] = *backend_textures[i];
    }
    new(uninitialized) skgpu::graphite::YUVABackendTextures(yuvaInfo, SkSpan<const skgpu::graphite::BackendTexture>(textures, SkYUVAInfo::kMaxPlanes));
}

extern "C" void C_YUVABackendTextures_destruct(skgpu::graphite::YUVABackendTextures* self) {
    self->~YUVABackendTextures();
}

extern "C" const SkYUVAInfo* C_YUVABackendTextures_yuvaInfo(const skgpu::graphite::YUVABackendTextures* self) {
    return &self->yuvaInfo();
}

extern "C" void C_YUVABackendTextures_planeTexture(const skgpu::graphite::YUVABackendTextures* self, int index, skgpu::graphite::BackendTexture* result) {
    *result = self->planeTexture(index);
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

extern "C" SkImage* C_SkImages_TextureFromYUVATexturesGraphite(
    skgpu::graphite::Recorder* recorder,
    const skgpu::graphite::YUVABackendTextures* yuvaTextures,
    SkColorSpace* imageColorSpace) {
    return SkImages::TextureFromYUVATextures(
            recorder,
            *yuvaTextures,
            sp(imageColorSpace)).release();
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
