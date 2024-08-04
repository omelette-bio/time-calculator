use pest_derive::Parser;
use pest::Parser;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use crate::operations::{Operations, Value};
use crate::time_data::TimeData;

#[derive(Parser)]
#[grammar = "parsing/grammar.pest"]
pub struct IdentParser;

pub fn parser(input: &str) {
    let mut pairs = IdentParser::parse(Rule::expression, input).expect("azy ya une erreur");
    // print_content(pairs.next().unwrap());
    //parse(pairs.next().unwrap());

    // let mut var_name = String::new();
    // let mut hrs = String::new();
    // let mut min = String::new();
    // let mut sec = String::new();
    // let mut ms = String::new();
    // traverse(pairs.next().unwrap(), &mut var_name, &mut hrs, &mut min, &mut sec, &mut ms);
    //
    // println!("{} {} {} {} {}", var_name, hrs, min, sec, ms);
}

pub fn print_content(input: &str) {
    let mut pairs = IdentParser::parse(Rule::expression, input).expect("azy ya une erreur");
    let mut pair = pairs.next().unwrap().clone().into_inner().next().unwrap();
    for inner_pair in pair.into_inner() {
        println!("{:?}", inner_pair.as_rule());
        for inner_inner_pair in inner_pair.into_inner() {
            println!("  {:?}", inner_inner_pair.as_rule());
            for inner_inner_inner_pair in inner_inner_pair.into_inner() {
                println!("      {:?}", inner_inner_inner_pair.as_rule());
                for inner_inner_inner_inner_pair in inner_inner_inner_pair.into_inner() {
                    println!("          {:?}", inner_inner_inner_inner_pair.as_rule());
                }
            }
        }
    }
}

pub fn parse(input: &str) -> Operations {
    let mut pairs = IdentParser::parse(Rule::expression, input).expect("azy ya une erreur");
    let mut pair = pairs.next().unwrap().clone().into_inner().next().unwrap();
    match pair.as_rule() {
        // expression = { calculus | var_decl | value  }
        Rule::calculus => {
            // enter in the pair to get access to the different elements of the pair
            let mut calc = pair.into_inner();
            // first parse the first value
            let value1=  parse_value(calc.next().unwrap().into_inner().next().unwrap());
            match calc.next().unwrap().as_str() {
                "+" => Operations::ADD(value1, parse_value(calc.next().unwrap().into_inner().next().unwrap())),
                "-" => Operations::SUB(value1, parse_value(calc.next().unwrap().into_inner().next().unwrap())),
                _ => unreachable!()
            }
        },
        Rule::var_decl => todo!(),
        Rule::value => Operations::VAL(parse_value(pair.into_inner().next().unwrap())),
        _ => unreachable!()
    }
}

pub fn parse_value(pair: Pair<Rule>) -> Value {
    //println!("{:?}", pair.as_rule());
    match pair.as_rule() {
        Rule::duration => {
            let mut dur = pair.into_inner();
            Value::Time(
                TimeData::new(
                    dur.next().unwrap().as_str().parse::<usize>().unwrap(),
                    dur.next().unwrap().as_str().parse::<u8>().unwrap(),
                    dur.next().unwrap().as_str().parse::<u8>().unwrap(),
                    dur.next().unwrap().as_str().parse::<u16>().unwrap()
                ))
        },
        Rule::var_call => Value::Variable(pair.as_str().trim().to_string()),
        _ => unreachable!()
    }
}

fn traverse(pair: Pair<Rule>, var_name: &mut String, hrs: &mut String, min: &mut String, sec: &mut String, ms: &mut String){
    match pair.as_rule() {
        Rule::var_call => {
            *var_name = pair.as_str().to_string();
        }
        Rule::hrs => {
            *hrs = pair.as_str().to_string();
        }
        Rule::min => {
            if min.is_empty() {
                *min = pair.as_str().to_string();
            } else {
                *sec = pair.as_str().to_string();
            }
        }
        Rule::ms => {
            *ms = pair.as_str().to_string();
        }
        _ => {
            for inner_pair in pair.into_inner() {
                traverse(inner_pair, var_name, hrs, min, sec, ms);
            }
        }
    }
}