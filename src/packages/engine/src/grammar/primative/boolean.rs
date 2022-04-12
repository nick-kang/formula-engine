use crate::error::Error;
use crate::error::Error::InvalidBoolean;
use crate::parser::Rule;
use crate::primative::Primative;
use pest::iterators::Pair;

pub fn evaluate(pair: Pair<Rule>) -> Result<Primative, Error> {
    match pair.as_str().parse::<bool>() {
        Ok(boolean) => Ok(Primative::Boolean(boolean)),
        Err(_) => Err(InvalidBoolean(pair.to_string())),
    }
}
