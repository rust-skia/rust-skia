/// Skia skparagraph Module C Wrapper Functions

#include "bindings.h"

#include "modules/skparagraph/include/DartTypes.h"
#include "modules/skparagraph/include/FontCollection.h"
#include "modules/skparagraph/include/Paragraph.h"
#include "modules/skparagraph/include/ParagraphBuilder.h"
#include "modules/skparagraph/include/ParagraphStyle.h"
#include "modules/skparagraph/include/TextShadow.h"
#include "modules/skparagraph/include/TextStyle.h"
#include "modules/skparagraph/include/TypefaceFontProvider.h"

using namespace skia::textlayout;

//
// FontCollection.h
//

extern "C" {

    void C_FontCollection_setAssetFontManager(FontCollection* self, const SkFontMgr* fontManager) {
        self->setAssetFontManager(spFromConst(fontManager));
    }

    void C_FontCollection_setDynamicFontManager(FontCollection* self, const SkFontMgr* fontManager) {
        self->setDynamicFontManager(spFromConst(fontManager));
    }

    void C_FontCollection_setTestFontManager(FontCollection* self, const SkFontMgr* fontManager) {
        self->setTestFontManager(spFromConst(fontManager));
    }

    void C_FontCollection_setDefaultFontManager(FontCollection* self, const SkFontMgr* fontManager) {
        self->setDefaultFontManager(spFromConst(fontManager));
    }

    void C_FontCollection_setDefaultFontManager2(FontCollection* self, const SkFontMgr* fontManager, const char* defaultFamilyName) {
        self->setDefaultFontManager(spFromConst(fontManager), defaultFamilyName);
    }

    SkFontMgr* C_FontCollection_getFallbackManager(const FontCollection* self) {
        return self->geFallbackManager().release();
    }

    SkTypeface* C_FontCollection_matchTypeface(FontCollection* self, const char* familyName, SkFontStyle fontStyle) {
        return self->matchTypeface(familyName, fontStyle).release();
    }

    SkTypeface* C_FontCollection_matchDefaultTypeface(FontCollection* self, SkFontStyle fontStyle) {
        return self->matchDefaultTypeface(fontStyle).release();
    }

    SkTypeface* C_FontCollection_defaultFallback(FontCollection* self, SkUnichar unicode, SkFontStyle fontStyle, const SkString* locale) {
        return self->defaultFallback(unicode, fontStyle, *locale).release();
    }
}

//
// ParagraphStyle.h
//

extern "C" {
    void C_StrutStyle_assign(StrutStyle* self, const StrutStyle* other) {
        *self = *other;
    }

    void C_StrutStyle_destruct(StrutStyle* self) {
        self->~StrutStyle();
    }

    const SkString* C_StrutStyle_getFontFamilies(const StrutStyle* self, size_t* count) {
        auto& v = self->getFontFamilies();
        *count = v.size();
        return v.data();
    }

    void C_StrutStyle_setFontFamilies(StrutStyle* self, const SkString* data, size_t count) {
        self->setFontFamilies(std::vector<SkString>(data, data + count));
    }
}

extern "C" {
    bool C_ParagraphStyle_Equals(const ParagraphStyle* left, const ParagraphStyle* right) {
        return *left == *right;
    }

    void C_ParagraphStyle_setStrutStyle(ParagraphStyle* self, const StrutStyle* strutStyle) {
        self->setStrutStyle(*strutStyle);
    }

    void C_ParagraphStyle_setEllipsis(ParagraphStyle* self, const StrutStyle* strutStyle) {
        self->setStrutStyle(*strutStyle);
    }
}

//
// TextShadow.h
//

extern "C" {
    void C_TextShadow_destruct(TextShadow* self) {
        self->~TextShadow();
    }
}

//
// Paragraph.h
//

struct TextBoxes {
    std::vector<TextBox> textBoxes;
};

extern "C" {
    void C_TextBoxes_destruct(TextBoxes* self) {
        self->~TextBoxes();
    }

    const TextBox* C_TextBoxes_ptr(TextBoxes* boxes) {
        return &boxes->textBoxes.front();
    }

    size_t C_TextBoxes_count(const TextBoxes* boxes) {
        return boxes->textBoxes.size();
    }
}

