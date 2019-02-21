extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

/// An automatic Clone & Drop implementation based on the reference counting
/// functions _ref() and _unref() for a newtype like struct.
#[proc_macro_derive(RCCloneDrop)]
pub fn ref_counted_clone_drop(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_clone_and_drop(&ast)
}

fn impl_clone_and_drop(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let quoted = quote! {
        impl Clone for #name {
            fn clone(&self) -> Self {
                self._ref();
                #name(self.0)
            }
        }

        impl Drop for #name {
            fn drop(&mut self) {
                self._unref();
            }
        }
    };

    TokenStream::from(quoted)
}
