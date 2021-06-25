pub mod win {
    use crate::cargo;
    use std::path::PathBuf;

    #[inline]
    fn validate_home(home: &str) -> Option<String> {
        let clang_cl: PathBuf = [home, "bin", "clang-cl.exe"].iter().collect();
        eprintln!("Checking for {:?}", clang_cl);
        if clang_cl.exists() {
            Some(home.to_string())
        } else {
            None
        }
    }

    /// Locate the LLVM installation which can be used to build skia.
    /// If the environment variable `LLVM_HOME` is present it will
    /// be used. Otherwise we search a set of common paths:
    ///   - C:\Program Files\LLVM
    ///   - C:\LLVM
    ///   - %USERPROFILE%\scoop\apps\llvm\current
    pub fn find_llvm_home() -> Option<String> {
        if let Some(llvm_home) = cargo::env_var("LLVM_HOME") {
            validate_home(&llvm_home)
        } else {
            // USERPROFILE *should* equate to HOME and always be defined.
            // TODO: If this isn't defined, should we try c:\Users\%USERNAME%?
            let userprofile =
                cargo::env_var("USERPROFILE").expect("Unable to resolve %USERPROFILE%");
            let common_roots = [
                String::from("C:\\Program Files\\LLVM"),
                String::from("C:\\LLVM"),
                format!("{}\\scoop\\apps\\llvm\\current", userprofile),
            ];
            for root in &common_roots {
                let root = validate_home(root);
                if root.is_some() {
                    return root;
                }
            }
            None
        }
    }
}
