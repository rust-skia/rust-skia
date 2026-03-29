#[cfg(feature = "glemu")]
mod glemu;
mod wasi;

#[cfg(feature = "glemu")]
pub use glemu::{drop_gl_context, register_gl_context, set_gl_context, web_sys_get_proc};
