//! This crate removes some boilerplate for structs that simply delegate
//! some of their methods to one or more of their fields.
//!
//! It gives you the `delegate!` macro, which delegates method calls to selected expressions (usually inner fields).
//!
//! ## Features:
//! - Delegate to a method with a different name
//! ```rust
//! use delegate::delegate;
//!
//! struct Stack { inner: Vec<u32> }
//! impl Stack {
//!     delegate! {
//!         to self.inner {
//!             #[call(push)]
//!             pub fn add(&mut self, value: u32);
//!         }
//!     }
//! }
//! ```
//! - Use an arbitrary inner field expression
//! ```rust
//! use delegate::delegate;
//!
//! use std::rc::Rc;
//! use std::cell::RefCell;
//! use std::ops::Deref;
//!
//! struct Wrapper { inner: Rc<RefCell<Vec<u32>>> }
//! impl Wrapper {
//!     delegate! {
//!         to self.inner.deref().borrow_mut() {
//!             pub fn push(&mut self, val: u32);
//!         }
//!     }
//! }
//! ```
//!
//! - Delegate to enum variants
//!
//! ```rust
//! use delegate::delegate;
//!
//! enum Enum {
//!     A(A),
//!     B(B),
//!     C { v: C },
//! }
//!
//! struct A {
//!     val: usize,
//! }
//!
//! impl A {
//!     fn dbg_inner(&self) -> usize {
//!         dbg!(self.val);
//!         1
//!     }
//! }
//! struct B {
//!     val_a: String,
//! }
//!
//! impl B {
//!     fn dbg_inner(&self) -> usize {
//!         dbg!(self.val_a.clone());
//!         2
//!     }
//! }
//!
//! struct C {
//!     val_c: f64,
//! }
//!
//! impl C {
//!     fn dbg_inner(&self) -> usize {
//!         dbg!(self.val_c);
//!         3
//!     }
//! }
//!
//! impl Enum {
//!     delegate! {
//!         // transformed to
//!         //
//!         // ```rust
//!         // match self {
//!         //     Enum::A(a) => a.dbg_inner(),
//!         //     Enum::B(b) => { println!("i am b"); b }.dbg_inner(),
//!         //     Enum::C { v: c } => { c }.dbg_inner(),
//!         // }
//!         // ```
//!         to match self {
//!             Enum::A(a) => a,
//!             Enum::B(b) => { println!("i am b"); b },
//!             Enum::C { v: c } => { c },
//!         } {
//!             fn dbg_inner(&self) -> usize;
//!         }
//!     }
//! }
//! ```
//!
//! - Change the return type of the delegated method using a `From` or `TryFrom` impl or omit it altogether
//! ```rust
//! use delegate::delegate;
//! struct Inner;
//! impl Inner {
//!     pub fn method(&self, num: u32) -> u32 { num }
//! }
//! struct Wrapper { inner: Inner }
//! impl Wrapper {
//!     delegate! {
//!         to self.inner {
//!             // calls method, converts result to u64 using `From`
//!             #[into]
//!             pub fn method(&self, num: u32) -> u64;
//!
//!             // calls method, returns ()
//!             #[call(method)]
//!             pub fn method_noreturn(&self, num: u32);
//!
//!             // calls method, converts result to i6 using `TryFrom`
//!             #[try_into]
//!             #[call(method)]
//!             pub fn method2(&self, num: u32) -> Result<u16, std::num::TryFromIntError>;
//!
//!             // calls method, converts result to i6 using `TryFrom`, unwrap the result
//!             #[try_into(unwrap)]
//!             #[call(method)]
//!             pub fn method3(&self, num: u32) -> u16;
//!         }
//!     }
//! }
//! ```
//! - Call `await` on async functions
//! ```rust
//! use delegate::delegate;
//!
//! struct Inner;
//! impl Inner {
//!     pub async fn method(&self, num: u32) -> u32 { num }
//! }
//! struct Wrapper { inner: Inner }
//! impl Wrapper {
//!     delegate! {
//!         to self.inner {
//!             // calls method(num).await, returns impl Future<Output = u32>
//!             pub async fn method(&self, num: u32) -> u32;
//!
//!             // calls method(num).await.into(), returns impl Future<Output = u64>
//!             #[into]
//!             #[call(method)]
//!             pub async fn method_into(&self, num: u32) -> u64;
//!         }
//!     }
//! }
//! ```
//! You can use the `#[await(true/false)]` attribute on delegated methods to specify if `.await` should
//! be generated after the delegated expression. It will be generated by default if the delegated
//! method is `async`.
//! - Delegate to multiple fields
//! ```rust
//! use delegate::delegate;
//!
//! struct MultiStack {
//!     left: Vec<u32>,
//!     right: Vec<u32>,
//! }
//! impl MultiStack {
//!     delegate! {
//!         to self.left {
//!             // Push an item to the top of the left stack
//!             #[call(push)]
//!             pub fn push_left(&mut self, value: u32);
//!         }
//!         to self.right {
//!             // Push an item to the top of the right stack
//!             #[call(push)]
//!             pub fn push_right(&mut self, value: u32);
//!         }
//!     }
//! }
//! ```
//! - Inserts `#[inline(always)]` automatically (unless you specify `#[inline]` manually on the method)
//! - Specify expressions in the signature that will be used as delegated arguments
//! ```rust
//! use delegate::delegate;
//! struct Inner;
//! impl Inner {
//!     pub fn polynomial(&self, a: i32, x: i32, b: i32, y: i32, c: i32) -> i32 {
//!         a + x * x + b * y + c
//!     }
//! }
//! struct Wrapper { inner: Inner, a: i32, b: i32, c: i32 }
//! impl Wrapper {
//!     delegate! {
//!         to self.inner {
//!             // Calls `polynomial` on `inner` with `self.a`, `self.b` and
//!             // `self.c` passed as arguments `a`, `b`, and `c`, effectively
//!             // calling `polynomial(self.a, x, self.b, y, self.c)`.
//!             pub fn polynomial(&self, [ self.a ], x: i32, [ self.b ], y: i32, [ self.c ]) -> i32 ;
//!             // Calls `polynomial` on `inner` with `0`s passed for arguments
//!             // `a` and `x`, and `self.b` and `self.c` for `b` and `c`,
//!             // effectively calling `polynomial(0, 0, self.b, y, self.c)`.
//!             #[call(polynomial)]
//!             pub fn linear(&self, [ 0 ], [ 0 ], [ self.b ], y: i32, [ self.c ]) -> i32 ;
//!         }
//!     }
//! }
//! ```
//! - Modify how will an input parameter be passed to the delegated method with parameter attribute modifiers.
//! Currently, the following modifiers are supported:
//!     - `#[into]`: Calls `.into()` on the parameter passed to the delegated method.
//! ```rust
//! use delegate::delegate;
//!
//! struct InnerType {}
//! impl InnerType {
//!     fn foo(&self, other: Self) {}
//! }
//!
//! impl From<Wrapper> for InnerType {
//!     fn from(wrapper: Wrapper) -> Self {
//!         wrapper.0
//!     }
//! }
//!
//! struct Wrapper(InnerType);
//! impl Wrapper {
//!     delegate! {
//!         to self.0 {
//!             // Calls `self.0.foo(other.into());`
//!             pub fn foo(&self, #[into] other: Self);
//!         }
//!     }
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;

