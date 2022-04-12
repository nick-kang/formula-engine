use crate::error::Error;
use crate::parser::Rule;
use crate::primative::Primative;
use anyhow::{format_err, Context};
use pest::iterators::Pair;

pub fn evaluate(pair: Pair<Rule>) -> Result<Primative, Error> {
    Ok(Primative::String(
        pair.into_inner()
            .next()
            .with_context(|| format_err!("Unable to parse"))?
            .as_span()
            .as_str()
            .to_string(),
    ))
}
