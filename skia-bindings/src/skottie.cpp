#include "bindings.h"

#include "modules/skottie/include/Skottie.h"

// Reference counting (SkNVRefCnt pattern)
extern "C" void C_skottie_Animation_ref(const skottie::Animation* self) {
    self->ref();
}

extern "C" void C_skottie_Animation_unref(const skottie::Animation* self) {
    self->unref();
}

extern "C" bool C_skottie_Animation_unique(const skottie::Animation* self) {
    return self->unique();
}

// Factory methods
extern "C" skottie::Animation* C_skottie_Animation_Make(const char* data, size_t length) {
    return skottie::Animation::Make(data, length).release();
}

extern "C" skottie::Animation* C_skottie_Animation_MakeFromFile(const char* path) {
    return skottie::Animation::MakeFromFile(path).release();
}

// Property accessors
extern "C" SkScalar C_skottie_Animation_duration(const skottie::Animation* self) {
    return self->duration();
}

extern "C" SkScalar C_skottie_Animation_fps(const skottie::Animation* self) {
    return self->fps();
}

extern "C" SkScalar C_skottie_Animation_inPoint(const skottie::Animation* self) {
    return self->inPoint();
}

extern "C" SkScalar C_skottie_Animation_outPoint(const skottie::Animation* self) {
    return self->outPoint();
}

extern "C" void C_skottie_Animation_size(const skottie::Animation* self, SkSize* size) {
    *size = self->size();
}

extern "C" void C_skottie_Animation_version(const skottie::Animation* self, SkString* version) {
    *version = self->version();
}

// Seeking (nullptr for InvalidationController)
extern "C" void C_skottie_Animation_seekFrame(skottie::Animation* self, double t) {
    self->seekFrame(t, nullptr);
}

extern "C" void C_skottie_Animation_seekFrameTime(skottie::Animation* self, double t) {
    self->seekFrameTime(t, nullptr);
}

extern "C" void C_skottie_Animation_seek(skottie::Animation* self, SkScalar t) {
    self->seek(t, nullptr);
}

// Rendering
extern "C" void C_skottie_Animation_render(
    const skottie::Animation* self,
    SkCanvas* canvas,
    const SkRect* dst
) {
    self->render(canvas, dst);
}

extern "C" void C_skottie_Animation_render_with_flags(
    const skottie::Animation* self,
    SkCanvas* canvas,
    const SkRect* dst,
    uint32_t flags
) {
    self->render(canvas, dst, static_cast<skottie::Animation::RenderFlags>(flags));
}
