#include "bindings.h"

#include "include/core/SkData.h"
#include "include/encode/SkWebpEncoder.h"

extern "C" {

bool C_SkWebpEncoder_Encode(
    SkWStream* stream,
    const SkPixmap* pixmap,
    SkWebpEncoder::Compression compression,
    float quality,
    int method)
{
    auto options = SkWebpEncoder::Options();
    options.fCompression = compression;
    options.fQuality = quality;
    options.fMethod = method;

    return SkWebpEncoder::Encode(stream, *pixmap, options);
}

SkData* C_SkWebpEncoder_EncodeImage(
    GrDirectContext* ctx,
    const SkImage* img,
    SkWebpEncoder::Compression compression,
    float quality,
    int method)
{
    auto options = SkWebpEncoder::Options();
    options.fCompression = compression;
    options.fQuality = quality;
    options.fMethod = method;

    return SkWebpEncoder::Encode(ctx, img, options).release();
}

}
