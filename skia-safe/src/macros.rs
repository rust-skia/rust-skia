/// Macro to mark a Rust type as [`NativeTransmutable`] and tests its layout.
#[macro_export]
macro_rules! native_transmutable {
    ($nt:ty, $rt:ty, $test_fn:ident) => {
        impl NativeTransmutable<$nt> for $rt {}
        #[test]
        fn $test_fn() {
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
