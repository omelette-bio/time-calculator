use std::rc::Rc;
use crate::eval_error::EvalError;
use crate::symbol_table::SymbolTable;
use crate::time_data::TimeData;

#[derive(Debug)]
pub enum Value {
    Time(TimeData), Variable(String)
}

#[derive(Debug)]
pub enum Operations {
    Add(Value, Value), Sub(Value, Value),
    Attr(String, Rc<Operations>), Val(Value), Wait
}

impl Value {
    pub fn get_val(&self, st: &SymbolTable) -> Result<TimeData, EvalError> {
        match self {
            Value::Time(td) => Ok(*td),
            Value::Variable(v_name) => {
                Ok(st.read_variable(v_name.clone())?)
            }
        }
    }
}

impl Operations {
    pub fn interp(&self, st: &mut SymbolTable) -> Result<Option<TimeData>, EvalError> {
        match self {
            Operations::Add(v1, v2) => Ok(Some(v1.get_val(st)? + v2.get_val(st)?)),
            Operations::Sub(v1, v2) => Ok(Some(v1.get_val(st)? - v2.get_val(st)?)),
            Operations::Attr(v_name, v) =>  {
                let value = v.interp(st)?.unwrap();
                st.attribute_value(v_name.clone(), value);
                Ok(None)
            },
            Operations::Val(v) => Ok(Some(v.get_val(st)?)),
            Operations::Wait => todo!()
        }
    }
}