/// Skia skparagraph Module C Wrapper Functions

#include "bindings.h"

#include "modules/skparagraph/include/DartTypes.h"
#include "modules/skparagraph/include/FontCollection.h"
#include "modules/skparagraph/include/Metrics.h"
#include "modules/skparagraph/include/ParagraphCache.h"
#include "modules/skparagraph/include/Paragraph.h"
#include "modules/skparagraph/include/ParagraphBuilder.h"
#include "modules/skparagraph/include/ParagraphStyle.h"
#include "modules/skparagraph/include/TextShadow.h"
#include "modules/skparagraph/include/TextStyle.h"
#include "modules/skparagraph/include/TypefaceFontProvider.h"

// m84: needs definition of SkFontData
#include "src/core/SkFontDescriptor.h"

using namespace skia::textlayout;

//
// FontCollection.h
//

struct Typefaces {
    std::vector<sk_sp<SkTypeface>> typefaces;
};

extern "C" {
    void C_Typefaces_construct(Typefaces* uninitialized) {
        new(uninitialized)Typefaces();
    }
    
    void C_Typefaces_destruct(Typefaces* self) {
        self->~Typefaces();
    }
    
    size_t C_Typefaces_count(const Typefaces* faces) {
        return faces->typefaces.size();
    }
    
    SkTypeface* C_Typefaces_get(const Typefaces* faces, size_t i) {
        return faces->typefaces[i].get();
    }
    
    SkTypeface* C_Typefaces_release(Typefaces* faces, size_t i) {
        return faces->typefaces[i].release();
    }
}

extern "C" {
    FontCollection* C_FontCollection_new() {
        return new FontCollection();
    }

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
        return self->getFallbackManager().release();
    }

    void C_FontCollection_findTypefaces(FontCollection* self, const SkStrings* familyNames, SkFontStyle fontStyle, Typefaces* typefaces) {
        auto tfs = self->findTypefaces(familyNames->strings, fontStyle);
        typefaces->typefaces = std::move(tfs);
    }

    SkTypeface* C_FontCollection_defaultFallback(FontCollection* self, SkUnichar unicode, SkFontStyle fontStyle, const SkString* locale) {
        return self->defaultFallback(unicode, fontStyle, *locale).release();
    }

    SkTypeface* C_FontCollection_defaultFallback2(FontCollection* self) {
        return self->defaultFallback().release();
    }

    bool C_FontCollection_fontFallbackEnabled(const FontCollection* self) {
        return const_cast<FontCollection*>(self)->fontFallbackEnabled();
    }

    ParagraphCache* C_FontCollection_paragraphCache(FontCollection* self) {
        return self->getParagraphCache();
    }
}

//
// ParagraphCache.h
//

extern "C" {
    void C_ParagraphCache_destruct(ParagraphCache* self) {
        self->~ParagraphCache();
    }

    int C_ParagraphCache_count(ParagraphCache* self) {
        return self->count();
    }
}

//
// ParagraphStyle.h
//

