pub use jqlang_proc_macro::jq;

#[macro_export]
macro_rules! run {
    ( $x:expr ) => {
        $crate::automaton::Automaton::from($x).execute()
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::json;
    use jqlang_proc_macro::jq;

    #[test]
    fn jq_macro_test() {
        let input = json!({
            "foo": "bar",
            "baz": [1, 2, 3]
        });
        let ast = jq!(input | .);
        let parsed = run!(ast);
        assert_eq!(input, parsed);
    }
}