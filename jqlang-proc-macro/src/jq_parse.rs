use proc_macro2::TokenStream;
use syn::parse::Parse;
use crate::tok::JqToken;
use quote::quote;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct JqAst {
    tokens: Vec<JqToken>,
}

impl JqAst {
    pub fn into_chain(self) -> TokenStream {
        let mut iter = self.tokens.into_iter();
        let input = if let Some(value) = iter.next() {
            match value {
                JqToken::JExpr(v) => quote!{ #v },
                JqToken::JIdent(i) => quote! { #i },
                _ => panic!("first token must be a JSON value"),
            }
        } else {
            panic!("no input value provided")
        };
        let mut filters = TokenStream::new();
        for token in iter {
            match token {
                // TODO: handle pipe more rigorously
                JqToken::Pipe(_) => { /* do nothing */ }
                JqToken::Filter(f) => {
                    let filter = match f {
                        crate::tok::JqFilterToken::Identity => quote! { ::jqlang::FilterOp::Identity },
                    };
                    filters.extend(std::iter::once(filter));
                }
                _ => panic!("expected filter"),
            }
        }
        quote! {
            ::jqlang::JqChain {
                input: #input,
                filters: vec![$(#filters),*],
            }
        }
    }
}


impl Parse for JqAst {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut tokens = Vec::new();
        while !input.is_empty() {
            let token = input.parse::<JqToken>()?;
            tokens.push(token);
        }
        Ok(JqAst { tokens })
    }
}

pub(crate) fn jq_parse(input: JqAst) -> proc_macro2::TokenStream {
    input.into_chain()
}

#[cfg(test)]
mod tests {
    use crate::tok::JqFilterToken;

    use super::*;
    use syn::{Expr, ExprLit, LitInt, Token};

    #[test]
    fn parse_works() {
        let input = quote! {
            42 | .
        };
        let parsed = syn::parse2::<JqAst>(input).unwrap();
        let expected = JqAst {
            tokens: vec![
                JqToken::JExpr(quote![ 42 ]),
                JqToken::Pipe,
                JqToken::Filter(JqFilterToken::Identity),
            ],
        };
        assert_eq!(parsed, expected);
    }
}