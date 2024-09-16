#![allow(dead_code)]

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::fmt::Write;
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token, Expr, Ident, Token, Type,
};

struct Property {
    paren_token: token::Paren,
    name: Ident,
    fat_arrow_token: Token![=>],
    body: Expr,
}

impl Parse for Property {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            paren_token: parenthesized!(content in input),
            name: content.parse()?,
            fat_arrow_token: input.parse()?,
            body: input.parse()?,
        })
    }
}

struct Attr {
    copy: Option<Token![*]>,
    name: Ident,
    optional: Option<Token![?]>,
    colon_token: Token![:],
    ty: Type,
    bracket_token: token::Bracket,
    getter: Property,
    comma_token: Token![,],
    setter: Property,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            copy: input.parse()?,
            name: input.parse()?,
            optional: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            bracket_token: bracketed!(content in input),
            getter: {
                let get = content.parse::<Ident>()?;

                if get != "get" {
                    return Err(syn::Error::new(
                        get.span(),
                        format!("expected `get`, found `{get}`"),
                    ));
                }

                content.parse()?
            },
            comma_token: content.parse()?,
            setter: {
                let set = content.parse::<Ident>()?;

                if set != "set" {
                    return Err(syn::Error::new(
                        set.span(),
                        format!("expected `set`, found `{set}`"),
                    ));
                }

                content.parse()?
            },
        })
    }
}

struct Data {
    name: Ident,
    fat_arrow_token: Token![=>],
    brace_token: token::Brace,
    attrs: Punctuated<Attr, Token![,]>,
}

impl Parse for Data {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            name: input.parse()?,
            fat_arrow_token: input.parse()?,
            brace_token: braced!(content in input),
            attrs: Punctuated::parse_separated_nonempty(&content)?,
        })
    }
}

fn attrs2(input: TokenStream) -> TokenStream2 {
    let data = match Data::parse.parse(input) {
        Ok(data) => data,
        Err(error) => return error.into_compile_error(),
    };

    let attrs = data
        .attrs
        .into_iter()
        .map(
            |Attr {
                 name,
                 optional,
                 copy,
                 ty,
                 getter:
                     Property {
                         name: getter_name,
                         body: getter_body,
                         ..
                     },
                 setter:
                     Property {
                         name: setter_name,
                         body: setter_body,
                         ..
                     },
                 ..
             }| {
                let native_name = Ident::new(
                    &name
                        .to_string()
                        .split('_')
                        .fold(String::new(), |mut data, word| {
                            write!(data, "{}{}", &word[0..1].to_uppercase(), &word[1..]).unwrap();

                            data
                        }),
                    Span::call_site(),
                );

                let native_has_name = Ident::new(
                    &format!("C_{}_has{native_name}", data.name),
                    Span::call_site(),
                );

                let get_name = Ident::new(&format!("get_{name}"), Span::call_site());
                let native_get_name = Ident::new(
                    &format!("C_{}_get{native_name}", data.name),
                    Span::call_site(),
                );
                let set_name = Ident::new(&format!("set_{name}"), Span::call_site());
                let native_set_name = Ident::new(
                    &format!("C_{}_set{native_name}", data.name),
                    Span::call_site(),
                );

                let getter = match [optional.is_some(), copy.is_some()] {
                    [true, true] => quote! {
                        pub fn #get_name(&self) -> Option<#ty> {
                            unsafe {
                                if sb::#native_has_name(self.native()) {
                                    let #getter_name = sb::#native_get_name(self.native()).as_ref().map(|value| *value);

                                    #getter_body
                                } else {
                                    None
                                }
                            }
                        }
                    },
                    [true, false] => quote! {
                        pub fn #get_name(&self) -> Option<&#ty> {
                            unsafe {
                                if sb::#native_has_name(self.native()) {
                                    let #getter_name = sb::#native_get_name(self.native()).as_ref();

                                    #getter_body
                                } else {
                                    None
                                }
                            }
                        }
                    },
                    [false, true] => quote! {
                        pub fn #get_name(&self) -> #ty {
                            unsafe {
                                let #getter_name = *sb::#native_get_name(self.native()).as_ref().unwrap_unchecked();

                                #getter_body
                            }
                        }
                    },
                    [false, false] => quote! {
                        pub fn #get_name(&self) -> &#ty {
                            unsafe {
                                let #getter_name = sb::#native_get_name(self.native()).as_ref().unwrap_unchecked();

                                #getter_body
                            }
                        }
                    }
                };

                quote! {
                    #getter

                    pub fn #set_name(&mut self, #setter_name: #ty) {
                        unsafe {
                            sb::#native_set_name(self.native_mut(), #setter_body)
                        }
                    }
                }
            },
        )
        .collect::<Vec<_>>();

    quote! {
        #(#attrs)*
    }
}

#[proc_macro]
pub fn attrs(input: TokenStream) -> TokenStream {
    attrs2(input).into()
}