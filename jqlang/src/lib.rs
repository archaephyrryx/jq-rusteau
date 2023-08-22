use serde_json::Value;

pub type CheapStr = std::rc::Rc<String>;

#[derive(Debug, Clone)]
pub enum FilterOp {
    Identity,
    ObjIdentIndex(CheapStr),
}

#[derive(Debug)]
pub struct JqChain {
    pub input: Value,
    pub filters: Vec<FilterOp>,
}