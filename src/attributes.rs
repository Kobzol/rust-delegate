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

pub enum ReturnExpression {
    Into(Option<TypePath>),
    TryInto,
    Unwrap,
}

pub struct ParsedAttributes<'a> {
    pub attributes: Vec<&'a Attribute>,
    pub target_method: Option<syn::Ident>,
    pub expressions: Vec<ReturnExpression>,
    pub generate_await: bool,
}

/// Iterates through the attributes of a method and filters special attributes.
/// - call => sets the name of the target method to call
/// - into => generates a `into()` call after the delegated expression
/// - try_into => generates a `try_into()` call after the delegated expression
/// - await => generates an `.await` expression after the delegated expression
/// - unwrap => generates a `unwrap()` call after the delegated expression
pub fn parse_attributes<'a>(
    attrs: &'a [syn::Attribute],
    method: &syn::TraitItemMethod,
) -> ParsedAttributes<'a> {
    let mut target_method: Option<syn::Ident> = None;
    let mut expressions: Vec<ReturnExpression> = vec![];
    let mut generate_await: Option<bool> = None;

    let attrs: Vec<&Attribute> = attrs
        .iter()
        .filter(|attribute| {
            if let syn::AttrStyle::Outer = attribute.style {
                let name = attribute
                    .path
                    .get_ident()
                    .map(|i| i.to_string())
                    .unwrap_or_default();
                match name.as_str() {
                    "call" => {
                        let target =
                            syn::parse2::<CallMethodAttribute>(attribute.tokens.clone()).unwrap();
                        if target_method.is_some() {
                            panic!(
                                "Multiple call attributes specified for {}",
                                method.sig.ident
                            )
                        }
                        target_method = Some(target.name);
                        return false;
                    }
                    "into" => {
                        let into = syn::parse2::<IntoAttribute>(attribute.tokens.clone()).unwrap();
                        expressions.push(ReturnExpression::Into(into.type_path));
                        return false;
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
                        expressions.push(ReturnExpression::TryInto);
                        return false;
                    }
                    "unwrap" => {
                        expressions.push(ReturnExpression::Unwrap);
                        return false;
                    }
                    "await" => {
                        if generate_await.is_some() {
                            panic!(
                                "Multiple `await` attributes specified for {}",
                                method.sig.ident
                            )
                        }
                        let generate =
                            syn::parse2::<GenerateAwaitAttribute>(attribute.tokens.clone())
                                .unwrap();
                        generate_await = Some(generate.literal.value);
                        return false;
                    }
                    _ => {}
                }
            }

            true
        })
        .collect();

    ParsedAttributes {
        attributes: attrs,
        target_method,
        generate_await: generate_await.unwrap_or_else(|| method.sig.asyncness.is_some()),
        expressions,
    }
}
