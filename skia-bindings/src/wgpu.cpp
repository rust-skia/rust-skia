// Bindings for Graphite's Dawn backend, built against the Dawn headers that
// `build_support/dawn.rs` generates.
//
// Skia's Dawn backend calls the WebGPU C functions of `webgpu.h`. rust-skia links no WebGPU
// implementation for them. They stay undefined here, and the `skia-wgpu` crate exports them on
// the `wgpu` crate (see docs/adr/0003 and docs/adr/0004).
//
// Only the C layer of WebGPU crosses into Rust. The functions below take the C handles
// (`WGPUInstance`, `WGPUDevice`, ...) and wrap them in Dawn's C++ classes on this side.

#include "include/core/SkSize.h"
#include "include/gpu/graphite/BackendTexture.h"
#include "include/gpu/graphite/Context.h"
#include "include/gpu/graphite/ContextOptions.h"
#include "include/gpu/graphite/dawn/DawnBackendContext.h"
#include "include/gpu/graphite/dawn/DawnGraphiteTypes.h"

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

// The backend texture takes its dimensions and format from `texture`, but no reference to it.
// The caller keeps `texture` alive while the backend texture is in use.
extern "C" void C_BackendTextures_MakeDawn(
    skgpu::graphite::BackendTexture* uninitialized,
    WGPUTexture texture) {
    new (uninitialized) skgpu::graphite::BackendTexture(
        skgpu::graphite::BackendTextures::MakeDawn(texture));
}
