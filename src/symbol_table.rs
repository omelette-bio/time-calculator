use std::collections::HashMap;
use crate::time_data::TimeData;
use crate::eval_error::EvalError;

#[derive(Debug, Default)]
pub struct SymbolTable {
    st: HashMap<String, TimeData> 
}

impl SymbolTable {
    pub fn new() -> Self { SymbolTable::default() }

    pub fn read_variable(&self, var_name: String) -> Result<TimeData, EvalError> { 
        match self.st.get(&var_name) {
            Some(&v) => Ok(v),
            None => Err(EvalError::VariableNotFound(var_name.clone()))
        } 
    }

    pub fn attribute_value(&mut self, var_name: String, data: TimeData) { self.st.insert(var_name, data); }
}
