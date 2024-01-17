use std::{
    error::Error,
    ffi::CStr,
    fmt,
    io::{self, Read},
    os::raw,
};

use skia_bindings as sb;
use skia_bindings::{SkData, SkTypeface};

use crate::{
    interop::{MemoryStream, NativeStreamBase, RustStream},
    prelude::*,
    Canvas, Data, FontMgr, FontStyle, RCHandle, Size,
};

pub type Dom = RCHandle<sb::SkSVGDOM>;
require_base_type!(sb::SkSVGDOM, sb::SkRefCnt);
unsafe_send_sync!(Dom);

impl NativeRefCounted for sb::SkSVGDOM {
    fn _ref(&self) {
        unsafe { sb::C_SkSVGDOM_ref(self) }
    }

    fn _unref(&self) {
        unsafe { sb::C_SkSVGDOM_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { sb::C_SkSVGDOM_unique(self) }
    }
}

/// Error when something goes wrong when loading an SVG file. Sadly, Skia doesn't give further
/// details so we can't return a more expressive error type, but we still use this instead of
/// `Option` to express the intent and allow for `Try`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoadError;

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to load svg (reason unknown)")
    }
}

impl Error for LoadError {
    fn description(&self) -> &str {
        "Failed to load svg (reason unknown)"
    }
}

impl From<LoadError> for io::Error {
    fn from(other: LoadError) -> Self {
        io::Error::new(io::ErrorKind::Other, other)
    }
}

#[derive(Debug)]
#[repr(C)]
struct LoadContext {
    font_mgr: FontMgr,
}

impl LoadContext {
    fn new(font_mgr: FontMgr) -> Self {
        Self { font_mgr }
    }

    fn native(&mut self) -> *mut raw::c_void {
        self as *mut _ as _
    }
}

extern "C" fn handle_load_type_face(
    resource_path: *const raw::c_char,
    resource_name: *const raw::c_char,
    load_context: *mut raw::c_void,
) -> *mut SkTypeface {
    let data = Data::from_ptr(handle_load(resource_path, resource_name, load_context));
    let load_context: &mut LoadContext = unsafe { &mut *(load_context as *mut LoadContext) };
    if let Some(data) = data {
        if let Some(typeface) = load_context.font_mgr.new_from_data(&data, None) {
            return typeface.into_ptr();
        }
    }

    load_context
        .font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap()
        .into_ptr()
}

extern "C" fn handle_load(
    resource_path: *const raw::c_char,
    resource_name: *const raw::c_char,
    _load_context: *mut raw::c_void,
) -> *mut SkData {
    unsafe {
        let mut is_base64 = false;
        if resource_path.is_null() {
            is_base64 = true;
        }

        let resource_path = CStr::from_ptr(resource_path);
        let resource_name = CStr::from_ptr(resource_name);

        if resource_path.to_string_lossy().is_empty() {
            is_base64 = true;
        }

        if cfg!(windows) && !resource_name.to_string_lossy().starts_with("data:") {
            is_base64 = false;
        }

        if is_base64 {
            let data = Dom::handle_load_base64(resource_name.to_string_lossy().as_ref());
            data.into_ptr()
        } else {
            // url returned in the resource_name on windows
            // https://github.com/rust-skia/rust-skia/pull/569#issuecomment-978034696
            let path = if cfg!(windows) {
                resource_name.to_string_lossy().to_string()
            } else {
                format!(
                    "{}/{}",
                    resource_path.to_string_lossy(),
                    resource_name.to_string_lossy()
                )
            };

            match ureq::get(&path).call() {
                Ok(response) => {
                    let mut reader = response.into_reader();
                    let mut data = Vec::new();
                    if reader.read_to_end(&mut data).is_err() {
                        data.clear();
                    };
                    let data = Data::new_copy(&data);
                    data.into_ptr()
                }
                Err(_) => {
                    let data = Data::new_empty();
                    data.into_ptr()
                }
            }
        }
    }
}

