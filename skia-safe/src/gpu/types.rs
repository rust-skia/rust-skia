use skia_bindings as sb;

pub use sb::skgpu_BackendApi as BackendApi;
variant_name!(BackendApi::Metal);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Budgeted(bool);
native_transmutable!(sb::skgpu_Budgeted, Budgeted, budgeted_layout);

#[allow(non_upper_case_globals)]
impl Budgeted {
    // we want this look like enum case names.
    pub const No: Budgeted = Budgeted(false);
    pub const Yes: Budgeted = Budgeted(true);
}

// TODO: CallbackResult

pub use skia_bindings::skgpu_Mipmapped as Mipmapped;

pub use skia_bindings::skgpu_Protected as Protected;
variant_name!(Protected::Yes);

pub use skia_bindings::skgpu_Renderable as Renderable;
variant_name!(Renderable::No);

pub use skia_bindings::skgpu_Origin as Origin;
variant_name!(Origin::TopLeft);
