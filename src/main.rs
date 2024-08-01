mod time_data;
mod eval_error;
mod symbol_table;

use time_data::TimeData;
use eval_error::EvalError;
use symbol_table::SymbolTable;

fn main() {
    let t_d1 = TimeData::new(0,45,32,461);
    let t_d2 = TimeData::new(0,48,19,690);
    let t_d3 = TimeData::new(0,43,39,632);
    
    let mut c = SymbolTable::new();
    c.attribute_value("X".to_string(), t_d1);
    
    let res = c.read_variable("X".to_string());
    let res2 = c.read_variable("C".to_string());

    if let Err(m) = res { println!("{}", m);}
    else { println!("{}", res.unwrap());}

    if let Err(m) = res2 { println!("{}", m);}
    else { println!("{}", res2.unwrap());}

    println!("{}", EvalError::InvalidData);
    println!("{}", EvalError::NegativeResult);
    println!("{}", t_d1);
    println!("{}", TimeData::from_ms(t_d1.to_ms()));
    println!("{}", t_d2 - t_d1);
    println!("{}", t_d2 - t_d3);
}

#[cfg(test)]
mod tests {
    use crate::time_data::TimeData;    

    #[test]
    fn test_add1() {
        let t1 = TimeData::new(0,1,31,99);
        assert_eq!(t1+t1, TimeData::new(0,3,2,198));
    }

    #[test]
    fn test_add2() {
        let t1 = TimeData::new(0,0,0,500);
        assert_eq!(t1+t1, TimeData::new(0,0,1,0));
    }

    #[test]
    fn test_add3() {
        let t1 = TimeData::new(0,1,20,256);
        let t2 = TimeData::new(0,2,26,812);
        assert_eq!(t1+t2, TimeData::new(0,3,47,68));
    }
    
    #[test]
    fn test_add4() {
        let t1 = TimeData::new(1,51,29,500);
        let t2 = TimeData::new(0,9,30,501);
        assert_eq!(t1+t2, TimeData::new(2,1,0,1));
    }
}
