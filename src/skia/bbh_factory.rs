use crate::prelude::*;
use rust_skia::SkBBHFactory;

pub type BBHFactory = Handle<SkBBHFactory>;

impl NativeDrop for SkBBHFactory {
    fn drop(&mut self) {
        // TODO: complete the implementation
        unimplemented!()
    }
}

// TODO: complete the implementation
impl Handle<SkBBHFactory> {
}