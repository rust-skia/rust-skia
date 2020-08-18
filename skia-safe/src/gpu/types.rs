pub use skia_bindings::GrBackendApi as BackendAPI;
#[test]
fn test_backend_api_layout() {
    let _ = BackendAPI::Dawn;
}

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::GrMipMapped as MipMapped;
#[test]
fn test_mip_mapped_naming() {
    let _ = MipMapped::Yes;
}

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::GrRenderable as Renderable;
#[test]
fn test_renderable_naming() {
    let _ = Renderable::No;
}

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::GrProtected as Protected;
#[test]
fn test_protected_naming() {
    let _ = Protected::Yes;
}

pub use skia_bindings::GrSurfaceOrigin as SurfaceOrigin;
#[test]
fn test_surface_origin_naming() {
    let _ = SurfaceOrigin::TopLeft;
}

// Note: BackendState is in gl/types.rs/
