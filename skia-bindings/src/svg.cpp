#include "include/core/SkCanvas.h"
#include "include/svg/SkSVGCanvas.h"

extern "C" void C_SVG_Types(SkSVGCanvas *) {}

extern "C" SkCanvas* C_SkSVGCanvas_Make(const SkRect* bounds, SkWStream* writer, uint32_t flags) {
    return SkSVGCanvas::Make(*bounds, writer, flags).release();
}
