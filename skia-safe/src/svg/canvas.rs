//! A [`crate::svg::Canvas`] that records drawing operations and emits them as SVG.
use std::{fmt, ops::Deref, ptr};

use skia_bindings::{self as sb, SkCanvas, SkDynamicMemoryWStream};

use crate::{Data, Rect, interop::DynamicMemoryWStream, prelude::*};

pub struct Canvas {
    canvas: *mut SkCanvas,
    /// The stream the SVG canvas writes into.
    ///
    /// C++ keeps a pointer to the stream until the canvas is deleted. The stream lives on the
    /// heap, so its address stays the same when this `Canvas` moves. It is held through a raw
    /// pointer instead of a `Box`, because Rust assumes that nothing else changes the contents of
    /// a `Box` while the value that owns it is passed to a function. C++ writes to the stream
    /// during such calls, for example when [`Canvas::end()`] deletes the canvas, and with a `Box`
    /// that would be undefined behavior.
    stream: ptr::NonNull<DynamicMemoryWStream>,
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            sb::C_SkCanvas_delete(self.canvas);
        }
        drop(unsafe { Box::from_raw(self.stream.as_ptr()) });
    }
}

impl Deref for Canvas {
    type Target = crate::Canvas;

    fn deref(&self) -> &Self::Target {
        unsafe { crate::Canvas::borrow_from_native(self.canvas) }
    }
}

bitflags! {
    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Flags : u32 {
        const CONVERT_TEXT_TO_PATHS = sb::SkSVGCanvas_Flags_kConvertTextToPaths_Flag as _;
        const NO_PRETTY_XML = sb::SkSVGCanvas_Flags_kNoPrettyXML_Flag as _;
        const RELATIVE_PATH_ENCODING = sb::SkSVGCanvas_Flags_kRelativePathEncoding_Flag as _;
    }
}

impl fmt::Debug for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Canvas")
            .field("canvas", unsafe {
                crate::Canvas::borrow_from_native(self.canvas)
            })
            .field("stream", unsafe { self.stream.as_ref() })
            .finish()
    }
}

impl Canvas {
    /// Creates a new SVG canvas.
    pub fn new(bounds: impl AsRef<Rect>, flags: impl Into<Option<Flags>>) -> Canvas {
        let bounds = bounds.as_ref();
        let flags = flags.into().unwrap_or_default();
        let stream =
            ptr::NonNull::new(Box::into_raw(Box::new(DynamicMemoryWStream::new()))).unwrap();
        let canvas = unsafe {
            sb::C_SkSVGCanvas_Make(
                bounds.native(),
                &raw mut (*stream.as_ptr().cast::<SkDynamicMemoryWStream>())._base,
                flags.bits(),
            )
        };
        Canvas { canvas, stream }
    }

    /// Ends the Canvas drawing and returns the resulting SVG.
    /// TODO: rename to into_svg() or into_svg_data()?
    pub fn end(mut self) -> Data {
        // note: flushing canvas + XMLStreamWriter does not seem to work,
        // we have to delete the canvas and destruct the stream writer
        // to get all data out _and_ keep the referential integrity.
        unsafe {
            sb::C_SkCanvas_delete(self.canvas);
        }
        self.canvas = ptr::null_mut();
        unsafe { (*self.stream.as_ptr()).detach_as_data() }
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;
    use crate::Rect;

    #[test]
    fn test_svg() {
        use crate::Paint;

        let canvas = Canvas::new(Rect::from_size((20, 20)), None);
        let paint = Paint::default();
        canvas.draw_circle((10, 10), 10.0, &paint);
        let data = canvas.end();
        let contents = String::from_utf8_lossy(data.as_bytes());
        dbg!(&contents);
        assert!(contents.contains(r#"<ellipse cx="10" cy="10" rx="10" ry="10"/>"#));
        assert!(contents.contains(r#"</svg>"#));
    }

    #[test]
    fn test_svg_without_ending() {
        use crate::Paint;
        let canvas = Canvas::new(Rect::from_size((20, 20)), None);
        let paint = Paint::default();
        canvas.draw_circle((10, 10), 10.0, &paint);
    }
}
