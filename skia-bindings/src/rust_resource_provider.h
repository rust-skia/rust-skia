#ifndef RUST_RESOURCE_PROVIDER_H
#define RUST_RESOURCE_PROVIDER_H

#include "bindings.h"
#include "modules/skresources/include/SkResources.h"
#include "include/core/SkFontMgr.h"

namespace ResourceProvider {
    extern "C" {
        typedef void (*Drop)(void*);
        typedef SkData *(*Load)(void*, const char[], const char[]);
        typedef skresources::ImageAsset* (*LoadImageAsset)(void*, const char[], const char[], const char[]);
        typedef SkTypeface *(*LoadTypeface)(void*, const char[], const char[]);
        typedef SkFontMgr *(*FontMgr)(void*);
    }
}

class RustResourceProvider final : public skresources::ResourceProvider {
public:
    struct Param {
        // The Rust provider, owned by the `RustResourceProvider` and released in its destructor
        // through `drop`.
        void* data;
        ::ResourceProvider::Drop drop;
        ::ResourceProvider::Load load;
        ::ResourceProvider::LoadImageAsset loadImageAsset;
        ::ResourceProvider::LoadTypeface loadTypeface;
        ::ResourceProvider::FontMgr fontMgr;
    };

    explicit RustResourceProvider(const Param& param)
    : _param(param)
    { }

    virtual ~RustResourceProvider() {
        _param.drop(_param.data);
    }

    sk_sp<SkData> load(const char resource_path[], const char resource_name[]) const override {
        return sp(_param.load(_param.data, resource_path, resource_name));
    }

    sk_sp<skresources::ImageAsset> loadImageAsset(
        const char resource_path[],
        const char resource_name[],
        const char resource_id[]) const override {
        return sp(_param.loadImageAsset(_param.data, resource_path, resource_name, resource_id));
    }

    sk_sp<SkTypeface> loadTypeface(const char name[], const char url[]) const override {
        return sp(_param.loadTypeface(_param.data, name, url));
    }

    // This is here to provide access to the FontMgr to the Dom.
    sk_sp<SkFontMgr> fontMgr() const {
        return sp(_param.fontMgr(_param.data));
    }

private:
    Param _param;
};

extern "C" RustResourceProvider* C_RustResourceProvider_New(const RustResourceProvider::Param* param);

#endif
