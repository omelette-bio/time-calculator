use std::rc::Rc;
use pest_derive::Parser;
use pest::Parser;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use crate::operations::{Operations, Value};
use crate::time_data::TimeData;

#[derive(Parser)]
#[grammar = "parsing/grammar.pest"]
pub struct IdentParser;

pub fn print_content(input: &str) {
    let mut pairs = IdentParser::parse(Rule::expression, input).expect("azy ya une erreur");
    let pair = pairs.next().unwrap().clone().into_inner().next().unwrap();
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

pub fn parse_input(input: &str) -> Operations {
    let mut pairs = IdentParser::parse(Rule::expression, input).expect("azy ya une erreur");
    parse(pairs.next().unwrap().clone().into_inner().next().unwrap())
}

pub fn parse(pair: Pair<Rule>) -> Operations {

    match pair.as_rule() {
        // expression = { calculus | var_decl | value  }
        Rule::calculus => {
            // enter the pair to get access to the different elements of the pair
            let mut calc = pair.into_inner();
            // first parse the first value
            let value1 = parse_value(calc.next().unwrap().into_inner().next().unwrap());
            match calc.next().unwrap().as_str() {
                "+" => Operations::Add(value1, parse_value(calc.next().unwrap().into_inner().next().unwrap())),
                "-" => Operations::Sub(value1, parse_value(calc.next().unwrap().into_inner().next().unwrap())),
                _ => unreachable!()
            }
        },
        Rule::var_decl => {
            let mut decl = pair.into_inner();
            
            let var = decl.next().unwrap().as_str().trim();
            let val = parse(decl.next().unwrap());

            Operations::Attr(var.to_string(), Rc::new(val))
        },
        Rule::value => Operations::Val(parse_value(pair.into_inner().next().unwrap())),
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