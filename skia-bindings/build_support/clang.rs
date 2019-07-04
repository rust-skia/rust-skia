/// Convert a Rust target architecture identifier to a clang target architecture identifier.
pub fn target_arch(arch: &str) -> &str {
    if arch == "aarch64" {
        "arm64"
    } else {
        arch
    }
}
