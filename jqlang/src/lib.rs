use serde::{Deserialize, de::Visitor};
use serde_json::Value;

pub mod error;
use error::CompileError;

#[derive(Debug, Clone)]
pub enum FilterOp {
    Identity,
}

#[derive(Debug)]
pub(crate) enum Token {
    JsonValue(Value),
    FiltOp(FilterOp),
}


#[derive(Debug)]
pub(crate) struct JqAst {
    tokens: Vec<Token>,
}


#[derive(Debug)]
pub(crate) struct JqChain {
    input: Value,
    filters: Vec<FilterOp>,
}

impl JqAst {
    pub fn compile(self) -> Result<JqChain, CompileError> {
        if self.tokens.is_empty() {
            return Err(CompileError::NoInput);
        } else {
            let mut input = None;
            let mut filters = Vec::new();

            for token in self.tokens {
                match token {
                    Token::JsonValue(v) => {
                        if input.is_none() {
                            input = Some(v);
                        } else {
                            return Err(CompileError::MultipleInput);
                        }
                    }
                    Token::FiltOp(f) => {
                        filters.push(f);
                    }
                }
            }
            Ok(JqChain {
                input: Value::Null,
                filters: Vec::new(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use jqlang_proc_macro::jq;

    #[test]
    fn jq_macro_test() {
        let input = json!({
            "foo": "bar",
            "baz": [1, 2, 3],
        });
        let output = jq!(input | .);
        assert_eq!(output, input);
    }
}