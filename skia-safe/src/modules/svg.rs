use crate::{
    interop::{MemoryStream, NativeStreamBase, RustStream},
    prelude::*,
    Canvas, Data, RCHandle, Size, Typeface,
};
use skia_bindings as sb;
use skia_bindings::{SkData, SkTypeface};
use std::{
    error::Error,
    fmt,
    io::{self, Read},
};

pub type Dom = RCHandle<sb::SkSVGDOM>;

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

        if is_base64 {
            let data = Dom::handle_load_base64(resource_name.to_string_lossy().as_ref());
            data.into_ptr()
        } else {
            let path = format!(
                "{}/{}",
                resource_path.to_string_lossy(),
                resource_name.to_string_lossy()
            );
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
        unsafe { sb::SkSVGDOM::render(self.native() as &_, canvas.native_mut()) }
    }

    pub fn container_size(&mut self, size: &Size) {
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
    let mut input = &*without_spaces;

    if input.len() % 4 == 0 {
        if input.ends_with("==") {
            input = &input[..input.len() - 2]
        } else if input.ends_with('=') {
            input = &input[..input.len() - 1]
        }
    }

    if input.len() % 4 == 1 {
        return Vec::new();
    }

    if input
        .chars()
        .any(|c| c != '+' && c != '/' && !c.is_alphanumeric())
    {
        return Vec::new();
    }
    match base64::decode_config(&input, base64::STANDARD.decode_allow_trailing_bits(true)) {
        Ok(bytes) => bytes,
        Err(_) => Vec::new(),
    }
}
