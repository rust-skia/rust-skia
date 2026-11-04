//! Resource provider interfaces for loading images, fonts, and other assets used by animation and vector modules (Skia's `skresources` module).
use std::borrow::Cow;
use std::ffi::{CStr, c_void};
use std::mem;
use std::os::raw;
use std::ptr;

use helpers::ResourceKind;
use skia_bindings::{
    self as sb, RustResourceProvider, RustResourceProvider_Param, SkData, SkFontMgr, SkRefCnt,
    SkRefCntBase, SkTypeface, skresources_ImageAsset,
};

use crate::{Data, FontMgr, Typeface, prelude::*};

/// Image asset proxy interface.
pub type ImageAsset = RCHandle<skresources_ImageAsset>;
require_base_type!(skresources_ImageAsset, SkRefCnt);

impl NativeRefCountedBase for skresources_ImageAsset {
    type Base = SkRefCntBase;
}

impl ImageAsset {
    /// Returns `true` if the image asset is animated.
    pub fn is_multi_frame(&self) -> bool {
        unsafe { sb::C_ImageAsset_isMultiFrame(self.native_mut_force()) }
    }

    // TODO: wrap getFrameData()

    /// Create a multi-frame image asset from the given encoded data. Clients must register the
    /// required decoding image codecs before calling this.
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

/// Strategy for how images are decoded.
///
/// Variants:
/// - [`ImageDecodeStrategy::LazyDecode`]: Images are decoded on-the-fly, at rasterization time. Large images may cause jank as decoding is expensive (and can thrash internal caches).
/// - [`ImageDecodeStrategy::PreDecode`]: Force-decode all images upfront, at the cost of potentially more RAM and slower animation build times.
pub use sb::skresources_ImageDecodeStrategy as ImageDecodeStrategy;
variant_name!(ImageDecodeStrategy::LazyDecode);

// TODO: Wrap ExternalTrackAsset

/// ResourceProvider is an interface that lets rich-content modules defer loading of external
/// resources (images, fonts, etc.) to embedding clients.
pub trait ResourceProvider {
    /// Load a generic resource (currently only nested animations) specified by `resource_path` +
    /// `resource_name`, and return as a [`Data`].
    fn load(&self, resource_path: &str, resource_name: &str) -> Option<Data>;

    /// Load an image asset specified by `resource_path` + `resource_name`, and returns the
    /// corresponding [`ImageAsset`] proxy.
    fn load_image_asset(
        &self,
        resource_path: &str,
        resource_name: &str,
        _resource_id: &str,
    ) -> Option<ImageAsset> {
        let data = self.load(resource_path, resource_name)?;
        ImageAsset::from_data(data, None)
    }

    /// Load an external font and return as a [`Typeface`].
    ///
    /// - `name` font name
    /// - `url` web font URL
    fn load_typeface(&self, name: &str, url: &str) -> Option<Typeface>;

    /// This is used in the SVG Dom and _should_ be used for implementing `load_typeface()`.
    fn font_mgr(&self) -> FontMgr;
}

pub type NativeResourceProvider = RCHandle<RustResourceProvider>;

impl NativeRefCountedBase for RustResourceProvider {
    type Base = SkRefCntBase;
}

impl<T: ResourceProvider + 'static> From<T> for NativeResourceProvider {
    fn from(resource_provider: T) -> Self {
        native_resource_provider(Box::new(resource_provider))
    }
}

impl From<Box<dyn ResourceProvider>> for NativeResourceProvider {
    fn from(resource_provider: Box<dyn ResourceProvider>) -> Self {
        native_resource_provider(resource_provider)
    }
}

