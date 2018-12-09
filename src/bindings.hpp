#include "SkCanvas.h"
#include "SkImageInfo.h"
#include "SkSurface.h"
#include "SkPath.h"
#include "SkRect.h"
#include "SkColor.h"
#include "SkPaint.h"
#include "SkTypes.h"

typedef struct SkCanvasBindings {
  void (*release_fn)();
  SkCanvas* canvas;
  SkImageInfo* info;
  size_t rowBytes;
  size_t size;
  char* data_ptr;
} SkCanvasBindings;

extern "C" SkCanvasBindings SkiaCreateCanvas(int width, int height);

extern "C" SkRect SkiaCreateRect(float width, float height);

extern "C" void SkiaClearCanvas(SkCanvas* canvas, SkColor color);
