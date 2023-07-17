#include "bindings.h"

// for VSCode
// TODO: remove that and add proper CMake support for VSCode
#ifndef SK_GL
    #define SK_GL
#endif

#include "include/core/SkSurfaceCharacterization.h"
#include "include/gpu/GrDirectContext.h"
#include "include/gpu/gl/GrGLExtensions.h"
#include "include/gpu/gl/GrGLInterface.h"
#include "include/gpu/gl/GrGLAssembleInterface.h"
#include "src/gpu/gl/GrGLDefines.h"

// Additional types not yet referenced.
extern "C" void C_GrGLTypes(GrGLSurfaceInfo *) {};

// core/SurfaceCharacterization.h

extern "C" void C_SkSurfaceCharacterization_createFBO0(
    const SkSurfaceCharacterization* self, 
    bool usesGLFBO0, 
    SkSurfaceCharacterization* uninitialized) {
    new(uninitialized) SkSurfaceCharacterization(self->createFBO0(usesGLFBO0));
}

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

// Copied implementation here, because pulling in src/gpu/gl/GrGLUtil.h pulls in too many other types and
// increases the size of bindings.rs considerably.

extern "C" GrGLFormat C_GrGLFormatFromGLEnum(GrGLenum glFormat) {
    switch (glFormat) {
        case GR_GL_RGBA8:                return GrGLFormat::kRGBA8;
        case GR_GL_R8:                   return GrGLFormat::kR8;
        case GR_GL_ALPHA8:               return GrGLFormat::kALPHA8;
        case GR_GL_LUMINANCE8:           return GrGLFormat::kLUMINANCE8;
        case GR_GL_LUMINANCE8_ALPHA8:    return GrGLFormat::kLUMINANCE8_ALPHA8;
        case GR_GL_BGRA8:                return GrGLFormat::kBGRA8;
        case GR_GL_RGB565:               return GrGLFormat::kRGB565;
        case GR_GL_RGBA16F:              return GrGLFormat::kRGBA16F;
        case GR_GL_LUMINANCE16F:         return GrGLFormat::kLUMINANCE16F;
        case GR_GL_R16F:                 return GrGLFormat::kR16F;
        case GR_GL_RGB8:                 return GrGLFormat::kRGB8;
        case GR_GL_RGBX8:                return GrGLFormat::kRGBX8;
        case GR_GL_RG8:                  return GrGLFormat::kRG8;
        case GR_GL_RGB10_A2:             return GrGLFormat::kRGB10_A2;
        case GR_GL_RGBA4:                return GrGLFormat::kRGBA4;
        case GR_GL_SRGB8_ALPHA8:         return GrGLFormat::kSRGB8_ALPHA8;
        case GR_GL_COMPRESSED_ETC1_RGB8: return GrGLFormat::kCOMPRESSED_ETC1_RGB8;
        case GR_GL_COMPRESSED_RGB8_ETC2: return GrGLFormat::kCOMPRESSED_RGB8_ETC2;
        case GR_GL_COMPRESSED_RGB_S3TC_DXT1_EXT: return GrGLFormat::kCOMPRESSED_RGB8_BC1;
        case GR_GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: return GrGLFormat::kCOMPRESSED_RGBA8_BC1;
        case GR_GL_R16:                  return GrGLFormat::kR16;
        case GR_GL_RG16:                 return GrGLFormat::kRG16;
        case GR_GL_RGBA16:               return GrGLFormat::kRGBA16;
        case GR_GL_RG16F:                return GrGLFormat::kRG16F;
        case GR_GL_STENCIL_INDEX8:       return GrGLFormat::kSTENCIL_INDEX8;
        case GR_GL_STENCIL_INDEX16:      return GrGLFormat::kSTENCIL_INDEX16;
        case GR_GL_DEPTH24_STENCIL8:     return GrGLFormat::kDEPTH24_STENCIL8;

        default:                         return GrGLFormat::kUnknown;
    }
}