/// Hands a boxed provider to a new C++ `RustResourceProvider`, which owns it from then on.
///
/// Both `From` impls go through here, so a concrete provider and a boxed trait object reach the
/// same callbacks. C++ receives a pointer to the `Box<P>` rather than the `Box<P>` itself, because
/// a `Box<dyn ResourceProvider>` is a fat pointer and does not fit into a `void*`.
/// `boxed_resource_provider_reaches_overrides_and_drops_once` pins this.
fn native_resource_provider<P: ResourceProvider + ?Sized + 'static>(
    resource_provider: Box<P>,
) -> NativeResourceProvider {
    let param = RustResourceProvider_Param {
        data: Box::into_raw(Box::new(resource_provider)).cast(),
        drop: Some(drop::<P>),
        load: Some(load::<P>),
        loadImageAsset: Some(load_image_asset::<P>),
        loadTypeface: Some(load_typeface::<P>),
        fontMgr: Some(font_mgr::<P>),
    };

    let skia_resource_provider =
        NativeResourceProvider::from_ptr(unsafe { sb::C_RustResourceProvider_New(&param) })
            .unwrap();

    return skia_resource_provider;

    unsafe extern "C" fn drop<P: ?Sized>(provider: *mut c_void) {
        mem::drop(unsafe { Box::from_raw(provider.cast::<Box<P>>()) });
    }

    unsafe extern "C" fn load<P: ResourceProvider + ?Sized>(
        provider: *mut c_void,
        resource_path: *const raw::c_char,
        resource_name: *const raw::c_char,
    ) -> *mut SkData {
        unsafe {
            provider_ref::<P>(provider)
                .load(&uncstr(resource_path), &uncstr(resource_name))
                .map(|data| data.into_ptr())
                .unwrap_or(ptr::null_mut())
        }
    }

    unsafe extern "C" fn load_image_asset<P: ResourceProvider + ?Sized>(
        provider: *mut c_void,
        resource_path: *const raw::c_char,
        resource_name: *const raw::c_char,
        resource_id: *const raw::c_char,
    ) -> *mut skresources_ImageAsset {
        unsafe {
            provider_ref::<P>(provider)
                .load_image_asset(
                    &uncstr(resource_path),
                    &uncstr(resource_name),
                    &uncstr(resource_id),
                )
                .map(|image_asset| image_asset.into_ptr())
                .unwrap_or(ptr::null_mut())
        }
    }

    unsafe extern "C" fn load_typeface<P: ResourceProvider + ?Sized>(
        provider: *mut c_void,
        name: *const raw::c_char,
        url: *const raw::c_char,
    ) -> *mut SkTypeface {
        unsafe {
            provider_ref::<P>(provider)
                .load_typeface(&uncstr(name), &uncstr(url))
                .map(|typeface| typeface.into_ptr())
                .unwrap_or(ptr::null_mut())
        }
    }

    unsafe extern "C" fn font_mgr<P: ResourceProvider + ?Sized>(
        provider: *mut c_void,
    ) -> *mut SkFontMgr {
        unsafe { provider_ref::<P>(provider).font_mgr().into_ptr() }
    }

    /// `provider` must be the pointer that `native_resource_provider` created together with
    /// callbacks for this same `P`. It stays valid until `drop` reclaims it.
    unsafe fn provider_ref<'a, P: ?Sized>(provider: *mut c_void) -> &'a P {
        unsafe { &*provider.cast::<Box<P>>() }
    }

    /// The string borrows from `ptr`, so the caller must not let `'a` outlive the C++ call that
    /// passed it.
    unsafe fn uncstr<'a>(ptr: *const raw::c_char) -> Cow<'a, str> {
        if !ptr.is_null() {
            return unsafe { CStr::from_ptr(ptr).to_string_lossy() };
        }
        "".into()
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
            ResourceKind::DownloadFromUrl(_) => None,
        }
    }

    fn load_typeface(&self, name: &str, url: &str) -> Option<Typeface> {
        helpers::load_typeface(self, &self.font_mgr, name, url)
    }

    fn font_mgr(&self) -> FontMgr {
        self.font_mgr.clone()
    }
}

impl LocalResourceProvider {
    pub fn new(font_mgr: impl Into<FontMgr>) -> Self {
        Self {
            font_mgr: font_mgr.into(),
        }
    }
}

/// Support a direct conversion from a [`FontMgr`] into a local native resource provider.
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
            ResourceKind::DownloadFromUrl(url) => match ureq::get(&url).call() {
                Ok(response) => {
                    let mut reader = response.into_body().into_reader();
                    let mut data = Vec::new();
                    use std::io::Read;
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

    fn font_mgr(&self) -> FontMgr {
        self.font_mgr.clone()
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
            return font_mgr.new_from_bytes(&data, None);
        }
        // Try to provide the default font if downloading fails.
        font_mgr.legacy_make_typeface(None, FontStyle::default())
    }

    #[derive(Debug)]
    pub enum ResourceKind {
        /// Data is base64, return it as is.
        Base64(Data),
        /// Attempt to download the data from the given Url.
        DownloadFromUrl(String),
    }

    /// Figure out the kind of data that should be loaded.
    pub fn identify_resource_kind(resource_path: &str, resource_name: &str) -> ResourceKind {
        const IS_WINDOWS_TARGET: bool = cfg!(target_os = "windows");

        if resource_path.is_empty() && (!IS_WINDOWS_TARGET || resource_name.starts_with("data:")) {
            return ResourceKind::Base64(load_base64(resource_name));
        }

        ResourceKind::DownloadFromUrl(if IS_WINDOWS_TARGET {
            resource_name.to_string()
        } else {
            format!("{resource_path}/{resource_name}")
        })
    }

    /// Try to parse base64 data from an data: URL. Returns empty [`Data`] if data can not be parsed.
    fn load_base64(data: &str) -> Data {
        let data: Vec<_> = data.split(',').collect();
        if data.is_empty() || !data[0].ends_with(";base64") {
            return Data::new_empty();
        }

        // remove spaces
        let spaces_removed = remove_html_space_characters(data[1]);
        // decode %xx
        let percent_decoded =
            percent_encoding::percent_decode_str(&spaces_removed).decode_utf8_lossy();
        // decode base64
        let result = decode_base64(&percent_decoded);
        Data::new_copy(result.as_slice())
    }

    const HTML_SPACE_CHARACTERS: &[char] =
        &['\u{0020}', '\u{0009}', '\u{000a}', '\u{000c}', '\u{000d}'];

    // https://github.com/servo/servo/blob/1610bd2bc83cea8ff0831cf999c4fba297788f64/components/script/dom/window.rs#L575
    fn remove_html_space_characters(value: &str) -> String {
        fn is_html_space(c: char) -> bool {
            HTML_SPACE_CHARACTERS.contains(&c)
        }
        value
            .chars()
            .filter(|&c| !is_html_space(c))
            .collect::<String>()
    }

    fn decode_base64(value: &str) -> Vec<u8> {
        base64::decode(value).unwrap_or_default()
    }

    mod base64 {
        use base64::{
            Engine, alphabet,
            engine::{self, GeneralPurposeConfig},
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
