extern crate jqlang;
extern crate serde;
extern crate serde_json;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

use quote::quote;

#[proc_macro]
pub fn jq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output: proc_macro2::TokenStream = {
        let mut tokens = input.into_iter();
        let mut init_tokens = tokens
            .take_while(|t| t.to_string() != "|")
            .collect::<proc_macro2::TokenStream>();
        let mut filters = Vec::new();
        let mut filter_tokens = tokens.skip(1);
        let mut current_filter_tokens = Vec::new();
        while let Some(token) = filter_tokens.next() {
            if token.to_string() == "|" {
                filters.push(filter_parse(current_filter_tokens));
                current_filter_tokens.clear();
            } else {
                current_filter_tokens.push(token);
            }
        }
        if !current_filter_tokens.is_empty() {
            filters.push(filter_parse(current_filter_tokens));
        }
        let mut tt_init = quote! { $( #init_tokens )+ };
        let mut filter_tts = Vec::new();
        for filter in filters.into_iter() {
            filter_tts.push(quote! { #filter });
        }
        let tt_filters  = (0..filter_tts.len()).map(|ix| {
            let x = filter_tts[ix];
            quote! { ::jqlang::Token::FiltOp(#x) }
        });
        let filter_seg = quote! { $( #tt_filters ),* };
        quote! {
            ::jqlang::JqAst { vec![::jqlang::Token::JsonValue(#tt_init) $(, ::jqlang::Token::FiltOp( #tt_filters ) )* ] }
        }
    };
    proc_macro::TokenStream::from(output)
}

fn filter_parse(current_filter_tokens: Vec<proc_macro2::TokenTree>) -> proc_macro2::TokenStream {
    match &current_filter_tokens[..] {
        &[x] if x.to_string() == "." => quote! { ::jqlang::FilterOp::Identity },
        _ => unreachable!("Invalid filter: {:?}", current_filter_tokens),
    }
}
