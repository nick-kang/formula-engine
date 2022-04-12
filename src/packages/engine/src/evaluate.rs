use crate::grammar::field::field;
use crate::grammar::primative;
use crate::parser::Rule;
use crate::prec_climber::PREC_CLIMBER;
use crate::primative::Primative;
use crate::{
    error::Error,
    grammar::{
        conditional::{and, eq, n_eq, or},
        control_flow::if_fn,
    },
};
use pest::iterators::{Pair, Pairs};
use serde_json::Value;
use std::collections::HashMap;

pub fn evaluate(pair: Pair<Rule>, data: &HashMap<String, Value>) -> Result<Primative, Error> {
    evaluate_raw(pair, data)
}

fn evaluate_expression(
    pairs: Pairs<Rule>,
    data: &HashMap<String, Value>,
) -> Result<Primative, Error> {
    PREC_CLIMBER.climb(
        pairs,
        |pair: Pair<Rule>| evaluate_raw(pair, data),
        |left: Result<Primative, Error>, op: Pair<Rule>, right: Result<Primative, Error>| {
            let lhs = extract_num(left)?;
            let rhs = extract_num(right)?;

            match op.as_rule() {
                Rule::add => Ok(Primative::Float64(lhs + rhs)),
                Rule::subtract => Ok(Primative::Float64(lhs - rhs)),
                Rule::multiply => Ok(Primative::Float64(lhs * rhs)),
                Rule::divide => Ok(Primative::Float64(lhs / rhs)),
                Rule::power => Ok(Primative::Float64(lhs.powf(rhs))),
                _ => unreachable!(),
            }
        },
    )
}

fn extract_num(result: Result<Primative, Error>) -> Result<f64, Error> {
    match result {
        Ok(primative) => match primative {
            Primative::Float64(num) => Ok(num),
            Primative::Boolean(bool) => Err(Error::InvalidNumber(bool.to_string())),
            Primative::Null => Err(Error::InvalidNumber("null".to_string())),
            Primative::String(s) => Err(Error::InvalidNumber(s)),
        },
        Err(e) => Err(e),
    }
}

pub fn evaluate_raw(pair: Pair<Rule>, data: &HashMap<String, Value>) -> Result<Primative, Error> {
    match pair.as_rule() {
    Rule::number => primative::number::evaluate(pair),
    Rule::string => primative::string::evaluate(pair),
    Rule::boolean => primative::boolean::evaluate(pair),
    Rule::null => primative::null::evaluate(pair),

    Rule::math_expression => evaluate_expression(pair.into_inner(), data),
    Rule::field => field::evaluate(pair, data),
    Rule::if_fn => if_fn::evaluate(pair.into_inner(), data),
    Rule::and => and::evaluate(pair.into_inner(), data),
    Rule::or => or::evaluate(pair.into_inner(), data),
    Rule::eq => eq::evaluate(pair.into_inner(), data),
    Rule::n_eq => n_eq::evaluate(pair.into_inner(), data),
    _ => unreachable!()

    // Rule::field => pair
    //   .into_inner()
    //   .next()
    //   .unwrap()
    //   .as_str()
    //   .parse::<f64>()
    //   .unwrap(),
    // Rule::EOI
    // // | Rule::number
    // | Rule::WHITESPACE
    // // | Rule::field
    // | Rule::field_id
    // | Rule::field_id_inner_quotes
    // | Rule::inner
    // | Rule::char
    // | Rule::primative
    // | Rule::add
    // | Rule::subtract
    // | Rule::multiply
    // | Rule::divide
    // | Rule::power
    // | Rule::math_operation
    // | Rule::term
    // | Rule::and
    // | Rule::or
    // | Rule::eq
    // | Rule::n_eq
    // | Rule::calculation => unreachable!(),
}
}
