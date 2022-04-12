use crate::error;
use crate::error::Error::{InvalidNumber, UnsupportedDataType};
use ::std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]

pub enum Primative {
    #[serde(rename = "string")]
    String(String),

    #[serde(rename = "float_64")]
    Float64(f64),

    #[serde(rename = "boolean")]
    Boolean(bool),

    #[serde(rename = "null")]
    Null,
}

impl TryFrom<&Value> for Primative {
    type Error = error::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(_) => Err(UnsupportedDataType(String::from("array"))),
            Value::Bool(bool) => Ok(Primative::Boolean(*bool)),
            Value::Null => Ok(Primative::Null),
            Value::Number(num) => match num.as_f64() {
                Some(x) => Ok(Primative::Float64(x)),
                None => Err(InvalidNumber(num.to_string())),
            },
            Value::String(s) => Ok(Primative::String(s.to_string())),
            Value::Object(_) => Err(UnsupportedDataType(String::from("object"))),
        }
    }
}

impl PartialEq for Primative {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Primative::Boolean(bool1) => match other {
                Primative::Boolean(bool2) => bool1 == bool2,
                _ => false,
            },
            Primative::Null => matches!(other, Primative::Null),
            Primative::Float64(num1) => match other {
                Primative::Float64(num2) => num1 == num2,
                _ => false,
            },
            Primative::String(string1) => match other {
                Primative::String(string2) => string1 == string2,
                _ => false,
            },
        }
    }
}
