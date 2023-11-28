#include "bindings.h"

#include "include/core/SkCanvas.h"
#include "include/svg/SkSVGCanvas.h"

#include "modules/svg/include/SkSVGDOM.h"
#include "modules/svg/include/SkSVGNode.h"
#include "modules/skresources/include/SkResources.h"

#include "include/core/SkStream.h"


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


extern "C" SkSVGDOM* C_SkSVGDOM_MakeFromStream(SkStream& stream, loadSkData loadCb, loadSkTypeface loadTfCb, void* loadContext) {
    auto provider = sk_make_sp<ImageResourceProvider>(loadCb, loadTfCb, loadContext);
    auto builder = SkSVGDOM::Builder();
    builder.setResourceProvider(provider);
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
