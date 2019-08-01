// codec/
#include "include/codec/SkEncodedOrigin.h"
// core/
#include "include/core/SkAnnotation.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkColor.h"
#include "include/core/SkColorFilter.h"
#include "include/core/SkContourMeasure.h"
#include "include/core/SkCubicMap.h"
#include "include/core/SkDataTable.h"
#include "include/core/SkDeferredDisplayListRecorder.h"
#include "include/core/SkDrawLooper.h"
#include "include/core/SkDrawable.h"
#include "include/core/SkDocument.h"
#include "include/core/SkFlattenable.h"
#include "include/core/SkFont.h"
#include "include/core/SkFontArguments.h"
#include "include/core/SkFontMetrics.h"
#include "include/core/SkFontMgr.h"
#include "include/core/SkGraphics.h"
#include "include/core/SkImage.h"
#include "include/core/SkImageEncoder.h"
#include "include/core/SkImageFilter.h"
#include "include/core/SkImageGenerator.h"
#include "include/core/SkImageInfo.h"
#include "include/core/SkMaskFilter.h"
#include "include/core/SkMultiPictureDraw.h"
#include "include/core/SkPaint.h"
#include "include/core/SkPath.h"
#include "include/core/SkPathMeasure.h"
#include "include/core/SkPicture.h"
#include "include/core/SkPictureRecorder.h"
#include "include/core/SkPixelRef.h"
#include "include/core/SkPoint3.h"
#include "include/core/SkRect.h"
#include "include/core/SkRegion.h"
#include "include/core/SkRRect.h"
#include "include/core/SkRSXform.h"
#include "include/core/SkStream.h"
#include "include/core/SkStrokeRec.h"
#include "include/core/SkSurface.h"
#include "include/core/SkSurfaceCharacterization.h"
#include "include/core/SkSwizzle.h"
#include "include/core/SkTextBlob.h"
#include "include/core/SkTypeface.h"
#include "include/core/SkTypes.h"
#include "include/core/SkYUVAIndex.h"
#include "include/core/SkYUVASizeInfo.h"
// docs/
#include "include/docs/SkPDFDocument.h"
// effects/
#include "include/effects/Sk1DPathEffect.h"
#include "include/effects/Sk2DPathEffect.h"
#include "include/effects/SkAlphaThresholdFilter.h"
#include "include/effects/SkArithmeticImageFilter.h"
#include "include/effects/SkBlurDrawLooper.h"
#include "include/effects/SkBlurImageFilter.h"
#include "include/effects/SkColorFilterImageFilter.h"
#include "include/effects/SkColorMatrix.h"
#include "include/effects/SkComposeImageFilter.h"
#include "include/effects/SkCornerPathEffect.h"
#include "include/effects/SkDashPathEffect.h"
#include "include/effects/SkDiscretePathEffect.h"
#include "include/effects/SkDisplacementMapEffect.h"
#include "include/effects/SkDropShadowImageFilter.h"
#include "include/effects/SkGradientShader.h"
#include "include/effects/SkImageSource.h"
#include "include/effects/SkLayerDrawLooper.h"
#include "include/effects/SkLightingImageFilter.h"
#include "include/effects/SkMagnifierImageFilter.h"
#include "include/effects/SkMatrixConvolutionImageFilter.h"
#include "include/effects/SkMergeImageFilter.h"
#include "include/effects/SkMorphologyImageFilter.h"
#include "include/effects/SkOffsetImageFilter.h"
#include "include/effects/SkPaintImageFilter.h"
#include "include/effects/SkPictureImageFilter.h"
#include "include/effects/SkPerlinNoiseShader.h"
#include "include/effects/SkTableColorFilter.h"
#include "include/effects/SkTileImageFilter.h"
#include "include/effects/SkXfermodeImageFilter.h"
// gpu/
#include "include/gpu/GrContext.h"
#include "include/gpu/GrBackendDrawableInfo.h"
// gpu/gl
#include "include/gpu/gl/GrGLInterface.h"
// pathops/
#include "include/pathops/SkPathOps.h"
// utils/
#include "include/utils/Sk3D.h"
#include "include/utils/SkCamera.h"
#include "include/utils/SkInterpolator.h"
#include "include/utils/SkNullCanvas.h"
#include "include/utils/SkParsePath.h"
#include "include/utils/SkShadowUtils.h"
#include "include/utils/SkTextUtils.h"

#if defined(SK_VULKAN)
#include "include/gpu/vk/GrVkVulkan.h"
#include "include/gpu/vk/GrVkTypes.h"
#include "include/gpu/vk/GrVkBackendContext.h"
#include "include/gpu/GrBackendSurface.h"
#endif

#if defined(SK_XML)
#include "include/svg/SkSVGCanvas.h"
#endif

template<typename T>
inline sk_sp<T> spFromConst(const T* pt) {
    return sk_sp<T>(const_cast<T*>(pt));
}

//
// codec/SkEncodedOrigin.h
//

