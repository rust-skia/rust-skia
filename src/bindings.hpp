#include "SkCanvas.h"
#include "SkImageInfo.h"
#include "SkSurface.h"
#include "SkPath.h"
#include "SkRect.h"
#include "SkColor.h"
#include "SkPaint.h"
#include "SkTypes.h"

#if defined(FEATURE_VULKAN)
  #include "GrContext.h"
  #include "vk/GrVkBackendContext.h"
#endif

extern "C" SkSurface* C_SkSurface_MakeRasterN32Premul(int width, int height, const SkSurfaceProps* surfaceProps) {
    return SkSurface::MakeRasterN32Premul(width, height, surfaceProps).release();
}

extern "C" SkImage* C_SkSurface_makeImageSnapshot(SkSurface* self) {
    return self->makeImageSnapshot().release();
}

extern "C" SkData* C_SkImage_encodeToData(SkImage* self) {
    return self->encodeToData().release();
}

extern "C" void C_SkData_unref(const SkData* self) {
    self->unref();
}

extern "C" void C_SkPaint_destruct(const SkPaint* self) {
    self->~SkPaint();
}

extern "C" void C_SkPath_destruct(const SkPath* self) {
    self->~SkPath();
}


typedef struct SkCanvasBindings {
  SkSurface* surface;
  void (*release_fn)();
  SkCanvas* canvas;
} SkCanvasBindings;

typedef struct SkSurfaceData {
  const unsigned char* data;
  size_t size;
} SkSurfaceData;

extern "C" SkCanvasBindings SkiaCreateCanvas(int width, int height);

extern "C" SkRect SkiaCreateRect(float width, float height);

extern "C" void SkiaClearCanvas(SkCanvas* canvas, SkColor color);

extern "C" SkSurfaceData SkiaGetSurfaceData(SkSurface* surface);
