#[cfg(all(windows, feature = "embed-icudtl"))]
pub fn init() {
    use std::sync::Mutex;

    static ICUDTL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/skia/icudtl.dat"));
    static MUTEX: Mutex<()> = Mutex::new(());

    extern "C" {
        fn C_SetICU(data: *const std::ffi::c_void);
    }

    // Using `Once` does not work for yet unknown reasons.
    // https://github.com/rust-skia/rust-skia/issues/566
    let lock = MUTEX.lock().unwrap();
    unsafe { C_SetICU(ICUDTL.as_ptr() as *const std::ffi::c_void) };
    drop(lock);
}

#[cfg(all(windows, not(feature = "embed-icudtl")))]
pub fn init() {
    use std::{env, fs};

    static ICUDTL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/skia/icudtl.dat"));

    let path = env::current_exe()
        .expect("Failed to resolve the current executable's path")
        .parent()
        .expect("Current executable's parent path does not point to a directory")
        .join("icudtl.dat");
    if path.exists() {
        return;
    }
    fs::write(path, &ICUDTL[..])
        .expect("Failed to write icudtl.dat into the current executable's directory");
}

#[cfg(not(windows))]
pub fn init() {}