extern "C" {
    void C_StrutStyle_CopyConstruct(StrutStyle* uninitialized, const StrutStyle* other) {
        new(uninitialized) StrutStyle(*other);
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

    bool C_StrutStyle_equals(const StrutStyle* self, const StrutStyle* rhs) {
        return *self == *rhs;
    }
}

extern "C" {
    void C_ParagraphStyle_CopyConstruct(ParagraphStyle* uninitialized, const ParagraphStyle* other) {
        new(uninitialized) ParagraphStyle(*other);
    }

    void C_ParagraphStyle_destruct(ParagraphStyle* self) {
        self->~ParagraphStyle();
    }

    bool C_ParagraphStyle_Equals(const ParagraphStyle* left, const ParagraphStyle* right) {
        return *left == *right;
    }
}

//
// TextShadow.h
//

extern "C" {
    bool C_TextShadow_Equals(const TextShadow* self, const TextShadow* other) {
        return *self == *other;
    }
}

//
// Metrics.h
//

extern "C" {
    void C_LineMetrics_destruct(LineMetrics* self) {
        self->~LineMetrics();
    }
    
    size_t C_LineMetrics_fLineMetrics_count(const LineMetrics* self, size_t begin, size_t end) {
        auto lower = self->fLineMetrics.lower_bound(begin);
        auto upper = self->fLineMetrics.upper_bound(end);
        return std::distance(lower, upper);
    }
    
    struct StyleMetricsRecord {
        size_t index;
        const StyleMetrics* metrics;
    };
    
    void C_LineMetrics_fLineMetrics_getRange(const LineMetrics* self, size_t begin, size_t end, StyleMetricsRecord* array) {
        auto lower = self->fLineMetrics.lower_bound(begin);
        auto upper = self->fLineMetrics.upper_bound(end);
        for (auto it = lower; it != upper; it++)
        {
            *array++ = StyleMetricsRecord { it->first, &it->second };
        }
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

    const TextBox* C_TextBoxes_ptr_count(const TextBoxes* boxes, size_t* count) {
        *count = boxes->textBoxes.size();
        return &boxes->textBoxes.front();
    }
}

struct LineMetricsVector {
    std::vector<LineMetrics> lineMetrics;
};

extern "C" {
    void C_LineMetricsVector_destruct(LineMetricsVector* self) {
        self->~LineMetricsVector();
    }

    const LineMetrics* C_LineMetricsVector_ptr_count(const LineMetricsVector* metrics, size_t* count) {
        *count = metrics->lineMetrics.size();
        return &metrics->lineMetrics.front();
    }
}


extern "C" {
    void C_Paragraph_delete(Paragraph* self) {
        delete self;
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

    void C_Paragraph_getRectsForPlaceholders(Paragraph* self, TextBoxes* uninitialized) {
        auto v = self->getRectsForPlaceholders();
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

    void C_Paragraph_getLineMetrics(Paragraph* self, LineMetricsVector* uninitialized) {
        auto v = new(uninitialized) LineMetricsVector();
        self->getLineMetrics(v->lineMetrics);
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

    void C_ParagraphBuilder_addText(ParagraphBuilder* self, const char* text, size_t len) {
        self->addText(text, len);
    }

    void C_ParagraphBuilder_addPlaceholder(ParagraphBuilder* self, const PlaceholderStyle* placeholderStyle) {
        self->addPlaceholder(*placeholderStyle);
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
    void C_TextStyle_Types(const Block*, const Placeholder*) {}

    void C_FontFeature_CopyConstruct(FontFeature* uninitialized, const FontFeature* other) {
        new(uninitialized) FontFeature(*other);
    }

    void C_FontFeature_destruct(FontFeature* self) {
        self->~FontFeature();
    }

    void C_TextStyle_Construct(TextStyle* uninitialized) {
        new(uninitialized) TextStyle();
    }

    void C_TextStyle_CopyConstruct(TextStyle* uninitialized, const TextStyle* other) {
        new(uninitialized) TextStyle(*other);
    }

    void C_TextStyle_destruct(TextStyle* self) {
        self->~TextStyle();
    }

    void C_TextStyle_addShadow(TextStyle* self, const TextShadow* shadow) {
        self->addShadow(*shadow);
    }

    void C_TextStyle_resetShadows(TextStyle* self) {
        self->resetShadows();
    }

    void C_TextStyle_addFontFeature(TextStyle* self, const SkString* fontFeature, int value) {
        self->addFontFeature(*fontFeature, value);
    }

    void C_TextStyle_resetFontFeatures(TextStyle* self) {
        self->resetFontFeatures();
    }
    
    const SkString* C_TextStyle_getFontFamilies(const TextStyle* self, size_t* count) {
        auto& v = self->getFontFamilies();
        *count = v.size();
        return v.data();
    }

    void C_TextStyle_setFontFamilies(TextStyle* self, const SkString* data, size_t count) {
        self->setFontFamilies(std::vector<SkString>(data, data + count));
    }

    void C_TextStyle_setTypeface(TextStyle* self, SkTypeface* typeface) {
        self->setTypeface(sk_sp<SkTypeface>(typeface));
    }
}

struct FontFeatures {
    std::vector<FontFeature> fontFeatures;
};

extern "C" const FontFeature *C_FontFeatures_ptr_count(const FontFeatures *features, size_t *count) {
    *count = features->fontFeatures.size();
    return &features->fontFeatures.front();
}

struct TextShadows {
    std::vector<TextShadow> textShadows;
};

extern "C" const TextShadow *C_TextShadows_ptr_count(const TextShadows *shadows, size_t *count) {
    *count = shadows->textShadows.size();
    return &shadows->textShadows.front();
}

//
// TypefaceFontProvider
//

extern "C" {
    TypefaceFontStyleSet* C_TypefaceFontStyleSet_new(const SkString* family_name) {
        return new TypefaceFontStyleSet(*family_name);
    }

    void C_TypefaceFontStyleSet_appendTypeface(TypefaceFontStyleSet* self, SkTypeface* typeface) {
        self->appendTypeface(sk_sp<SkTypeface>(typeface));
    }
}

extern "C" {
    TypefaceFontProvider* C_TypefaceFontProvider_new() {
        return new TypefaceFontProvider();
    }

    size_t C_TypefaceFontProvider_registerTypeface(TypefaceFontProvider* self, SkTypeface* typeface, const SkString* alias) {
        if (alias) {
            return self->registerTypeface(sk_sp<SkTypeface>(typeface), *alias);
        }
        return self->registerTypeface(sk_sp<SkTypeface>(typeface));
    }
}
