use std::fmt;

#[derive(Debug)]
pub enum EvalError {
    InvalidData,
    NegativeResult,
    VariableNotFound(String)
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::InvalidData => write!(f, "Error, can't put negative values to duration."),
            EvalError::NegativeResult => write!(f, "Error, the result of the substraction is negative."),
            EvalError::VariableNotFound(v) => write!(f, "Error, the variable {} has no value.", v)
        }
    }  
}
