use crate::parser::Rule;
use crate::primative::Primative;
use crate::{error::Error, evaluate};
use anyhow::{format_err, Context};
use pest::iterators::Pairs;
use serde_json::Value;
use std::collections::HashMap;

pub fn evaluate(mut pairs: Pairs<Rule>, data: &HashMap<String, Value>) -> Result<Primative, Error> {
    let condition_primative = evaluate::evaluate_raw(
        pairs
            .next()
            .with_context(|| format_err!("Unable to parse"))?,
        data,
    )?;

    let condition = match condition_primative {
        Primative::Boolean(val) => val,
        Primative::String(val) => return Err(Error::InvalidBoolean(val)),
        Primative::Float64(val) => return Err(Error::InvalidBoolean(val.to_string())),
        Primative::Null => return Err(Error::InvalidBoolean(String::from("null"))),
    };

    let value_if_true = pairs
        .next()
        .with_context(|| format_err!("Unable to parse"))?;

    if condition {
        evaluate::evaluate_raw(value_if_true, data)
    } else {
        let value_if_false = pairs
            .next()
            .with_context(|| format_err!("Unable to parse"))?;

        evaluate::evaluate_raw(value_if_false, data)
    }
}
