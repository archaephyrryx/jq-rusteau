use proc_macro2::{TokenTree, TokenStream};
use serde_json::{Value, json};
use syn::{parse::Parse, Token, Ident, token::Group, Expr};
use quote::quote;

#[derive(Debug, Clone, PartialEq)]
pub enum JqFilterToken {
    Identity,
}

impl Parse for JqFilterToken {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![.]) {
            Ok(JqFilterToken::Identity)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(PartialEq)]
pub enum JqToken {
    Pipe,
    Filter(JqFilterToken),
    JIdent(Ident),
    JExpr(Value),
}


impl Parse for JqToken {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![|]) {
            input.parse::<Token![|]>().map(|_| JqToken::Pipe)
        } else if lookahead.peek(Token![.]) {
            input.parse().map(JqToken::Filter)
        } else if lookahead.peek(Ident) {
            input.parse().map(JqToken::JIdent)
        } else if lookahead.peek(Group) {
            let tmp = input.parse::<TokenStream>()?;
            Ok(JqToken::JExpr(quote! {json!(#tmp) }.to_string().parse().unwrap()))
        } else {
            Err(lookahead.error())
        }
    }
}