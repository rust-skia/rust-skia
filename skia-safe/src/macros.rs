#![macro_use]

/// Macro to mark a Rust type as NativeTransmutable and test its layout.
#[macro_export]
macro_rules! native_transmutable {
    ($nt:ty, $rt:ty, $test_fn:ident) => {
        impl crate::prelude::NativeTransmutable<$nt> for $rt {}
        #[test]
        fn $test_fn() {
            use crate::prelude::NativeTransmutable;
            <$rt>::test_layout();
        }
    };
}

/// Macro that implements Send and Sync.
#[macro_export]
macro_rules! unsafe_send_sync {
    ($t: ty) => {
        unsafe impl Send for $t {}
        unsafe impl Sync for $t {}
    };
}

/// Macro that verifies a variant name at compile time.
#[macro_export]
macro_rules! variant_name {
    ($t:expr, $test_fn:ident) => {
        #[test]
        fn $test_fn() {
            let _ = $t;
        }
    };
}
