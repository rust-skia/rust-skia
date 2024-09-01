use std::{borrow::Cow, ffi, mem, os::raw, ptr};

use skia_bindings::{
    self as sb, skresources_ImageAsset, skresources_ResourceProvider, RustResourceProvider_Param,
    SkData, SkRefCnt, SkRefCntBase, SkTypeface, TraitObject,
};

use crate::{prelude::*, Data, Typeface};

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

// pub type ExternalTrackAsset = RCHandle<skresources_ExternalTrackAsset>;
// require_base_type!(skresources_ExternalTrackAsset, SkRefCnt);

// impl NativeRefCountedBase for skresources_ExternalTrackAsset {
//     type Base = SkRefCntBase;
// }

pub trait ResourceProvider {
    fn load(&self, resource_path: &str, resource_name: &str) -> Option<Data>;
    fn load_image_asset(
        &self,
        resource_path: &str,
        resource_name: &str,
        resource_id: &str,
    ) -> Option<ImageAsset>;
    fn load_typeface(&self, name: &str, url: &str) -> Option<Typeface>;
}

pub type NativeResourceProvider = RCHandle<skresources_ResourceProvider>;

impl NativeRefCountedBase for skresources_ResourceProvider {
    type Base = SkRefCntBase;
}

impl<T: ResourceProvider + 'static> From<T> for NativeResourceProvider {
    fn from(value: T) -> Self {
        let b: Box<dyn ResourceProvider> = Box::new(value);
        Self::from(b)
    }
}

impl From<Box<dyn ResourceProvider>> for NativeResourceProvider {
    fn from(resource_provider: Box<dyn ResourceProvider>) -> Self {
        let param = RustResourceProvider_Param {
            trait_: unsafe {
                mem::transmute::<Box<dyn ResourceProvider>, TraitObject>(resource_provider)
            },
            drop: Some(drop),
            load: Some(load),
            loadImageAsset: Some(load_image_asset),
            loadTypeface: Some(load_typeface),
        };

        let skia_resource_provider =
            NativeResourceProvider::from_ptr(unsafe { sb::C_RustResourceProvider_New(&param) })
                .unwrap();

        return skia_resource_provider;

        extern "C" fn drop(provider: TraitObject) {
            mem::drop(unsafe {
                mem::transmute::<TraitObject, Box<dyn ResourceProvider>>(provider)
            });
        }

        extern "C" fn load(
            provider: TraitObject,
            resource_path: *const raw::c_char,
            resource_name: *const raw::c_char,
        ) -> *mut SkData {
            unsafe {
                provider_ref(&provider)
                    .load(&uncstr(resource_path), &uncstr(resource_name))
                    .map(|data| data.into_ptr())
                    .unwrap_or(ptr::null_mut())
            }
        }

        extern "C" fn load_image_asset(
            provider: TraitObject,
            resource_path: *const raw::c_char,
            resource_name: *const raw::c_char,
            resource_id: *const raw::c_char,
        ) -> *mut skresources_ImageAsset {
            unsafe {
                provider_ref(&provider)
                    .load_image_asset(
                        &uncstr(resource_path),
                        &uncstr(resource_name),
                        &uncstr(resource_id),
                    )
                    .map(|image_asset| image_asset.into_ptr())
                    .unwrap_or(ptr::null_mut())
            }
        }

        extern "C" fn load_typeface(
            provider: TraitObject,
            name: *const raw::c_char,
            url: *const raw::c_char,
        ) -> *mut SkTypeface {
            unsafe {
                provider_ref(&provider)
                    .load_typeface(&uncstr(name), &uncstr(url))
                    .map(|typeface| typeface.into_ptr())
                    .unwrap_or(ptr::null_mut())
            }
        }

        unsafe fn provider_ref(provider: &TraitObject) -> &dyn ResourceProvider {
            mem::transmute(*provider)
        }

        unsafe fn uncstr(ptr: *const raw::c_char) -> Cow<'static, str> {
            if !ptr.is_null() {
                return ffi::CStr::from_ptr(ptr).to_string_lossy();
            }
            "".into()
        }
    }
}
