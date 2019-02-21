extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(RCCopyClone)]
pub fn ref_counted_copy_clone(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_drop_and_clone(&ast)
}

fn impl_drop_and_clone(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let r = quote! {
        impl Drop for #name {
            fn drop(&mut self) {
                self._unref();
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                self._ref();
                #name(self.0)
            }
        }
    };

    TokenStream::from(r)
}
