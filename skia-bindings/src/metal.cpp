#ifndef SK_METAL
    #define SK_METAL
#endif

#include "bindings.h"

#include "include/core/SkSurface.h"
#include "include/gpu/mtl/GrMtlBackendContext.h"
#include "include/gpu/GrDirectContext.h"

//
// core/SkSurface.h
//

extern "C" SkSurface *C_SkSurface_MakeFromCAMetalLayer(GrRecordingContext *context,
                                                       GrMTLHandle layer,
                                                       GrSurfaceOrigin origin,
                                                       int sampleCnt,
                                                       SkColorType colorType,
                                                       SkColorSpace *colorSpace,
                                                       const SkSurfaceProps *surfaceProps,
                                                       GrMTLHandle *drawable) {
    return SkSurface::MakeFromCAMetalLayer(context, layer, origin, sampleCnt, colorType, sp(colorSpace), surfaceProps,
                                           drawable).release();
}

extern "C" SkSurface *C_SkSurface_MakeFromMTKView(GrRecordingContext *context,
                                                  GrMTLHandle mtkView,
                                                  GrSurfaceOrigin origin,
                                                  int sampleCnt,
                                                  SkColorType colorType,
                                                  SkColorSpace *colorSpace,
                                                  const SkSurfaceProps *surfaceProps) {
    return SkSurface::MakeFromMTKView(context, mtkView, origin, sampleCnt, colorType, sp(colorSpace), surfaceProps
    ).release();
}

//
// gpu/GrDirectContext.h
//

extern "C" GrDirectContext *C_GrContext_MakeMetal(
    const GrMtlBackendContext *context,
    const GrContextOptions *options)
{
    if (options)
    {
        return GrDirectContext::MakeMetal(*context, *options).release();
    }
    return GrDirectContext::MakeMetal(*context).release();
}

//
// gpu/mtl/GrMtlBackendContext.h
//

extern "C" void C_GrMtlBackendContext_Construct(
    GrMtlBackendContext* unintialized, 
    const void* device, const void* queue, const void* binaryArchive) {
    new (unintialized) GrMtlBackendContext();
    unintialized->fDevice.retain(device);
    unintialized->fQueue.retain(queue);
    unintialized->fBinaryArchive.retain(binaryArchive);
}

extern "C" void C_GrMtlBackendContext_Destruct(GrMtlBackendContext* self) {
    self->~GrMtlBackendContext();
}

//
// gpu/mtl/GrMtlTypes.h
//

extern "C" void C_GrMtlTextureInfo_Construct(GrMtlTextureInfo* unintialized, const void* texture) {
    new (unintialized) GrMtlTextureInfo();
    unintialized->fTexture.retain(texture);
}

extern "C" void C_GrMtlTextureInfo_Destruct(GrMtlTextureInfo* self) {
    self->~GrMtlTextureInfo();
}

extern "C" bool C_GrMtlTextureInfo_Equals(const GrMtlTextureInfo* lhs, const GrMtlTextureInfo* rhs) {
    return *lhs == *rhs;
}

//
// gpu/GrBackendSurface.h
//

extern "C" void C_GrBackendFormat_ConstructMtl(GrBackendFormat* uninitialized, GrMTLPixelFormat format) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeMtl(format));
}


extern "C" void C_GrBackendTexture_ConstructMtl(GrBackendTexture* uninitialized, int width, int height, GrMipMapped mipMapped, const GrMtlTextureInfo* mtlInfo) {
    new(uninitialized)GrBackendTexture(width, height, mipMapped, *mtlInfo);
}

extern "C" void C_GrBackendRenderTarget_ConstructMtl(GrBackendRenderTarget* uninitialized, int width, int height, int sampleCnt, const GrMtlTextureInfo* mtlInfo) {
    new(uninitialized)GrBackendRenderTarget(width, height, sampleCnt, *mtlInfo);
}
