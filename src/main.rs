mod time_data;
mod eval_error;
mod symbol_table;
mod operations;
mod parsing;

use std::io;
use std::io::{BufRead, Write};
use symbol_table::SymbolTable;
use crate::parsing::parse;

fn prompt() {
    print!("time calculator # ");
    io::stdout().flush().unwrap();
}

fn main() {
    prompt();
    
    let mut st = SymbolTable::default();
    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        let line = line.unwrap();
        match parse::parse_input(&line).interp(&mut st) {
            Ok(res) => {
                if let Some(td) = res { println!("{}", td) }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        prompt();
    }
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
