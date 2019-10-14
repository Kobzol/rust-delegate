extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::Error;

mod kw {
    syn::custom_keyword!(target);
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
        input.parse::<kw::target>()?;
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

struct TargetMethod {
    name: syn::Ident,
}

impl syn::parse::Parse for TargetMethod {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let content;
        syn::parenthesized!(content in input);
        Ok(TargetMethod {
            name: content.parse()?,
        })
    }
}

fn transform_attributes(
    attrs: &[syn::Attribute],
    method: &syn::TraitItemMethod,
) -> (Vec<syn::Attribute>, Option<syn::Ident>) {
    let mut name: Option<syn::Ident> = None;
    let attrs: Vec<syn::Attribute> = attrs
        .iter()
        .filter(|attr| {
            if let syn::AttrStyle::Outer = attr.style {
                if attr
                    .path
                    .is_ident(syn::Ident::new("target_method", attr.span()))
                {
                    let stream = &attr.tts;

                    let target = syn::parse2::<TargetMethod>(stream.clone()).unwrap();
                    if name.is_some() {
                        panic!(
                            "Multiple target_method attributes specified for {}",
                            method.sig.ident
                        )
                    }
                    name = Some(target.name.clone());

                    return false;
                }
            }

            true
        })
        .cloned()
        .collect();

    (attrs, name)
}

fn has_inline_attribute(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| {
            if let syn::AttrStyle::Outer = attr.style {
                attr.path.is_ident(syn::Ident::new("inline", attr.span()))
            } else {
                false
            }
        })
        .count()
        > 0
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

            let (attrs, name) = transform_attributes(&method.attributes, &input);

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

            let span = input.span();
            quote::quote_spanned! {span=>
            #(#attrs)*
            #inline
            #visibility #signature {
                #delegator_attribute.#name(#(#args),*)
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