extern "C" GrGLenum C_GrGLFormatToEnum(GrGLFormat format) {
    switch (format) {
        case GrGLFormat::kRGBA8:                return GR_GL_RGBA8;
        case GrGLFormat::kR8:                   return GR_GL_R8;
        case GrGLFormat::kALPHA8:               return GR_GL_ALPHA8;
        case GrGLFormat::kLUMINANCE8:           return GR_GL_LUMINANCE8;
        case GrGLFormat::kLUMINANCE8_ALPHA8:    return GR_GL_LUMINANCE8_ALPHA8;
        case GrGLFormat::kBGRA8:                return GR_GL_BGRA8;
        case GrGLFormat::kRGB565:               return GR_GL_RGB565;
        case GrGLFormat::kRGBA16F:              return GR_GL_RGBA16F;
        case GrGLFormat::kLUMINANCE16F:         return GR_GL_LUMINANCE16F;
        case GrGLFormat::kR16F:                 return GR_GL_R16F;
        case GrGLFormat::kRGB8:                 return GR_GL_RGB8;
        case GrGLFormat::kRGBX8:                return GR_GL_RGBX8;
        case GrGLFormat::kRG8:                  return GR_GL_RG8;
        case GrGLFormat::kRGB10_A2:             return GR_GL_RGB10_A2;
        case GrGLFormat::kRGBA4:                return GR_GL_RGBA4;
        case GrGLFormat::kSRGB8_ALPHA8:         return GR_GL_SRGB8_ALPHA8;
        case GrGLFormat::kCOMPRESSED_ETC1_RGB8: return GR_GL_COMPRESSED_ETC1_RGB8;
        case GrGLFormat::kCOMPRESSED_RGB8_ETC2: return GR_GL_COMPRESSED_RGB8_ETC2;
        case GrGLFormat::kCOMPRESSED_RGB8_BC1:  return GR_GL_COMPRESSED_RGB_S3TC_DXT1_EXT;
        case GrGLFormat::kCOMPRESSED_RGBA8_BC1: return GR_GL_COMPRESSED_RGBA_S3TC_DXT1_EXT;
        case GrGLFormat::kR16:                  return GR_GL_R16;
        case GrGLFormat::kRG16:                 return GR_GL_RG16;
        case GrGLFormat::kRGBA16:               return GR_GL_RGBA16;
        case GrGLFormat::kRG16F:                return GR_GL_RG16F;
        case GrGLFormat::kSTENCIL_INDEX8:       return GR_GL_STENCIL_INDEX8;
        case GrGLFormat::kSTENCIL_INDEX16:      return GR_GL_STENCIL_INDEX16;
        case GrGLFormat::kDEPTH24_STENCIL8:     return GR_GL_DEPTH24_STENCIL8;
        case GrGLFormat::kUnknown:              return 0;
    }
    SkUNREACHABLE;
}

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
// gpu/GrDirectContext.h
//

extern "C" GrDirectContext* C_GrDirectContext_MakeGL(GrGLInterface* interface, const GrContextOptions* options) {
    if (interface) {
        if (options) {
            return GrDirectContext::MakeGL(sp(interface), *options).release();
        } 
        return GrDirectContext::MakeGL(sp(interface)).release();
    } 
    if (options) {
        return GrDirectContext::MakeGL(*options).release();
    }
    return GrDirectContext::MakeGL().release();
}

extern "C" void C_GrBackendFormat_ConstructGL(GrBackendFormat* uninitialized, GrGLenum format, GrGLenum target) {
    new(uninitialized)GrBackendFormat(GrBackendFormat::MakeGL(format, target));
}

extern "C" void C_GrBackendTexture_ConstructGL(GrBackendTexture* uninitialized, int width, int height, GrMipMapped mipMapped, const GrGLTextureInfo* glInfo) {
    new(uninitialized)GrBackendTexture(width, height, mipMapped, *glInfo);
}

extern "C" void C_GrBackendRenderTarget_ConstructGL(GrBackendRenderTarget* uninitialized, int width, int height, int sampleCnt, int stencilBits, const GrGLFramebufferInfo* glInfo) {
    new(uninitialized)GrBackendRenderTarget(width, height, sampleCnt, stencilBits, *glInfo);
}
