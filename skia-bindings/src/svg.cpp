#include "bindings.h"

#include "include/svg/SkSVGCanvas.h"

#include "modules/svg/include/SkSVGDOM.h"
#include "modules/skresources/include/SkResources.h"

#include "include/core/SkStream.h"


typedef SkData* (*loadSkData)(const char resource_path[], const char resource_name[]);

typedef SkTypeface* (*loadSkTypeface)(const char resource_path[], const char resource_name[]);


extern "C" void C_SVG_Types(SkSVGCanvas *) {}

extern "C" SkCanvas* C_SkSVGCanvas_Make(const SkRect* bounds, SkWStream* writer, uint32_t flags) {
    return SkSVGCanvas::Make(*bounds, writer, flags).release();
}




class ImageResourceProvider final : public skresources::ResourceProvider {

private:
    loadSkData _loadCb;
    loadSkTypeface _loadTfCb;

public:
    ImageResourceProvider(loadSkData loadCb, loadSkTypeface loadTfCb) {
        _loadCb = loadCb;
        _loadTfCb = loadTfCb;
    }


    sk_sp<SkData> load(const char resource_path [],
                       const char resource_name []) const {
        return sp(((loadSkData)_loadCb)(resource_path,resource_name));
    }


    sk_sp<skresources::ImageAsset> loadImageAsset(const char resource_path [],
                                                  const char resource_name [],
                                                  const char /*resource_id*/ []) const {
        auto data = this->load(resource_path, resource_name);
        return skresources::MultiFrameImageAsset::Make(data);
    }


    sk_sp<SkTypeface> loadTypeface(const char name[],
                                   const char url[]) const {
        return sp(((loadSkTypeface)_loadTfCb)(url,name));
    }

    ~ImageResourceProvider() {}

};


extern "C" SkSVGDOM* C_SkSVGDOM_MakeFromStream(SkStream& stream, loadSkData loadCb, loadSkTypeface loadTfCb) {
    auto provider = sk_make_sp<ImageResourceProvider>(loadCb, loadTfCb);
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


//
// SkStream
//

class RustStream : public SkStream {
    void *m_data;
    size_t m_length;
    bool m_isEof;

    size_t (*m_read)(void *, void *, size_t);

    bool (*m_seekAbsolute)(void *, size_t);

    bool (*m_seekRelative)(void *, long);

public:
    RustStream(
            void *data,
            size_t length,
            size_t (*read)(void *, void *, size_t),
            bool (*seekAbsolute)(void *, size_t),
            bool (*seekRelative)(void *, long)
    );

    size_t read(void *buffer, size_t count);

    bool rewind();

    bool seek(size_t pos);

    bool move(long offset);

    bool isAtEnd() const;

    bool hasLength() const;

    size_t getLength() const;
};

RustStream::RustStream(
        void *data,
        size_t length,
        size_t (*read)(void *, void *, size_t),
        bool (*seekAbsolute)(void *, size_t),
        bool (*seekRelative)(void *, long)
) :
        m_data(data),
        m_length(length),
        m_isEof(false),
        m_read(read),
        m_seekAbsolute(seekAbsolute),
        m_seekRelative(seekRelative) {}

size_t RustStream::read(void *buffer, size_t count) {
    if (this->m_isEof) return 0;

    size_t out = (this->m_read)(this->m_data, buffer, count);

    if (!out) {
        this->m_isEof = true;
    }

    return out;
}

bool RustStream::rewind() {
    return this->seek(0);
}

bool RustStream::seek(size_t pos) {
    if (this->m_seekAbsolute) {
        return (this->m_seekAbsolute)(this->m_data, pos);
    } else {
        return false;
    }
}

bool RustStream::move(long offset) {
    if (this->m_seekRelative) {
        return (this->m_seekRelative)(this->m_data, offset);
    } else {
        return false;
    }
}

bool RustStream::hasLength() const {
    return this->m_length != (size_t) - 1;
}

size_t RustStream::getLength() const {
    return this->m_length;
}

bool RustStream::isAtEnd() const {
    return this->m_isEof;
}

extern "C" void C_RustStream_construct(
        RustStream *out,
        void *data,
        size_t length,
        size_t (*read)(void *, void *, size_t),
        bool (*seekAbsolute)(void *, size_t),
        bool (*seekRelative)(void *, long)
) {
    new(out) RustStream(data, length, read, seekAbsolute, seekRelative);
}
