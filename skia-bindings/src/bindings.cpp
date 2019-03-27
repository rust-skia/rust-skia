// core/
#include "SkTypes.h"
#include "SkCanvas.h"
#include "SkColor.h"
#include "SkColorFilter.h"
#include "SkFont.h"
#include "SkFontMetrics.h"
#include "SkImageFilter.h"
#include "SkImageInfo.h"
#include "SkMaskFilter.h"
#include "SkPaint.h"
#include "SkPath.h"
#include "SkPicture.h"
#include "SkPictureRecorder.h"
#include "SkPoint3.h"
#include "SkRect.h"
#include "SkSurface.h"
#include "SkYUVAIndex.h"
#include "SkRegion.h"
#include "SkStrokeRec.h"
#include "SkTextBlob.h"
#include "SkTypeface.h"
// effects/
#include "Sk1DPathEffect.h"
#include "Sk2DPathEffect.h"
#include "SkCornerPathEffect.h"
#include "SkDashPathEffect.h"
#include "SkDiscretePathEffect.h"
#include "SkGradientShader.h"
#include "SkPerlinNoiseShader.h"
#include "SkTableColorFilter.h"
// gpu/
#include "GrContext.h"
// gpu/gl
#include "gl/GrGLInterface.h"

#if defined(SK_VULKAN)

#include "vk/GrVkVulkan.h"
#include "vk/GrVkTypes.h"
#include "vk/GrVkBackendContext.h"
#include "GrBackendSurface.h"

#endif

template<typename T>
inline sk_sp<T> spFromConst(const T* pt) {
    return sk_sp<T>(const_cast<T*>(pt));
}

//
// SkSurface
//

