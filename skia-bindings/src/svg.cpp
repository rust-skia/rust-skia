#include "bindings.h"

#include "include/core/SkCanvas.h"
#include "include/svg/SkSVGCanvas.h"

#include "modules/svg/include/SkSVGDOM.h"
#include "modules/svg/include/SkSVGNode.h"
#include "modules/skresources/include/SkResources.h"

#include "include/core/SkStream.h"

extern "C" void C_Resource_Types(const skresources::ExternalTrackAsset *)
{}

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
        typedef skresources::ExternalTrackAsset *(*LoadAudioAsset)(TraitObject, const char resource_path[], const char resource_name[], const char resource_id[]);
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
    {
    }

    virtual ~RustResourceProvider() {
        _param.drop(_param.trait);
    }

    sk_sp<SkData> load(const char resource_path[],
                       const char resource_name[]) const override {
        return sk_sp<SkData>(_param.load(_param.trait, resource_path, resource_name));
    }

    sk_sp<skresources::ImageAsset> loadImageAsset(const char resource_path[],
                                                  const char resource_name[],
                                                  const char resource_id[]) const override {
        return sk_sp<skresources::ImageAsset>(_param.loadImageAsset(_param.trait, resource_path, resource_name, resource_id));
    }

    sk_sp<SkTypeface> loadTypeface(const char name[],
                                   const char url[]) const override {
        return sk_sp<SkTypeface>(_param.loadTypeface(_param.trait, name, url));
    }
 
private:
    Param _param;
};

extern "C" RustResourceProvider* C_RustResourceProvider_New(const RustResourceProvider::Param* param) {
    return new RustResourceProvider(*param);
}

typedef SkData* (*loadSkData)(const char resource_path[], const char resource_name[], void* context);
typedef SkTypeface* (*loadSkTypeface)(const char resource_path[], const char resource_name[], void* context);

class ImageResourceProvider final : public skresources::ResourceProvider {

private:
    loadSkData _loadCb;
    loadSkTypeface _loadTfCb;
    void* _loadContext;

public:
    ImageResourceProvider(loadSkData loadCb, loadSkTypeface loadTfCb, void* loadContext) {
        _loadCb = loadCb;
        _loadTfCb = loadTfCb;
        _loadContext = loadContext;
    }

    sk_sp<SkData> load(const char resource_path [],
                       const char resource_name []) const {
        return sp(_loadCb(resource_path, resource_name, _loadContext));
    }


    sk_sp<skresources::ImageAsset> loadImageAsset(const char resource_path [],
                                                  const char resource_name [],
                                                  const char /*resource_id*/ []) const {
        auto data = this->load(resource_path, resource_name);
        return skresources::MultiFrameImageAsset::Make(data);
    }


    sk_sp<SkTypeface> loadTypeface(const char name[],
                                   const char url[]) const {
        return sp(_loadTfCb(url, name, _loadContext));
    }

    ~ImageResourceProvider() {}

};


extern "C" SkSVGDOM* C_SkSVGDOM_MakeFromStream(SkStream& stream, SkFontMgr* fontMgr, loadSkData loadCb, loadSkTypeface loadTfCb, void* loadContext) {
    auto provider = sk_make_sp<ImageResourceProvider>(loadCb, loadTfCb, loadContext);
    auto builder = SkSVGDOM::Builder();
    builder.setResourceProvider(provider);
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
