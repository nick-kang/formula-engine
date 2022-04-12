use crate::parser::Rule;
use crate::primative::Primative;
use crate::{error::Error, evaluate};
use pest::iterators::Pairs;
use serde_json::Value;
use std::collections::HashMap;

pub fn and_or_util(
    pairs: Pairs<Rule>,
    data: &HashMap<String, Value>,
    function_name: String,
) -> Result<Vec<bool>, Error> {
    let mut booleans = Vec::new();

    for pair in pairs {
        let evaluated_pair = evaluate::evaluate_raw(pair, data)?;

        match evaluated_pair {
            Primative::Boolean(val) => booleans.push(val),
            Primative::String(val) => return Err(Error::InvalidBoolean(val)),
            Primative::Float64(val) => return Err(Error::InvalidBoolean(val.to_string())),
            Primative::Null => return Err(Error::InvalidBoolean(String::from("null"))),
        }
    }

    if booleans.is_empty() {
        return Err(Error::MissingInputsVariadic { function_name });
    }

    Ok(booleans)
}
