use std::fmt;

pub enum EvalError {
    InvalidData,
    NegativeResult
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::InvalidData => write!(f, "Error, can't put negative values to duration."),
            EvalError::NegativeResult => write!(f, "Error, the result of the substraction is negative.")
        }
    }  
}
