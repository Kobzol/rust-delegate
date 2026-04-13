use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, ItemImpl, ItemTrait, Token, TraitItem};

struct DelegateTraitArgs {
    target: Ident,
}

impl Parse for DelegateTraitArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        if key != "target" {
            return Err(syn::Error::new(key.span(), "expected `target`"));
        }
        input.parse::<Token![=]>()?;
        Ok(DelegateTraitArgs {
            target: input.parse()?,
        })
    }
}

/// Placed on a trait definition. Emits the trait unchanged plus a hidden
/// helper macro that knows all the trait's method signatures.
pub fn delegatable_trait_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_def = parse_macro_input!(item as ItemTrait);
    let trait_name = &trait_def.ident;
    let helper_macro_name = format_ident!("__delegate_impl_{}", trait_name);

    let method_sigs: Vec<_> = trait_def
        .items
        .iter()
        .filter_map(|item| {
            if let TraitItem::Fn(method) = item {
                if method.default.is_none() {
                    return Some(method.sig.clone());
                }
            }
            None
        })
        .collect();

    let method_arms: Vec<_> = method_sigs
        .iter()
        .map(|sig| {
            let method_name = &sig.ident;

            let params: Vec<_> = sig
                .inputs
                .iter()
                .filter_map(|arg| {
                    if let syn::FnArg::Typed(pt) = arg {
                        Some(&pt.pat)
                    } else {
                        None
                    }
                })
                .collect();

            let await_tok = if sig.asyncness.is_some() {
                quote! { .await }
            } else {
                quote! {}
            };

            let has_ref = sig.receiver().map_or(false, |r| r.reference.is_some());
            let has_mut = sig.receiver().map_or(false, |r| r.mutability.is_some());

            // $field is an ident; we generate self.$field inside the fn body
            // where `self` is naturally in scope.
            let target_expr = if has_ref && has_mut {
                quote! { &mut self.$field }
            } else if has_ref {
                quote! { &self.$field }
            } else {
                quote! { self.$field }
            };

            let body = quote! {
                #[inline]
                #sig {
                    <_ as $trait_path>::#method_name(#target_expr, #(#params),*) #await_tok
                }
            };

            (method_name.clone(), body)
        })
        .collect();

    let method_names: Vec<_> = method_arms.iter().map(|(name, _)| name.clone()).collect();
    let method_bodies: Vec<_> = method_arms.iter().map(|(_, body)| body.clone()).collect();

    let result = quote! {
        #trait_def

        #[doc(hidden)]
        #[macro_export]
        macro_rules! #helper_macro_name {
            ($field:ident, [$($skip:ident),*], $trait_path:path) => {
                #(
                    #helper_macro_name!(@maybe #method_names, $field, [$($skip),*], $trait_path);
                )*
            };

            // Skip: method name matches first element of skip list
            #(
                (@maybe #method_names, $field:ident, [#method_names $(, $($rest:ident),*)?], $trait_path:path) => {};
            )*

            // No match on first: pop and recurse
            (@maybe $method:ident, $field:ident, [$first:ident $(, $($rest:ident),*)?], $trait_path:path) => {
                #helper_macro_name!(@maybe $method, $field, [$($($rest),*)?], $trait_path);
            };

            // Empty skip list: emit delegation
            #(
                (@maybe #method_names, $field:ident, [], $trait_path:path) => {
                    #method_bodies
                };
            )*
        }
    };

    result.into()
}

/// Placed on `impl Trait for Struct { ... }`. Methods present in the block
/// are kept; all other trait methods are delegated to `target`.
/// Requires the trait to be annotated with `#[delegatable_trait]`.
pub fn delegate_trait_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as DelegateTraitArgs);
    let impl_block = parse_macro_input!(item as ItemImpl);

    let Some((_, ref trait_path, _)) = impl_block.trait_ else {
        return syn::Error::new(Span::call_site(), "delegate_trait requires a trait impl block")
            .to_compile_error()
            .into();
    };

    let target = &args.target;
    let trait_path = trait_path.clone();
    let self_ty = &impl_block.self_ty;
    let generics = &impl_block.generics;
    let impl_attrs = &impl_block.attrs;
    let unsafety = &impl_block.unsafety;
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let override_idents: Vec<Ident> = impl_block
        .items
        .iter()
        .filter_map(|item| {
            if let syn::ImplItem::Fn(m) = item {
                Some(m.sig.ident.clone())
            } else {
                None
            }
        })
        .collect();

    let user_items = &impl_block.items;

    let trait_ident = &trait_path
        .segments
        .last()
        .expect("trait path must have at least one segment")
        .ident;
    let helper_macro_name = format_ident!("__delegate_impl_{}", trait_ident);

    let result = quote! {
        #(#impl_attrs)*
        #unsafety impl #impl_generics #trait_path for #self_ty #where_clause {
            #(#user_items)*
            #helper_macro_name!(#target, [#(#override_idents),*], #trait_path);
        }
    };

    result.into()
}