use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{parse_quote, Attribute, Error, ExprMethodCall, Meta};

mod kw {
    syn::custom_keyword!(to);
    syn::custom_keyword!(target);
}

#[derive(Clone)]
enum ArgumentModifier {
    Into,
}

#[derive(Clone)]
enum DelegatedInput {
    Input {
        parameter: syn::FnArg,
        modifier: Option<ArgumentModifier>,
    },
    Argument(syn::Expr),
}

fn get_argument_modifier(attribute: syn::Attribute) -> Result<ArgumentModifier, Error> {
    let meta = attribute.parse_meta()?;
    if let Meta::Path(mut path) = meta {
        if path.segments.len() == 1 {
            let segment = path.segments.pop().unwrap();
            if segment.value().arguments.is_empty() && segment.value().ident == "into" {
                return Ok(ArgumentModifier::Into);
            }
        }
    };

    panic!("The attribute argument has to be `from`, like this: `#[from] a: u32`.")
}

impl syn::parse::Parse for DelegatedInput {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Bracket) {
            let content;
            let _bracket_token = syn::bracketed!(content in input);
            let expression: syn::Expr = content.parse()?;
            Ok(Self::Argument(expression))
        } else {
            let (input, modifier) = if lookahead.peek(syn::token::Pound) {
                let mut attributes = input.call(syn::Attribute::parse_outer)?;
                if attributes.len() > 1 {
                    panic!("You can specify at most a single attribute for each parameter in a delegated method");
                }
                let modifier = get_argument_modifier(attributes.pop().unwrap())
                    .expect("Could not parse argument modifier attribute");

                let input: syn::FnArg = input.parse()?;
                (input, Some(modifier))
            } else {
                (input.parse()?, None)
            };

            Ok(Self::Input {
                parameter: input,
                modifier,
            })
        }
    }
}