extern "C" void C_SkEncodedOriginToMatrix(SkEncodedOrigin origin, int w, int h, SkMatrix* matrix) {
    *matrix = SkEncodedOriginToMatrix(origin, w, h);
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

extern "C" SkSurface* C_SkSurface_MakeRenderTarget2(
        GrContext* context,
        const SkSurfaceCharacterization& characterization,
        SkBudgeted budgeted) {
    return SkSurface::MakeRenderTarget(
            context,
            characterization,
            budgeted).release();
}

extern "C" SkSurface *C_SkSurface_MakeFromBackendTexture2(
        GrContext *context,
        const SkSurfaceCharacterization &characterization,
        const GrBackendTexture *backendTexture) {
    return SkSurface::MakeFromBackendTexture(
            context,
            characterization,
            *backendTexture).release();
}

extern "C" SkSurface* C_SkSurface_MakeNull(int width, int height) {
    return SkSurface::MakeNull(width, height).release();
}

extern "C" void C_SkSurface_imageInfo(SkSurface* self, SkImageInfo* info) {
    *info = self->imageInfo();
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

extern "C" SkSurface *C_SkSurface_makeSurface2(
        SkSurface *self,
        int width, int height) {
    return self->makeSurface(width, height).release();
}

//
// core/SkSurfaceCharacterization.h
//

extern "C" void C_SkSurfaceCharacterization_destruct(SkSurfaceCharacterization* self) {
    self->~SkSurfaceCharacterization();
}

extern "C" bool C_SkSurfaceCharacterization_equals(const SkSurfaceCharacterization* self, const SkSurfaceCharacterization* rhs) {
    return *self == *rhs;
}

extern "C" const SkImageInfo* C_SkSurfaceCharacterization_imageInfo(const SkSurfaceCharacterization* self) {
    return &self->imageInfo();
}

//
// SkImage
//

extern "C" SkImage* C_SkImage_MakeRasterData(const SkImageInfo* info, const SkData* pixels, size_t rowBytes) {
    return SkImage::MakeRasterData(*info, spFromConst(pixels), rowBytes).release();
}

extern "C" SkImage* C_SkImage_MakeFromBitmap(const SkBitmap* bitmap) {
    return SkImage::MakeFromBitmap(*bitmap).release();
}

extern "C" SkImage* C_SkImage_MakeFromGenerator(SkImageGenerator* imageGenerator, const SkIRect* subset) {
    return SkImage::MakeFromGenerator(std::unique_ptr<SkImageGenerator>(imageGenerator), subset).release();
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

extern "C" SkShader* C_SkImage_makeShader(const SkImage* self, SkTileMode tileMode1, SkTileMode tileMode2, const SkMatrix* localMatrix) {
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

extern "C" SkImage *C_SkImage_makeWithFilter(const SkImage *self, GrContext *context,
                                             const SkImageFilter *filter, const SkIRect *subset,
                                             const SkIRect *clipBounds, SkIRect *outSubset,
                                             SkIPoint *offset) {
    return self->makeWithFilter(context, filter, *subset, *clipBounds, outSubset, offset).release();
}

extern "C" SkImage* C_SkImage_makeColorSpace(const SkImage* self, const SkColorSpace* target) {
    return self->makeColorSpace(spFromConst(target)).release();
}

//
// core/SkImageEncoder.h
//

extern "C" SkData *C_SkEncodePixmap(const SkPixmap *src, SkEncodedImageFormat format, int quality) {
    return SkEncodePixmap(*src, format, quality).release();
}

extern "C" SkData *C_SkEncodeBitmap(const SkBitmap *src, SkEncodedImageFormat format, int quality) {
    return SkEncodeBitmap(*src, format, quality).release();
}

//
// core/SkData.h
//

extern "C" void C_SkData_ref(const SkData* self) {
    self->ref();
}

extern "C" void C_SkData_unref(const SkData* self) {
    self->unref();
}

extern "C" bool C_SkData_unique(const SkData* self) {
    return self->unique();
}

extern "C" SkData* C_SkData_MakeWithCopy(const void* data, size_t length) {
    return SkData::MakeWithCopy(data, length).release();
}

extern "C" SkData* C_SkData_MakeSubset(const SkData* src, size_t offset, size_t length) {
    return SkData::MakeSubset(src, offset, length).release();
}

extern "C" SkData* C_SkData_MakeUninitialized(size_t length) {
    return SkData:: MakeUninitialized(length).release();
}

extern "C" SkData* C_SkData_MakeWithCString(const char* cstr) {
    return SkData::MakeWithCString(cstr).release();
}

extern "C" SkData* C_SkData_MakeWithoutCopy(const void* data, size_t length) {
    return SkData::MakeWithoutCopy(data, length).release();
}

extern "C" SkData* C_SkData_MakeEmpty() {
    return SkData::MakeEmpty().release();
}

//
// core/SkMultiPictureDraw.h
//

extern "C" void C_SkMultiPictureDraw_destruct(SkMultiPictureDraw* self) {
    self->~SkMultiPictureDraw();
}

//
// core/SkPaint.h
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

extern "C" void C_SkPaint_setImageFilter(SkPaint* self, const SkImageFilter* imageFilter) {
    self->setImageFilter(spFromConst(imageFilter));
}

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

extern "C" bool C_SkPath_isValid(const SkPath* self) {
    return self->isValid();
}

extern "C" void C_SkPath_Iter_destruct(SkPath::Iter* self) {
    self->~Iter();
}

extern "C" bool C_SkPath_Iter_isCloseLine(const SkPath::Iter* self) {
    return self->isCloseLine();
}

extern "C" void C_SkPath_RawIter_destruct(SkPath::RawIter* self) {
    self->~RawIter();
}

//
// SkPathMeasure
//

extern "C" void C_SkPathMeasure_destruct(const SkPathMeasure* self) {
    self->~SkPathMeasure();
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

extern "C" bool C_SkImageInfo_Equals(const SkImageInfo* lhs, const SkImageInfo* rhs) {
    return *lhs == *rhs;
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

extern "C" bool C_SkImageInfo_gammaCloseToSRGB(const SkImageInfo* self) {
    return self->gammaCloseToSRGB();
}

extern "C" void C_SkImageInfo_reset(SkImageInfo* self) {
    self->reset();
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

extern "C" bool C_SkColorSpace_unique(const SkColorSpace* self) {
    return self->unique();
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

extern "C" void C_SkMatrix44_ConstructIdentity(SkMatrix44* uninitialized) {
    new(uninitialized) SkMatrix44(SkMatrix44::kIdentity_Constructor);
}

extern "C" void C_SkMatrix44_ConstructNaN(SkMatrix44* uninitialized) {
    new(uninitialized) SkMatrix44(SkMatrix44::kNaN_Constructor);
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
    return &((*self)[static_cast<int>(index)]);
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

extern "C" void C_SkBitmap_setPixelRef(SkBitmap* self, const SkPixelRef* pixelRef, int dx, int dy) {
    self->setPixelRef(spFromConst(pixelRef), dx, dy);
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

extern "C" SkShader* C_SkBitmap_makeShader(const SkBitmap* self, SkTileMode tmx, SkTileMode tmy, const SkMatrix* localMatrix) {
    return self->makeShader(tmx, tmy, localMatrix).release();
}

//
// SkPicture
//

extern "C" SkPicture* C_SkPicture_MakeFromData(const SkData* data) {
    return SkPicture::MakeFromData(data).release();
}

extern "C" SkPicture* C_SkPicture_MakeFromData2(const void* data, size_t size) {
    return SkPicture::MakeFromData(data, size).release();
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

extern "C" SkShader* C_SkPicture_makeShader(const SkPicture* self, SkTileMode tmx, SkTileMode tmy, const SkMatrix* localMatrix, const SkRect* tileRect) {
    return self->makeShader(tmx, tmy, localMatrix, tileRect).release();
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

extern "C" void C_SkRegion_Iterator_Construct(SkRegion::Iterator* uninitialized) {
    new(uninitialized) SkRegion::Iterator();
}

extern "C" void C_SkRegion_Iterator_destruct(SkRegion::Iterator* self) {
    self->~Iterator();
}

extern "C" const SkRegion* C_SkRegion_Iterator_rgn(const SkRegion::Iterator* self) {
    return self->rgn();
}

extern "C" void C_SkRegion_Cliperator_destruct(SkRegion::Cliperator* self) {
    self->~Cliperator();
}

extern "C" void C_SkRegion_Spanerator_destruct(SkRegion::Spanerator* self) {
    self->~Spanerator();
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

extern "C" bool C_SkTextBlob_unique(const SkTextBlob* self) {
    return self->unique();
}

extern "C" SkTextBlob* C_SkTextBlob_MakeFromText(const void* text, size_t byteLength, const SkFont* font, SkTextEncoding encoding) {
    return SkTextBlob::MakeFromText(text, byteLength, *font, encoding).release();
}

extern "C" SkTextBlob *C_SkTextBlob_MakeFromPosTextH(const void *text, size_t byteLength,
                                                     const SkScalar xpos[], SkScalar constY, const SkFont *font,
                                                     SkTextEncoding encoding) {
    return SkTextBlob::MakeFromPosTextH(text, byteLength, xpos, constY, *font, encoding).release();
}

extern "C" SkTextBlob *C_SkTextBlob_MakeFromPosText(const void *text, size_t byteLength,
                                                    const SkPoint pos[],
                                                    const SkFont *font,
                                                    SkTextEncoding encoding) {
    return SkTextBlob::MakeFromPosText(text, byteLength, pos, *font, encoding).release();
}

extern "C" SkTextBlob *C_SkTextBlob_MakeFromRSXform(const void *text, size_t byteLength,
                                                    const SkRSXform xform[],
                                                    const SkFont *font,
                                                    SkTextEncoding encoding) {
    return SkTextBlob::MakeFromRSXform(text, byteLength, xform, *font, encoding).release();
}


extern "C" void C_SkTextBlobBuilder_destruct(SkTextBlobBuilder* self) {
    self->~SkTextBlobBuilder();
}

extern "C" SkTextBlob* C_SkTextBlobBuilder_make(SkTextBlobBuilder* self) {
    return self->make().release();
}

//
// core/SkTypeface.h
//

extern "C" bool C_SkTypeface_isBold(const SkTypeface* self) {
    return self->isBold();
}

extern "C" bool C_SkTypeface_isItalic(const SkTypeface* self) {
    return self->isItalic();
}

extern "C" SkTypeface* C_SkTypeface_MakeDefault() {
    return SkTypeface::MakeDefault().release();
}

extern "C" SkTypeface* C_SkTypeface_MakeFromName(const char familyName[], SkFontStyle fontStyle) {
    return SkTypeface::MakeFromName(familyName, fontStyle).release();
}

/*
extern "C" SkTypeface* C_SkTypeface_MakeFromFile(const char path[], int index) {
    return SkTypeface::MakeFromFile(path, index).release();
}
*/

extern "C" SkTypeface* C_SkTypeface_MakeFromData(const SkData* data, int index) {
    return SkTypeface::MakeFromData(sk_sp<SkData>(const_cast<SkData*>(data)), index).release();
}

extern "C" SkTypeface* C_SkTypeface_makeClone(const SkTypeface* self, const SkFontArguments* arguments) {
    return self->makeClone(*arguments).release();
}

extern "C" SkData* C_SkTypeface_serialize(const SkTypeface* self, SkTypeface::SerializeBehavior behavior) {
    return self->serialize(behavior).release();
}

extern "C" SkTypeface* C_SkTypeface_MakeDeserialize(SkStream* stream) {
    return SkTypeface::MakeDeserialize(stream).release();
}

extern "C" SkData* C_SkTypeface_copyTableData(const SkTypeface* self, SkFontTableTag tag) {
    return self->copyTableData(tag).release();
}

extern "C" void C_SkTypeface_LocalizedStrings_unref(SkTypeface::LocalizedStrings* self) {
    self->unref();
}

extern "C" bool C_SkTypeface_LocalizedStrings_next(SkTypeface::LocalizedStrings* self, SkString* string, SkString* language) {
    auto ls = SkTypeface::LocalizedString();
    if (self->next(&ls)) {
        *string = ls.fString;
        *language = ls.fLanguage;
        return true;
    }
    return false;
}

//
// core/SkYUVASizeInfo.h
//

extern "C" bool C_SkYUVASizeInfo_equals(const SkYUVASizeInfo* l, const SkYUVASizeInfo* r) {
    return *l == *r;
}

extern "C" size_t C_SkYUVASizeInfo_computeTotalBytes(const SkYUVASizeInfo* self) {
    return self->computeTotalBytes();
}

//
// core/SkFlattenable.h
//

extern "C" const char* C_SkFlattenable_getTypeName(const SkFlattenable* self) {
    return self->getTypeName();
}

extern "C" SkData* C_SkFlattenable_serialize(const SkFlattenable* self) {
    return self->serialize().release();
}

//
// core/SkFont.h
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
// core/SkFontArguments.h
//

extern "C" void C_SkFontArguments_construct(SkFontArguments* uninitialized) {
    new(uninitialized) SkFontArguments();
}

extern "C" void C_SkFontArguments_destruct(SkFontArguments* self) {
    self->~SkFontArguments();
}

extern "C" void C_SkFontArguments_setCollectionIndex(SkFontArguments* self, int collectionIndex) {
    self->setCollectionIndex(collectionIndex);
}

extern "C" void C_SkFontArguments_setVariationDesignPosition(SkFontArguments* self, SkFontArguments::VariationPosition position) {
    self->setVariationDesignPosition(position);
}

extern "C" SkFontArguments::VariationPosition
C_SkFontArguments_getVariationDesignPosition(const SkFontArguments *self) {
    return self->getVariationDesignPosition();
}

//
// core/SkFontMgr.h
//

extern "C" int C_SkFontStyleSet_count(SkFontStyleSet* self) {
    return self->count();
}

extern "C" void C_SkFontStyleSet_getStyle(SkFontStyleSet* self, int index, SkFontStyle* fontStyle, SkString* style) {
    self->getStyle(index, fontStyle, style);
}

extern "C" SkTypeface* C_SkFontStyleSet_createTypeface(SkFontStyleSet* self, int index) {
    return self->createTypeface(index);
}

extern "C" SkTypeface* C_SkFontStyleSet_matchStyle(SkFontStyleSet* self, const SkFontStyle* pattern) {
    return self->matchStyle(*pattern);
}

// note: this function _consumes_ / deletes the stream.
extern "C" SkTypeface* C_SkFontMgr_makeFromStream(const SkFontMgr* self, SkStreamAsset* stream, int ttcIndex) {
    return self->makeFromStream(std::unique_ptr<SkStreamAsset>(stream), ttcIndex).release();
}

extern "C" SkFontMgr* C_SkFontMgr_RefDefault() {
    return SkFontMgr::RefDefault().release();
}

//
// core/SkFontParameters.h
//

extern "C" bool C_SkFontParameters_Variation_Axis_isHidden(const SkFontParameters::Variation::Axis* self) {
    return self->isHidden();
}

extern "C" void C_SkFontParameters_Variation_Axis_setHidden(SkFontParameters::Variation::Axis* self, bool hidden) {
    self->setHidden(hidden);
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

extern "C" bool C_SkVertices_unique(const SkVertices* self) {
    return self->unique();
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
// SkVertices::Bone
//

extern "C" SkRect C_SkVertices_Bone_mapRect(const SkVertices::Bone* self, const SkRect* rect) {
    return self->mapRect(*rect);
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

extern "C" SkDrawable* C_SkPictureRecorder_finishRecordingAsDrawable(SkPictureRecorder* self) {
    return self->finishRecordingAsDrawable().release();
}

//
// SkColorFilter
//

extern "C" SkColorFilter* C_SkColorFilter_makeComposed(const SkColorFilter* self, const SkColorFilter* inner) {
    return self->makeComposed(spFromConst(inner)).release();
}

extern "C" bool C_SkColorFilter_asAColorMode(const SkColorFilter* self, SkColor* color, SkBlendMode* mode) {
    return self->asAColorMode(color, mode);
}

extern "C" bool C_SkColorFilter_asAColorMatrix(const SkColorFilter* self, SkScalar matrix[20]) {
    return self->asAColorMatrix(matrix);
}

extern "C" uint32_t C_SkColorFilter_getFlags(const SkColorFilter* self) {
    return self->getFlags();
}

extern "C" SkColorFilter* C_SkColorFilter_Deserialize(const void* data, size_t size) {
    return SkColorFilter::Deserialize(data, size).release();
}

//
// SkColorFilters
//

extern "C" SkColorFilter* C_SkColorFilters_Compose(const SkColorFilter* outer, const SkColorFilter* inner) {
    return SkColorFilters::Compose(spFromConst(outer), spFromConst(inner)).release();
}

extern "C" SkColorFilter* C_SkColorFilters_Blend(const SkColor c, SkBlendMode blendMode) {
    return SkColorFilters::Blend(c, blendMode).release();
}


extern "C" SkColorFilter* C_SkColorFilters_Matrix(const SkColorMatrix* colorMatrix) {
    return SkColorFilters::Matrix(*colorMatrix).release();
}

extern "C" SkColorFilter* C_SkColorFilters_MatrixRowMajor(const SkScalar array[20]) {
    return SkColorFilters::Matrix(array).release();
}

extern "C" SkColorFilter* C_SkColorFilters_LinearToSRGBGamma() {
    return SkColorFilters::LinearToSRGBGamma().release();
}

extern "C" SkColorFilter* C_SkColorFilters_SRGBToLinearGamma() {
    return SkColorFilters::SRGBToLinearGamma().release();
}

extern "C" SkColorFilter* C_SkColorFilters_Lerp(float t, const SkColorFilter* dst, const SkColorFilter* src) {
    return SkColorFilters::Lerp(t, spFromConst(dst), spFromConst(src)).release();
}

//
// SkContourMeasureIter
//

extern "C" void C_SkContourMeasureIter_destruct(SkContourMeasureIter* self) {
    self->~SkContourMeasureIter();
}

extern "C" SkContourMeasure* C_SkContourMeasureIter_next(SkContourMeasureIter* self) {
    return self->next().release();
}

//
// core/SkDataTable.h
//

extern "C" SkDataTable *C_SkDataTable_MakeEmpty() {
    return SkDataTable::MakeEmpty().release();
}

extern "C" SkDataTable *C_SkDataTable_MakeCopyArrays(const void *const *ptrs,
                                                     const size_t *sizes, int count) {
    return SkDataTable::MakeCopyArrays(ptrs, sizes, count).release();
}

extern "C" SkDataTable *C_SkDataTable_MakeCopyArray(const void *array, size_t elemSize, int count) {
    return SkDataTable::MakeCopyArray(array, elemSize, count).release();
}

//
// core/SkDeferredDisplayListRecorder.h
//

extern "C" void C_SkDeferredDisplayListRecorder_destruct(SkDeferredDisplayListRecorder* self) {
    self->~SkDeferredDisplayListRecorder();
}

extern "C" SkDeferredDisplayList* C_SkDeferredDisplayListRecorder_detach(SkDeferredDisplayListRecorder* self) {
    return self->detach().release();
}

extern "C" void C_SkDeferredDisplayList_delete(SkDeferredDisplayList* self) {
    delete self;
}

//
// core/SkDrawLooper.h
//

extern "C" bool C_SkDrawLooper_asABlurShadow(const SkDrawLooper* self, SkDrawLooper::BlurShadowRec& br) {
    return self->asABlurShadow(&br);
}

extern "C" SkDrawLooper* C_SkDrawLooper_Deserialize(const void* data, size_t length) {
    return SkDrawLooper::Deserialize(data, length).release();
}

//
// core/SkDrawable.h
//

extern "C" SkDrawable::GpuDrawHandler *C_SkDrawable_snapGpuDrawHandler(SkDrawable *self, GrBackendApi backendApi,
                                                                       const SkMatrix *matrix,
                                                                       const SkIRect *clipBounds,
                                                                       const SkImageInfo *bufferInfo) {
    return self->snapGpuDrawHandler(backendApi, *matrix, *clipBounds, *bufferInfo).release();
}

extern "C" SkDrawable* C_SkDrawable_Deserialize(const void* data, size_t length) {
    return SkDrawable::Deserialize(data, length).release();
}

extern "C" void C_SkDrawable_GpuDrawHandler_destruct(SkDrawable::GpuDrawHandler *self) {
    self->~GpuDrawHandler();
}

extern "C" void C_SkDrawable_GpuDrawHandler_draw(SkDrawable::GpuDrawHandler *self, const GrBackendDrawableInfo *info) {
    self->draw(*info);
}

//
// SkImageFilter
//

extern "C" SkRect C_SkImageFilter_computeFastBounds(const SkImageFilter* self, const SkRect* bounds) {
    return self->computeFastBounds(*bounds);
}

extern "C" SkImageFilter* C_SkImageFilter_makeWithLocalMatrix(const SkImageFilter* self, const SkMatrix* matrix) {
    return self->makeWithLocalMatrix(*matrix).release();
}

extern "C" SkImageFilter* C_SkImageFilter_MakeMatrixFilter(const SkMatrix* matrix, SkFilterQuality quality, const SkImageFilter* input) {
    return SkImageFilter::MakeMatrixFilter(*matrix, quality, spFromConst(input)).release();
}

extern "C" SkImageFilter* C_SkImageFilter_Deserialize(const void* data, size_t length) {
    return SkImageFilter::Deserialize(data, length).release();
}

//
// SkImageGenerator
//

extern "C" void C_SkImageGenerator_delete(SkImageGenerator *self) {
    delete self;
}

extern "C" SkData *C_SkImageGenerator_refEncodedData(SkImageGenerator *self) {
    return self->refEncodedData().release();
}

extern "C" SkImageGenerator *C_SkImageGenerator_MakeFromEncoded(const SkData *data) {
    return SkImageGenerator::MakeFromEncoded(spFromConst(data)).release();
}

extern "C" SkImageGenerator *C_SkImageGenerator_MakeFromPicture(
        const SkISize *size,
        const SkPicture *picture,
        const SkMatrix *matrix,
        const SkPaint *paint,
        SkImage::BitDepth bd,
        const SkColorSpace *cs) {
    return SkImageGenerator::MakeFromPicture(
            *size,
            spFromConst(picture),
            matrix,
            paint,
            bd,
            spFromConst(cs)).release();
}

//
// SkString
//

extern "C" void C_SkString_destruct(SkString* self) {
    self->~SkString();
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

extern "C" SkPathEffect* C_SkPathEffect_Deserialize(const void* data, size_t length) {
    return SkPathEffect::Deserialize(data, length).release();
}

//
// SkPixmap
//

extern "C" void C_SkPixmap_destruct(SkPixmap* self) {
    self->~SkPixmap();
}

extern "C" void C_SkPixmap_setColorSpace(SkPixmap* self, const SkColorSpace* colorSpace) {
    self->setColorSpace(spFromConst(colorSpace));
}

extern "C" SkISize C_SkPixmap_dimensions(const SkPixmap *self) {
    return self->dimensions();
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

extern "C" SkMaskFilter* C_SkMaskFilter_Deserialize(const void* data, size_t length) {
    return SkMaskFilter::Deserialize(data, length).release();
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

extern "C" SkShader* C_SkShaders_Empty() {
    return SkShaders::Empty().release();
}

extern "C" SkShader* C_SkShaders_Color(SkColor color) {
    return SkShaders::Color(color).release();
}

extern "C" SkShader* C_SkShaders_Color2(const SkColor4f* color, const SkColorSpace* colorSpace) {
    return SkShaders::Color(*color, spFromConst(colorSpace)).release();
}

extern "C" SkShader* C_SkShaders_Blend(SkBlendMode mode, const SkShader* dst, const SkShader* src, const SkMatrix* localMatrix) {
    return SkShaders::Blend(mode, spFromConst(dst), spFromConst(src), localMatrix).release();
}

extern "C" SkShader* C_SkShaders_Lerp(float t, const SkShader* dst, const SkShader* src, const SkMatrix* localMatrix) {
    return SkShaders::Lerp(t, spFromConst(dst), spFromConst(src), localMatrix).release();
}

extern "C" SkShader* C_SkShaders_Lerp2(const SkShader* red, const SkShader* dst, const SkShader* src, const SkMatrix* localMatrix) {
    return SkShaders::Lerp(spFromConst(red), spFromConst(dst), spFromConst(src), localMatrix).release();
}

extern "C" SkShader* C_SkShader_Deserialize(const void* data, size_t length) {
    // note: dynamic_cast may lead to a linker error here on iOS x86_64
    // https://github.com/rust-skia/rust-skia/issues/146
    // "typeinfo for SkShader", referenced from:
    //      _C_SkShader_Deserialize in libcanvasnative.a(bindings.o)
    return (SkShader*)(SkShader::Deserialize(SkFlattenable::Type::kSkShaderBase_Type, data, length).release());
}

//
// SkStream
//

extern "C" void C_SkStream_delete(SkStream* stream) {
    delete stream;
}

//
// SkWStream
//

extern "C" void C_SkWStream_destruct(SkWStream* self) {
    self->~SkWStream();
}

extern "C" bool C_SkWStream_write(SkWStream* self, const void* buffer, size_t size) {
    return self->write(buffer, size);
}

//
// SkMemoryStream: public SkStreamMemory
//

extern "C" SkMemoryStream* C_SkMemoryStream_MakeDirect(const void* data, size_t length) {
    return SkMemoryStream::MakeDirect(data, length).release();
}

//
// SkDynamicMemoryWStream : public SkWStream
//

extern "C" void C_SkDynamicMemoryWStream_Construct(SkDynamicMemoryWStream* uninitialized) {
    new(uninitialized) SkDynamicMemoryWStream();
}

extern "C" SkData* C_SkDynamicMemoryWStream_detachAsData(SkDynamicMemoryWStream* self) {
    return self->detachAsData().release();
}

extern "C" SkStreamAsset* C_SkDynamicMemoryWStream_detachAsStream(SkDynamicMemoryWStream* self) {
    return self->detachAsStream().release();
}

//
// SkGradientShader
//

extern "C" SkShader* C_SkGradientShader_MakeLinear(const SkPoint pts[2], const SkColor colors[], const SkScalar pos[], int count, SkTileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeLinear(pts, colors, pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeLinear2(const SkPoint pts[2], const SkColor4f colors[], const SkColorSpace* colorSpace, const SkScalar pos[], int count, SkTileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeLinear(pts, colors, spFromConst(colorSpace), pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeRadial(const SkPoint* center, SkScalar radius, const SkColor colors[], const SkScalar pos[], int count, SkTileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeRadial(*center, radius, colors, pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeRadial2(const SkPoint* center, SkScalar radius, const SkColor4f colors[], const SkColorSpace* colorSpace, const SkScalar pos[], int count, SkTileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeRadial(*center, radius, colors, spFromConst(colorSpace), pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeTwoPointConical(const SkPoint* start, SkScalar startRadius, const SkPoint* end, SkScalar endRadius, const SkColor colors[], const SkScalar pos[], int count, SkTileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeTwoPointConical(*start, startRadius, *end, endRadius, colors, pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeTwoPointConical2(const SkPoint* start, SkScalar startRadius, const SkPoint* end, SkScalar endRadius, const SkColor4f colors[], const SkColorSpace* colorSpace, const SkScalar pos[], int count, SkTileMode mode, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeTwoPointConical(*start, startRadius, *end, endRadius, colors, spFromConst(colorSpace), pos, count, mode, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeSweep(SkScalar cx, SkScalar cy, const SkColor colors[], const SkScalar pos[], int count, SkTileMode mode, SkScalar startAngle, SkScalar endAngle, uint32_t flags, const SkMatrix* localMatrix) {
    return SkGradientShader::MakeSweep(cx, cy, colors, pos, count, mode, startAngle, endAngle, flags, localMatrix).release();
}

extern "C" SkShader* C_SkGradientShader_MakeSweep2(SkScalar cx, SkScalar cy, const SkColor4f colors[], const SkColorSpace* colorSpace, const SkScalar pos[], int count, SkTileMode mode, SkScalar startAngle, SkScalar endAngle, uint32_t flags, const SkMatrix* localMatrix) {
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
// effects/SkPath1DPathEffect
//

extern "C" SkPathEffect* C_SkPath1DPathEffect_Make(const SkPath* path, SkScalar advance, SkScalar phase, SkPath1DPathEffect::Style style) {
    return SkPath1DPathEffect::Make(*path, advance, phase, style).release();
}

//
// effects/SkLine2DPathEffect
//

extern "C" SkPathEffect* C_SkLine2DPathEffect_Make(SkScalar width, const SkMatrix* matrix) {
    return SkLine2DPathEffect::Make(width, *matrix).release();
}

//
// effects/SkPath2DPathEffect
//

extern "C" SkPathEffect* C_SkPath2DPathEffect_Make(const SkMatrix* matrix, const SkPath* path) {
    return SkPath2DPathEffect::Make(*matrix, *path).release();
}

//
// effects/SkAlphaThresholdFilter
//

extern "C" SkImageFilter *
C_SkAlphaThresholdFilter_Make(const SkRegion &region, SkScalar innerMin, SkScalar outerMax, const SkImageFilter &input,
                              const SkImageFilter::CropRect *cropRect) {
    return SkAlphaThresholdFilter::Make(region, innerMin, outerMax, spFromConst(&input), cropRect).release();
}

//
// effects/SkArithmeticImageFilter
//

extern "C" SkImageFilter *C_SkArithmeticImageFilter_Make(float k1, float k2, float k3, float k4, bool enforcePMColor,
                                                         const SkImageFilter &background,
                                                         const SkImageFilter &foreground,
                                                         const SkImageFilter::CropRect *cropRect) {
    return SkArithmeticImageFilter::Make(k1, k2, k3, k4, enforcePMColor, spFromConst(&background),
                                         spFromConst(&foreground), cropRect).release();
}

//
// effects/SkBlurDrawLooper
//

extern "C" SkDrawLooper* C_SkBlurDrawLooper_Make(SkColor color, SkScalar sigma, SkScalar dx, SkScalar dy) {
    return SkBlurDrawLooper::Make(color, sigma, dx, dy).release();
}

// note: SkColorSpace's ref count should not be increased before passing it here.
extern "C" SkDrawLooper* C_SkBlurDrawLooper_Make2(SkColor4f color, const SkColorSpace* cs, SkScalar sigma, SkScalar dx, SkScalar dy) {
    return SkBlurDrawLooper::Make(color, const_cast<SkColorSpace*>(cs), sigma, dx, dy).release();
}

//
// effects/SkBlurImageFilter
//

extern "C" SkImageFilter *C_SkBlurImageFilter_Make(SkScalar sigmaX, SkScalar sigmaY, const SkImageFilter &input,
                                                   const SkImageFilter::CropRect *cropRect,
                                                   SkBlurImageFilter::TileMode tileMode) {
    return SkBlurImageFilter::Make(sigmaX, sigmaY, spFromConst(&input), cropRect, tileMode).release();
}

//
// effects/SkColorFilterImageFilter
//

extern "C" SkImageFilter *C_SkColorFilterImageFilter_Make(const SkColorFilter &cf, const SkImageFilter &input,
                                                          const SkImageFilter::CropRect *cropRect) {
    return SkColorFilterImageFilter::Make(spFromConst(&cf), spFromConst(&input), cropRect).release();
}

//
// effects/SkColorMatrix.h
//

extern "C" bool C_SkColorMatrix_equals(const SkColorMatrix* lhs, const SkColorMatrix* rhs) {
    return *lhs == *rhs;
}

extern "C" float* C_SkColorMatrix_get20(const SkColorMatrix* self, float m[20]) {
    return self->get20(m);
}

extern "C" void C_SkColorMatrix_set20(SkColorMatrix* self, const float m[20]) {
    self->set20(m);
}

//
// effects/SkComposeImageFilter
//

extern "C" SkImageFilter *C_SkComposeImageFilter_Make(const SkImageFilter &outer, const SkImageFilter &inner) {
    return SkComposeImageFilter::Make(spFromConst(&outer), spFromConst(&inner)).release();
}

//
// effects/SkCornerPathEffect
//

extern "C" SkPathEffect* C_SkCornerPathEffect_Make(SkScalar radius) {
    return SkCornerPathEffect::Make(radius).release();
}

//
// effects/SkDashPathEffect
//

extern "C" SkPathEffect* C_SkDashPathEffect_Make(const SkScalar intervals[], int count, SkScalar phase) {
    return SkDashPathEffect::Make(intervals, count, phase).release();
}

//
// effects/SkDiscretePathEffect
//

extern "C" SkPathEffect* C_SkDiscretePathEffect_Make(SkScalar segLength, SkScalar dev, uint32_t seedAssist) {
    return SkDiscretePathEffect::Make(segLength, dev, seedAssist).release();
}

//
// effects/SkDisplacementMapEffect
//

extern "C" SkImageFilter *C_SkDisplacementMapEffect_Make(SkDisplacementMapEffect::ChannelSelectorType xChannelSelector,
                                                         SkDisplacementMapEffect::ChannelSelectorType yChannelSelector,
                                                         SkScalar scale, const SkImageFilter &displacement,
                                                         const SkImageFilter &color,
                                                         const SkImageFilter::CropRect *cropRect) {

    return SkDisplacementMapEffect::Make(xChannelSelector, yChannelSelector, scale, spFromConst(&displacement),
                                         spFromConst(&color), cropRect).release();
}

//
// effects/SkDropShadowImageFilter
//

extern "C" SkImageFilter *C_SkDropShadowImageFilter_Make(SkScalar dx, SkScalar dy, SkScalar sigmaX, SkScalar sigmaY,
                                                         SkColor color, SkDropShadowImageFilter::ShadowMode shadowMode,
                                                         const SkImageFilter &input,
                                                         const SkImageFilter::CropRect *cropRect) {
    return SkDropShadowImageFilter::Make(dx, dy, sigmaX, sigmaY, color, shadowMode, spFromConst(&input),
                                         cropRect).release();
}

//
// effects/SkImageSource
//

extern "C" SkImageFilter *C_SkImageSource_Make(const SkImage &image) {
    return SkImageSource::Make(spFromConst(&image)).release();
}

extern "C" SkImageFilter *
C_SkImageSource_Make2(const SkImage &image, const SkRect &srcRect, const SkRect &dstRect,
                      SkFilterQuality filterQuality) {
    return SkImageSource::Make(spFromConst(&image), srcRect, dstRect, filterQuality).release();
}

//
// effects/SkLayerDrawLooper
//

extern "C" void C_SkLayerDrawLooper_Builder_destruct(SkLayerDrawLooper::Builder* self) {
    self->~Builder();
}

extern "C" SkDrawLooper* C_SkLayerDrawLooper_Builder_detach(SkLayerDrawLooper::Builder* self) {
    return self->detach().release();
}

//
// effects/SkLightingImageFilter
//

extern "C" SkImageFilter *
C_SkLightingImageFilter_MakeDistantLitDiffuse(const SkPoint3 &direction, SkColor lightColor, SkScalar surfaceScale,
                                              SkScalar kd, const SkImageFilter &input,
                                              const SkImageFilter::CropRect *cropRect) {
    return SkLightingImageFilter::MakeDistantLitDiffuse(direction, lightColor, surfaceScale, kd, spFromConst(&input),
                                                        cropRect).release();
}

extern "C" SkImageFilter *
C_SkLightingImageFilter_MakePointLitDiffuse(const SkPoint3 &location, SkColor lightColor, SkScalar surfaceScale,
                                            SkScalar kd, const SkImageFilter &input,
                                            const SkImageFilter::CropRect *cropRect) {
    return SkLightingImageFilter::MakePointLitDiffuse(location, lightColor, surfaceScale, kd, spFromConst(&input),
                                                      cropRect).release();
}

extern "C" SkImageFilter *
C_SkLightingImageFilter_MakeSpotLitDiffuse(const SkPoint3 &location,
                                           const SkPoint3 &target, SkScalar specularExponent, SkScalar cutoffAngle,
                                           SkColor lightColor, SkScalar surfaceScale, SkScalar kd,
                                           const SkImageFilter *input, const SkImageFilter::CropRect *cropRect) {
    return SkLightingImageFilter::MakeSpotLitDiffuse(location, target, specularExponent, cutoffAngle, lightColor,
                                                     surfaceScale, kd, spFromConst(input), cropRect).release();
}

extern "C" SkImageFilter *
C_SkLightingImageFilter_MakeDistantLitSpecular(const SkPoint3 &direction,
                                               SkColor lightColor, SkScalar surfaceScale, SkScalar ks,
                                               SkScalar shininess, const SkImageFilter &input,
                                               const SkImageFilter::CropRect *cropRect) {
    return SkLightingImageFilter::MakeDistantLitSpecular(direction, lightColor, surfaceScale, ks, shininess,
                                                         spFromConst(&input), cropRect).release();
}

extern "C" SkImageFilter *
C_SkLightingImageFilter_MakePointLitSpecular(const SkPoint3 &location,
                                             SkColor lightColor, SkScalar surfaceScale, SkScalar ks,
                                             SkScalar shininess, const SkImageFilter &input,
                                             const SkImageFilter::CropRect *cropRect) {
    return SkLightingImageFilter::MakePointLitSpecular(location, lightColor, surfaceScale, ks, shininess,
                                                       spFromConst(&input), cropRect).release();
}

extern "C" SkImageFilter *
C_SkLightingImageFilter_MakeSpotLitSpecular(const SkPoint3 &location,
                                            const SkPoint3 &target, SkScalar specularExponent, SkScalar cutoffAngle,
                                            SkColor lightColor, SkScalar surfaceScale, SkScalar ks,
                                            SkScalar shininess, const SkImageFilter &input,
                                            const SkImageFilter::CropRect *cropRect) {
    return SkLightingImageFilter::MakeSpotLitSpecular(location, target, specularExponent, cutoffAngle, lightColor,
                                                      surfaceScale, ks, shininess, spFromConst(&input),
                                                      cropRect).release();
}

//
// effects/SkMagnifierImageFilter
//

extern "C" SkImageFilter *
C_SkMagnifierImageFilter_Make(const SkRect &srcRect, SkScalar inset, const SkImageFilter &input,
                              const SkImageFilter::CropRect *cropRect) {
    return SkMagnifierImageFilter::Make(srcRect, inset, spFromConst(&input), cropRect).release();
}

//
// effects/SkMatrixConvolutionImageFilter
//

extern "C" SkImageFilter *
C_SkMatrixConvolutionImageFilter_Make(const SkISize &kernelSize,
                                      const SkScalar *kernel,
                                      SkScalar gain,
                                      SkScalar bias,
                                      const SkIPoint &kernelOffset,
                                      SkMatrixConvolutionImageFilter::TileMode tileMode,
                                      bool convolveAlpha,
                                      const SkImageFilter &input,
                                      const SkImageFilter::CropRect *cropRect) {
    return SkMatrixConvolutionImageFilter::Make(kernelSize, kernel, gain, bias, kernelOffset, tileMode, convolveAlpha,
                                                spFromConst(&input), cropRect).release();
}

//
// effects/SkMergeImageFilter
//

extern "C" SkImageFilter *
C_SkMergeImageFilter_Make(const SkImageFilter *const filters[], int count, const SkImageFilter::CropRect *cropRect) {
    auto array = new sk_sp<SkImageFilter>[count];
    for (int i = 0; i < count; ++i) {
        array[i] = spFromConst(filters[i]);
    }
    auto imageFilter = SkMergeImageFilter::Make(array, count, cropRect).release();
    delete[] array;
    return imageFilter;
}

//
// effects/SkMorphologyImageFiter
//

extern "C" SkImageFilter *C_SkDilateImageFilter_Make(int radiusX, int radiusY, const SkImageFilter &input,
                                                     const SkImageFilter::CropRect *cropRect) {
    return SkDilateImageFilter::Make(radiusX, radiusY, spFromConst(&input), cropRect).release();
}

extern "C" SkImageFilter *C_SkErodeImageFilter_Make(int radiusX, int radiusY, const SkImageFilter &input,
                                                    const SkImageFilter::CropRect *cropRect) {
    return SkErodeImageFilter::Make(radiusX, radiusY, spFromConst(&input), cropRect).release();
}

//
// effects/SkOffsetImageFilter
//

extern "C" SkImageFilter *C_SkOffsetImageFilter_Make(SkScalar dx, SkScalar dy, const SkImageFilter &input,
                                                     const SkImageFilter::CropRect *cropRect) {
    return SkOffsetImageFilter::Make(dx, dy, spFromConst(&input), cropRect).release();
}

//
// effects/SkPaintImageFilter
//

extern "C" SkImageFilter *C_SkPaintImageFilter_Make(const SkPaint &paint, const SkImageFilter::CropRect *cropRect) {
    return SkPaintImageFilter::Make(paint, cropRect).release();
}

//
// effects/SkPictureImageFilter
//

extern "C" SkImageFilter *C_SkPictureImageFilter_Make(const SkPicture &picture, const SkRect *cropRect) {
    if (cropRect) {
        return SkPictureImageFilter::Make(spFromConst(&picture), *cropRect).release();
    } else {
        return SkPictureImageFilter::Make(spFromConst(&picture)).release();
    }
}

//
// effects/SkTableColorFilter
//

extern "C" SkColorFilter* C_SkTableColorFilter_Make(const uint8_t table[256]) {
    return SkTableColorFilter::Make(table).release();
}

extern "C" SkColorFilter* C_SkTableColorFilter_MakeARGB(const uint8_t tableA[256], const uint8_t tableR[256], const uint8_t tableG[256], const uint8_t tableB[256]) {
    return SkTableColorFilter::MakeARGB(tableA, tableR, tableG, tableB).release();
}

//
// effects/SkTileImageFilter
//

extern "C" SkImageFilter *C_SkTileImageFilter_Make(const SkRect &src, const SkRect &dst, const SkImageFilter &input) {
    return SkTileImageFilter::Make(src, dst, spFromConst(&input)).release();
}

//
// effects/SkXfermodeImageFilter
//

extern "C" SkImageFilter *
C_SkXfermodeImageFilter_Make(SkBlendMode mode, const SkImageFilter &background, const SkImageFilter *foreground,
                             const SkImageFilter::CropRect *cropRect) {
    return SkXfermodeImageFilter::Make(mode, spFromConst(&background), spFromConst(foreground), cropRect).release();
}

//
// docs/SkPDFDocument
//

extern "C" void C_SkPDF_Metadata_Construct(SkPDF::Metadata* uninitialized) {
    new(uninitialized)SkPDF::Metadata();
}

extern "C" void C_SkPDF_Metadata_destruct(SkPDF::Metadata* self) {
    self->~Metadata();
}

extern "C" SkDocument* C_SkPDF_MakeDocument(SkWStream* stream, const SkPDF::Metadata* metadata) {
    return SkPDF::MakeDocument(stream, *metadata).release();
}

//
// GrBackendFormat
//

extern "C" void C_GrBackendFormat_CopyConstruct(GrBackendFormat* uninitialized, const GrBackendFormat* format) {
    new(uninitialized)GrBackendFormat(*format);
}

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
// gpu/GrContext.h
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

//
// gpu/GrBackendDrawableInfo.h
//

extern "C" void C_GrBackendDrawableInfo_construct(GrBackendDrawableInfo* uninitialized) {
    new(uninitialized) GrBackendDrawableInfo();
}

extern "C" void C_GrBackendDrawableInfo_destruct(GrBackendDrawableInfo* self) {
    self->~GrBackendDrawableInfo();
}

extern "C" bool C_GrBackendDrawableInfo_isValid(const GrBackendDrawableInfo* self) {
    return self->isValid();
}

extern "C" GrBackendApi C_GrBackendDrawableInfo_backend(const GrBackendDrawableInfo* self) {
    return self->backend();
}

#if defined(SK_VULKAN)

extern "C" bool C_GrBackendDrawableInfo_getVkDrawableInfo(const GrBackendDrawableInfo* self, GrVkDrawableInfo* info) {
    return self->getVkDrawableInfo(info);
}

#endif

//
// pathops/
//

extern "C" void C_SkOpBuilder_Construct(SkOpBuilder* uninitialized) {
    new(uninitialized) SkOpBuilder();
}

extern "C" void C_SkOpBuilder_destruct(SkOpBuilder* self) {
    self->~SkOpBuilder();
}

//
// utils
//

extern "C" Sk3DView* C_Sk3DView_new() {
    return new Sk3DView();
}

extern "C" void C_Sk3DView_delete(Sk3DView* self) {
    delete self;
}

extern "C" void C_SkInterpolator_destruct(SkInterpolator* self) {
    self->~SkInterpolator();
}

extern "C" void C_SkInterpolator_setRepeatCount(SkInterpolator* self, SkScalar repeatCount) {
    self->setRepeatCount(repeatCount);
}

extern "C" void C_SkInterpolator_setReset(SkInterpolator* self, bool reset) {
    self->setReset(reset);
}

extern "C" void C_SkInterpolator_setMirror(SkInterpolator* self, bool mirror) {
    self->setMirror(mirror);
}

extern "C" SkCanvas* C_SkMakeNullCanvas() {
    return SkMakeNullCanvas().release();
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
                GrProtected isProtected,
                const GrVkYcbcrConversionInfo* ycbcrConversionInfo) {
    new (uninitialized) GrVkImageInfo(image, *alloc, imageTiling, layout, format, levelCount, currentQueueFamily, isProtected, *ycbcrConversionInfo);
}

extern "C" void C_GrVkImageInfo_updateImageLayout(GrVkImageInfo* self, VkImageLayout layout) {
    self->updateImageLayout(layout);
}

extern "C" bool C_GrVkImageInfo_Equals(const GrVkImageInfo* lhs, const GrVkImageInfo* rhs) {
    return *lhs == *rhs;
}

#endif

#if defined(SK_XML)

extern "C" SkCanvas* C_SkSVGCanvas_Make(const SkRect* bounds, SkWStream* writer, uint32_t flags) {
    return SkSVGCanvas::Make(*bounds, writer, flags).release();
}

#endif
