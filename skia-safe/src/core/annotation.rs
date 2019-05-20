use crate::{Canvas, Rect, Point, Data};

pub enum Annotate {}

impl Annotate {
    pub fn rect_with_url(canvas: &mut Canvas, rect: impl AsRef<Rect>, data: &Data) {
        unimplemented!()
    }

    pub fn named_destination(canvas: &mut Canvas, point: impl Into<Point>, data: &Data)  {
        unimplemented!()
    }

    pub fn link_to_destination(canvas: &mut Canvas, rect: impl AsRef<Rect>, data: &Data) {
        unimplemented!()
    }
}

impl Canvas {
    // TODO: explicit URL as str (or is there an URL type in std?)
    pub fn annotate_rect_with_url(&mut self, rect: impl AsRef<Rect>, data: &Data) -> &mut Self {
        Annotate::rect_with_url(self, rect, data);
        self
    }

    // TODO: is data a string here, and if so which encoding?
    pub fn annotate_named_destination(&mut self, point: impl Into<Point>, data: &Data) -> &mut Self {
        Annotate::named_destination(self, point, data);
        self
    }

    // TODO: explicit str?
    pub fn annotate_link_to_destination(&mut self, rect: impl AsRef<Rect>, data: &Data) -> &mut Self {
        Annotate::link_to_destination(self, rect, data);
        self
    }
}
