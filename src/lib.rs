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

    pub(crate) trait RefCount : Drop {
        fn refer(&self);
    }

    pub(crate) struct RefCounted<T: RefCount> {
        inner: T
    }

    impl<T: RefCount> Deref for RefCounted<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl<T: RefCount> From<T> for RefCounted<T> {
        fn from(value: T) -> Self {
            RefCounted { inner: value }
        }
    }

    impl<T: RefCount + Clone> Clone for RefCounted<T> {

        fn clone(&self) -> Self {
            self.inner.refer();
            self.inner.clone().into()
        }
    }

    pub(crate) trait Native<T> {
        fn native(&self) -> T;
    }
}