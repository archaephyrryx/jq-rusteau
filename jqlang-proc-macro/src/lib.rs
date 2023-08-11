extern crate jqlang;
extern crate serde;
extern crate serde_json;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

use syn::parse_macro_input;

mod jq_parse;
mod tok;

use jq_parse::jq_parse;
use jq_parse::JqAst;

#[proc_macro]
pub fn jq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as JqAst);
    jq_parse(input).into()
}