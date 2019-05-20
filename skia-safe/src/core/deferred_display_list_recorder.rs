use crate::{Canvas, SurfaceCharacterization};

// TODO: complete implementation
pub enum DeferredDisplayListRecorder {}
// TODO: declared in core/private/, so be opaque I guess.
pub enum DeferredDisplayList {}

impl DeferredDisplayListRecorder {
    pub fn new(characterization: &SurfaceCharacterization) -> Self {
        unimplemented!()
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        unimplemented!()
    }

    pub fn detach(&mut self) -> DeferredDisplayList {
        unimplemented!()
    }

    // TODO: makePromiseTexture()?
    // TODO: makeYUVAPromiseTexture()?
}
