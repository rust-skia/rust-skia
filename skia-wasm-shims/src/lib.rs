#[cfg(feature = "glemu")]
mod glemu;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasi;

#[cfg(feature = "glemu")]
pub use glemu::{drop_gl_context, register_gl_context, set_gl_context, web_sys_get_proc};
