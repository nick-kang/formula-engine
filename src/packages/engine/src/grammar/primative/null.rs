use crate::error::Error;
use crate::parser::Rule;
use crate::primative::Primative;
use pest::iterators::Pair;

pub fn evaluate(_: Pair<Rule>) -> Result<Primative, Error> {
    Ok(Primative::Null)
}
