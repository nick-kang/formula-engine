use crate::parser::Rule;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unsupported data type: `{0}`")]
    UnsupportedDataType(String),

    #[error("Invalid number: `{0}`")]
    InvalidNumber(String),

    #[error("Invalid boolean: `{0}`")]
    InvalidBoolean(String),

    #[error("Field {field_id:?} not found")]
    FieldNotFound { field_id: String },

    #[error("{function_name:?}() requires at least 1 input")]
    MissingInputsVariadic { function_name: String },

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error(transparent)]
    Parse(#[from] pest::error::Error<Rule>),
}
