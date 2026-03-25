#include "bindings.h"

// for VSCode
// TODO: remove that and add proper CMake support for VSCode
#ifndef SK_DIRECT3D
    #define SK_DIRECT3D
#endif

#include "include/gpu/ganesh/GrBackendSurface.h"
#include "include/gpu/ganesh/GrDirectContext.h"
#include "include/gpu/ganesh/d3d/GrD3DBackendContext.h"
#include "include/gpu/ganesh/d3d/GrD3DBackendSurface.h"
#include "include/gpu/ganesh/d3d/GrD3DDirectContext.h"

// Additional types not yet referenced.
extern "C" void C_GrD3DTypes(GrD3DSurfaceInfo *) {};

//
// gpu/d3d/GrD3DTypes.h
//

extern "C" void C_GrD3DTextureResourceInfo_Construct(GrD3DTextureResourceInfo* uninitialized) {
    new(uninitialized) GrD3DTextureResourceInfo();
}

//
// gpu/GrBackendSurface.h
//

extern "C" void C_GrBackendFormat_ConstructD3D(GrBackendFormat* uninitialized, DXGI_FORMAT format) {
    new(uninitialized)GrBackendFormat(GrBackendFormats::MakeD3D(format));
}

extern "C" bool C_GrBackendFormats_AsDxgiFormat(const GrBackendFormat* format, DXGI_FORMAT* dxgiFormat) {
    *dxgiFormat = GrBackendFormats::AsDxgiFormat(*format);
    return *dxgiFormat != DXGI_FORMAT_UNKNOWN;
}

extern "C" GrBackendTexture* C_GrBackendTexture_newD3D(
    int width, int height,
    const GrD3DTextureResourceInfo* resourceInfo, 
    const char* label,
    size_t labelCount) {
    return new GrBackendTexture(
            GrBackendTextures::MakeD3D(width, height, *resourceInfo, std::string_view(label, labelCount)));
}

extern "C" bool C_GrBackendTextures_GetD3DTextureResourceInfo(
        const GrBackendTexture* texture,
        GrD3DTextureResourceInfo* info) {
    *info = GrBackendTextures::GetD3DTextureResourceInfo(*texture);
    return info->fResource.get() != nullptr;
}

extern "C" void C_GrBackendTextures_SetD3DResourceState(
        GrBackendTexture* texture,
        GrD3DResourceStateEnum state) {
    GrBackendTextures::SetD3DResourceState(texture, state);
}

extern "C" void C_GrBackendRenderTarget_ConstructD3D(GrBackendRenderTarget* uninitialized, int width, int height, const GrD3DTextureResourceInfo* resourceInfo) {
    new(uninitialized)GrBackendRenderTarget(GrBackendRenderTargets::MakeD3D(width, height, *resourceInfo));
}

extern "C" bool C_GrBackendRenderTargets_GetD3DTextureResourceInfo(
        const GrBackendRenderTarget* renderTarget,
        GrD3DTextureResourceInfo* info) {
    *info = GrBackendRenderTargets::GetD3DTextureResourceInfo(*renderTarget);
    return info->fResource.get() != nullptr;
}

extern "C" void C_GrBackendRenderTargets_SetD3DResourceState(
        GrBackendRenderTarget* renderTarget,
        GrD3DResourceStateEnum state) {
    GrBackendRenderTargets::SetD3DResourceState(renderTarget, state);
}

//
// gpu/GrDirectContext.h
//

extern "C" GrDirectContext* C_GrDirectContext_MakeD3D(
    const GrD3DBackendContext* backendContext,
    const GrContextOptions* options) {
    if (options) {
        return GrDirectContexts::MakeD3D(*backendContext, *options).release();
    }
    return GrDirectContexts::MakeD3D(*backendContext).release();
}
