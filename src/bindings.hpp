#include "SkCanvas.h"
#include "SkImageInfo.h"
#include "SkSurface.h"
#include "SkPath.h"
#include "SkRect.h"
#include "SkColor.h"
#include "SkPaint.h"
#include "SkTypes.h"

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
