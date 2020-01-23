#include "include/gpu/GrContext.h"

//
// gpu/GrContext.h
//

extern "C" GrContext* C_GrContext_MakeMetal(void* device, void* queue) {
    return GrContext::MakeMetal(device, queue).release();
}