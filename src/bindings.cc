#include <iostream>
#include <vector>
#include "SkCanvas.h"
#include "SkImageInfo.h"
#include "SkSurface.h"
#include "SkPath.h"
#include "SkRect.h"
#include "SkColor.h"
#include "SkPaint.h"
#include "SkTypes.h"

#include "./bindings.hpp"

using namespace std;

extern "C" SkCanvasBindings SkiaCreateCanvas(int width, int height) {
  auto surface = SkSurface::MakeRasterN32Premul(width, height);
  auto canvas = surface->getCanvas();
  auto release = [surface]() {
    surface->unref();
  };
  static auto static_release = release;
  void (*ptr)() = []() { static_release(); };
  SkCanvasBindings sk_canvas_bindings = { surface.get(), ptr, canvas };
  surface->ref();
  return sk_canvas_bindings;
}

extern "C" SkRect SkiaCreateRect(float width, float height) {
  return SkRect::MakeWH(width, height);
}

extern "C" void SkiaClearCanvas(SkCanvas* canvas, SkColor color) {
  canvas->clear(color);
}

extern "C" SkSurfaceData SkiaGetSurfaceData(SkSurface* surface) {
  sk_sp<SkImage> img(surface->makeImageSnapshot());
  if (!img) { return { nullptr, 0 }; }
  sk_sp<SkData> png(img->encodeToData());
  if (!png) { return { nullptr, 0 }; }
  SkSurfaceData result = { png->bytes(), png->size() };
  return result;
}
