#![allow(dead_code)]

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::fmt::Write;
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token, Expr, Ident, LitStr, Token, Type,
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

struct AttrName {
    value: Ident,
    alias: Option<LitStr>,
}

impl Parse for AttrName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(LitStr) {
            Self {
                alias: input.parse()?,
                value: {
                    input.parse::<Token![as]>()?;

                    input.parse()?
                },
            }
        } else {
            Self {
                value: input.parse()?,
                alias: None,
            }
        })
    }
}

struct Attr {
    copy: Option<Token![*]>,
    name: AttrName,
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
        let brace_content;

        Ok(Self {
            name: input.parse()?,
            fat_arrow_token: input.parse()?,
            brace_token: braced!(brace_content in input),
            attrs: Punctuated::parse_separated_nonempty(&brace_content)?,
        })
    }
}

fn attrs2(input: TokenStream) -> TokenStream2 {
    let Data { name, attrs, .. } = match Data::parse.parse(input) {
        Ok(data) => data,
        Err(error) => return error.into_compile_error(),
    };

    let attrs = attrs
        .into_iter()
        .map(
            |Attr {
                 name: AttrName {
                    value: attr,
                    alias,
                },
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
                    &alias.map(|alias| alias.value()).unwrap_or_else(|| attr.to_string())
                        .split('_')
                        .fold(String::new(), |mut data, word| {
                            write!(data, "{}{}", &word[0..1].to_uppercase(), &word[1..]).unwrap();

                            data
                        }),
                    Span::call_site(),
                );

                let native_has_name = Ident::new(
                    &format!("C_{name}_has{native_name}"),
                    Span::call_site(),
                );

                let get_name = Ident::new(&format!("get_{attr}"), Span::call_site());
                let native_get_name = Ident::new(
                    &format!("C_{name}_get{native_name}"),
                    Span::call_site(),
                );
                let set_name = Ident::new(&format!("set_{attr}"), Span::call_site());
                let native_set_name = Ident::new(
                    &format!("C_{name}_set{native_name}"),
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

                    pub fn #set_name(&self, #setter_name: #ty) {
                        unsafe {
                            sb::#native_set_name(self.native_mut_force(), #setter_body)
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
