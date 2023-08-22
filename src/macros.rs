#[macro_export]
macro_rules! jq {
    ( $x:expr => $script:tt ) => {
        {
            use ::chumsky::Parser;

            match ::jqlang_parse::parser().parse($script) {
                Ok(filters) => Ok(::jqlang::JqChain { input: json!($x), filters }),
                Err(e) => Err(e)
            }
        }
    };
}


#[macro_export]
macro_rules! run {
    ( $x:expr ) => {
        $crate::automaton::Automaton::from($x).execute()
    };
}

#[cfg(test)]
pub mod tests {
    use serde_json::json;

    #[test]
    fn jq_macro_test() {
        let input = json!({
            "foo": "bar",
            "baz": [1, 2, 3]
        });
        let Ok(ast) = jq!(input => ".") else { panic!("Bad parse!") };
        let parsed = run!(ast);
        assert_eq!(input, parsed);
    }
}