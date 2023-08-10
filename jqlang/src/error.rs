#[derive(Debug)]
pub enum CompileError{
    NoInput,
    MultipleInput,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileError::NoInput => write!(f, "compilation error: no input value or stream provided"),
            CompileError::MultipleInput => write!(f, "compilation error: multiple input values or streams provided"),
        }
    }
}

impl std::error::Error for CompileError {}
