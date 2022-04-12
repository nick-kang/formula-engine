use super::error_body::ErrorBody;

#[catch(400)]
pub fn handle_400() -> ErrorBody {
    ErrorBody {
        status: 400,
        message: String::from("Bad request"),
    }
}

#[catch(default)]
pub fn handle_default() -> ErrorBody {
    ErrorBody {
        status: 500,
        message: String::from("Internal server error"),
    }
}
