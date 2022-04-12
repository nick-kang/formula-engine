use super::error_body::ErrorBody;

#[derive(Debug)]
pub enum ApiError {
    MissingApiKey,
    InvalidApiKey,
    MalformedAuthHeader,
    Unknown(String),
    TooManyRequests,
}

impl From<ApiError> for ErrorBody {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::MissingApiKey => ErrorBody {
                status: 400,
                message: String::from("Credentials missing in `Authorization` header"),
            },
            ApiError::InvalidApiKey => ErrorBody {
                status: 401,
                message: String::from("Invalid credentials"),
            },
            ApiError::MalformedAuthHeader => ErrorBody {
                status: 400,
                message: String::from("Malformed `Authorization` header"),
            },
            ApiError::Unknown(value) => {
                println!("Internal auth error: {}", value);
                ErrorBody {
                    status: 500,
                    message: String::from("Internal server error"),
                }
            }
            ApiError::TooManyRequests => ErrorBody {
                status: 429,
                message: String::from("Too many requests"),
            },
        }
    }
}
