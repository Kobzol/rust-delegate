extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::Error;
use std::ops::Deref;

struct DelegatedMethod {
    method: syn::TraitItemMethod,
    attributes: Vec<syn::Attribute>,
    visibility: syn::Visibility
}

impl syn::parse::Parse for DelegatedMethod {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let visibility = input.call(syn::Visibility::parse)?;

        Ok(DelegatedMethod {
            method: input.parse()?,
            attributes,
            visibility
        })
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

            let mut methods = vec![];
            while !input.is_empty() {
                methods.push(input.parse::<DelegatedMethod>().unwrap());
            }

            Ok(DelegatedSegment { delegator, methods })
        })
    }
}

fn transform_attributes(attrs: &Vec<syn::Attribute>,
                        method: &syn::TraitItemMethod) -> (Vec<syn::Attribute>, Option<syn::Ident>) {
    let mut name: Option<syn::Ident> = None;
    let attrs: Vec<syn::Attribute> = attrs.iter().filter(|attr| {
        if let syn::AttrStyle::Outer = attr.style {
            if attr.path.is_ident(syn::Ident::new("target_method", attr.span())) {
                let stream = &attr.tts;
                // TODO: token::brace
                let expr: Result<syn::Expr, _> = syn::parse2(stream.clone());

                match expr {
                    Ok(content) => {
                        match content {
                            syn::Expr::Paren(p) => match p.expr.deref() {
                                syn::Expr::Path(path) => {
                                    if path.path.segments.len() > 1 {
                                        panic!("Use a simple string for target method name for {}", method.sig.ident);
                                    }

                                    let (segment, _) = path.path.segments.first().unwrap().into_tuple();
                                    name = Some(segment.ident.clone());
                                }
                                _ => panic!("Use a string for target method name for {}", method.sig.ident)
                            }
                            _ => panic!("Use target_method(name) to specify target method name for {}", method.sig.ident)
                        }
                    }
                    _ => panic!("Include a target method name for {}", method.sig.ident)
                }

                return false
            }
        }

        true
    }).map(|arg| arg.clone()).collect();

    (attrs, name)
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
    let functions = delegator.methods.iter().map(|method| {
        let input = &method.method;
        let signature = &input.sig;
        let inputs = &input.sig.decl.inputs;

        let (attrs, name) = transform_attributes(&method.attributes, &input);

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

        let name = match &name {
            Some(n) => &n,
            None => &input.sig.ident
        };
        let inline = if has_inline_attribute(&attrs) {
            quote!()
        } else {
            quote! { #[inline(always)] }
        };
        let visibility = &method.visibility;

        let span = input.span();
        quote::quote_spanned! {span=>
            #(#attrs)*
            #inline
            #visibility #signature {
                #delegator_attribute.#name(#(#args),*)
            }
        }
    });

    let result = quote! {
        #(#functions)*
    };
    result.into()
}
