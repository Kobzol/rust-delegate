use std::collections::VecDeque;

use syn::parse::ParseStream;
use syn::{Attribute, Error, Meta, TypePath};

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

struct IntoAttribute {
    type_path: Option<TypePath>,
}

impl syn::parse::Parse for IntoAttribute {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            let type_path: TypePath = content.parse().map_err(|error| {
                Error::new(
                    input.span(),
                    format!("{error}\nExpected type name, e.g. #[into(u32)]"),
                )
            })?;

            Ok(IntoAttribute {
                type_path: Some(type_path),
            })
        } else {
            Ok(IntoAttribute { type_path: None })
        }
    }
}

pub struct TraitTarget {
    type_path: TypePath,
}

impl syn::parse::Parse for TraitTarget {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let content;
        syn::parenthesized!(content in input);

        let type_path: TypePath = content.parse().map_err(|error| {
            Error::new(
                input.span(),
                format!("{error}\nExpected trait path, e.g. #[through(foo::MyTrait)]"),
            )
        })?;

        Ok(TraitTarget { type_path })
    }
}

#[derive(Clone)]
pub enum ReturnExpression {
    Into(Option<TypePath>),
    TryInto,
    Unwrap,
}

enum ParsedAttribute {
    ReturnExpression(ReturnExpression),
    Await(bool),
    TargetMethod(syn::Ident),
    ThroughTrait(TraitTarget),
}

fn parse_attributes(
    attrs: &[Attribute],
) -> (
    impl Iterator<Item = ParsedAttribute> + '_,
    impl Iterator<Item = &Attribute>,
) {
    let (parsed, other): (Vec<_>, Vec<_>) = attrs
        .iter()
        .map(|attribute| {
            let parsed = if let syn::AttrStyle::Outer = attribute.style {
                let name = attribute
                    .path
                    .get_ident()
                    .map(|i| i.to_string())
                    .unwrap_or_default();
                match name.as_str() {
                    "call" => {
                        let target = syn::parse2::<CallMethodAttribute>(attribute.tokens.clone())
                            .expect("Cannot parse `call` attribute");
                        Some(ParsedAttribute::TargetMethod(target.name))
                    }
                    "into" => {
                        let into = syn::parse2::<IntoAttribute>(attribute.tokens.clone())
                            .expect("Cannot parse `into` attribute");
                        Some(ParsedAttribute::ReturnExpression(ReturnExpression::Into(
                            into.type_path,
                        )))
                    }
                    "try_into" => {
                        let meta = attribute
                            .parse_meta()
                            .expect("Invalid `try_into` arguments");

                        if let Meta::List(list) = meta {
                            if let Some(syn::NestedMeta::Meta(Meta::Path(path))) =
                                list.nested.first()
                            {
                                if path.is_ident("unwrap") {
                                    panic!(
                                        "Replace #[try_into(unwrap)] with\n#[try_into]\n#[unwrap]",
                                    );
                                }
                            }
                        }
                        Some(ParsedAttribute::ReturnExpression(ReturnExpression::TryInto))
                    }
                    "unwrap" => Some(ParsedAttribute::ReturnExpression(ReturnExpression::Unwrap)),
                    "await" => {
                        let generate =
                            syn::parse2::<GenerateAwaitAttribute>(attribute.tokens.clone())
                                .expect("Cannot parse `await` attribute");
                        Some(ParsedAttribute::Await(generate.literal.value))
                    }
                    "through" => Some(ParsedAttribute::ThroughTrait(
                        syn::parse2::<TraitTarget>(attribute.tokens.clone())
                            .expect("Cannot parse `through` attribute"),
                    )),
                    _ => None,
                }
            } else {
                None
            };

            (parsed, attribute)
        })
        .partition(|(parsed, _)| parsed.is_some());
    (
        parsed.into_iter().map(|(parsed, _)| parsed.unwrap()),
        other.into_iter().map(|(_, attr)| attr),
    )
}

pub struct MethodAttributes<'a> {
    pub attributes: Vec<&'a Attribute>,
    pub target_method: Option<syn::Ident>,
    pub expressions: VecDeque<ReturnExpression>,
    pub generate_await: Option<bool>,
    pub target_trait: Option<TypePath>,
}

