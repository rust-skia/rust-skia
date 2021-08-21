#include "bindings.h"
#include "include/gpu/GrDirectContext.h"
#include "include/gpu/GrBackendDrawableInfo.h"
#include "include/gpu/GrYUVABackendTextures.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkDrawable.h"
#include "include/core/SkSurface.h"
#include "include/core/SkSurfaceCharacterization.h"
#include "include/core/SkImageGenerator.h"

//
// core/SkSurface.h
//

extern "C" SkSurface* C_SkSurface_MakeFromBackendTexture(
        GrRecordingContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        int sampleCnt,
        SkColorType colorType,
        SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeFromBackendTexture(
            context,
            *backendTexture,
            origin,
            sampleCnt,
            colorType,
            sp(colorSpace), surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeFromBackendRenderTarget(
        GrRecordingContext* context,
        const GrBackendRenderTarget* backendRenderTarget,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps
        ) {
    return SkSurface::MakeFromBackendRenderTarget(
            context,
            *backendRenderTarget,
            origin,
            colorType,
            sp(colorSpace),
            surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeRenderTarget(
    GrRecordingContext* context,
    SkBudgeted budgeted,
    const SkImageInfo* imageInfo,
    int sampleCount, GrSurfaceOrigin surfaceOrigin,
    const SkSurfaceProps* surfaceProps,
    bool shouldCreateWithMips) {
    return SkSurface::MakeRenderTarget(
            context,
            budgeted,
            *imageInfo,
            sampleCount,
            surfaceOrigin,
            surfaceProps,
            shouldCreateWithMips).release();
}

extern "C" SkSurface* C_SkSurface_MakeRenderTarget2(
        GrRecordingContext* context,
        const SkSurfaceCharacterization& characterization,
        SkBudgeted budgeted) {
    return SkSurface::MakeRenderTarget(
            context,
            characterization,
            budgeted).release();
}

extern "C" void C_SkSurface_getBackendTexture(
        SkSurface* self,
        SkSurface::BackendHandleAccess handleAccess,
        GrBackendTexture* backendTexture) {
    *backendTexture = self->getBackendTexture(handleAccess);
}

extern "C" void C_SkSurface_getBackendRenderTarget(
        SkSurface* self,
        SkSurface::BackendHandleAccess handleAccess,
        GrBackendRenderTarget *backendRenderTarget) {
    *backendRenderTarget = self->getBackendRenderTarget(handleAccess);
}

//
// core/SkSurfaceCharacterization.h
//

extern "C" void C_SkSurfaceCharacterization_createResized(
    const SkSurfaceCharacterization* self, int width, int height, SkSurfaceCharacterization* uninitialized) {
    new(uninitialized) SkSurfaceCharacterization(self->createResized(width, height));
}

extern "C" void C_SkSurfaceCharacterization_createBackendFormat(
    const SkSurfaceCharacterization* self, 
    SkColorType colorType, 
    const GrBackendFormat* backendFormat,
    SkSurfaceCharacterization* uninitialized) {
    new(uninitialized) SkSurfaceCharacterization(self->createBackendFormat(colorType, *backendFormat));
}

extern "C" const SkImageInfo* C_SkSurfaceCharacterization_imageInfo(const SkSurfaceCharacterization* self) {
    return &self->imageInfo();
}

//
// core/SkImageGenerator.h
//

extern "C" bool C_SkImageGenerator_isValid(const SkImageGenerator* self, GrRecordingContext* context) {
    return self->isValid(context);
}

//
// gpu/GrBackendSurface.h
//

// GrBackendRenderTarget

extern "C" void C_GrBackendRenderTarget_Construct(GrBackendRenderTarget* uninitialized) {
    new(uninitialized) GrBackendRenderTarget();
}

extern "C" void C_GrBackendRenderTarget_CopyConstruct(GrBackendRenderTarget* uninitialized, const GrBackendRenderTarget* renderTarget) {
    new(uninitialized) GrBackendRenderTarget(*renderTarget);
}

extern "C" void C_GrBackendRenderTarget_destruct(GrBackendRenderTarget* self) {
    self->~GrBackendRenderTarget();
}

extern "C" void C_GrBackendRenderTarget_getBackendFormat(const GrBackendRenderTarget* self, GrBackendFormat* uninitialized) {
    new(uninitialized) GrBackendFormat(self->getBackendFormat());
}

// GrBackendTexture

extern "C" void C_GrBackendTexture_Construct(GrBackendTexture* uninitialized) {
    new(uninitialized) GrBackendTexture();
}

extern "C" void C_GrBackendTexture_CopyConstruct(GrBackendTexture* uninitialized, const GrBackendTexture* texture) {
    new(uninitialized) GrBackendTexture(*texture);
}

extern "C" void C_GrBackendTexture_destruct(const GrBackendTexture* self) {
    self->~GrBackendTexture();
}

extern "C" void C_GrBackendTexture_getBackendFormat(const GrBackendTexture* self, GrBackendFormat* format) {
    *format = self->getBackendFormat();
}

// GrBackendFormat

extern "C" void C_GrBackendFormat_Construct(GrBackendFormat* uninitialized) {
    new(uninitialized)GrBackendFormat();
}

extern "C" void C_GrBackendFormat_destruct(GrBackendFormat* self) {
    self->~GrBackendFormat();
}

extern "C" bool C_GrBackendFormat_Equals(const GrBackendFormat* lhs, const GrBackendFormat* rhs) {
    return *lhs == *rhs;
}

extern "C" void C_GrBackendFormat_makeTexture2D(const GrBackendFormat* self, GrBackendFormat* format) {
    *format = self->makeTexture2D();
}

//
// gpu/GrBackendSurfaceMutableState.h
//

extern "C" void C_GrBackendSurfaceMutableState_Construct(GrBackendSurfaceMutableState* uninitialized) {
    new(uninitialized)GrBackendSurfaceMutableState();
}

extern "C" void C_GrBackendSurfaceMutableState_destruct(GrBackendSurfaceMutableState* self) {
    self->~GrBackendSurfaceMutableState();
}

//
// gpu/GrRecordingContext.h
//

// GrContext_Base.h
extern "C" GrDirectContext* C_GrRecordingContext_asDirectContext(GrRecordingContext* self) {
    return self->asDirectContext();
}

// GrContext_Base.h
extern "C" GrBackendApi C_GrRecordingContext_backend(const GrRecordingContext* self) {
    return self->backend();
}

extern "C" void C_GrRecordingContext_defaultBackendFormat(const GrRecordingContext* self, SkColorType ct, GrRenderable renderable, GrBackendFormat* result) {
    *result = self->defaultBackendFormat(ct, renderable);
}

// GrContext_Base.h
extern "C" void C_GrRecordingContext_compressedBackendFormat(const GrRecordingContext* self, SkImage::CompressionType compressionType, GrBackendFormat* backendFormat) {
    *backendFormat = self->compressedBackendFormat(compressionType);
}

extern "C" bool C_GrRecordingContext_abandoned(GrRecordingContext* self) {
    return self->abandoned();
}

//
// gpu/GrDirectContext.h
//

extern "C" void C_GrDirectContext_flushAndSubmit(GrDirectContext* self) {
    self->flushAndSubmit();
}

extern "C" void C_GrDirectContext_compressedBackendFormat(const GrDirectContext* self, SkImage::CompressionType compression, GrBackendFormat* result) {
    *result = self->compressedBackendFormat(compression);
}

extern "C" void C_GrDirectContext_directContextId(const GrDirectContext* self, GrDirectContext::DirectContextID* r) {
    *r = self->directContextID();
}

extern "C" void C_GrDirectContext_performDeferredCleanup(GrDirectContext* self, long msNotUsed, bool scratchResourcesOnly) {
    self->performDeferredCleanup(std::chrono::milliseconds(msNotUsed), scratchResourcesOnly);
}

//
// gpu/GrContextOptions.h
//

extern "C" void C_GrContextOptions_Construct(GrContextOptions* uninitialized) {
    new(uninitialized) GrContextOptions();
}

//
// gpu/GrRecordingContext.h
//

extern "C" bool C_GrRecordingContext_colorTypeSupportedAsSurface(const GrRecordingContext* self, SkColorType colorType) {
    return self->colorTypeSupportedAsSurface(colorType);
}

//
// gpu/GrBackendDrawableInfo.h
//

extern "C" void C_GrBackendDrawableInfo_Construct(GrBackendDrawableInfo* uninitialized) {
    new(uninitialized) GrBackendDrawableInfo();
}

extern "C" void C_GrBackendDrawableInfo_Construct2(GrBackendDrawableInfo* uninitialized, const GrVkDrawableInfo* info) {
    new(uninitialized) GrBackendDrawableInfo(*info);
}

extern "C" void C_GrBackendDrawableInfo_destruct(GrBackendDrawableInfo* self) {
    self->~GrBackendDrawableInfo();
}

extern "C" bool C_GrBackendDrawableInfo_isValid(const GrBackendDrawableInfo* self) {
    return self->isValid();
}

extern "C" GrBackendApi C_GrBackendDrawableInfo_backend(const GrBackendDrawableInfo* self) {
    return self->backend();
}

//
// gpu/GrYUVABackendTextures.h
//

extern "C" void C_GrYUVABackendTextureInfo_destruct(GrYUVABackendTextureInfo* self) {
    self->~GrYUVABackendTextureInfo();
}

extern "C" void C_GrYUVABackendTextureInfo_CopyConstruct(GrYUVABackendTextureInfo* uninitialized, const GrYUVABackendTextureInfo* src) {
    new(uninitialized) GrYUVABackendTextureInfo(*src);
}

extern "C" bool C_GrYUVABackendTextureInfo_equals(const GrYUVABackendTextureInfo* a, const GrYUVABackendTextureInfo* b) {
    return *a == *b;
}

extern "C" void C_GrYUVABackendTextures_destruct(GrYUVABackendTextures* self) {
    self->~GrYUVABackendTextures();
}

extern "C" const GrBackendTexture* C_GrYUVABackendTextures_textures(const GrYUVABackendTextures* self) {
    return self->textures().data();
}

//
// core/SkCanvas.h
//

extern "C" GrRecordingContext* C_SkCanvas_recordingContext(SkCanvas* self) {
    return self->recordingContext();
}

//
// core/SkDrawable.h
//

extern "C" SkDrawable::GpuDrawHandler *C_SkDrawable_snapGpuDrawHandler(SkDrawable *self, GrBackendApi backendApi,
                                                                       const SkMatrix *matrix,
                                                                       const SkIRect *clipBounds,
                                                                       const SkImageInfo *bufferInfo) {
    return self->snapGpuDrawHandler(backendApi, *matrix, *clipBounds, *bufferInfo).release();
}

extern "C" void C_SkDrawable_GpuDrawHandler_delete(SkDrawable::GpuDrawHandler *self) {
    delete self;
}

extern "C" void C_SkDrawable_GpuDrawHandler_draw(SkDrawable::GpuDrawHandler *self, const GrBackendDrawableInfo *info) {
    self->draw(*info);
}

//
// core/SkImage.h
//


extern "C" SkImage *C_SkImage_MakeTextureFromCompressed(GrDirectContext *context, SkData *data, int width, int height,
                                                SkImage::CompressionType type, GrMipMapped mipMapped,
                                                GrProtected prot) {
    return SkImage::MakeTextureFromCompressed(context, sp(data), width, height, type, mipMapped, prot).release();
}

extern "C" void C_SkImage_getBackendTexture(
        const SkImage* self,
        bool flushPendingGrContextIO,
        GrSurfaceOrigin* origin,
        GrBackendTexture* result)
{
    *result = self->getBackendTexture(flushPendingGrContextIO, origin);
}

extern "C" SkImage* C_SkImage_MakeFromTexture(
        GrRecordingContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkAlphaType alphaType,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromTexture(context, *backendTexture, origin, colorType, alphaType, sp(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeCrossContextFromPixmap(
        GrDirectContext* context,
        const SkPixmap* pixmap,
        bool buildMips,
        bool limitToMaxTextureSize) {
    return SkImage::MakeCrossContextFromPixmap(context, *pixmap, buildMips, limitToMaxTextureSize).release();
}

extern "C" SkImage* C_SkImage_MakeFromAdoptedTexture(
        GrRecordingContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkAlphaType alphaType,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromAdoptedTexture(context, *backendTexture, origin, colorType, alphaType, sp(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATextures(
    GrRecordingContext* context,
    const GrYUVABackendTextures* yuvaTextures,
    SkColorSpace* imageColorSpace) {
    return SkImage::MakeFromYUVATextures(context, *yuvaTextures, sp(imageColorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVAPixmaps(
    GrRecordingContext* context,
    const SkYUVAPixmaps* pixmaps,
    GrMipmapped buildMips,
    bool limitToMaxTextureSize,
    SkColorSpace* imageColorSpace
) {
    return SkImage::MakeFromYUVAPixmaps(context, *pixmaps, buildMips, limitToMaxTextureSize, sp(imageColorSpace)).release();
}

extern "C" SkImage* C_SkImage_makeTextureImage(
        const SkImage* self,
        GrDirectContext* context,
        GrMipMapped mipMapped,
        SkBudgeted budgeted) {
    return self->makeTextureImage(context, mipMapped, budgeted).release();
}
