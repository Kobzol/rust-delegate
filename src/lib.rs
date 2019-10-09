extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use std::collections::HashMap;
use syn;
use syn::export::TokenStream2;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::Error;

mod kw {
    syn::custom_keyword!(to);
}

struct DelegatedMethod {
    method: syn::TraitItemMethod,
    attributes: Vec<syn::Attribute>,
    visibility: syn::Visibility,
}

impl syn::parse::Parse for DelegatedMethod {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let visibility = input.call(syn::Visibility::parse)?;

        Ok(DelegatedMethod {
            method: input.parse()?,
            attributes,
            visibility,
        })
    }
}

struct DelegatedSegment {
    delegator: syn::Expr,
    methods: Vec<DelegatedMethod>,
}

impl syn::parse::Parse for DelegatedSegment {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        input.parse::<kw::to>()?;
        input.parse::<syn::Expr>().and_then(|delegator| {
            let delegator = match delegator {
                syn::Expr::Field(_) => delegator,
                syn::Expr::MethodCall(_) => delegator,
                syn::Expr::Call(_) => delegator,
                _ => panic!("Use a field expression to select delegator (e.g. self.inner)"),
            };

            let content;
            syn::braced!(content in input);

            let mut methods = vec![];
            while !content.is_empty() {
                methods.push(content.parse::<DelegatedMethod>().unwrap());
            }

            Ok(DelegatedSegment { delegator, methods })
        })
    }
}

struct DelegationBlock {
    segments: Vec<DelegatedSegment>,
}

impl syn::parse::Parse for DelegationBlock {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let mut segments = vec![];
        while !input.is_empty() {
            segments.push(input.parse()?);
        }

        Ok(DelegationBlock { segments })
    }
}

struct TargetMethodAttribute {
    name: syn::Ident,
}

impl syn::parse::Parse for TargetMethodAttribute {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let content;
        syn::parenthesized!(content in input);
        Ok(TargetMethodAttribute {
            name: content.parse()?,
        })
    }
}

/// Iterates through the attributes of a method and filters special attributes.
/// target_method => sets the name of the target method
/// into => generates a `into()` call for the returned value
///
/// Returns tuple (blackbox attributes, name, into)
fn parse_attributes<'a>(
    attrs: &'a [syn::Attribute],
    method: &syn::TraitItemMethod,
) -> (Vec<&'a syn::Attribute>, Option<syn::Ident>, bool) {
    let mut name: Option<syn::Ident> = None;
    let mut into: Option<bool> = None;
    let mut map: HashMap<&str, Box<dyn FnMut(TokenStream2) -> ()>> = Default::default();
    map.insert(
        "target_method",
        Box::new(|stream| {
            let target = syn::parse2::<TargetMethodAttribute>(stream).unwrap();
            if name.is_some() {
                panic!(
                    "Multiple target_method attributes specified for {}",
                    method.sig.ident
                )
            }
            name = Some(target.name.clone());
        }),
    );
    map.insert(
        "into",
        Box::new(|_| {
            if into.is_some() {
                panic!(
                    "Multiple into attributes specified for {}",
                    method.sig.ident
                )
            }
            into = Some(true);
        }),
    );

    let attrs: Vec<&syn::Attribute> = attrs
        .iter()
        .filter(|attr| {
            if let syn::AttrStyle::Outer = attr.style {
                for (ident, callback) in map.iter_mut() {
                    if attr.path.is_ident(syn::Ident::new(ident, attr.span())) {
                        callback(attr.tts.clone());
                        return false;
                    }
                }
            }

            true
        })
        .collect();

    drop(map);
    (attrs, name, into.unwrap_or(false))
}

/// Returns true if there are any `inline` attributes in the input.
fn has_inline_attribute(attrs: &[&syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if let syn::AttrStyle::Outer = attr.style {
            attr.path.is_ident(syn::Ident::new("inline", attr.span()))
        } else {
            false
        }
    })
}

#[proc_macro]
pub fn delegate(tokens: TokenStream) -> TokenStream {
    let block: DelegationBlock = syn::parse_macro_input!(tokens);
    let sections = block.segments.iter().map(|delegator| {
        let delegator_attribute = &delegator.delegator;
        let functions = delegator.methods.iter().map(|method| {
            let input = &method.method;
            let signature = &input.sig;
            let inputs = &input.sig.decl.inputs;

            let (attrs, name, into) = parse_attributes(&method.attributes, &input);

            if input.default.is_some() {
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
                            if ident.ident == "self" {
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

            let body = quote::quote! { #delegator_attribute.#name(#(#args),*) };
            let span = input.span();
            let body = match &signature.decl.output {
                syn::ReturnType::Default => quote::quote! { #body; },
                syn::ReturnType::Type(_, ret_type) => {
                    if into {
                        quote::quote! { std::convert::Into::<#ret_type>::into(#body) }
                    }
                    else {
                        body
                    }
                }
            };

            quote::quote_spanned! {span=>
                #(#attrs)*
                #inline
                #visibility #signature {
                    #body
                }
            }
        });

        quote! { #(#functions)* }
    });

    let result = quote! {
        #(#sections)*
    };
    result.into()
}