struct DelegatedMethod {
    method: syn::TraitItemMethod,
    attributes: Vec<syn::Attribute>,
    visibility: syn::Visibility,
    arguments: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
}

// Given an input parameter from a function signature, create a function
// argument used to call the delegate function: omit receiver, extract an
// identifier from a typed input parameter (and wrap it in an `Expr`).
fn parse_input_into_argument_expression(
    function_name: &syn::Ident,
    input: &syn::FnArg,
) -> Option<syn::Expr> {
    match input {
        // Parse inputs of the form `x: T` to retrieve their identifiers.
        syn::FnArg::Typed(typed) => {
            match &*typed.pat {
                // This should not happen, I think. If it does,
                // it will be ignored as if it were the
                // receiver.
                syn::Pat::Ident(ident) if ident.ident == "self" => None,
                // Expression in the form `x: T`. Extract the
                // identifier, wrap it in Expr for type compatibility with bracketed expressions,
                // and append it
                // to the argument list.
                syn::Pat::Ident(ident) => {
                    let path_segment = syn::PathSegment {
                        ident: ident.ident.clone(),
                        arguments: syn::PathArguments::None,
                    };
                    let mut segments = syn::punctuated::Punctuated::new();
                    segments.push(path_segment);
                    let path = syn::Path {
                        leading_colon: None,
                        segments,
                    };
                    let ident_as_expr = syn::Expr::from(syn::ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path,
                    });
                    Some(ident_as_expr)
                }
                // Other more complex argument expressions are not covered.
                _ => panic!(
                    "You have to use simple identifiers for delegated method parameters ({})",
                    function_name // The signature is not constructed yet. We make due.
                ),
            }
        }
        // Skip any `self`/`&self`/`&mut self` argument, since
        // it does not appear in the argument list and it's
        // already added to the parameter list.
        syn::FnArg::Receiver(_receiver) => None,
    }
}

impl syn::parse::Parse for DelegatedMethod {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let visibility = input.call(syn::Visibility::parse)?;

        // Unchanged from Parse from TraitItemMethod
        let constness: Option<syn::Token![const]> = input.parse()?;
        let asyncness: Option<syn::Token![async]> = input.parse()?;
        let unsafety: Option<syn::Token![unsafe]> = input.parse()?;
        let abi: Option<syn::Abi> = input.parse()?;
        let fn_token: syn::Token![fn] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        let generics: syn::Generics = input.parse()?;

        let content;
        let paren_token = syn::parenthesized!(content in input);

