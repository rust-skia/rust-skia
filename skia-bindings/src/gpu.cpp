#include "bindings.h"

#include "include/gpu/GpuTypes.h"
#include "include/gpu/MutableTextureState.h"

extern "C" void C_GpuUnreferencedTypes(skgpu::Origin*) {}

//
// gpu/MutableTextureState.h
//

extern "C" skgpu::MutableTextureState* C_MutableTextureState_Construct() {
    return new skgpu::MutableTextureState();
}

extern "C" skgpu::MutableTextureState* C_MutableTextureState_CopyConstruct(const skgpu::MutableTextureState* state) {
    return new skgpu::MutableTextureState(*state);
}

extern "C" skgpu::BackendApi C_MutableTextureState_backend(const skgpu::MutableTextureState* self) {
    return self->backend();
}
