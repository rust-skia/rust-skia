use std::{
    error::Error,
    fmt,
    io::{self},
};

use skia_bindings as sb;

use super::resources::NativeResourceProvider;
use crate::{
    interop::{MemoryStream, NativeStreamBase, RustStream},
    prelude::*,
    Canvas, FontMgr, Size,
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

impl fmt::Debug for Dom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dom").finish()
    }
}

impl Dom {
    pub fn read<R: io::Read>(
        mut reader: R,
        resource_provider: impl Into<NativeResourceProvider>,
        font_mgr: impl Into<FontMgr>,
    ) -> Result<Self, LoadError> {
        let mut reader = RustStream::new(&mut reader);
        let stream = reader.stream_mut();
        let resource_provider = resource_provider.into();
        let font_mgr = font_mgr.into();

        let out = unsafe {
            sb::C_SkSVGDOM_MakeFromStream(stream, resource_provider.into_ptr(), font_mgr.into_ptr())
        };

        Self::from_ptr(out).ok_or(LoadError)
    }

    pub fn from_str(
        svg: impl AsRef<str>,
        resource_provider: impl Into<NativeResourceProvider>,
        font_mgr: impl Into<FontMgr>,
    ) -> Result<Self, LoadError> {
        Self::from_bytes(svg.as_ref().as_bytes(), resource_provider, font_mgr)
    }

    pub fn from_bytes(
        svg: &[u8],
        resource_provider: impl Into<NativeResourceProvider>,
        font_mgr: impl Into<FontMgr>,
    ) -> Result<Self, LoadError> {
        let mut ms = MemoryStream::from_bytes(svg);
        let resource_provider = resource_provider.into();
        let font_mgr = font_mgr.into();

        let out = unsafe {
            sb::C_SkSVGDOM_MakeFromStream(
                ms.native_mut().as_stream_mut(),
                resource_provider.into_ptr(),
                font_mgr.into_ptr(),
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
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, path::Path};

    use super::Dom;
    use crate::{surfaces, EncodedImageFormat, FontMgr, Surface};

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
        let dom = Dom::from_str(svg, font_mgr.clone(), font_mgr).unwrap();
        dom.render(canvas);
        // save_surface_to_tmp(&mut surface);
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
}
