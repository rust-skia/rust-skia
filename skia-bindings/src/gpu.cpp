#include "bindings.h"
#include "include/gpu/GrContext.h"
#include "include/gpu/GrBackendDrawableInfo.h"
#include "include/core/SkDrawable.h"
#include "include/core/SkSurface.h"
#include "include/core/SkSurfaceCharacterization.h"

//
// core/SkSurface.h
//

extern "C" SkSurface* C_SkSurface_MakeFromBackendTexture(
        GrContext* context,
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
        GrContext* context,
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

extern "C" SkSurface* C_SkSurface_MakeFromBackendTextureAsRenderTarget(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        int sampleCnt,
        SkColorType colorType,
        SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeFromBackendTextureAsRenderTarget(
            context,
            *backendTexture,
            origin,
            sampleCnt,
            colorType,
            sp(colorSpace), surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeRenderTarget(
    GrContext* context,
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
        GrContext* context,
        const SkSurfaceCharacterization& characterization,
        SkBudgeted budgeted) {
    return SkSurface::MakeRenderTarget(
            context,
            characterization,
            budgeted).release();
}

extern "C" SkSurface *C_SkSurface_MakeFromBackendTexture2(
        GrContext *context,
        const SkSurfaceCharacterization &characterization,
        const GrBackendTexture *backendTexture) {
    return SkSurface::MakeFromBackendTexture(
            context,
            characterization,
            *backendTexture).release();
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

extern "C" const SkImageInfo* C_SkSurfaceCharacterization_imageInfo(const SkSurfaceCharacterization* self) {
    return &self->imageInfo();
}

//
// core/SkImage.h
//

extern "C" void C_SkImage_getBackendTexture(
        const SkImage* self,
        bool flushPendingGrContextIO,
        GrSurfaceOrigin* origin,
        GrBackendTexture* result)
{
    *result = self->getBackendTexture(flushPendingGrContextIO, origin);
}

//
// gpu/GrBackendSurface.h
//

extern "C" void C_GrBackendRenderTarget_Construct(GrBackendRenderTarget* uninitialized) {
    new(uninitialized) GrBackendRenderTarget();
}

extern "C" void C_GrBackendRenderTarget_destruct(GrBackendRenderTarget* self) {
    self->~GrBackendRenderTarget();
}

extern "C" void C_GrBackendTexture_Construct(GrBackendTexture* uninitialized) {
    new(uninitialized) GrBackendTexture();
}

extern "C" void C_GrBackendTexture_destruct(const GrBackendTexture* self) {
    self->~GrBackendTexture();
}

//
// GrBackendFormat
//

extern "C" void C_GrBackendFormat_Construct(GrBackendFormat* uninitialized) {
    new(uninitialized)GrBackendFormat();
}

extern "C" void C_GrBackendFormat_ConstructGL(GrBackendFormat* uninitialized, GrGLenum format, GrGLenum target) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeGL(format, target));
}

extern "C" void C_GrBackendFormat_destruct(GrBackendFormat* self) {
    self->~GrBackendFormat();
}

extern "C" bool C_GrBackendFormat_Equals(const GrBackendFormat* lhs, const GrBackendFormat* rhs) {
    return *lhs == *rhs;
}

//
// gpu/GrContext.h
//

extern "C" bool C_GrContext_colorTypeSupportedAsSurface(const GrContext* self, SkColorType colorType) {
    return self->colorTypeSupportedAsSurface(colorType);
}

extern "C" bool C_GrContext_abandoned(const GrContext* self) {
    return self->abandoned();
}

extern "C" void C_GrContext_flush(GrContext* self) {
    self->flush();
}

extern "C" size_t C_GrContext_ComputeImageSize(SkImage* image, GrMipMapped mm, bool useNextPow2) {
    return GrContext::ComputeImageSize(sp(image), mm, useNextPow2);
}

extern "C" void C_GrContext_defaultBackendFormat(const GrContext* self, SkColorType ct, GrRenderable renderable, GrBackendFormat* result) {
    *result = self->defaultBackendFormat(ct, renderable);
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
