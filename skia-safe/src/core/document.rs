use crate::prelude::*;
use crate::{scalar, Rect, Canvas, Data};
use crate::interop::DynamicMemoryWStream;
use skia_bindings::{SkDocument, SkRefCntBase};

pub struct Document<State> {

    // note: order matters here, first the document must be
    // dropped _and then_ the stream.
    document: RCHandle<SkDocument>,
    stream: DynamicMemoryWStream,

    state: State,
}

impl NativeRefCountedBase for SkDocument {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

pub mod document {
    use skia_bindings::SkCanvas;

    /// Document is currently closed. May contain several pages.
    pub struct Open {
        pub(crate) pages: usize
    }

    /// Document is currently drawing on a page.
    pub struct OnPage {
        pub(crate) canvas: *mut SkCanvas,
        pub(crate) page: usize }
}

impl<S> Document<S> {

    pub fn abort(mut self) {
        unsafe {
            self.document.native_mut().abort()
        }
        drop(self)
    }
}

impl Document<document::Open> {

    /// The number of pages in this document.
    pub fn pages(&self) -> usize {
        self.state.pages
    }

    // This function consumes the document and returns a document containing a canvas that represents the
    // page it's currently drawing on.
    pub fn begin_page(mut self, (width, height) : (scalar, scalar), content: Option<&Rect>) -> Document<document::OnPage> {
        let canvas = unsafe {
            self.document.native_mut().beginPage(width, height, content.native_ptr_or_null())
        };

        Document {
            stream: self.stream,
            document: self.document,
            state: document::OnPage {
                canvas,
                page: self.state.pages + 1
            }
        }
    }

    pub fn close(mut self) -> Data {
        unsafe {
            self.document.native_mut().close();
        };
        self.stream.detach_as_data()
    }
}

impl Document<document::OnPage> {

    /// The current page we are currently drawing on.
    pub fn page(&self) -> usize {
        self.state.page
    }

    /// Borrows the canvas for this page on the document.
    pub fn canvas(&mut self) -> &mut Canvas {
        Canvas::borrow_from_native(unsafe {
            &mut *self.state.canvas
        })
    }

    /// Ends the page.
    /// This function consumes the document and returns a new open document.
    pub fn end_page(mut self) -> Document<document::Open> {
        unsafe {
            self.document.native_mut().endPage();
        }

        Document {
            stream: self.stream,
            document: self.document,
            state: document::Open {
                pages: self.state.page
            }
        }

        // TODO: think about providing a close that implicitly ends the page and calls close on the Open document.
        // TODO: think about providing a begin_page that implicitly ends the current page.
    }
}
