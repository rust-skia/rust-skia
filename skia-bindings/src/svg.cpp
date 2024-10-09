#include "bindings.h"

#include "include/core/SkCanvas.h"
#include "include/svg/SkSVGCanvas.h"

#include "modules/svg/include/SkSVGDOM.h"
#include "modules/svg/include/SkSVGNode.h"
#include "modules/skresources/include/SkResources.h"

#include "include/core/SkStream.h"

extern "C" bool C_ImageAsset_isMultiFrame(skresources::ImageAsset* self) {
    return self->isMultiFrame();
}

extern "C" skresources::ImageAsset* C_MultiFrameImageAsset_Make(
    SkData* data, skresources::ImageDecodeStrategy decodeStrategy)
{
    return skresources::MultiFrameImageAsset::Make(sp(data), decodeStrategy).release();
}

namespace ResourceProvider { 
    extern "C" {
        typedef void (*Drop)(TraitObject);

        typedef SkData *(*Load)(TraitObject, const char resource_path[], const char resource_name[]);
        typedef skresources::ImageAsset* (*LoadImageAsset)(TraitObject, const char resource_path[], const char resource_name[], const char resource_id[]);
        typedef SkTypeface *(*LoadTypeface)(TraitObject, const char name[], const char url[]);
    }
}

class RustResourceProvider final : public skresources::ResourceProvider {
public:
    struct Param {
        TraitObject trait;
        ::ResourceProvider::Drop drop;
        ::ResourceProvider::Load load;
        ::ResourceProvider::LoadImageAsset loadImageAsset;
        ::ResourceProvider::LoadTypeface loadTypeface;
    };

    explicit RustResourceProvider(const Param& param) 
    : _param(param) 
    { }

    virtual ~RustResourceProvider() {
        _param.drop(_param.trait);
    }

    sk_sp<SkData> load(const char resource_path[], const char resource_name[]) const override {
        return sk_sp<SkData>(_param.load(_param.trait, resource_path, resource_name));
    }

    sk_sp<skresources::ImageAsset> loadImageAsset(
        const char resource_path[],
        const char resource_name[],
        const char resource_id[]) const override {
        return sk_sp<skresources::ImageAsset>(_param.loadImageAsset(_param.trait, resource_path, resource_name, resource_id));
    }

    sk_sp<SkTypeface> loadTypeface(const char name[], const char url[]) const override {
        return sk_sp<SkTypeface>(_param.loadTypeface(_param.trait, name, url));
    }
 
private:
    Param _param;
};

extern "C" skresources::ResourceProvider* C_RustResourceProvider_New(const RustResourceProvider::Param* param) {
    return new RustResourceProvider(*param);
}

extern "C" SkSVGDOM* C_SkSVGDOM_MakeFromStream(
    SkStream& stream,
    skresources::ResourceProvider* provider,
    SkFontMgr* fontMgr) 
{
    auto builder = SkSVGDOM::Builder();
    builder.setResourceProvider(sp(provider));
    builder.setFontManager(sp(fontMgr));
    return builder.make(stream).release();
}

extern "C" void C_SkSVGDOM_ref(const SkSVGDOM* self) {
    self->ref();
}

extern "C" void C_SkSVGDOM_unref(const SkSVGDOM* self) {
    self->unref();
}

extern "C" bool C_SkSVGDOM_unique(const SkSVGDOM* self) {
    return self->unique();
}

extern "C" void C_SkSVGDOM_setContainerSize(SkSVGDOM* self, const SkSize& size){
    self->setContainerSize(size);
}
