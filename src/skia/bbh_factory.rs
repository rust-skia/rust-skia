use crate::prelude::*;
use rust_skia::SkBBHFactory;

// TODO: complete the implementation
pub type BBHFactory = Handle<SkBBHFactory>;

impl NativeDrop for SkBBHFactory {
    fn drop(&mut self) {
        unimplemented!()
    }
}

// TODO: complete the implementation
impl Handle<SkBBHFactory> {
}