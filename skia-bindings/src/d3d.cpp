#include "bindings.h"

// for VSCode
// TODO: remove that and add proper CMake support for VSCode
#ifndef SK_DIRECT3D
    #define SK_DIRECT3D
#endif

#include "include/gpu/GrBackendSurface.h"
#include "include/gpu/GrContext.h"
#include "include/gpu/d3d/GrD3DBackendContext.h"

//
// gpu/GrBackendSurface.h
//

extern "C" void C_GrBackendFormat_ConstructDxgi(GrBackendFormat* uninitialized, DXGI_FORMAT format) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeDxgi(format));
}

extern "C" void C_GrBackendTexture_ConstructD3D(GrBackendTexture* uninitialized, int width, int height, const GrD3DTextureResourceInfo* resourceInfo) {
    new(uninitialized)GrBackendTexture(width, height, *resourceInfo);
}

extern "C" void C_GrBackendRenderTarget_ConstructD3D(GrBackendRenderTarget* uninitialized, int width, int height, int sampleCnt, const GrD3DTextureResourceInfo* resourceInfo) {
    new(uninitialized)GrBackendRenderTarget(width, height, sampleCnt, *resourceInfo);
}

//
// gpu/GrContext.h
//

extern "C" GrContext* C_GrContext_MakeDirect3D(const GrD3DBackendContext* backendContext) {
    return GrContext::MakeDirect3D(*backendContext).release();
}
