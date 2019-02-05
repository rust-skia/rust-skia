// the safe bindings one are accessible with skia::.
pub mod skia;

// temporarily re-export all bindings for now.
pub mod bindings {
    pub use rust_skia::*;
}

#[cfg(test)]
mod tests {
}