        // Parse inputs (method parameters) and arguments. The parameters
        // constitute the parameter list of the signature of the delegating
        // method so it must include all inputs, except bracketed expressions.
        // The argument list constitutes the list of arguments used to call the
        // delegated function. It must include all inputs, excluding the
        // receiver (self-type) input. The arguments must all be parsed to
        // retrieve the expressions inside of the brackets as well as variable
        // identifiers of ordinary inputs. The arguments must preserve the order
        // of the inputs.
        let delegated_inputs =
            content.parse_terminated::<DelegatedInput, syn::Token![,]>(DelegatedInput::parse)?;
        let mut inputs: syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]> =
            syn::punctuated::Punctuated::new();
        let mut arguments: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]> =
            syn::punctuated::Punctuated::new();

        // First, combine the cases for pairs with cases for end, to remove
        // redundancy below.
        delegated_inputs
            .into_pairs()
            .map(|punctuated_pair| match punctuated_pair {
                syn::punctuated::Pair::Punctuated(item, comma) => (item, Some(comma)),
                syn::punctuated::Pair::End(item) => (item, None),
            })
            .for_each(|pair| match pair {
                // This input is a bracketed argument (eg. `[ self.x ]`). It
                // is omitted in the signature of the delegator, but the
                // expression inside the brackets is used in the body of the
                // delegator, as an arugnment to the delegated function (eg.
                // `self.x`). The argument needs to be generated in the
                // appropriate position with respect other arguments and non-
                // argument inputs. As long as inputs are added to the
                // `arguments` vector in order of occurance, this is trivial.
                (DelegatedInput::Argument(argument), maybe_comma) => {
                    arguments.push_value(argument);
                    if let Some(comma) = maybe_comma {
                        arguments.push_punct(comma)
                    }
                }
                // The input is a standard function parameter with a name and
                // a type (eg. `x: T`). This input needs to be reflected in
                // the delegator signature as is (eg. `x: T`). The identifier
                // also needs to be included in the argument list in part
                // (eg. `x`). The argument list needs to preserve the order of
                // the inputs with relation to arguments (see above), so the
                // parsing is best done here (previously it was done at
                // generation).
                (
                    DelegatedInput::Input {
                        parameter,
                        modifier,
                    },
                    maybe_comma,
                ) => {
                    inputs.push_value(parameter.clone());
                    if let Some(comma) = maybe_comma {
                        inputs.push_punct(comma);
                    }
                    let maybe_argument = parse_input_into_argument_expression(&ident, &parameter);
                    if let Some(mut argument) = maybe_argument {
                        if let Some(modifier) = modifier {
                            let span = argument.span();
                            match modifier {
                                ArgumentModifier::Into => {
                                    argument = syn::Expr::from(ExprMethodCall {
                                        attrs: vec![],
                                        receiver: Box::new(argument),
                                        dot_token: Default::default(),
                                        method: Ident::new("into", span),
                                        turbofish: None,
                                        paren_token,
                                        args: Default::default(),
                                    });
                                }
                            }
                        }

                        arguments.push(argument);
                        if let Some(comma) = maybe_comma {
                            arguments.push_punct(comma);
                        }
                    }
                }
            });

        // Unchanged from Parse from TraitItemMethod
        let output: syn::ReturnType = input.parse()?;
        let where_clause: Option<syn::WhereClause> = input.parse()?;

        // This needs to be generated manually, because inputs need to be
        // separated into actual inputs that go in the signature (the
        // parameters) and the additional expressions in square brackets which
        // go into the arguments vector (artguments of the call on the method
        // on the inner object).
        let signature = syn::Signature {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            paren_token,
            inputs,
            output,
            variadic: None,
            generics: syn::Generics {
                where_clause,
                ..generics
            },
        };

        // Check if the input contains a semicolon or a brace. If it contains
        // a semicolon, we parse it (to retain token location information) and
        // continue. However, if it contains a brace, this indicates that
        // there is a default definition of the method. This is not supported,
        // so in that case we error out.
        let lookahead = input.lookahead1();
        let semi_token: Option<syn::Token![;]> = if lookahead.peek(syn::Token![;]) {
            Some(input.parse()?)
        } else {
            panic!(
                "Do not include implementation of delegated functions ({})",
                signature.ident
            );
        };

        // This needs to be populated from scratch because of the signature above.
        let method = syn::TraitItemMethod {
            // All attributes are attached to `DelegatedMethod`, since they
            // presumably pertain to the process of delegation, not the
            // signature of the delegator.
            attrs: Vec::new(),
            sig: signature,
            default: None,
            semi_token,
        };

        Ok(DelegatedMethod {
            method,
            attributes,
            visibility,
            arguments,
        })
    }
}

