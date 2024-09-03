use std::{borrow::Cow, ffi, mem, os::raw, ptr};

use helpers::ResourceKind;
use skia_bindings::{
    self as sb, skresources_ImageAsset, skresources_ResourceProvider, RustResourceProvider_Param,
    SkData, SkRefCnt, SkRefCntBase, SkTypeface, TraitObject,
};

use crate::{prelude::*, Data, FontMgr, Typeface};

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
        _resource_id: &str,
    ) -> Option<ImageAsset> {
        let data = self.load(resource_path, resource_name)?;
        ImageAsset::from_data(data, None)
    }

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

/// A resource provider that loads only local / inline base64 resources.
#[derive(Debug)]
pub struct LocalResourceProvider {
    font_mgr: FontMgr,
}

impl ResourceProvider for LocalResourceProvider {
    fn load(&self, resource_path: &str, resource_name: &str) -> Option<Data> {
        match helpers::identify_resource_kind(resource_path, resource_name) {
            ResourceKind::Base64(data) => Some(data),
            ResourceKind::DownloadFrom(_) => None,
        }
    }

    fn load_typeface(&self, name: &str, url: &str) -> Option<Typeface> {
        helpers::load_typeface(self, &self.font_mgr, name, url)
    }
}

impl LocalResourceProvider {
    pub fn new(font_mgr: impl Into<FontMgr>) -> Self {
        Self {
            font_mgr: font_mgr.into(),
        }
    }
}

/// Support a direct conversion from a [`FontMgr`] nito a local native resource provider.
impl From<FontMgr> for NativeResourceProvider {
    fn from(font_mgr: FontMgr) -> Self {
        LocalResourceProvider::new(font_mgr).into()
    }
}

#[cfg(feature = "ureq")]
#[derive(Debug)]
/// A resource provider that uses ureq for downloading resources.
pub struct UReqResourceProvider {
    font_mgr: FontMgr,
}

#[cfg(feature = "ureq")]
impl UReqResourceProvider {
    pub fn new(font_mgr: impl Into<FontMgr>) -> Self {
        Self {
            font_mgr: font_mgr.into(),
        }
    }
}

#[cfg(feature = "ureq")]
impl ResourceProvider for UReqResourceProvider {
    fn load(&self, resource_path: &str, resource_name: &str) -> Option<Data> {
        match helpers::identify_resource_kind(resource_path, resource_name) {
            ResourceKind::Base64(data) => Some(data),
            ResourceKind::DownloadFrom(url) => match ureq::get(&url).call() {
                Ok(response) => {
                    let mut reader = response.into_reader();
                    let mut data = Vec::new();
                    if reader.read_to_end(&mut data).is_err() {
                        data.clear();
                    };
                    Some(Data::new_copy(&data))
                }
                Err(_) => None,
            },
        }
    }

    fn load_typeface(&self, name: &str, url: &str) -> Option<Typeface> {
        helpers::load_typeface(self, &self.font_mgr, name, url)
    }
}

/// Helpers that assist in implementing resource providers
pub mod helpers {
    use super::ResourceProvider;
    use crate::{Data, FontMgr, FontStyle, Typeface};

    /// Load a typeface via the `load()` function and generate it using a `FontMgr` instance.
    pub fn load_typeface(
        provider: &dyn ResourceProvider,
        font_mgr: &FontMgr,
        name: &str,
        url: &str,
    ) -> Option<Typeface> {
        if let Some(data) = provider.load(url, name) {
            return font_mgr.new_from_data(&data, None);
        }
        // Try to provide the default font if downloading fails.
        font_mgr.legacy_make_typeface(None, FontStyle::default())
    }

    #[derive(Debug)]
    pub enum ResourceKind {
        /// Data is base64, return it as is.
        Base64(Data),
        /// Attempt to download the data from the given Url.
        DownloadFrom(String),
    }

    /// Figure out the kind of data that should be loaded.
    pub fn identify_resource_kind(resource_path: &str, resource_name: &str) -> ResourceKind {
        const IS_WINDOWS_TARGET: bool = cfg!(target_os = "windows");

        if resource_path.is_empty() && (!IS_WINDOWS_TARGET || resource_name.starts_with("data:")) {
            return ResourceKind::Base64(load_base64(resource_name));
        }

        ResourceKind::DownloadFrom(if IS_WINDOWS_TARGET {
            resource_name.to_string()
        } else {
            format!("{resource_path}/{resource_name}")
        })
    }

    /// Try to load base64 data. Returns empty if data can not be loaded.
    fn load_base64(data: &str) -> Data {
        let data: Vec<_> = data.split(',').collect();
        if data.len() > 1 {
            let result = decode_base64(data[1]);
            return Data::new_copy(result.as_slice());
        }
        Data::new_empty()
    }

    type StaticCharVec = &'static [char];

    const HTML_SPACE_CHARACTERS: StaticCharVec =
        &['\u{0020}', '\u{0009}', '\u{000a}', '\u{000c}', '\u{000d}'];

    // https://github.com/servo/servo/blob/1610bd2bc83cea8ff0831cf999c4fba297788f64/components/script/dom/window.rs#L575
    fn decode_base64(value: &str) -> Vec<u8> {
        fn is_html_space(c: char) -> bool {
            HTML_SPACE_CHARACTERS.iter().any(|&m| m == c)
        }
        let without_spaces = value
            .chars()
            .filter(|&c| !is_html_space(c))
            .collect::<String>();

        base64::decode(&without_spaces).unwrap_or_default()
    }

    mod base64 {
        use base64::{
            alphabet,
            engine::{self, GeneralPurposeConfig},
            Engine,
        };

        pub fn decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
            ENGINE.decode(input)
        }

        const ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(
            &alphabet::STANDARD,
            GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true),
        );
    }

    #[test]
    fn decoding_base64() {
        use std::str::from_utf8;

        // padding length of 0-2 should be supported
        assert_eq!("Hello", from_utf8(&decode_base64("SGVsbG8=")).unwrap());
        assert_eq!("Hello!", from_utf8(&decode_base64("SGVsbG8h")).unwrap());
        assert_eq!(
            "Hello!!",
            from_utf8(&decode_base64("SGVsbG8hIQ==")).unwrap()
        );

        // padding length of 3 is invalid
        assert_eq!(0, decode_base64("SGVsbG8hIQ===").len());

        // if input length divided by 4 gives a remainder of 1 after padding removal, it's invalid
        assert_eq!(0, decode_base64("SGVsbG8hh").len());
        assert_eq!(0, decode_base64("SGVsbG8hh=").len());
        assert_eq!(0, decode_base64("SGVsbG8hh==").len());

        // invalid characters in the input
        assert_eq!(0, decode_base64("$GVsbG8h").len());
    }
}
