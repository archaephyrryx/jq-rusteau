use jqlang::JqChain;
use jqlang::FilterOp;
use serde_json::Value;

use crate::filter::Filter;
use crate::filter::Identity;

#[derive(Debug)]
pub struct Transformation<T = Value, U = Value> {
    pub(crate) inner: Box<dyn Filter<T, U>>,
}

impl<T, U> Transformation<T, U> {
    pub fn new(inner: Box<dyn Filter<T, U>>) -> Self {
        Self { inner }
    }
}

impl From<FilterOp> for Transformation {
    fn from(op: FilterOp) -> Self {
        match op {
            FilterOp::Identity => Self::new(Box::new(Identity)),
        }
    }
}

pub struct Automaton {
    input: Value,
    transformations: Vec<Transformation>,
}

impl Automaton {
    pub fn execute(self) -> Value {
        let mut value = self.input;
        for transformation in self.transformations {
            value = transformation.inner.on_value(value);
        }
        value
    }
}

impl From<JqChain> for Automaton {
    fn from(chain: JqChain) -> Self {
        let transformations = chain.filters.into_iter().map(Transformation::from).collect();

        Self {
            input: chain.input,
            transformations,
        }
    }
}
