use std::{
    error::Error,
    fmt,
    io::{self, Read},
    str::FromStr,
};

use skia_bindings as sb;
use skia_bindings::{SkData, SkTypeface};

use crate::{
    interop::{MemoryStream, NativeStreamBase, RustStream},
    prelude::*,
    Canvas, Data, RCHandle, Size, Typeface,
};

pub type Dom = RCHandle<sb::SkSVGDOM>;
require_base_type!(sb::SkSVGDOM, sb::SkRefCnt);

unsafe impl Send for Dom {}

unsafe impl Sync for Dom {}

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

extern "C" fn handle_load_type_face(
    resource_path: *const ::std::os::raw::c_char,
    resource_name: *const ::std::os::raw::c_char,
) -> *mut SkTypeface {
    let data = Data::from_ptr(handle_load(resource_path, resource_name));
    match data {
        None => {}
        Some(data) => {
            if let Some(typeface) = Typeface::from_data(data, None) {
                return typeface.into_ptr();
            }
        }
    }

    Typeface::default().into_ptr()
}

extern "C" fn handle_load(
    resource_path: *const ::std::os::raw::c_char,
    resource_name: *const ::std::os::raw::c_char,
) -> *mut SkData {
    unsafe {
        let mut is_base64 = false;
        if resource_path.is_null() {
            is_base64 = true;
        }

        let resource_path = std::ffi::CStr::from_ptr(resource_path);
        let resource_name = std::ffi::CStr::from_ptr(resource_name);

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

impl FromStr for Dom {
    type Err = LoadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

impl Dom {
    fn handle_load_base64(data: &str) -> Data {
        let data: Vec<_> = data.split(',').collect();
        if data.len() > 1 {
            let result = decode_base64(data[1]);
            return Data::new_copy(result.as_slice());
        }
        Data::new_empty()
    }

    pub fn read<R: io::Read>(mut reader: R) -> Result<Self, LoadError> {
        let mut reader = RustStream::new(&mut reader);
        let stream = reader.stream_mut();

        let out = unsafe {
            sb::C_SkSVGDOM_MakeFromStream(stream, Some(handle_load), Some(handle_load_type_face))
        };

        Self::from_ptr(out).ok_or(LoadError)
    }

    pub fn from_bytes(svg: &[u8]) -> Result<Self, LoadError> {
        let mut ms = MemoryStream::from_bytes(svg);

        let out = unsafe {
            sb::C_SkSVGDOM_MakeFromStream(
                ms.native_mut().as_stream_mut(),
                Some(handle_load),
                Some(handle_load_type_face),
            )
        };
        Self::from_ptr(out).ok_or(LoadError)
    }

    pub fn render(&self, canvas: &mut Canvas) {
        // TODO: may be we should init ICU whenever we expose a Canvas?
        #[cfg(all(feature = "embed-icudtl", feature = "textlayout"))]
        crate::icu::init();

        unsafe { sb::SkSVGDOM::render(self.native() as &_, canvas.native_mut()) }
    }

    pub fn set_container_size(&mut self, size: impl Into<Size>) {
        let size = size.into();
        unsafe { sb::C_SkSVGDOM_setContainerSize(self.native_mut(), size.native()) }
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
    use crate::modules::svg::decode_base64;
    use crate::Canvas;

    use super::Dom;

    #[test]
    fn render_simple_svg() {
        // https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/410.svg
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
            <path d="M30,1h40l29,29v40l-29,29h-40l-29-29v-40z" stroke="#;000" fill="none"/>
            <path d="M31,3h38l28,28v38l-28,28h-38l-28-28v-38z" fill="#a23"/>
            <text x="50" y="68" font-size="48" fill="#FFF" text-anchor="middle"><![CDATA[410]]></text>
            </svg>"##;
        let mut canvas = Canvas::new((256, 256), None).unwrap();
        let dom = str::parse::<Dom>(svg).unwrap();
        dom.render(&mut canvas)
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
