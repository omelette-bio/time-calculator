use crate::time_data::TimeData;

#[derive(Debug)]
pub enum Value {
    Time(TimeData), Variable(String)
}

#[derive(Debug)]
pub enum Operations {
    ADD(Value, Value), SUB(Value, Value),
    ATTR(String, Value), VAL(Value), WAIT
}