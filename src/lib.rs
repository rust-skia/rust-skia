pub mod graphics;
pub mod skia;
mod skia_euclid;

#[macro_use]
extern crate bitflags;

// temporariliy required for the canvas example.
pub mod bindings {
    pub use rust_skia::*;
}

mod prelude {
    use rust_skia::*;
    use std::intrinsics::transmute;

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

    pub trait RefCount {
        fn ref_cnt(&self) -> i32;
    }

    impl RefCount for SkRefCntBase {
        // the problem here is that the binding generator represents std::atomic as an u8 (we
        // are lucky that the C alignment rules make space for an i32), so to get the ref
        // counter, we need to get the u8 pointer to fRefCnt and interpret it as an i32 pointer.
        fn ref_cnt(&self) -> i32 {
            let ptr: *const i32 = unsafe { transmute(&self.fRefCnt) };
            unsafe { *ptr }
        }
    }
    impl RefCount for SkRefCnt {
        fn ref_cnt(&self) -> i32 {
            self._base.ref_cnt()
        }
    }

    impl RefCount for SkNVRefCnt {
        fn ref_cnt(&self) -> i32 {
            let ptr: *const i32 = unsafe { transmute(&self.fRefCnt) };
            unsafe { *ptr }
        }
    }

    #[cfg(test)]
    impl RefCount for SkData {
        fn ref_cnt(&self) -> i32 {
            self._base.ref_cnt()
        }
    }

    #[cfg(test)]
    impl RefCount for SkSurface {
        fn ref_cnt(&self) -> i32 {
            self._base.ref_cnt()
        }
    }

    #[cfg(test)]
    impl RefCount for SkColorSpace {
        fn ref_cnt(&self) -> i32 {
            self._base.ref_cnt()
        }
    }

    /// Indicates that the type has a native representation and
    /// can convert to and from it. This is for cases in which we
    /// can't use the From / Into traits, because we pull in the
    /// rust type from another crate.
    pub(crate) trait NativeRepresentation<N> {
        fn to_native(&self) -> N;
        fn from_native(native: &N) -> Self;
    }
}