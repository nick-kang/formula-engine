use crate::error::Error;
use crate::error::Error::InvalidNumber;
use crate::parser::Rule;
use crate::primative::Primative;
use pest::iterators::Pair;

pub fn evaluate(pair: Pair<Rule>) -> Result<Primative, Error> {
    match pair.as_str().parse::<f64>() {
        Ok(num) => Ok(Primative::Float64(num)),
        Err(_) => Err(InvalidNumber(pair.to_string())),
    }
}
