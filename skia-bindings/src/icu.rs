#[cfg(windows)]
pub fn init() {
    use std::env;
    let icudtl = include_bytes!(concat!(env!("OUT_DIR"), "/skia/icudtl.dat"));

    #[cfg(feature = "embed-icudtl")]
    {
        unsafe { crate::C_SetICU(&icudtl[0] as &'static u8 as *const u8 as _) };
    }

    #[cfg(not(feature = "embed-icudtl"))]
    {
        use std::fs;

        let path = env::current_exe()
            .expect("failed to resolve the current executable's path")
            .parent()
            .expect("current executable's path does not point to a directory")
            .join("icudtl.dat");
        if path.exists() {
            return;
        };
        fs::write(path, &icu_dtl[..])
            .expect("failed to write icudtl.dat into the current executable's directory");
    }
}

#[cfg(not(windows))]
pub fn init() {}