extern "C" {
    void C_Paragraph_delete(Paragraph* self) {
        delete self;
    }

    bool C_Paragraph_didExceedMaxLines(Paragraph* self) {
        return self->didExceedMaxLines();
    }

    void C_Paragraph_layout(Paragraph* self, SkScalar width) {
        self->layout(width);
    }

    void C_Paragraph_paint(Paragraph* self, SkCanvas* canvas, SkScalar x, SkScalar y) {
        self->paint(canvas, x, y);
    }

    void C_Paragraph_getRectsForRange(Paragraph *self, unsigned start, unsigned end, RectHeightStyle rectHeightStyle,
                                            RectWidthStyle rectWidthStyle, TextBoxes* uninitialized) {
        auto v = self->getRectsForRange(start, end, rectHeightStyle, rectWidthStyle);
        new(uninitialized) TextBoxes{std::move(v)};
    }

    void C_Paragraph_getGlyphPositionAtCoordinate(Paragraph* self, SkScalar x, SkScalar y, PositionWithAffinity* position) {
        *position = self->getGlyphPositionAtCoordinate(x, y);
    }

    void C_Paragraph_getWordBoundary(Paragraph* self, unsigned offset, size_t range[2]) {
        auto sk_range = self->getWordBoundary(offset);
        range[0] = sk_range.start;
        range[1] = sk_range.end;
    }

    size_t C_Paragraph_lineNumber(Paragraph* self) {
        return self->lineNumber();
    }

    void C_Paragraph_markDirty(Paragraph* self) {
        self->markDirty();
    }
}

//
// ParagraphBuilder.h
//

extern "C" {
    void C_ParagraphBuilder_delete(ParagraphBuilder* self) {
        delete self;
    }

    void C_ParagraphBuilder_pushStyle(ParagraphBuilder* self, const TextStyle* style) {
        self->pushStyle(*style);
    }

    void C_ParagraphBuilder_pop(ParagraphBuilder* self) {
        self->pop();
    }

    void C_ParagraphBuilder_peekStyle(ParagraphBuilder* self, TextStyle* style) {
        *style = self->peekStyle();
    }

    void C_ParagraphBuilder_addText(ParagraphBuilder* self, const char* text) {
        self->addText(text);
    }

    void C_ParagraphBuilder_setParagraphStyle(ParagraphBuilder* self, const ParagraphStyle* style) {
        self->setParagraphStyle(*style);
    }

    Paragraph* C_ParagraphBuilder_Build(ParagraphBuilder* self) {
        return self->Build().release();
    }

    ParagraphBuilder* C_ParagraphBuilder_make(const ParagraphStyle* style, const FontCollection* fontCollection) {
        return ParagraphBuilder::make(*style, spFromConst(fontCollection)).release();
    }
}

//
// TextStyle.h
//

extern "C" {
    void C_TextStyle_assign(TextStyle* self, const TextStyle* other) {
        *self = *other;
    }

    void C_TextStyle_destruct(TextStyle* self) {
        self->~TextStyle();
    }

    size_t C_TextStyle_getShadowNumber(const TextStyle* self) {
        return self->getShadowNumber();
    }

    void C_TextStyle_getShadows(const TextStyle* self, TextShadow shadowsOut[]) {
        auto shadows = self->getShadows();
        for(std::vector<TextShadow>::size_type i = 0; i != shadows.size(); ++i) {
            shadowsOut[i] = shadows[i];
        }
    }

    void C_TextStyle_addShadow(TextStyle* self, const TextShadow* shadow) {
        self->addShadow(*shadow);
    }

    void C_TextStyle_resetShadows(TextStyle* self) {
        self->resetShadows();
    }

    const SkString* C_TextStyle_getFontFamilies(const TextStyle* self, size_t* count) {
        auto& v = self->getFontFamilies();
        *count = v.size();
        return v.data();
    }

    void C_TextStyle_setFontFamilies(TextStyle* self, const SkString* data, size_t count) {
        self->setFontFamilies(std::vector<SkString>(data, data + count));
    }

    void C_TextStyle_setTypeface(TextStyle* self, const SkTypeface* typeface) {
        self->setTypeface(spFromConst(typeface));
    }

    void C_TextStyle_getFontMetrics(const TextStyle* self, SkFontMetrics* metrics) {
        self->getFontMetrics(metrics);
    }
}

