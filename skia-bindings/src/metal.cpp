#include "include/gpu/GrContext.h"

//
// gpu/GrContext.h
//

extern "C" GrContext* C_GrContext_MakeMetal(void* device, void* queue) {
    return GrContext::MakeMetal(device, queue).release();
}

//
// gpu/mtl/GrMtlTypes.h
//

extern "C" void C_GrMtlTextureInfo_Construct(GrMtlTextureInfo* unintialized, const void* texture) {
    new (unintialized) GrMtlTextureInfo();
    unintialized->fTexture = sk_cf_obj<const void*>(texture);
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
