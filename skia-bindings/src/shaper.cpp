// We always compile SkShaper.h with HARFBUZZ & ICU for now.
#define SK_SHAPER_HARFBUZZ_AVAILABLE

#include "SkShaper.h"
#include "SkFontMgr.h"

extern "C" SkShaper* C_SkShaper_MakePrimitive() {
    return SkShaper::MakePrimitive().release();
}

extern "C" SkShaper* C_SkShaper_MakeShaperDrivenWrapper() {
    return SkShaper::MakeShaperDrivenWrapper().release();
}

extern "C" SkShaper* C_SkShaper_MakeShapeThenWrap() {
    return SkShaper::MakeShapeThenWrap().release();
}

extern "C" SkShaper* C_SkShaper_Make() {
    return SkShaper::Make().release();
}

extern "C" void C_SkShaper_delete(SkShaper* self) {
    delete self;
}

extern "C" void C_SkShaper_RunIterator_delete(SkShaper::RunIterator* self) {
    delete self;
}

extern "C" void C_SkShaper_RunIterator_consume(SkShaper::RunIterator* self)  {
    self->consume();
}

extern "C" size_t C_SkShaper_RunIterator_endOfCurrentRun(const SkShaper::RunIterator* self) {
    return self->endOfCurrentRun();
}

extern "C" bool C_SkShaper_RunIterator_atEnd(const SkShaper::RunIterator* self) {
    return self->atEnd();
}

extern "C" const SkFont* C_SkShaper_FontRunIterator_currentFont(const SkShaper::FontRunIterator* self) {
    return &self->currentFont();
}

extern "C" SkShaper::FontRunIterator* C_SkShaper_MakeFontMgrRunIterator(const char* utf8, size_t utf8Bytes, const SkFont* font, SkFontMgr* fallback) {
    return SkShaper::MakeFontMgrRunIterator(utf8, utf8Bytes, *font, sk_sp<SkFontMgr>(fallback)).release();
}

extern "C" uint8_t C_SkShaper_BiDiRunIterator_currentLevel(const SkShaper::BiDiRunIterator* self) {
    return self->currentLevel();
}

extern "C" SkShaper::BiDiRunIterator* C_SkShaper_MakeIcuBidiRunIterator(const char* utf8, size_t utf8Bytes, uint8_t bidiLevel) {
    return SkShaper::MakeIcuBiDiRunIterator(utf8, utf8Bytes, bidiLevel).release();
}

extern "C" SkFourByteTag C_SkShaper_ScriptRunIterator_currentScript(const SkShaper::ScriptRunIterator* self) {
    return self->currentScript();
}

extern "C" SkShaper::ScriptRunIterator* C_SkShaper_MakeHbIcuScriptRunIterator(const char* utf8, size_t utf8Bytes) {
    return SkShaper::MakeHbIcuScriptRunIterator(utf8, utf8Bytes).release();
}

extern "C" const char* C_SkShaper_LanguageRunIterator_currentLanguage(const SkShaper::LanguageRunIterator* self) {
    return self->currentLanguage();
}

extern "C" SkShaper::LanguageRunIterator* C_SkShaper_MakeStdLanguageRunIterator(const char* utf8, size_t utf8Bytes) {
    return SkShaper::MakeStdLanguageRunIterator(utf8, utf8Bytes).release();
}

extern "C" void C_SkShaper_RunHandler_delete(SkShaper::RunHandler* self) {
    delete self;
}

// TODO: support RunHandler

extern "C" SkTextBlob* C_SkTextBlobBuilderRunHandler_makeBlob(SkTextBlobBuilderRunHandler* self) {
    return self->makeBlob().release();
}
