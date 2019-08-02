/// This function writes the file `icudtl.dat` into the current's executable directory
/// making sure that it's available when text shaping is used in Skia.
///
/// If your executable directory can not be written to, make sure that `icudtl.dat` is
/// available.
///
/// It's currently not possible to load `icudtl.dat` from another location.
#[cfg(windows)]
pub fn init() {
    use std::{env, fs};

    let path = env::current_exe()
        .expect("failed to resolve the current executable's path")
        .parent()
        .expect("current executable's path does not point to a directory")
        .join("icudtl.dat");
    if path.exists() {
        return;
    };
    let icu_dtl = include_bytes!(concat!(env!("OUT_DIR"), "/skia/icudtl.dat"));
    fs::write(path, &icu_dtl[..])
        .expect("failed to write icudtl.dat into the current executable's directory");
}

#[cfg(not(windows))]
pub fn init() {}
