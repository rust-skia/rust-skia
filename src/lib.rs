pub mod graphics;
pub mod skia;

// temporariliy required for the canvas example.
pub mod bindings {
    pub use rust_skia::*;
}

mod prelude {
    use std::ops::Deref;

    pub trait ToOption {
        type Target;

        fn to_option(self) -> Option<Self::Target>;
    }

    impl<T> ToOption for *mut T {
        type Target = *mut T;

        fn to_option(self) -> Option<Self::Target> {
            if self.is_null()
            { None } else { Some(self) }
        }
    }
}