extern "C" SkSurface* C_SkSurface_MakeRasterDirect(const SkImageInfo* imageInfo, void* pixels, size_t rowBytes, const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeRasterDirect(*imageInfo, pixels, rowBytes, surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeRaster(const SkImageInfo* imageInfo, size_t rowBytes, const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeRaster(*imageInfo, rowBytes, surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeRasterN32Premul(int width, int height, const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeRasterN32Premul(width, height, surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeFromBackendTexture(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        int sampleCnt,
        SkColorType colorType,
        const SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeFromBackendTexture(
            context,
            *backendTexture,
            origin,
            sampleCnt,
            colorType,
            spFromConst(colorSpace), surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeFromBackendRenderTarget(
        GrContext* context,
        const GrBackendRenderTarget* backendRenderTarget,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        const SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps
        ) {
    return SkSurface::MakeFromBackendRenderTarget(
            context,
            *backendRenderTarget,
            origin,
            colorType,
            spFromConst(colorSpace),
            surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeFromBackendTextureAsRenderTarget(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        int sampleCnt,
        SkColorType colorType,
        const SkColorSpace* colorSpace,
        const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeFromBackendTextureAsRenderTarget(
            context,
            *backendTexture,
            origin,
            sampleCnt,
            colorType,
            spFromConst(colorSpace), surfaceProps).release();
}

extern "C" SkSurface* C_SkSurface_MakeRenderTarget(
    GrContext* context,
    SkBudgeted budgeted,
    const SkImageInfo* imageInfo,
    int sampleCount, GrSurfaceOrigin surfaceOrigin,
    const SkSurfaceProps* surfaceProps,
    bool shouldCreateWithMips) {
    return SkSurface::MakeRenderTarget(
            context,
            budgeted,
            *imageInfo,
            sampleCount,
            surfaceOrigin,
            surfaceProps,
            shouldCreateWithMips).release();
}

extern "C" SkImage* C_SkSurface_makeImageSnapshot(SkSurface* self, const SkIRect* bounds) {
    if (bounds) {
        return self->makeImageSnapshot(*bounds).release();
    } else {
        return self->makeImageSnapshot().release();
    }
}

extern "C" void C_SkSurface_getBackendTexture(
        SkSurface* self,
        SkSurface::BackendHandleAccess handleAccess,
        GrBackendTexture* backendTexture) {
    *backendTexture = self->getBackendTexture(handleAccess);
}

extern "C" void C_SkSurface_getBackendRenderTarget(
        SkSurface* self,
        SkSurface::BackendHandleAccess handleAccess,
        GrBackendRenderTarget *backendRenderTarget) {
    *backendRenderTarget = self->getBackendRenderTarget(handleAccess);
}

extern "C" SkSurface* C_SkSurface_makeSurface(
        SkSurface* self,
        const SkImageInfo* imageInfo) {
    return self->makeSurface(*imageInfo).release();
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

extern "C" SkImage* C_SkImage_MakeFromEncoded(const SkData* encoded, const SkIRect* subset) {
    return SkImage::MakeFromEncoded(spFromConst(encoded), subset).release();
}

extern "C" SkImage* C_SkImage_MakeFromCompressed(GrContext* context, const SkData* encoded, int width, int height, SkImage::CompressionType type) {
    return SkImage::MakeFromCompressed(context, spFromConst(encoded), width, height, type).release();
}

extern "C" SkImage* C_SkImage_MakeFromTexture(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkAlphaType alphaType,
        const SkColorSpace* colorSpace) {
    return SkImage::MakeFromTexture(context, *backendTexture, origin, colorType, alphaType, spFromConst(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeCrossContextFromEncoded(
        GrContext* context,
        const SkData* data,
        bool buildMips,
        const SkColorSpace* dstColorSpace,
        bool limitToMaxTextureSize
        ) {
    return SkImage::MakeCrossContextFromEncoded(context, spFromConst(data), buildMips, const_cast<SkColorSpace*>(dstColorSpace), limitToMaxTextureSize).release();
}

extern "C" SkImage* C_SkImage_MakeFromAdoptedTexture(
        GrContext* context,
        const GrBackendTexture* backendTexture,
        GrSurfaceOrigin origin,
        SkColorType colorType,
        SkAlphaType alphaType,
        const SkColorSpace* colorSpace) {
    return SkImage::MakeFromAdoptedTexture(context, *backendTexture, origin, colorType, alphaType, spFromConst(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATexturesCopy(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture yuvaTextures[],
        const SkYUVAIndex yuvaIndices[4],
        SkISize imageSize,
        GrSurfaceOrigin imageOrigin,
        const SkColorSpace* colorSpace) {
    return SkImage::MakeFromYUVATexturesCopy(
            context,
            yuvColorSpace, yuvaTextures, yuvaIndices,
            imageSize, imageOrigin, spFromConst(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATexturesCopyWithExternalBackend(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture yuvaTextures[],
        const SkYUVAIndex yuvaIndices[4],
        SkISize imageSize,
        GrSurfaceOrigin imageOrigin,
        const GrBackendTexture& backendTexture,
        const SkColorSpace* colorSpace) {
    return SkImage::MakeFromYUVATexturesCopyWithExternalBackend(
            context,
            yuvColorSpace, yuvaTextures, yuvaIndices,
            imageSize, imageOrigin, backendTexture,
            spFromConst(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromYUVATextures(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture yuvaTextures[],
        const SkYUVAIndex yuvaIndices[4],
        SkISize imageSize,
        GrSurfaceOrigin imageOrigin,
        const SkColorSpace* colorSpace) {
    return SkImage::MakeFromYUVATextures(
            context,
            yuvColorSpace, yuvaTextures, yuvaIndices,
            imageSize, imageOrigin, spFromConst(colorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromNV12TexturesCopy(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture nv12Textures[2],
        GrSurfaceOrigin imageOrigin,
        const SkColorSpace* imageColorSpace) {
    return SkImage::MakeFromNV12TexturesCopy(
            context, yuvColorSpace, nv12Textures, imageOrigin,
            spFromConst(imageColorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromNV12TexturesCopyWithExternalBackend(
        GrContext* context,
        SkYUVColorSpace yuvColorSpace,
        const GrBackendTexture nv12Textures[2],
        GrSurfaceOrigin imageOrigin,
        const GrBackendTexture* backendTexture,
        const SkColorSpace* imageColorSpace) {
    return SkImage::MakeFromNV12TexturesCopyWithExternalBackend(
            context,
            yuvColorSpace, nv12Textures, imageOrigin, *backendTexture,
            spFromConst(imageColorSpace)).release();
}

extern "C" SkImage* C_SkImage_MakeFromPicture(
        const SkPicture* picture,
        const SkISize* dimensions,
        const SkMatrix* matrix,
        const SkPaint* paint,
        SkImage::BitDepth bitDepth,
        const SkColorSpace* colorSpace) {
    return SkImage::MakeFromPicture(spFromConst(picture), *dimensions, matrix, paint, bitDepth, spFromConst(colorSpace)).release();
}

extern "C" SkShader* C_SkImage_makeShader(const SkImage* self, SkShader::TileMode tileMode1, SkShader::TileMode tileMode2, const SkMatrix* localMatrix) {
    return self->makeShader(tileMode1, tileMode2, localMatrix).release();
}

extern "C" void C_SkImage_getBackendTexture(
        const SkImage* self,
        bool flushPendingGrContextIO,
        GrSurfaceOrigin* origin,
        GrBackendTexture* result)
{
    *result = self->getBackendTexture(flushPendingGrContextIO, origin);
}

extern "C" SkData* C_SkImage_encodeToData(const SkImage* self, SkEncodedImageFormat imageFormat) {
    return self->encodeToData(imageFormat, 100).release();
}

extern "C" SkData* C_SkImage_refEncodedData(const SkImage* self) {
    return self->refEncodedData().release();
}

extern "C" SkImage* C_SkImage_makeSubset(const SkImage* self, const SkIRect* subset) {
    return self->makeSubset(*subset).release();
}

extern "C" SkImage* C_SkImage_makeTextureImage(
        const SkImage* self,
        GrContext* context,
        const SkColorSpace* dstColorSpace,
        GrMipMapped mipMapped) {
    return self->makeTextureImage(context, const_cast<SkColorSpace*>(dstColorSpace), mipMapped).release();
}

extern "C" SkImage* C_SkImage_makeNonTextureImage(const SkImage* self) {
    return self->makeNonTextureImage().release();
}

extern "C" SkImage* C_SkImage_makeRasterImage(const SkImage* self) {
    return self->makeRasterImage().release();
}

extern "C" SkImage* C_SkImage_makeColorSpace(const SkImage* self, const SkColorSpace* target) {
    return self->makeColorSpace(spFromConst(target)).release();
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

extern "C" SkData* C_SkData_MakeWithCopy(const void* data, size_t length) {
    return SkData::MakeWithCopy(data, length).release();
}

extern "C" SkData* C_SkData_MakeWithoutCopy(const void* data, size_t length) {
    return SkData::MakeWithoutCopy(data, length).release();
}

extern "C" SkData* C_SkData_MakeSubset(const SkData* src, size_t offset, size_t length) {
    return SkData::MakeSubset(src, offset, length).release();
}

extern "C" SkData* C_SkData_MakeEmpty() {
    return SkData::MakeEmpty().release();
}

//
// SkPaint
//

extern "C" void C_SkPaint_destruct(SkPaint* self) {
    self->~SkPaint();
}

extern "C" void C_SkPaint_copy(SkPaint* self, const SkPaint* rhs) {
    *self = *rhs;
}

extern "C" bool C_SkPaint_Equals(const SkPaint* lhs, const SkPaint* rhs) {
    return *lhs == *rhs;
}

extern "C" void C_SkPaint_setShader(SkPaint* self, const SkShader* shader) {
    self->setShader(spFromConst(shader));
}

extern "C" void C_SkPaint_setColorFilter(SkPaint* self, const SkColorFilter* colorFilter) {
    self->setColorFilter(spFromConst(colorFilter));
}

extern "C" void C_SkPaint_setPathEffect(SkPaint* self, const SkPathEffect* pathEffect) {
    self->setPathEffect(spFromConst(pathEffect));
}

extern "C" void C_SkPaint_setMaskFilter(SkPaint* self, const SkMaskFilter* maskFilter) {
    self->setMaskFilter(spFromConst(maskFilter));
}

extern "C" SkFontHinting C_SkPaint_getHinting(const SkPaint* self) {
    return self->getHinting();
}

// postponed

/*
extern "C" void C_SkPaint_setImageFilter(SkPaint* self, SkImageFilter* imageFilter) {
    self->setImageFilter(sk_sp<SkImageFilter>(imageFilter));
}
*/

//
// SkPath
//

extern "C" void C_SkPath_destruct(const SkPath* self) {
    self->~SkPath();
}

extern "C" bool C_SkPath_Equals(const SkPath* lhs, const SkPath* rhs) {
    return *lhs == *rhs;
}

extern "C" SkData* C_SkPath_serialize(const SkPath* self) {
    return self->serialize().release();
}

extern "C" SkPath::FillType C_SkPath_ConvertToNonInverseFillType(SkPath::FillType fill) {
    return SkPath::ConvertToNonInverseFillType(fill);
}

//
// SkCanvas
// Note: bindgen layout is broken, so we are forced to allocate Canvas instances on the heap only.
//

extern "C" SkCanvas* C_SkCanvas_newEmpty() {
    return new SkCanvas();
}

extern "C" SkCanvas* C_SkCanvas_newWidthHeightAndProps(int width, int height, const SkSurfaceProps* props) {
    return new SkCanvas(width, height, props);
}

extern "C" SkCanvas* C_SkCanvas_newFromBitmap(const SkBitmap* bitmap) {
    return new SkCanvas(*bitmap);
}

extern "C" SkCanvas* C_SkCanvas_newFromBitmapAndProps(const SkBitmap* bitmap, const SkSurfaceProps* props) {
    return new SkCanvas(*bitmap, *props);
}

extern "C" void C_SkCanvas_delete(const SkCanvas* self) {
    delete self;
}

extern "C" SkCanvas* C_SkCanvas_MakeRasterDirect(const SkImageInfo* info, void* pixels, size_t row_bytes, const SkSurfaceProps* props) {
    return SkCanvas::MakeRasterDirect(*info, pixels, row_bytes, props).release();
}

extern "C" void C_SkCanvas_imageInfo(const SkCanvas* self, SkImageInfo* info) {
    *info = self->imageInfo();
}

extern "C" void C_SkCanvas_getBaseLayerSize(const SkCanvas* self, SkISize* size) {
    *size = self->getBaseLayerSize();
}

extern "C" SkSurface* C_SkCanvas_makeSurface(SkCanvas* self, const SkImageInfo* info, const SkSurfaceProps* props) {
    return self->makeSurface(*info, props).release();
}

extern "C" GrContext* C_SkCanvas_getGrContext(SkCanvas* self) {
    return self->getGrContext();
}

extern "C" bool C_SkCanvas_isClipEmpty(const SkCanvas* self) {
    return self->isClipEmpty();
}

extern "C" bool C_SkCanvas_isClipRect(const SkCanvas* self) {
    return self->isClipRect();
}

extern "C" void C_SkCanvas_discard(SkCanvas* self) {
    self->discard();
}

//
// SkAutoCanvasRestore
//

#undef SkAutoCanvasRestore

extern "C" void C_SkAutoCanvasRestore_Construct(SkAutoCanvasRestore* uninitialized, SkCanvas* canvas, bool doSave) {
    new(uninitialized) SkAutoCanvasRestore(canvas, doSave);
}

extern "C" void C_SkAutoCanvasRestore_destruct(const SkAutoCanvasRestore* self) {
    self->~SkAutoCanvasRestore();
}

extern "C" void C_SkAutoCanvasRestore_restore(SkAutoCanvasRestore* self) {
    self->restore();
}

//
// SkImageInfo
//

extern "C" void C_SkImageInfo_Construct(SkImageInfo* uninitialized) {
    new (uninitialized) SkImageInfo();
}

extern "C" void C_SkImageInfo_destruct(SkImageInfo* self) {
    self->~SkImageInfo();
}

extern "C" void C_SkImageInfo_Copy(const SkImageInfo* from, SkImageInfo* to) {
    *to = *from;
}

extern "C" void C_SkImageInfo_Make(SkImageInfo* self, int width, int height, SkColorType ct, SkAlphaType at, const SkColorSpace* cs) {
    *self = SkImageInfo::Make(width, height, ct, at, spFromConst(cs));
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

//
// SkMatrix44
//

extern "C" void C_SkMatrix44_Construct(SkMatrix44* uninitialized) {
    new(uninitialized) SkMatrix44();
}

extern "C" void C_SkMatrix44_CopyConstruct(SkMatrix44* uninitialized, const SkMatrix44* source) {
    new(uninitialized) SkMatrix44(*source);
}

extern "C" void C_SkMatrix44_ConstructIdentity(SkMatrix44* uninitialized) {
    new(uninitialized) SkMatrix44(SkMatrix44::kIdentity_Constructor);
}

extern "C" void C_SkMatrix44_destruct(SkMatrix44* self) {
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

extern "C" SkScalar* C_SkMatrix_SubscriptMut(SkMatrix* self, size_t index) {
    return &((*self)[index]);
}

//
// SkSurfaceProps
//

extern "C" bool C_SkSurfaceProps_Equals(const SkSurfaceProps* self, const SkSurfaceProps* rhs) {
    return *self == *rhs;
}

//
// SkBitmap
//

extern "C" void C_SkBitmap_Construct(SkBitmap* uninitialized) {
    new (uninitialized) SkBitmap();
}

extern "C" void C_SkBitmap_destruct(SkBitmap* self) {
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

extern "C" SkRect C_SkPicture_cullRect(const SkPicture* self) {
    return self->cullRect();
}

extern "C" int C_SkPicture_approximateOpCount(const SkPicture* self) {
    return self->approximateOpCount();
}

// note: returning size_t produces a linker error.
extern "C" void C_SkPicture_approximateBytesUsed(const SkPicture* self, size_t* out) {
    *out = self->approximateBytesUsed();
}

//
// SkRRect
//

extern "C" bool C_SkRRect_Equals(const SkRRect* lhs, const SkRRect* rhs) {
    return *lhs == *rhs;
}

//
// GrBackendTexture
//

extern "C" void C_GrBackendTexture_destruct(const GrBackendTexture* self) {
    self->~GrBackendTexture();
}

//
// SkRegion
//

extern "C" void C_SkRegion_destruct(SkRegion* region) {
    region->~SkRegion();
}

extern "C" bool C_SkRegion_Equals(const SkRegion* lhs, const SkRegion* rhs) {
    return *lhs == *rhs;
}

//
// SkFontStyle
//

extern "C" void C_SkFontStyle_Construct(SkFontStyle* uninitialized) {
    new(uninitialized) SkFontStyle();
}

extern "C" bool C_SkFontStyle_Equals(const SkFontStyle* lhs, const SkFontStyle* rhs) {
    return *lhs == *rhs;
}

//
// SkTextBlob
//

extern "C" void C_SkTextBlob_ref(const SkTextBlob* self) {
    self->ref();
}

extern "C" void C_SkTextBlob_unref(const SkTextBlob* self) {
    self->unref();
}

extern "C" SkTextBlob* C_SkTextBlob_MakeFromText(const void* text, size_t byteLength, const SkFont* font, SkTextEncoding encoding) {
    return SkTextBlob::MakeFromText(text, byteLength, *font, encoding).release();
}

//
// SkTypeface
//

extern "C" SkTypeface* C_SkTypeface_MakeDefault() {
    return SkTypeface::MakeDefault().release();
}

extern "C" SkTypeface* C_SkTypeface_MakeFromName(const char familyName[], SkFontStyle fontStyle) {
    return SkTypeface::MakeFromName(familyName, fontStyle).release();
}

extern "C" SkTypeface* C_SkTypeface_MakeFromFile(const char path[], int index) {
    return SkTypeface::MakeFromFile(path, index).release();
}

extern "C" SkTypeface* C_SkTypeface_MakeFromData(const SkData* data, int index) {
    return SkTypeface::MakeFromData(sk_sp<SkData>(const_cast<SkData*>(data)), index).release();
}

extern "C" SkData* C_SkTypeface_serialize(const SkTypeface* self, SkTypeface::SerializeBehavior behavior) {
    return self->serialize(behavior).release();
}

//
// SkFont
//

extern "C" void C_SkFont_ConstructFromTypeface(SkFont* uninitialized, const SkTypeface* typeface) {
    new(uninitialized) SkFont(spFromConst(typeface));
}

extern "C" void C_SkFont_ConstructFromTypefaceWithSize(SkFont* uninitialized, const SkTypeface* typeface, SkScalar size) {
    new(uninitialized) SkFont(spFromConst(typeface), size);
}

extern "C" void C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew(SkFont* uninitialized, const SkTypeface* typeface, SkScalar size, SkScalar scaleX, SkScalar skewX) {
    new(uninitialized) SkFont(spFromConst(typeface), size, scaleX, skewX);
}

extern "C" bool C_SkFont_Equals(const SkFont* self, const SkFont* other) {
    return *self == *other;
}

extern "C" void C_SkFont_makeWithSize(const SkFont* self, SkScalar size, SkFont* result) {
    *result = self->makeWithSize(size);
}

extern "C" SkTypeface* C_SkFont_getTypeface(SkFont* self) {
    return self->getTypeface();
}

extern "C" void C_SkFont_setTypeface(SkFont* self, const SkTypeface* tf) {
    self->setTypeface(spFromConst(tf));
}

extern "C" void C_SkFont_destruct(SkFont* self) {
    self->~SkFont();
}

//
// SkVertices
//

extern "C" void C_SkVertices_ref(const SkVertices* self) {
    self->ref();
}

extern "C" void C_SkVertices_unref(const SkVertices* self) {
    self->unref();
}

extern "C" SkVertices* C_SkVertices_MakeCopy(
    SkVertices::VertexMode mode, int vertexCount,
    const SkPoint positions[],
    const SkPoint texs[],
    const SkColor colors[],
    const SkVertices::BoneIndices boneIndices[],
    const SkVertices::BoneWeights boneWeights[],
    int indexCount,
    const uint16_t indices[],
    bool isVolatile) {
    return SkVertices::MakeCopy(mode, vertexCount, positions, texs, colors, boneIndices, boneWeights, indexCount, indices, isVolatile).release();
}

extern "C" SkVertices* C_SkVertices_applyBones(const SkVertices* self, const SkVertices::Bone bones[], int boneCount) {
    return self->applyBones(bones, boneCount).release();
}

extern "C" SkVertices* C_SkVertices_Decode(const void* buffer, size_t length) {
    return SkVertices::Decode(buffer, length).release();
}

extern "C" SkData* C_SkVertices_encode(const SkVertices* self) {
    return self->encode().release();
}

//
// SkVertices::Builder
//

extern "C" void C_SkVertices_Builder_destruct(SkVertices::Builder* builder) {
    builder->~Builder();
}

extern "C" SkVertices* C_SkVertices_Builder_detach(SkVertices::Builder* builder) {
    return builder->detach().release();
}

//
// SkPictureRecorder
//

extern "C" void C_SkPictureRecorder_destruct(SkPictureRecorder *self) {
    self->~SkPictureRecorder();
}

extern "C" SkPicture* C_SkPictureRecorder_finishRecordingAsPicture(SkPictureRecorder* self, const SkRect* cullRect) {
    if (cullRect){
        return self->finishRecordingAsPictureWithCull(*cullRect).release();
    } else {
        return self->finishRecordingAsPicture().release();
    }
}

//
// SkColorFilter
//

extern "C" SkColorFilter* C_SkColorFilter_MakeModeFilter(const SkColor* c, const SkBlendMode* blendMode) {
    return SkColorFilter::MakeModeFilter(*c, *blendMode).release();
}

extern "C" SkColorFilter* C_SkColorFilter_makeComposed(const SkColorFilter* self, const SkColorFilter* inner) {
    return self->makeComposed(spFromConst(inner)).release();
}

extern "C" SkColorFilter* C_SkColorFilter_MakeMatrixFilterRowMajor255(const SkScalar array[20]) {
    return SkColorFilter::MakeMatrixFilterRowMajor255(array).release();
}

extern "C" SkColorFilter* C_SkColorFilter_MakeLinearToSRGBGamma() {
    return SkColorFilter::MakeLinearToSRGBGamma().release();
}

extern "C" SkColorFilter* C_SkColorFilter_MakeSRGBToLinearGamma() {
    return SkColorFilter::MakeSRGBToLinearGamma().release();
}

extern "C" bool C_SkColorFilter_asColorMode(const SkColorFilter* self, SkColor* color, SkBlendMode* mode) {
    return self->asColorMode(color, mode);
}

extern "C" bool C_SkColorFilter_asColorMatrix(const SkColorFilter* self, SkScalar matrix[20]) {
    return self->asColorMatrix(matrix);
}

extern "C" bool C_SkColorFilter_asComponentTable(const SkColorFilter* self, SkBitmap* table) {
    return self->asComponentTable(table);
}

extern "C" uint32_t C_SkColorFilter_getFlags(const SkColorFilter* self) {
    return self->getFlags();
}

//
// SkStrokeRec
//

extern "C" void C_SkStrokeRec_destruct(SkStrokeRec* self) {
    self->~SkStrokeRec();
}

extern "C" void C_SkStrokeRec_copy(const SkStrokeRec* self, SkStrokeRec* other) {
    *other = *self;
}

extern "C" bool C_SkStrokeRec_hasEqualEffect(const SkStrokeRec* self, const SkStrokeRec* other) {
    return self->hasEqualEffect(*other);
}

//
// SkPathEffect
//

extern "C" SkPathEffect* C_SkPathEffect_MakeSum(const SkPathEffect* first, const SkPathEffect* second) {
    return SkPathEffect::MakeSum(spFromConst(first), spFromConst(second)).release();
}

extern "C" SkPathEffect* C_SkPathEffect_MakeCompose(const SkPathEffect* outer, const SkPathEffect* inner) {
    return SkPathEffect::MakeCompose(spFromConst(outer), spFromConst(inner)).release();
}

extern "C" void C_SkPathEffect_PointData_Construct(SkPathEffect::PointData* unitialized) {
    new(unitialized) SkPathEffect::PointData();
}

extern "C" void C_SkPathEffect_PointData_deletePoints(SkPathEffect::PointData* self) {
    delete [] self->fPoints;
    self->fPoints = nullptr;
}

//
// SkMaskFilter
//

extern "C" SkMaskFilter* C_SkMaskFilter_MakeBlur(SkBlurStyle style, SkScalar sigma, bool respectCTM) {
    return SkMaskFilter::MakeBlur(style, sigma, respectCTM).release();
}

extern "C" SkMaskFilter* C_SkMaskFilter_Compose(const SkMaskFilter* outer, const SkMaskFilter* inner) {
    return SkMaskFilter::MakeCompose(spFromConst(outer), spFromConst(inner)).release();
}

extern "C" SkMaskFilter* C_SkMaskFilter_Combine(const SkMaskFilter* filterA, const SkMaskFilter* filterB, SkCoverageMode coverageMode) {
    return SkMaskFilter::MakeCombine(spFromConst(filterA), spFromConst(filterB), coverageMode).release();
}

extern "C" SkMaskFilter* C_SkMaskFilter_makeWithMatrix(const SkMaskFilter* self, const SkMatrix* matrix) {
    return self->makeWithMatrix(*matrix).release();
}

//
// SkSize
//

extern "C" SkISize C_SkSize_toFloor(const SkSize* size) {
    return size->toFloor();
}

//
// SkShader
//

extern "C" bool C_SkShader_isOpaque(const SkShader* self) {
    return self->isOpaque();
}

extern "C" bool C_SkShader_isAImage(const SkShader* self) {
    return self->isAImage();
}

extern "C" SkShader::GradientType C_SkShader_asAGradient(const SkShader* self, SkShader::GradientInfo* info) {
    return self->asAGradient(info);
}

extern "C" SkShader* C_SkShader_makeWithLocalMatrix(const SkShader* self, const SkMatrix* matrix) {
    return self->makeWithLocalMatrix(*matrix).release();
}

extern "C" SkShader* C_SkShader_makeWithColorFilter(const SkShader* self, const SkColorFilter* colorFilter) {
    return self->makeWithColorFilter(spFromConst(colorFilter)).release();
}

extern "C" SkShader* C_SkShader_MakeEmptyShader() {
    return SkShader::MakeEmptyShader().release();
}

extern "C" SkShader* C_SkShader_MakeColorShader(SkColor color) {
    return SkShader::MakeColorShader(color).release();
}

extern "C" SkShader* C_SkShader_MakeColorShader2(const SkColor4f* color, const SkColorSpace* colorSpace) {
    return SkShader::MakeColorShader(*color, spFromConst(colorSpace)).release();
}

extern "C" SkShader* C_SkShader_MakeCompose(const SkShader* dst, const SkShader* src, SkBlendMode mode, float lerp) {
    return SkShader::MakeCompose(spFromConst(dst), spFromConst(src), mode, lerp).release();
}

extern "C" SkShader* C_SkShader_MakeMixer(const SkShader* dst, const SkShader* src, float lerp) {
    return SkShader::MakeMixer(spFromConst(dst), spFromConst(src), lerp).release();
}

extern "C" SkShader* C_SkShader_MakeBitmapShader(const SkBitmap* src, SkShader::TileMode tmx, SkShader::TileMode tmy, const SkMatrix* localMatrix) {
    return SkShader::MakeBitmapShader(*src, tmx, tmy, localMatrix).release();
}

extern "C" SkShader* C_SkShader_MakePictureShader(const SkPicture* src, SkShader::TileMode tmx, SkShader::TileMode tmy, const SkMatrix* localMatrix, const SkRect* tile) {
    return SkShader::MakePictureShader(spFromConst(src), tmx, tmy, localMatrix, tile).release();
}

extern "C" SkShader* C_SkShader_makeAsALocalMatrixShader(const SkShader* self, SkMatrix* localMatrix) {
    return self->makeAsALocalMatrixShader(localMatrix).release();
}

//
// SkGradientShader
//

extern "C" SkShader* C_SkGradientShader_MakeLinear(const SkPoint pts[2], const SkColor colors[], const SkScalar pos[], int count, SkShader::TileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeLinear(pts, colors, pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeLinear2(const SkPoint pts[2], const SkColor4f colors[], const SkColorSpace* colorSpace, const SkScalar pos[], int count, SkShader::TileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeLinear(pts, colors, spFromConst(colorSpace), pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeRadial(const SkPoint* center, SkScalar radius, const SkColor colors[], const SkScalar pos[], int count, SkShader::TileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeRadial(*center, radius, colors, pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeRadial2(const SkPoint* center, SkScalar radius, const SkColor4f colors[], const SkColorSpace* colorSpace, const SkScalar pos[], int count, SkShader::TileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeRadial(*center, radius, colors, spFromConst(colorSpace), pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeTwoPointConical(const SkPoint* start, SkScalar startRadius, const SkPoint* end, SkScalar endRadius, const SkColor colors[], const SkScalar pos[], int count, SkShader::TileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeTwoPointConical(*start, startRadius, *end, endRadius, colors, pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeTwoPointConical2(const SkPoint* start, SkScalar startRadius, const SkPoint* end, SkScalar endRadius, const SkColor4f colors[], const SkColorSpace* colorSpace, const SkScalar pos[], int count, SkShader::TileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeTwoPointConical(*start, startRadius, *end, endRadius, colors, spFromConst(colorSpace), pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeSweep(SkScalar cx, SkScalar cy, const SkColor colors[], const SkScalar pos[], int count, SkShader::TileMode mode, SkScalar startAngle, SkScalar endAngle, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeSweep(cx, cy, colors, pos, count, mode, startAngle, endAngle, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeSweep2(SkScalar cx, SkScalar cy, const SkColor4f colors[], const SkColorSpace* colorSpace, const SkScalar pos[], int count, SkShader::TileMode mode, SkScalar startAngle, SkScalar endAngle, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeSweep(cx, cy, colors, spFromConst(colorSpace), pos, count, mode, startAngle, endAngle, flags, localMatrix).release();
}

//
// SkPerlinNoiseShader
//

extern "C" SkShader* C_SkPerlinNoiseShader_MakeFractalNoise(SkScalar baseFrequencyX, SkScalar baseFrequencyY, int numOctaves, SkScalar seed, const SkISize* tileSize) {
    return SkPerlinNoiseShader::MakeFractalNoise(baseFrequencyX, baseFrequencyY, numOctaves, seed, tileSize).release();
}

extern "C" SkShader* C_SkPerlinNoiseShader_MakeTurbulence(SkScalar baseFrequencyX, SkScalar baseFrequencyY, int numOctaves, SkScalar seed, const SkISize* tileSize) {
    return SkPerlinNoiseShader::MakeTurbulence(baseFrequencyX, baseFrequencyY, numOctaves, seed, tileSize).release();
}

extern "C" SkShader* C_SkPerlinNoiseShader_MakeImprovedNoise(SkScalar baseFrequencyX, SkScalar baseFrequencyY, int numOctaves, SkScalar z) {
    return SkPerlinNoiseShader::MakeImprovedNoise(baseFrequencyX, baseFrequencyY, numOctaves, z).release();
}

//
// SkTableColorFilter
//

extern "C" SkColorFilter* C_SkTableColorFilter_Make(const uint8_t table[256]) {
    return SkTableColorFilter::Make(table).release();
}

extern "C" SkColorFilter* C_SkTableColorFilter_MakeARGB(const uint8_t tableA[256], const uint8_t tableR[256], const uint8_t tableG[256], const uint8_t tableB[256]) {
    return SkTableColorFilter::MakeARGB(tableA, tableR, tableG, tableB).release();
}

//
// SkPath1DPathEffect
//

extern "C" SkPathEffect* C_SkPath1DPathEffect_Make(const SkPath* path, SkScalar advance, SkScalar phase, SkPath1DPathEffect::Style style) {
    return SkPath1DPathEffect::Make(*path, advance, phase, style).release();
}

//
// SkLine2DPathEffect
//

extern "C" SkPathEffect* C_SkLine2DPathEffect_Make(SkScalar width, const SkMatrix* matrix) {
    return SkLine2DPathEffect::Make(width, *matrix).release();
}

//
// SkPath2DPathEffect
//

extern "C" SkPathEffect* C_SkPath2DPathEffect_Make(const SkMatrix* matrix, const SkPath* path) {
    return SkPath2DPathEffect::Make(*matrix, *path).release();
}

//
// SkCornerPathEffect
//

extern "C" SkPathEffect* C_SkCornerPathEffect_Make(SkScalar radius) {
    return SkCornerPathEffect::Make(radius).release();
}

//
// SkDashPathEffect
//

extern "C" SkPathEffect* C_SkDashPathEffect_Make(const SkScalar intervals[], int count, SkScalar phase) {
    return SkDashPathEffect::Make(intervals, count, phase).release();
}

//
// SkDiscretePathEffect
//

extern "C" SkPathEffect* C_SkDiscretePathEffect_Make(SkScalar segLength, SkScalar dev, uint32_t seedAssist) {
    return SkDiscretePathEffect::Make(segLength, dev, seedAssist).release();
}

//
// GrBackendFormat
//

extern "C" void C_GrBackendFormat_destruct(GrBackendFormat* self) {
    self->~GrBackendFormat();
}

extern "C" bool C_GrBackendFormat_Equals(const GrBackendFormat* lhs, const GrBackendFormat* rhs) {
    return *lhs == *rhs;
}

//
// GrBackendRenderTarget
//

extern "C" void C_GrBackendRenderTarget_destruct(GrBackendRenderTarget* self) {
    self->~GrBackendRenderTarget();
}

extern "C" GrBackendApi C_GrBackendRenderTarget_backend(const GrBackendRenderTarget* self) {
    return self->backend();
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
// GrGLInterface
//

extern "C" const GrGLInterface* C_GrGLInterface_MakeNativeInterface() {
    return GrGLMakeNativeInterface().release();
}

//
// GrContext
//

extern "C" GrContext* C_GrContext_MakeGL(const GrGLInterface* interface) {
    if (interface)
        return GrContext::MakeGL(sk_sp<const GrGLInterface>(interface)).release();
    else
        return GrContext::MakeGL().release();
}

extern "C" bool C_GrContext_colorTypeSupportedAsSurface(const GrContext* self, SkColorType colorType) {
    return self->colorTypeSupportedAsSurface(colorType);
}

#if defined(SK_VULKAN)

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

extern "C" GrContext* C_GrContext_MakeVulkan(const GrVkBackendContext* vkBackendContext) {
    return GrContext::MakeVulkan(*vkBackendContext).release();
}

//
// GrVkTypes.h
//

extern "C" void C_GrVkAlloc_Construct(GrVkAlloc* uninitialized, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, uint32_t flags) {
    new (uninitialized) GrVkAlloc(memory, offset, size, flags);
}

extern "C" bool C_GrVkAlloc_Equals(const GrVkAlloc* lhs, const GrVkAlloc* rhs) {
    return *lhs == *rhs;
}

extern "C" void C_GrVkYcbcrConversionInfo_Construct(
        GrVkYcbcrConversionInfo* uninitialized,
        VkSamplerYcbcrModelConversion ycbcrModel,
        VkSamplerYcbcrRange ycbcrRange,
        VkChromaLocation xChromaOffset,
        VkChromaLocation yChromaOffset,
        VkFilter chromaFilter,
        VkBool32 forceExplicitReconstruction,
        uint64_t externalFormat,
        VkFormatFeatureFlags externalFormatFeatures) {
    new (uninitialized) GrVkYcbcrConversionInfo(ycbcrModel, ycbcrRange, xChromaOffset, yChromaOffset, chromaFilter, forceExplicitReconstruction, externalFormat, externalFormatFeatures);
}

extern "C" bool C_GrVkYcbcrConversionInfo_Equals(const GrVkYcbcrConversionInfo* lhs, const GrVkYcbcrConversionInfo* rhs) {
    return *lhs == *rhs;
}

extern "C" void C_GrVkImageInfo_Construct(GrVkImageInfo* uninitialized,
                VkImage image, const GrVkAlloc* alloc, VkImageTiling imageTiling, VkImageLayout layout,
                VkFormat format, uint32_t levelCount,
                uint32_t currentQueueFamily,
                const GrVkYcbcrConversionInfo* ycbcrConversionInfo) {
    new (uninitialized) GrVkImageInfo(image, *alloc, imageTiling, layout, format, levelCount, currentQueueFamily, *ycbcrConversionInfo);
}

extern "C" void C_GrVkImageInfo_updateImageLayout(GrVkImageInfo* self, VkImageLayout layout) {
    self->updateImageLayout(layout);
}

extern "C" bool C_GrVkImageInfo_Equals(const GrVkImageInfo* lhs, const GrVkImageInfo* rhs) {
    return *lhs == *rhs;
}

#endif