/// Iterates through the attributes of a method and filters special attributes.
/// - call => sets the name of the target method to call
/// - into => generates a `into()` call after the delegated expression
/// - try_into => generates a `try_into()` call after the delegated expression
/// - await => generates an `.await` expression after the delegated expression
/// - unwrap => generates a `unwrap()` call after the delegated expression
/// - throuhg => generates a UFCS call (`Target::method(&<expr>, ...)`) around the delegated expression
pub fn parse_method_attributes<'a>(
    attrs: &'a [Attribute],
    method: &syn::TraitItemMethod,
) -> MethodAttributes<'a> {
    let mut target_method: Option<syn::Ident> = None;
    let mut expressions: Vec<ReturnExpression> = vec![];
    let mut generate_await: Option<bool> = None;
    let mut target_trait: Option<TraitTarget> = None;

    let (parsed, other) = parse_attributes(attrs);
    for attr in parsed {
        match attr {
            ParsedAttribute::ReturnExpression(expr) => expressions.push(expr),
            ParsedAttribute::Await(value) => {
                if generate_await.is_some() {
                    panic!(
                        "Multiple `await` attributes specified for {}",
                        method.sig.ident
                    )
                }
                generate_await = Some(value);
            }
            ParsedAttribute::TargetMethod(target) => {
                if target_method.is_some() {
                    panic!(
                        "Multiple call attributes specified for {}",
                        method.sig.ident
                    )
                }
                target_method = Some(target);
            }
            ParsedAttribute::ThroughTrait(target) => {
                if target_trait.is_some() {
                    panic!(
                        "Multiple through attributes specified for {}",
                        method.sig.ident
                    )
                }
                target_trait = Some(target);
            }
        }
    }

    MethodAttributes {
        attributes: other.into_iter().collect(),
        target_method,
        generate_await,
        expressions: expressions.into(),
        target_trait: target_trait.map(|t| t.type_path),
    }
}

pub struct SegmentAttributes {
    pub expressions: Vec<ReturnExpression>,
    pub generate_await: Option<bool>,
    pub target_trait: Option<TypePath>,
}

pub fn parse_segment_attributes(attrs: &[Attribute]) -> SegmentAttributes {
    let mut expressions: Vec<ReturnExpression> = vec![];
    let mut generate_await: Option<bool> = None;
    let mut target_trait: Option<TraitTarget> = None;

    let (parsed, mut other) = parse_attributes(attrs);
    if other.next().is_some() {
        panic!("Only return expression attributes can be used on a segment (into, try_into, unwrap or await).");
    }

    for attribute in parsed {
        match attribute {
            ParsedAttribute::ReturnExpression(expr) => expressions.push(expr),
            ParsedAttribute::Await(value) => {
                if generate_await.is_some() {
                    panic!("Multiple `await` attributes specified for segment");
                }
                generate_await = Some(value);
            }
            ParsedAttribute::ThroughTrait(target) => {
                if target_trait.is_some() {
                    panic!("Multiple `through` attributes specified for segment");
                }
                target_trait = Some(target);
            }
            ParsedAttribute::TargetMethod(_) => {
                panic!("Call attribute cannot be specified on a `to <expr>` segment.");
            }
        }
    }
    SegmentAttributes {
        expressions,
        generate_await,
        target_trait: target_trait.map(|t| t.type_path),
    }
}

/// Applies default values from the segment and adds them to the method attributes.
pub fn combine_attributes<'a>(
    mut method_attrs: MethodAttributes<'a>,
    segment_attrs: &SegmentAttributes,
) -> MethodAttributes<'a> {
    if method_attrs.generate_await.is_none() {
        method_attrs.generate_await = segment_attrs.generate_await;
    }

    if method_attrs.target_trait.is_none() {
        method_attrs.target_trait = segment_attrs.target_trait.clone();
    }

    for expr in &segment_attrs.expressions {
        match expr {
            ReturnExpression::Into(path) => {
                if !method_attrs
                    .expressions
                    .iter()
                    .any(|expr| matches!(expr, ReturnExpression::Into(_)))
                {
                    method_attrs
                        .expressions
                        .push_front(ReturnExpression::Into(path.clone()));
                }
            }
            _ => method_attrs.expressions.push_front(expr.clone()),
        }
    }

    method_attrs
}
