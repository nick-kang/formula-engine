use crate::error::Error;
use crate::grammar::conditional::and_or_util::and_or_util;
use crate::parser::Rule;
use crate::primative::Primative;
use pest::iterators::Pairs;
use serde_json::Value;
use std::collections::HashMap;

pub fn evaluate(pairs: Pairs<Rule>, data: &HashMap<String, Value>) -> Result<Primative, Error> {
    let booleans = and_or_util(pairs, data, String::from("and"))?;

    Ok(evaluate_raw(booleans))
}

fn evaluate_raw(booleans: Vec<bool>) -> Primative {
    Primative::Boolean(!booleans.contains(&false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Primative;

    #[test]
    fn it_returns_true() {
        assert_eq!(Primative::Boolean(true), evaluate_raw(vec![true]));
        assert_eq!(Primative::Boolean(true), evaluate_raw(vec![true, true]));
    }

    #[test]
    fn it_returns_false() {
        assert_eq!(Primative::Boolean(false), evaluate_raw(vec![false]));
        assert_eq!(Primative::Boolean(false), evaluate_raw(vec![false, true]));
    }
}
