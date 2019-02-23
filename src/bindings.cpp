#include "SkTypes.h"
#include "SkCanvas.h"
#include "SkColor.h"
#include "SkImageInfo.h"
#include "SkPaint.h"
#include "SkPath.h"
#include "SkRect.h"
#include "SkSurface.h"
#include "SkPicture.h"
#include "SkYUVAIndex.h"

#include "GrContext.h"

#if defined(SK_VULKAN)

#include "GrBackendSurface.h"
#include "vk/GrVkBackendContext.h"

#endif

//
// SkSurface
//

extern "C" SkSurface* C_SkSurface_MakeRasterN32Premul(int width, int height, const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeRasterN32Premul(width, height, surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeRenderTarget(
    GrContext* context,
    SkBudgeted budgeted,
    const SkImageInfo* imageInfo) {
    return SkSurface::MakeRenderTarget(context, budgeted, *imageInfo).release();
}

extern "C" SkImage* C_SkSurface_makeImageSnapshot(SkSurface* self) {
    return self->makeImageSnapshot().release();
}

//
// SkImage
//

extern "C" SkImage* C_SkImage_MakeRasterData(const SkImageInfo* info, SkData* pixels, size_t rowBytes) {
    return SkImage::MakeRasterData(*info, sk_sp<SkData>(pixels), rowBytes).release();
}

extern "C" SkImage* C_SkImage_MakeFromBitmap(const SkBitmap* bitmap) {
    return SkImage::MakeFromBitmap(*bitmap).release();
}

extern "C" SkImage* C_SkImage_MakeFromEncoded(SkData* encoded, const SkIRect* subset) {
    return SkImage::MakeFromEncoded(sk_sp<SkData>(encoded), subset).release();
}

extern "C" SkImage* C_SkImage_MakeFromTexture(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkAlphaType alphaType,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromTexture(context, *backendTexture, origin, colorType, alphaType, sk_sp<SkColorSpace>(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeCrossContextFromEncoded(
        GrContext* context,
        SkData* data,
        bool buildMips,
        SkColorSpace* dstColorSpace,
        bool limitToMaxTextureSize
        ) {
    return SkImage::MakeCrossContextFromEncoded(context, sk_sp<SkData>(data), buildMips, dstColorSpace, limitToMaxTextureSize).release();
}

extern "C" SkImage* C_SkImage_MakeFromAdoptedTexture(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkAlphaType alphaType,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromAdoptedTexture(context, *backendTexture, origin, colorType, alphaType, sk_sp<SkColorSpace>(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATexturesCopy(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture yuvaTextures[],
        const SkYUVAIndex yuvaIndices[4],
        SkISize imageSize,
        GrSurfaceOrigin imageOrigin,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromYUVATexturesCopy(context, yuvColorSpace, yuvaTextures, yuvaIndices, imageSize, imageOrigin, sk_sp<SkColorSpace>(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATexturesCopyWithExternalBackend(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture yuvaTextures[],
        const SkYUVAIndex yuvaIndices[4],
        SkISize imageSize,
        GrSurfaceOrigin imageOrigin,
        const GrBackendTexture& backendTexture,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromYUVATexturesCopyWithExternalBackend(context, yuvColorSpace, yuvaTextures, yuvaIndices, imageSize, imageOrigin, backendTexture, sk_sp<SkColorSpace>(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATextures(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture yuvaTextures[],
        const SkYUVAIndex yuvaIndices[4],
        SkISize imageSize,
        GrSurfaceOrigin imageOrigin,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromYUVATextures(context, yuvColorSpace, yuvaTextures, yuvaIndices, imageSize, imageOrigin, sk_sp<SkColorSpace>(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromNV12TexturesCopy(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture nv12Textures[2],
        GrSurfaceOrigin imageOrigin,
        SkColorSpace* imageColorSpace) {
    return SkImage::MakeFromNV12TexturesCopy(context, yuvColorSpace, nv12Textures, imageOrigin, sk_sp<SkColorSpace>(imageColorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture nv12Textures[2],
        GrSurfaceOrigin imageOrigin,
        const GrBackendTexture* backendTexture,
        SkColorSpace* imageColorSpace) {
    return SkImage::MakeFromNV12TexturesCopyWithExternalBackend(context, yuvColorSpace, nv12Textures, imageOrigin, *backendTexture, sk_sp<SkColorSpace>(imageColorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromPicture(
        SkPicture* picture,
        const SkISize* dimensions,
        const SkMatrix* matrix,
        const SkPaint* paint,
        SkImage::BitDepth bitDepth,
        SkColorSpace* colorSpace) {
    return SkImage::MakeFromPicture(sk_sp<SkPicture>(picture), *dimensions, matrix, paint, bitDepth, sk_sp<SkColorSpace>(colorSpace)).release();
}

extern "C" void C_SkImage_getBackendTexture(
        const SkImage* self,
        bool flushPendingGrContextIO,
        GrSurfaceOrigin* origin,
        GrBackendTexture* result)
{
    *result = self->getBackendTexture(flushPendingGrContextIO, origin);
}

extern "C" SkData* C_SkImage_encodeToData(const SkImage* self) {
    return self->encodeToData().release();
}

extern "C" SkData* C_SkImage_refEncodedData(const SkImage* self) {
    return self->refEncodedData().release();
}

extern "C" SkImage* C_SkImage_makeSubset(const SkImage* self, const SkIRect* subset) {
    return self->makeSubset(*subset).release();
}

extern "C" SkImage* C_SkImage_makeTextureImage(const SkImage* self, GrContext* context, SkColorSpace* dstColorSpace, GrMipMapped mipMapped) {
    return self->makeTextureImage(context, dstColorSpace, mipMapped).release();
}

extern "C" SkImage* C_SkImage_makeNonTextureImage(const SkImage* self) {
    return self->makeNonTextureImage().release();
}

extern "C" SkImage* C_SkImage_makeRasterImage(const SkImage* self) {
    return self->makeRasterImage().release();
}

extern "C" SkImage* C_SkImage_makeColorSpace(const SkImage* self, SkColorSpace* target) {
    return self->makeColorSpace(sk_sp<SkColorSpace>(target)).release();
}

//
// SkData
//

extern "C" void C_SkData_ref(const SkData* self) {
    self->ref();
}

extern "C" void C_SkData_unref(const SkData* self) {
    self->unref();
}

//
// SkPaint
//

extern "C" void C_SkPaint_destruct(const SkPaint* self) {
    self->~SkPaint();
}

extern "C" void C_SkPath_destruct(const SkPath* self) {
    self->~SkPath();
}

extern "C" void C_SkCanvas_destruct(const SkCanvas* self) {
    self->~SkCanvas();
}

//
// SkImageInfo
//

extern "C" void C_SkImageInfo_Construct(SkImageInfo* uninitialized) {
    new (uninitialized) SkImageInfo();
}

extern "C" void C_SkImageInfo_Destruct(SkImageInfo* self) {
    self->~SkImageInfo();
}

extern "C" void C_SkImageInfo_Copy(const SkImageInfo* from, SkImageInfo* to) {
    *to = *from;
}

extern "C" void C_SkImageInfo_Make(SkImageInfo* self, int width, int height, SkColorType ct, SkAlphaType at, SkColorSpace* cs) {
    *self = SkImageInfo::Make(width, height, ct, at, sk_sp<SkColorSpace>(cs));
}

extern "C" void C_SkImageInfo_MakeS32(SkImageInfo* self, int width, int height, SkAlphaType at) {
    *self = SkImageInfo::MakeS32(width, height, at);
}

extern "C" SkColorSpace* C_SkImageInfo_colorSpace(const SkImageInfo* self) {
    // note: colorSpace returns just a pointer without increasing the reference counter.
    SkColorSpace* cs = self->colorSpace();
    if (cs) cs->ref();
    return cs;
}

//
// SkColorSpace
//

extern "C" void C_SkColorSpace_ref(const SkColorSpace* self) {
    self->ref();
}

extern "C" void C_SkColorSpace_unref(const SkColorSpace* self) {
    self->unref();
}

extern "C" SkColorSpace* C_SkColorSpace_MakeSRGB() {
    return SkColorSpace::MakeSRGB().release();
}

extern "C" SkColorSpace* C_SkColorSpace_MakeSRGBLinear() {
    return SkColorSpace::MakeSRGBLinear().release();
}

extern "C" SkColorSpace* C_SkColorSpace_MakeRGB(SkColorSpace::RenderTargetGamma gamma, SkColorSpace::Gamut gamut) {
    return SkColorSpace::MakeRGB(gamma, gamut).release();
}

extern "C" SkColorSpace* C_SkColorSpace_MakeRGB2(SkColorSpace::RenderTargetGamma gamma, const SkMatrix44* toXYZD50) {
    return SkColorSpace::MakeRGB(gamma, *toXYZD50).release();
}

extern "C" SkColorSpace* C_SkColorSpace_MakeRGB3(const SkColorSpaceTransferFn* coeffs, SkColorSpace::Gamut gamut) {
    return SkColorSpace::MakeRGB(*coeffs, gamut).release();
}

extern "C" SkColorSpace* C_SkColorSpace_MakeRGB4(const SkColorSpaceTransferFn* coeffs, const SkMatrix44* toXYZD50) {
    return SkColorSpace::MakeRGB(*coeffs, *toXYZD50).release();
}

extern "C" SkColorSpace* C_SkColorSpace_MakeRGB5(SkGammaNamed gammaNamed, const SkMatrix44* toXYZD50) {
    return SkColorSpace::MakeRGB(gammaNamed, *toXYZD50).release();
}

extern "C" SkColorSpace* C_SkColorSpace_makeLinearGamma(const SkColorSpace* self) {
    return self->makeLinearGamma().release();
}

extern "C" SkColorSpace* C_SkColorSpace_makeSRGBGamma(const SkColorSpace* self) {
    return self->makeSRGBGamma().release();
}

extern "C" SkColorSpace* C_SkColorSpace_makeColorSpin(const SkColorSpace* self) {
    return self->makeColorSpin().release();
}

extern "C" SkData* C_SkColorSpace_serialize(const SkColorSpace* self) {
    return self->serialize().release();
}

extern "C" SkColorSpace* C_SkColorSpace_Deserialize(const void* data, size_t length) {
    return SkColorSpace::Deserialize(data, length).release();
}

extern "C" SkGammaNamed C_SkColorSpace_gammaNamed(const SkColorSpace* self) {
    return self->gammaNamed();
}

//
// SkMatrix44
//

// calling SkMatrix44::new(Uninitialized) leads to linker error.
extern "C" void C_SkMatrix44_Construct(SkMatrix44* uninitialized) {
    new(uninitialized) SkMatrix44();
}

extern "C" void C_SkMatrix44_Destruct(SkMatrix44* self) {
    self->~SkMatrix44();
}

// SkMatrix44_Equals is not generated by bindgen.
extern "C" bool C_SkMatrix44_Equals(const SkMatrix44* self, const SkMatrix44* rhs) {
    return *self == *rhs;
}

// SkMatrix44_SkMatrix conversion.
extern "C" void C_SkMatrix44_SkMatrix(const SkMatrix44* self, SkMatrix* m) {
    *m = *self;
}

extern "C" void C_SkMatrix44_Mul(const SkMatrix44* self, const SkMatrix44* rhs, SkMatrix44* result) {
    *result = *self * *rhs;
}

extern "C" void C_SkMatrix44_MulV4(const SkMatrix44* self, const SkVector4* rhs, SkVector4* result) {
    *result = *self * *rhs;
}

//
// SkMatrix
//

extern "C" bool C_SkMatrix_Equals(const SkMatrix* self, const SkMatrix* rhs) {
    return *self == *rhs;
}

//
// SkSurfaceProps
//

extern "C" bool C_SkSurfaceProps_Equals(const SkSurfaceProps* self, const SkSurfaceProps* rhs) {
    return *self == *rhs;
}

// SkBitmap

extern "C" void C_SkBitmap_Construct(SkBitmap* uninitialized) {
    new (uninitialized) SkBitmap();
}

extern "C" void C_SkBitmap_Destruct(SkBitmap* self) {
    self->~SkBitmap();
}

extern "C" void C_SkBitmap_Copy(const SkBitmap* from, SkBitmap* to) {
    *to = *from;
}

extern "C" SkColorSpace* C_SkBitmap_colorSpace(const SkBitmap* self) {
    // note: colorSpace returns a pointer without increasing the reference counter.
    SkColorSpace* cs = self->colorSpace();
    if (cs) cs->ref();
    return cs;
}

extern "C" bool C_SkBitmap_ComputeIsOpaque(const SkBitmap* self) {
    return SkBitmap::ComputeIsOpaque(*self);
}

extern "C" bool C_SkBitmap_tryAllocN32Pixels(SkBitmap* self, int width, int height, bool isOpaque) {
    return self->tryAllocN32Pixels(width, height, isOpaque);
}

extern "C" bool C_SkBitmap_tryAllocPixels(SkBitmap* self) {
    return self->tryAllocPixels();
}

extern "C" bool C_SkBitmap_readyToDraw(const SkBitmap* self) {
    return self->readyToDraw();
}

extern "C" void C_SkBitmap_eraseARGB(const SkBitmap* self, U8CPU a, U8CPU r, U8CPU g, U8CPU b) {
    self->eraseARGB(a, r, g, b);
}

extern "C" float C_SkBitmap_getAlphaf(const SkBitmap* self, int x, int y) {
    return self->getAlphaf(x, y);
}

extern "C" bool C_SkBitmap_extractAlpha(const SkBitmap* self, SkBitmap* dst, const SkPaint* paint, SkIPoint* offset) {
    return self->extractAlpha(dst, paint, offset);
}

//
// SkPicture
//

extern "C" SkPicture* C_SkPicture_MakeFromData(const SkData* data) {
    return SkPicture::MakeFromData(data).release();
}

extern "C" SkData* C_SkPicture_serialize(const SkPicture* self) {
    return self->serialize().release();
}

extern "C" SkPicture* C_SkPicture_MakePlaceholder(const SkRect& cull) {
    return SkPicture::MakePlaceholder(cull).release();
}

extern "C" void C_SkPicture_playback(const SkPicture* self, SkCanvas* canvas) {
    self->playback(canvas);
}
/*
extern "C" void C_SkPicture_cullRect(const SkPicture* self, SkRect* result) {
    *result = self->cullRect();
}
*/

extern "C" SkRect C_SkPicture_cullRect(const SkPicture* self) {
    return self->cullRect();
}

//
// GrBackendTexture
//

extern "C" void C_GrBackendTexture_destruct(const GrBackendTexture* self) {
    self->~GrBackendTexture();
}

#if defined(SK_VULKAN)

extern "C" SkSurface* C_SkSurface_MakeFromBackendTexture(
    GrContext* context,
    const GrBackendTexture* backendTexture,
    GrSurfaceOrigin origin,
    int sampleCnt,
    SkColorType colorType) {
    return SkSurface::MakeFromBackendTexture(context, *backendTexture, origin, sampleCnt, colorType, nullptr, nullptr).release();
}

extern "C" void C_SkSurface_getBackendTexture(
        SkSurface* self,
        SkSurface::BackendHandleAccess handleAccess,
        GrBackendTexture* backendTexture) {
    *backendTexture = self->getBackendTexture(handleAccess);
}

// The GrVkBackendContext struct binding's length is too short
// because of the std::function that is used in it.

typedef PFN_vkVoidFunction (*GetProcFn)(const char* name, VkInstance instance, VkDevice device);
typedef const void* (*GetProcFnVoidPtr)(const char* name, VkInstance instance, VkDevice device);

extern "C" void* C_GrVkBackendContext_New(
        void* instance,
        void* physicalDevice,
        void* device,
        void* queue,
        uint32_t graphicsQueueIndex,

        /* PFN_vkVoidFunction makes us trouble on the Rust side */
        GetProcFnVoidPtr getProc) {

    auto& context = *new GrVkBackendContext();
    context.fInstance = static_cast<VkInstance>(instance);
    context.fPhysicalDevice = static_cast<VkPhysicalDevice>(physicalDevice);
    context.fDevice = static_cast<VkDevice>(device);
    context.fQueue = static_cast<VkQueue>(queue);
    context.fGraphicsQueueIndex = graphicsQueueIndex;

    context.fGetProc = *(reinterpret_cast<GetProcFn*>(&getProc));
    return &context;
}

extern "C" void C_GrVkBackendContext_Delete(void* vkBackendContext) {
    delete static_cast<GrVkBackendContext*>(vkBackendContext);
}

extern "C" GrContext* C_GrContext_MakeVulkan(const void* vkBackendContext) {
    return GrContext::MakeVulkan(*static_cast<const GrVkBackendContext*>(vkBackendContext)).release();
}

#endif