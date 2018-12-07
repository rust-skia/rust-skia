#include "SkCanvas.h"
#include "SkImageInfo.h"
#include "SkSurface.h"
#include "SkPath.h"

typedef struct SkCanvasBindings {
  void (*release_fn)();
  SkCanvas* canvas;
  SkImageInfo* info;
  size_t rowBytes;
  size_t size;
  char* data_ptr;
} SkCanvasBindings;

extern "C" SkCanvasBindings SkiaCreateCanvas(int width, int height);
