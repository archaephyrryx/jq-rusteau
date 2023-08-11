use serde_json::Value;

#[derive(Debug, Clone)]
pub enum FilterOp {
    Identity,
}

#[derive(Debug)]
pub struct JqChain {
    pub input: Value,
    pub filters: Vec<FilterOp>,
}