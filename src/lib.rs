extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::Error;

struct DelegatedMethod {
    method: syn::TraitItemMethod,
    name: Option<syn::Ident>,
}

impl syn::parse::Parse for DelegatedMethod {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let method = input.parse::<syn::TraitItemMethod>()?;
        let lookahead = input.lookahead1();

        let name = if lookahead.peek(syn::Token![use]) {
            input.parse::<syn::Token![use]>()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                input.parse()?
            } else {
                panic!("Add a delegated method name for {}", method.sig.ident);
            }
        } else {
            None
        };

        Ok(DelegatedMethod { method, name })
    }
}

struct DelegatedSegment {
    delegator: syn::ExprField,
    methods: Vec<DelegatedMethod>,
}

impl syn::parse::Parse for DelegatedSegment {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        input.parse::<syn::Expr>().and_then(|delegator| {
            let delegator = match delegator {
                syn::Expr::Field(field) => field,
                _ => panic!("Use a field expression to select delegator (e.g. self.inner)"),
            };

            let mut methods: Vec<DelegatedMethod> = vec![];
            loop {
                let method = input.parse::<DelegatedMethod>();
                match method {
                    Ok(res) => methods.push(res),
                    Err(_) => break,
                }
            }

            Ok(DelegatedSegment { delegator, methods })
        })
    }
}

fn has_inline_attribute(attrs: &Vec<syn::Attribute>) -> bool {
    attrs
        .iter()
        .filter(|attr| {
            if let syn::AttrStyle::Outer = attr.style {
                attr.path.is_ident(syn::Ident::new("inline", attr.span()))
            } else {
                false
            }
        })
        .count() > 0
}

#[proc_macro]
pub fn delegate(tokens: TokenStream) -> TokenStream {
    let delegator: DelegatedSegment = syn::parse_macro_input!(tokens);

    let delegator_attribute = delegator.delegator;
    let functions = delegator.methods.iter().map(|fun| {
        let input = &fun.method;
        let signature = &input.sig;
        let inputs = &input.sig.decl.inputs;
        let attrs = &input.attrs;

        if let Some(_) = input.default {
            panic!(
                "Do not include implementation of delegated functions ({})",
                signature.ident
            );
        }
        let args: Vec<syn::Ident> = inputs
            .iter()
            .filter_map(|i| match i {
                syn::FnArg::Captured(capt) => match &capt.pat {
                    syn::Pat::Ident(ident) => {
                        if ident.ident.to_string() == "self" {
                            None
                        } else {
                            Some(ident.ident.clone())
                        }
                    }
                    _ => panic!(
                        "You have to use simple identifiers for delegated method parameters ({})",
                        input.sig.ident
                    ),
                },
                _ => None,
            })
            .collect();

        let name = match &fun.name {
            Some(rename) => &rename,
            None => &input.sig.ident,
        };
        let inline = if has_inline_attribute(attrs) {
            quote! {}
        } else {
            quote! { #[inline(always)] }
        };

        let span = input.span();
        quote::quote_spanned! {span=>
            #(#attrs)*
            #inline
            pub #signature {
                #delegator_attribute.#name(#(#args),*)
            }
        }
    });

    let result = quote! {
        #(#functions)*
    };
    result.into()
}