struct DelegatedSegment {
    delegator: syn::Expr,
    methods: Vec<DelegatedMethod>,
}

impl syn::parse::Parse for DelegatedSegment {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        if let Ok(keyword) = input.parse::<kw::target>() {
            return Err(Error::new(keyword.span(), "You are using the old `target` expression, which is deprecated. Please replace `target` with `to`."));
        } else {
            input.parse::<kw::to>()?;
        }

        syn::Expr::parse_without_eager_brace(input).and_then(|delegator| {
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

struct CallMethodAttribute {
    name: syn::Ident,
}

impl syn::parse::Parse for CallMethodAttribute {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let content;
        syn::parenthesized!(content in input);
        Ok(CallMethodAttribute {
            name: content.parse()?,
        })
    }
}

struct GenerateAwaitAttribute {
    literal: syn::LitBool,
}

impl syn::parse::Parse for GenerateAwaitAttribute {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let content;
        syn::parenthesized!(content in input);
        Ok(GenerateAwaitAttribute {
            literal: content.parse()?,
        })
    }
}

enum TryIntoMethod {
    Raw,
    Unwrap,
}

struct ParsedAttributes<'a> {
    attributes: Vec<&'a syn::Attribute>,
    target_method: Option<syn::Ident>,
    generate_into: bool,
    generate_await: bool,
    try_into: Option<TryIntoMethod>,
}

/// Iterates through the attributes of a method and filters special attributes.
/// - call => sets the name of the target method to call
/// - into => generates a `into()` call after the delegated expression
/// - try_into => generates a `try_into()` call after delegated expression
/// - await => generates an `.await` expression after the delegated expression
fn parse_attributes<'a>(
    attrs: &'a [syn::Attribute],
    method: &syn::TraitItemMethod,
) -> ParsedAttributes<'a> {
    let mut target_method: Option<syn::Ident> = None;
    let mut generate_into: Option<bool> = None;
    let mut generate_await: Option<bool> = None;
    let mut try_into: Option<TryIntoMethod> = None;

    // https://github.com/rust-lang/rust/issues/70263
    #[allow(clippy::type_complexity)]
    let mut map: HashMap<&str, Box<dyn FnMut(&Attribute)>> = Default::default();

    map.insert(
        "call",
        Box::new(|attribute| {
            let target = syn::parse2::<CallMethodAttribute>(attribute.tokens.clone()).unwrap();
            if target_method.is_some() {
                panic!(
                    "Multiple call attributes specified for {}",
                    method.sig.ident
                )
            }
            target_method = Some(target.name);
        }),
    );
    map.insert(
        "target_method",
        Box::new(|_| {
            panic!("You are using the old `target_method` attribute, which is deprecated. Please replace `target_method` with `call`.");
        }),
    );
    map.insert(
        "into",
        Box::new(|_| {
            if generate_into.is_some() {
                panic!(
                    "Multiple `into` attributes specified for {}",
                    method.sig.ident
                )
            }
            generate_into = Some(true);
        }),
    );
    map.insert(
        "try_into",
        Box::new(|attribute| {
            if try_into.is_some() {
                panic!(
                    "Multiple `try_into` attributes specified for {}",
                    method.sig.ident
                )
            }
            let meta = attribute
                .parse_meta()
                .expect("Invalid `try_into` arguments");

            if let Meta::List(list) = meta {
                if let Some(syn::NestedMeta::Meta(Meta::Path(path))) = list.nested.first() {
                    if path.is_ident("unwrap") {
                        try_into = Some(TryIntoMethod::Unwrap);
                        return;
                    }
                    panic!(
                        "Unknown `try_into` argument {} specified for {}",
                        path.to_token_stream(),
                        method.sig.ident
                    )
                }
            }
            try_into = Some(TryIntoMethod::Raw);
        }),
    );
    map.insert(
        "await",
        Box::new(|attribute| {
            if generate_await.is_some() {
                panic!(
                    "Multiple `await` attributes specified for {}",
                    method.sig.ident
                )
            }
            let generate = syn::parse2::<GenerateAwaitAttribute>(attribute.tokens.clone()).unwrap();
            generate_await = Some(generate.literal.value);
        }),
    );

    let attrs: Vec<&syn::Attribute> = attrs
        .iter()
        .filter(|attr| {
            if let syn::AttrStyle::Outer = attr.style {
                for (ident, callback) in map.iter_mut() {
                    if attr.path.is_ident(ident) {
                        callback(attr);
                        return false;
                    }
                }
            }

            true
        })
        .collect();

    drop(map);

    ParsedAttributes {
        attributes: attrs,
        target_method,
        generate_into: generate_into.unwrap_or(false),
        generate_await: generate_await.unwrap_or_else(|| method.sig.asyncness.is_some()),
        try_into,
    }
}