impl fmt::Debug for Dom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dom").finish()
    }
}

impl Dom {
    pub fn read<R: io::Read>(
        mut reader: R,
        font_mgr: impl Into<FontMgr>,
    ) -> Result<Self, LoadError> {
        let mut reader = RustStream::new(&mut reader);
        let stream = reader.stream_mut();
        let mut load_context = LoadContext::new(font_mgr.into());

        let out = unsafe {
            sb::C_SkSVGDOM_MakeFromStream(
                stream,
                Some(handle_load),
                Some(handle_load_type_face),
                load_context.native(),
            )
        };

        Self::from_ptr(out).ok_or(LoadError)
    }

    pub fn from_str(svg: impl AsRef<str>, font_mgr: impl Into<FontMgr>) -> Result<Self, LoadError> {
        Self::from_bytes(svg.as_ref().as_bytes(), font_mgr)
    }

    pub fn from_bytes(svg: &[u8], font_mgr: impl Into<FontMgr>) -> Result<Self, LoadError> {
        let mut ms = MemoryStream::from_bytes(svg);
        let mut load_context = LoadContext::new(font_mgr.into());

        let out = unsafe {
            sb::C_SkSVGDOM_MakeFromStream(
                ms.native_mut().as_stream_mut(),
                Some(handle_load),
                Some(handle_load_type_face),
                load_context.native(),
            )
        };
        Self::from_ptr(out).ok_or(LoadError)
    }

    pub fn render(&self, canvas: &Canvas) {
        // TODO: may be we should init ICU whenever we expose a Canvas?
        #[cfg(all(feature = "embed-icudtl", feature = "textlayout"))]
        crate::icu::init();

        unsafe { sb::SkSVGDOM::render(self.native() as &_, canvas.native_mut()) }
    }

    pub fn set_container_size(&mut self, size: impl Into<Size>) {
        let size = size.into();
        unsafe { sb::C_SkSVGDOM_setContainerSize(self.native_mut(), size.native()) }
    }

    fn handle_load_base64(data: &str) -> Data {
        let data: Vec<_> = data.split(',').collect();
        if data.len() > 1 {
            let result = decode_base64(data[1]);
            return Data::new_copy(result.as_slice());
        }
        Data::new_empty()
    }
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

    match base64::decode(&without_spaces) {
        Ok(bytes) => bytes,
        Err(_) => Vec::new(),
    }
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

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, path::Path};

    use super::Dom;
    use crate::{modules::svg::decode_base64, surfaces, EncodedImageFormat, FontMgr, Surface};

    #[test]
    fn render_simple_svg() {
        // https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/410.svg
        // Note: height and width must be set
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" height = "256" width = "256">
            <path d="M30,1h40l29,29v40l-29,29h-40l-29-29v-40z" stroke="#;000" fill="none"/>
            <path d="M31,3h38l28,28v38l-28,28h-38l-28-28v-38z" fill="#a23"/>
            <text x="50" y="68" font-size="48" fill="#FFF" text-anchor="middle"><![CDATA[410]]></text>
            </svg>"##;
        let mut surface = surfaces::raster_n32_premul((256, 256)).unwrap();
        let canvas = surface.canvas();
        let font_mgr = FontMgr::new();
        let dom = Dom::from_str(svg, font_mgr).unwrap();
        dom.render(canvas);
        // save_surface_to_tmp(surface);
    }

    #[allow(unused)]
    fn save_surface_to_tmp(surface: &mut Surface) {
        let image = surface.image_snapshot();
        let data = image.encode(None, EncodedImageFormat::PNG, None).unwrap();
        write_file(data.as_bytes(), Path::new("/tmp/test.png"));

        pub fn write_file(bytes: &[u8], path: &Path) {
            let mut file = File::create(path).expect("failed to create file");
            file.write_all(bytes).expect("failed to write to file");
        }
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
