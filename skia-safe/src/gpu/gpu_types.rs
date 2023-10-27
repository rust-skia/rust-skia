use skia_bindings as sb;

pub use skia_bindings::skgpu_BackendApi as BackendApi;
variant_name!(BackendApi::Metal);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Budgeted(bool);

native_transmutable!(sb::skgpu_Budgeted, Budgeted, budgeted_layout);

#[allow(non_upper_case_globals)]
impl Budgeted {
    #[deprecated(since = "0.29.0", note = "use No")]
    pub const NO: Budgeted = Budgeted(false);
    #[deprecated(since = "0.29.0", note = "use Yes")]
    pub const YES: Budgeted = Budgeted(true);

    // we want this look like enum case names.
    pub const No: Budgeted = Budgeted(false);
    pub const Yes: Budgeted = Budgeted(true);
}

// TODO: CallbackResult

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::skgpu_Mipmapped as Mipmapped;

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::skgpu_Protected as Protected;
variant_name!(Protected::Yes);

// TODO: this should be a newtype(bool) I guess with implementations
//       of From<bool> and Deref?
pub use skia_bindings::skgpu_Renderable as Renderable;
variant_name!(Renderable::No);

pub use skia_bindings::skgpu_Origin as Origin;
variant_name!(Origin::TopLeft);
