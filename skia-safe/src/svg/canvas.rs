use crate::interop::DynamicMemoryWStream;
use crate::prelude::*;
use crate::{Data, Rect};
use skia_bindings::{
    C_SkCanvas_delete, C_SkSVGCanvas_Make, C_SkXMLStreamWriter_destruct, SkCanvas,
    SkXMLStreamWriter,
};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::ptr;

pub struct Canvas {
    canvas: *mut SkCanvas,
    xml_stream_writer: Option<Pin<Box<XMLStreamWriter>>>,
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
        let mut xml_stream_writer = Box::pin(XMLStreamWriter::from_native(unsafe {
            SkXMLStreamWriter::new(&mut stream.native_mut()._base)
        }));
        let canvas = unsafe {
            C_SkSVGCanvas_Make(bounds.native(), &mut xml_stream_writer.native_mut()._base)
        };
        Canvas {
            canvas,
            xml_stream_writer: Some(xml_stream_writer),
            stream,
        }
    }

    /// Ends the Canvas drawing and returns the resulting SVG.
    /// TODO: rename to into_svg() or into_svg_data()?
    pub fn end(mut self) -> Data {
        // note: flushing canvas + XMLStreamWriter does not seem to work,
        // we have to delete the canvas and destruct the stream writer
        // to get all data out _and_ keep the referential integrity.
        unsafe {
            C_SkCanvas_delete(self.canvas);
        }
        self.canvas = ptr::null_mut();
        self.xml_stream_writer = None;
        self.stream.detach_as_data()
    }
}

type XMLStreamWriter = Handle<SkXMLStreamWriter>;

impl NativeDrop for SkXMLStreamWriter {
    fn drop(&mut self) {
        unsafe {
            C_SkXMLStreamWriter_destruct(self);
        }
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
    dbg!(&contents);
    assert!(contents
        .contains(r#"<ellipse fill="rgb(0,0,0)" stroke="none" cx="10" cy="10" rx="10" ry="10"/>"#));
    assert!(contents.contains(r#"</svg>"#));
}

#[test]
fn test_svg_without_ending() {
    use crate::Paint;
    let mut canvas = Canvas::new(&Rect::from_size((20, 20)));
    let paint = Paint::default();
    canvas.draw_circle((10, 10), 10.0, &paint);
}
