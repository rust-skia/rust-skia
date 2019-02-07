// temporarily re-export all bindings for now.
pub mod bindings {
    pub use rust_skia::*;
}

// the safe bindings one are accessible with skia::.
pub mod skia {
    mod canvas;
    pub use canvas::*;

    mod data;
    pub use data::*;

    mod image;
    pub use image::*;

    mod paint;
    pub use paint::*;

    mod path;
    pub use path::*;

    mod rect;
    pub use rect::*;

    mod surface;
    pub use surface::*;
}

pub mod graphics {

    #[cfg(feature = "vulkan")]
    mod vulkan;
    #[cfg(feature = "vulkan")]
    pub use vulkan::*;

    mod context;
    pub use context::*;

    mod backend_texture;
    pub use backend_texture::*;
}


#[cfg(test)]
mod tests {
}
