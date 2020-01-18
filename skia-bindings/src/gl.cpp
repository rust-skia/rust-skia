#include "bindings.h"
#include "include/gpu/GrContext.h"
#include "include/gpu/gl/GrGLExtensions.h"
#include "include/gpu/gl/GrGLInterface.h"
#include "include/gpu/gl/GrGLAssembleInterface.h"

//
// GrGLTextureInfo
//

extern "C" bool C_GrGLTextureInfo_Equals(const GrGLTextureInfo* lhs, const GrGLTextureInfo* rhs) {
    return *lhs == *rhs;
}

//
// GrGLFramebufferInfo
//

extern "C" bool C_GrGLFramebufferInfo_Equals(const GrGLFramebufferInfo* lhs, const GrGLFramebufferInfo* rhs) {
    return *lhs == *rhs;
}

//
// gpu/gl/
//

extern "C" void C_GPU_GL_Types(GrGLBackendState *) {}

//
// gpu/gl/GrGLInterface.h
//

extern "C" void C_GrGLExtensions_destruct(GrGLExtensions* self) {
    self->~GrGLExtensions();
}

extern "C" void C_GrGLExtensions_reset(GrGLExtensions* self) {
    self->reset();
}

//
// gpu/gl/GrGLInterface.h
//

extern "C" const GrGLInterface* C_GrGLInterface_MakeNativeInterface() {
    return GrGLMakeNativeInterface().release();
}

extern "C" GrGLExtensions* C_GrGLInterface_extensions(GrGLInterface* self) {
    return &self->fExtensions;
}

//
// gpu/gl/GrGLAssembleInterface.h
//

typedef const void* (*GLGetProcFnVoidPtr)(void* ctx, const char name[]);

extern "C" const GrGLInterface* C_GrGLInterface_MakeAssembledInterface(void *ctx, GLGetProcFnVoidPtr get) {
    return GrGLMakeAssembledInterface(ctx, reinterpret_cast<GrGLGetProc>(get)).release();
}

//
// gpu/GrContext.h
//

extern "C" GrContext* C_GrContext_MakeGL(GrGLInterface* interface) {
    if (interface)
        return GrContext::MakeGL(sp(interface)).release();
    else
        return GrContext::MakeGL().release();
}

extern "C" void C_GrBackendFormat_ConstructGL(GrBackendFormat* uninitialized, GrGLenum format, GrGLenum target) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeGL(format, target));
}
