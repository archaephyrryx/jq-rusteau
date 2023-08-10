use std::fmt::Debug;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::value::{ Value, Number };

    #[test]
    fn identity_json_value() {
        let input = Value::Number(Number::from(42));
        let filt = Identity;
        let output = filt.on_value(input.clone());
        assert_eq!(input, output);
    }
}
