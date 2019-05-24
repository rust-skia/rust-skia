use crate::{Canvas, Picture, Matrix, Paint};
use std::marker::PhantomData;

// TODO: complete the implementation:

/*
#[repr(C)]
pub struct MultiPictureDraw<'a>(*mut SkMultiPictureDraw, PhantomData<&'a ()>);
*/

pub struct MultiPictureDraw<'a>(PhantomData<&'a ()>);

impl<'a> MultiPictureDraw<'a> {
    pub fn new(reserve: impl Into<Option<usize>>) -> MultiPictureDraw<'a> {
        unimplemented!()
    }

    #[must_use]
    pub fn add<'b>(self, canvas: &'b Canvas, picture: &'b Picture, matrix: Option<&'b Matrix>, paint: Option<&'b Paint>) -> MultiPictureDraw<'b> {
        unimplemented!()
    }

    #[must_use]
    pub fn draw<'b>(self, flush: impl Into<Option<bool>>) -> MultiPictureDraw<'b> {
        unimplemented!()
    }

    #[must_use]
    pub fn reset<'b>(self) -> MultiPictureDraw<'b> {
        unimplemented!()
    }
}
