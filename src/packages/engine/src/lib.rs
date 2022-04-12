#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod error;
mod evaluate;
mod grammar;
mod parser;
mod prec_climber;
pub mod primative;

use crate::error::Error;
use crate::parser::FormulaParser;
use crate::parser::Rule;
use crate::pest::Parser;
use crate::primative::Primative;
use serde_json::Value;
use std::collections::HashMap;

pub fn eval(expression: &str, data_payload: &HashMap<String, Value>) -> Result<Primative, Error> {
    match FormulaParser::parse(Rule::calculation, expression) {
        Ok(mut parsed) => {
            // let data_payload: HashMap<String, Value> =
            //     serde_json::from_str(data_payload_str).unwrap();
            let response = evaluate::evaluate(parsed.next().unwrap(), data_payload);
            println!("Response: {:#?}", response);
            response
        }
        Err(err) => {
            println!("Err: {:#?}", err);
            Err(Error::Parse(err))
        }
    }
}
