use crate::interop::DynamicMemoryWStream;
use crate::prelude::*;
use crate::{Canvas, Data, Rect, Size};
use skia_bindings::{SkDocument, SkRefCntBase};
use std::{pin::Pin, ptr};

pub struct Document<State = state::Open> {
    // note: order matters here, first the document must be
    // dropped _and then_ the stream.
    document: RCHandle<SkDocument>,
    stream: Pin<Box<DynamicMemoryWStream>>,

    state: State,
}

impl NativeRefCountedBase for SkDocument {
    type Base = SkRefCntBase;
}

pub mod state {
    use skia_bindings::SkCanvas;
    use std::ptr;

    /// Document is currently open. May contain several pages.
    pub struct Open {
        pub(crate) pages: usize,
    }

    /// Document is currently on a page and can be drawn onto.
    pub struct OnPage {
        pub(crate) canvas: ptr::NonNull<SkCanvas>,
        pub(crate) page: usize,
    }
}

impl<S> Document<S> {
    pub fn abort(mut self) {
        unsafe { self.document.native_mut().abort() }
        drop(self)
    }
}

impl Document {
    pub(crate) fn new(
        stream: Pin<Box<DynamicMemoryWStream>>,
        document: RCHandle<SkDocument>,
    ) -> Self {
        Document {
            document,
            stream,
            state: state::Open { pages: 0 },
        }
    }

    /// The number of pages in this document.
    pub fn pages(&self) -> usize {
        self.state.pages
    }

    // This function consumes the document and returns a document containing a
    // canvas that represents the page it's currently drawing on.
    pub fn begin_page(
        mut self,
        size: impl Into<Size>,
        content: Option<&Rect>,
    ) -> Document<state::OnPage> {
        let size = size.into();
        let canvas = unsafe {
            self.document.native_mut().beginPage(
                size.width,
                size.height,
                content.native_ptr_or_null(),
            )
        };

        Document {
            stream: self.stream,
            document: self.document,
            state: state::OnPage {
                canvas: ptr::NonNull::new(canvas).unwrap(),
                page: self.state.pages + 1,
            },
        } as _
    }

    /// Close the document and return the encoded representation.
    /// This function consumes and drops the document.
    pub fn close(mut self) -> Data {
        unsafe {
            self.document.native_mut().close();
        };
        self.stream.detach_as_data()
    }
}

impl Document<state::OnPage> {
    /// The current page we are currently drawing on.
    pub fn page(&self) -> usize {
        self.state.page
    }

    /// Borrows the canvas for the current page on the document.
    pub fn canvas(&mut self) -> &mut Canvas {
        Canvas::borrow_from_native(unsafe { self.state.canvas.as_mut() })
    }

    /// Ends the page.
    /// This function consumes the document and returns a new open document that
    /// contains the pages drawn so far.
    pub fn end_page(mut self) -> Document {
        unsafe {
            self.document.native_mut().endPage();
        }

        Document {
            stream: self.stream,
            document: self.document,
            state: state::Open {
                pages: self.state.page,
            },
        }

        // TODO: think about providing a close() function that implicitly ends the page
        //       and calls close() on the Open document.
        // TODO: think about providing a begin_page() function that implicitly ends the
        //       current page.
    }
}
