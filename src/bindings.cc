#include <iostream>
#include <vector>
#include "SkCanvas.h"
#include "SkImageInfo.h"
#include "SkSurface.h"

#include "./bindings.hpp"

using namespace std;

extern "C" SkCanvasBindings SkiaCreateCanvas(int width, int height) {
  auto info = SkImageInfo::MakeN32Premul(width, height);
  auto rowBytes = info.minRowBytes();
  auto size = info.computeByteSize(rowBytes);
  vector<char> pixelMemory(size);  // allocate memory
  auto data_ptr = &pixelMemory[0];
  auto surface =
          SkSurface::MakeRasterDirect(
                  info, data_ptr, rowBytes);
  auto canvas = surface->getCanvas();
  auto release = [surface]() {
    surface->unref();
  };
  static auto static_release = release;
  void (*ptr)() = []() { static_release(); };
  SkCanvasBindings release_holder = { ptr, canvas, &info, rowBytes, size, data_ptr };
  surface->ref();
  return release_holder;
}
