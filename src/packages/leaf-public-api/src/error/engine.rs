use engine::error::Error as EngineError;

use super::error_body::ErrorBody;

impl From<EngineError> for ErrorBody {
    fn from(error: EngineError) -> Self {
        match error {
            EngineError::UnsupportedDataType(val) => ErrorBody {
                status: 400,
                message: val,
            },
            EngineError::InvalidNumber(val) => ErrorBody {
                status: 400,
                message: val,
            },
            EngineError::InvalidBoolean(val) => ErrorBody {
                status: 400,
                message: val,
            },
            EngineError::FieldNotFound { field_id } => ErrorBody {
                status: 400,
                message: format!("Field `{}` not found in variables", field_id),
            },
            EngineError::MissingInputsVariadic { function_name } => ErrorBody {
                status: 400,
                message: format!("Expected at least 1 input in `{}()`", function_name),
            },
            EngineError::Other(val) => {
                println!("{:#?}", val);
                ErrorBody {
                    status: 500,
                    message: String::from("Internal server error"),
                }
            }
            EngineError::Parse(val) => ErrorBody {
                status: 400,
                message: val.to_string(),
            },
        }
    }
}
