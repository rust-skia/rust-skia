use skia_bindings::{
    self as sb, skresources_ExternalTrackAsset, skresources_ImageAsset, SkRefCnt, SkRefCntBase,
};

use crate::{prelude::*, Data};

pub type ImageAsset = RCHandle<skresources_ImageAsset>;
require_base_type!(skresources_ImageAsset, SkRefCnt);

impl NativeRefCountedBase for skresources_ImageAsset {
    type Base = SkRefCntBase;
}

impl ImageAsset {
    pub fn is_multi_frame(&self) -> bool {
        unsafe { sb::C_ImageAsset_isMultiFrame(self.native_mut_force()) }
    }

    // TODO: wrap getFrameData()

    // MultiFrameImageAsset

    pub fn from_data(
        data: impl Into<Data>,
        decode_strategy: impl Into<Option<ImageDecodeStrategy>>,
    ) -> Option<Self> {
        let decode_strategy = decode_strategy
            .into()
            .unwrap_or(ImageDecodeStrategy::LazyDecode);

        ImageAsset::from_ptr(unsafe {
            sb::C_MultiFrameImageAsset_Make(data.into().into_ptr(), decode_strategy)
        })
    }

    // TODO: Wrapping Make(SkCodec) requires us to put a lifetime on the ImageAsset.
}

pub use sb::skresources_ImageDecodeStrategy as ImageDecodeStrategy;
variant_name!(ImageDecodeStrategy::LazyDecode);

pub type ExternalTrackAsset = RCHandle<skresources_ExternalTrackAsset>;
require_base_type!(skresources_ExternalTrackAsset, SkRefCnt);

impl NativeRefCountedBase for skresources_ExternalTrackAsset {
    type Base = SkRefCntBase;
}

