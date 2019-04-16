use crate::interop::DynamicMemoryWStream;
use crate::prelude::{NativeAccess, NativeTransmutable};
use crate::{Data, Rect};
use skia_bindings::{C_SkCanvas_delete, C_SkSVGCanvas_Make, SkCanvas, SkXMLStreamWriter};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

pub struct Canvas {
    canvas: *mut SkCanvas,
    #[allow(dead_code)]
    xml_stream_writer: Pin<Box<SkXMLStreamWriter>>,
    stream: Pin<Box<DynamicMemoryWStream>>,
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            C_SkCanvas_delete(self.canvas);
        }
    }
}

impl Deref for Canvas {
    type Target = crate::Canvas;

    fn deref(&self) -> &Self::Target {
        crate::Canvas::borrow_from_native(unsafe { &mut *self.canvas })
    }
}

impl DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        crate::Canvas::borrow_from_native(unsafe { &mut *self.canvas })
    }
}

impl Canvas {
    /// Creates a new SVG canvas.
    pub fn new<B: AsRef<Rect>>(bounds: B) -> Canvas {
        let bounds = bounds.as_ref();
        let mut stream = Box::pin(DynamicMemoryWStream::new());
        let mut xml_stream_writer =
            Box::pin(unsafe { SkXMLStreamWriter::new(&mut stream.native_mut()._base) });
        let canvas = unsafe { C_SkSVGCanvas_Make(bounds.native(), &mut xml_stream_writer._base) };
        Canvas {
            canvas,
            xml_stream_writer,
            stream,
        }
    }

    /// Ends the Canvas drawing and returns the resulting SVG.
    /// TODO: rename to into_svg() or into_svg_data()?
    pub fn end(mut self) -> Data {
        self.deref_mut().flush();
        self.stream.detach_as_data()
    }
}

#[test]
fn test_svg() {
    use crate::Paint;
    let mut canvas = Canvas::new(&Rect::from_size((20, 20)));
    let paint = Paint::default();
    canvas.draw_circle((10, 10), 10.0, &paint);
    let data = canvas.end();
    let contents = String::from_utf8_lossy(data.bytes());
    assert!(contents
        .contains(r#"<ellipse fill="rgb(0,0,0)" stroke="none" cx="10" cy="10" rx="10" ry="10"/>"#));
}
