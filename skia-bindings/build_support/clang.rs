/// Convert a Rust target architecture identifier to a clang target architecture identifier.
pub fn target_arch(arch: &str) -> &str {
    match arch {
        "aarch64" => "arm64",
        "i686" => "x86",
        arch => arch,
    }
}
