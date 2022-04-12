use crate::error::Error;
use crate::error::Error::FieldNotFound;
use crate::parser::Rule;
use crate::primative::Primative;
use anyhow::{format_err, Context};
use pest::iterators::Pair;
use serde_json::Value;
use std::collections::HashMap;

pub fn evaluate(pair: Pair<Rule>, data: &HashMap<String, Value>) -> Result<Primative, Error> {
    let field_id = pair
        .into_inner()
        .next()
        .with_context(|| format_err!("Unable to parse"))?
        .as_str();

    let field = data.get(field_id).ok_or_else(|| FieldNotFound {
        field_id: String::from(field_id),
    })?;

    Primative::try_from(field)
}
