// Bindings for Graphite's Dawn backend, built against the checked-in Dawn headers in `dawn/`.
//
// Skia's Dawn backend calls the WebGPU C functions of `webgpu.h`. rust-skia links no WebGPU
// implementation for them. They forward through Dawn's function table (`dawn_proc`), and the
// `skia-wgpu` crate installs a table that calls the `wgpu` crate (see docs/adr/0003).
//
// Only the C layer of WebGPU crosses into Rust. The functions below take the C handles
// (`WGPUInstance`, `WGPUDevice`, ...) and wrap them in Dawn's C++ classes on this side.

#include "include/core/SkSize.h"
#include "include/gpu/graphite/BackendTexture.h"
#include "include/gpu/graphite/Context.h"
#include "include/gpu/graphite/ContextOptions.h"
#include "include/gpu/graphite/dawn/DawnBackendContext.h"
#include "include/gpu/graphite/dawn/DawnGraphiteTypes.h"

#include "dawn/dawn_proc.h"

//
// gpu/graphite/dawn/DawnBackendContext.h
//

// Each handle gains one reference, which the backend context releases when it is destructed.
extern "C" void C_DawnBackendContext_Construct(
    skgpu::graphite::DawnBackendContext* uninitialized,
    WGPUInstance instance,
    WGPUDevice device,
    WGPUQueue queue) {
    new (uninitialized) skgpu::graphite::DawnBackendContext();
    uninitialized->fInstance = wgpu::Instance(instance);
    uninitialized->fDevice = wgpu::Device(device);
    uninitialized->fQueue = wgpu::Queue(queue);
}

extern "C" void C_DawnBackendContext_destruct(skgpu::graphite::DawnBackendContext* self) {
    self->~DawnBackendContext();
}

extern "C" skgpu::graphite::Context* C_ContextFactory_MakeDawn(
    const skgpu::graphite::DawnBackendContext* backendContext,
    const skgpu::graphite::ContextOptions* options) {
    return skgpu::graphite::ContextFactory::MakeDawn(*backendContext, *options).release();
}

//
// gpu/graphite/dawn/DawnGraphiteTypes.h
//

// The backend texture references `texture` and takes its dimensions and format from it.
extern "C" void C_BackendTextures_MakeDawn(
    skgpu::graphite::BackendTexture* uninitialized,
    WGPUTexture texture) {
    new (uninitialized) skgpu::graphite::BackendTexture(
        skgpu::graphite::BackendTextures::MakeDawn(texture));
}
