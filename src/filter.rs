use std::fmt::Debug;

use jqlang::CheapStr;
use serde_json::Value;

pub trait Filter<Input, Output>
where Input: Clone + Debug,
      Output: Clone + Debug,
      Self: Debug,
{
    fn on_value(&self, input: Input) -> Output;
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Identity;

impl<T: Clone + Debug> Filter<T, T> for Identity {
    fn on_value(&self, input: T) -> T {
        input
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct ObjIndex {
    index: CheapStr,
}

impl From<&str> for ObjIndex {
    fn from(index: &str) -> Self {
        Self { index: std::rc::Rc::new(index.to_string()) }
    }
}

impl From<CheapStr> for ObjIndex {
    fn from(index: CheapStr) -> Self {
        Self { index }
    }
}

impl Filter<Value, Value> for ObjIndex {
    fn on_value(&self, input: Value) -> Value {
        input[self.index.as_str()].clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{value::{ Value, Number }, json};

    #[test]
    fn identity_json_value() {
        let input = Value::Number(Number::from(42));
        let filt = Identity;
        let output = filt.on_value(input.clone());
        assert_eq!(input, output);
    }

    #[test]
    fn index_simple() {
        let input = json!({
            "foo": "bar",
            "baz": [1, 2, 3]
        });
        let filt0 = ObjIndex::from("foo");
        let filt1 = ObjIndex::from("baz");

        assert_eq!(filt0.on_value(input.clone()), json!("bar"));
        assert_eq!(filt1.on_value(input.clone()), json!([1, 2, 3]));
    }

}