/// Returns true if there are any `inline` attributes in the input.
fn has_inline_attribute(attrs: &[&syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if let syn::AttrStyle::Outer = attr.style {
            attr.path.is_ident("inline")
        } else {
            false
        }
    })
}

struct MatchVisitor {
    name: Ident,
    args: Vec<syn::Expr>,
}

impl VisitMut for MatchVisitor {
    fn visit_arm_mut(&mut self, arm: &mut syn::Arm) {
        let body = arm.body.clone();
        let name = &self.name;
        let args = &self.args;
        arm.body = parse_quote!( { #body.#name(#(#args),*) });
    }
}

#[proc_macro]
pub fn delegate(tokens: TokenStream) -> TokenStream {
    let block: DelegationBlock = syn::parse_macro_input!(tokens);
    let sections = block.segments.iter().map(|delegator| {
        let delegator_attribute = &delegator.delegator;

        let functions = delegator.methods.iter().map(|method| {
            let input = &method.method;
            let signature = &input.sig;

            let attributes = parse_attributes(&method.attributes, input);

            if input.default.is_some() {
                panic!(
                    "Do not include implementation of delegated functions ({})",
                    signature.ident
                );
            }

            // Generate an argument vector from Punctuated list.
            let args: Vec<syn::Expr> = method.arguments.clone().into_iter().collect();

            let name = match &attributes.target_method {
                Some(n) => n,
                None => &input.sig.ident,
            };
            let inline = if has_inline_attribute(&attributes.attributes) {
                quote!()
            } else {
                quote! { #[inline(always)] }
            };
            let visibility = &method.visibility;

            let body = if let syn::Expr::Match(expr_match) = delegator_attribute {
                let mut expr_match = expr_match.clone();
                MatchVisitor {
                    name: name.clone(),
                    args,
                }
                .visit_expr_match_mut(&mut expr_match);
                expr_match.into_token_stream()
            } else {
                quote::quote! { #delegator_attribute.#name(#(#args),*) }
            };
            let body = if attributes.generate_await {
                quote::quote! { #body.await }
            } else {
                body
            };

            let span = input.span();
            let body = match &signature.output {
                syn::ReturnType::Default => quote::quote! { #body; },
                syn::ReturnType::Type(_, ret_type) => {
                    if attributes.generate_into {
                        quote::quote! { ::std::convert::Into::<#ret_type>::into(#body) }
                    } else if let Some(try_into) = attributes.try_into {
                        match try_into {
                            TryIntoMethod::Raw => {
                                quote::quote! { ::std::convert::TryInto::try_into(#body) }
                            }
                            TryIntoMethod::Unwrap => {
                                quote::quote! { ::std::convert::TryInto::try_into(#body).unwrap() }
                            }
                        }
                    } else {
                        body
                    }
                }
            };

            let attrs = attributes.attributes;
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
