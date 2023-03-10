#![macro_use]

/// Macro to mark a Rust type as NativeTransmutable and test its layout.
#[macro_export]
macro_rules! native_transmutable {
    ($nt:ty, $rt:ty, $test_fn:ident) => {
        impl $crate::prelude::NativeTransmutable<$nt> for $rt {}
        #[test]
        fn $test_fn() {
            use $crate::prelude::NativeTransmutable;
            <$rt>::test_layout();
        }
    };
}

#[macro_export]
macro_rules! require_type_equality {
    ($t: ty, $nt: ty) => {
        const _: fn(&$t) = |a| {
            let _: &$nt = a;
        };
    };
}

#[macro_export]
macro_rules! require_base_type {
    ($t: ty, $nt: ty) => {
        const _: fn(&$t) = |a| {
            let _: &$nt = &(a._base);
        };
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
    ($t:expr) => {
        const _: fn() = || {
            let _ = $t;
        };
    };
}
