use crate::{Canvas, Data, Point, Rect};

pub mod annotate {
    //! Annotates a [`crate::Canvas`] with URLs, named destinations, and links to destinations,
    //! for backends that support annotations, such as PDF.
    use crate::prelude::*;
    use crate::{Canvas, Data, Point, Rect};
    use skia_bindings::{
        SkAnnotateLinkToDestination, SkAnnotateNamedDestination, SkAnnotateRectWithURL,
    };

    /// Annotates the canvas by associating the specified URL with the specified rectangle (in
    /// local coordinates, just like `drawRect`).
    ///
    /// The URL is expected to be escaped and be valid 7-bit ASCII.
    ///
    /// If the backend of this canvas does not support annotations, this call is safely ignored.
    ///
    /// - `canvas` canvas to annotate
    /// - `rect` rectangle to associate with the URL
    /// - `data` URL data
    pub fn rect_with_url(canvas: &Canvas, rect: impl AsRef<Rect>, data: &Data) {
        unsafe {
            SkAnnotateRectWithURL(
                canvas.native_ptr(),
                rect.as_ref().native(),
                data.native_mut_force(),
            )
        }
    }

    /// Annotates the canvas by associating a name with the specified point.
    ///
    /// If the backend of this canvas does not support annotations, this call is safely ignored.
    ///
    /// - `canvas` canvas to annotate
    /// - `point` point to associate with the name
    /// - `data` name data
    pub fn named_destination(canvas: &Canvas, point: impl Into<Point>, data: &Data) {
        unsafe {
            SkAnnotateNamedDestination(
                canvas.native_ptr(),
                point.into().native(),
                data.native_mut_force(),
            )
        }
    }

    /// Annotates the canvas by making the specified rectangle link to a named destination.
    ///
    /// If the backend of this canvas does not support annotations, this call is safely ignored.
    ///
    /// - `canvas` canvas to annotate
    /// - `rect` rectangle to link
    /// - `data` destination name data
    pub fn link_to_destination(canvas: &Canvas, rect: impl AsRef<Rect>, data: &Data) {
        unsafe {
            SkAnnotateLinkToDestination(
                canvas.native_ptr(),
                rect.as_ref().native(),
                data.native_mut_force(),
            )
        }
    }
}

impl Canvas {
    // TODO: accept str or the Url type from the url crate?
    /// Annotates the canvas by associating the specified URL with the specified rectangle (in
    /// local coordinates, just like `drawRect`).
    ///
    /// The URL is expected to be escaped and be valid 7-bit ASCII.
    ///
    /// If the backend of this canvas does not support annotations, this call is safely ignored.
    ///
    /// - `rect` rectangle to associate with the URL
    /// - `data` URL data
    pub fn annotate_rect_with_url(&self, rect: impl AsRef<Rect>, data: &Data) -> &Self {
        annotate::rect_with_url(self, rect, data);
        self
    }

    // TODO: is data a string here, and if so, of what encoding?
    /// Annotates the canvas by associating a name with the specified point.
    ///
    /// If the backend of this canvas does not support annotations, this call is safely ignored.
    ///
    /// - `point` point to associate with the name
    /// - `data` name data
    pub fn annotate_named_destination(&self, point: impl Into<Point>, data: &Data) -> &Self {
        annotate::named_destination(self, point, data);
        self
    }

    // TODO: use str?
    /// Annotates the canvas by making the specified rectangle link to a named destination.
    ///
    /// If the backend of this canvas does not support annotations, this call is safely ignored.
    ///
    /// - `rect` rectangle to link
    /// - `data` destination name data
    pub fn annotate_link_to_destination(&self, rect: impl AsRef<Rect>, data: &Data) -> &Self {
        annotate::link_to_destination(self, rect, data);
        self
    }
}
