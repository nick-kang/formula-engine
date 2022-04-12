use crate::parser::Rule;
use crate::primative::Primative;
use crate::{error::Error, evaluate};
use anyhow::{format_err, Context};
use pest::iterators::Pairs;
use serde_json::Value;
use std::collections::HashMap;

pub fn evaluate(mut pairs: Pairs<Rule>, data: &HashMap<String, Value>) -> Result<Primative, Error> {
    let left = evaluate::evaluate_raw(
        pairs
            .next()
            .with_context(|| format_err!("Unable to parse"))?,
        data,
    )?;

    let right = evaluate::evaluate_raw(
        pairs
            .next()
            .with_context(|| format_err!("Unable to parse"))?,
        data,
    )?;

    Ok(Primative::Boolean(left == right))
}
