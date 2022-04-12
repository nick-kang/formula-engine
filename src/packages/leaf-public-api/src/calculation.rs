use engine::primative::Primative;
use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    auth::api_key::ApiKey,
    error::{api::ApiError, error_body::ErrorBody},
};

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    calculation: String,

    #[serde(default)]
    variables: HashMap<String, Value>,
}

#[derive(Serialize, Debug)]
pub struct ResponseBody {
    answer: Primative,
}

#[post("/calculation", data = "<body>")]
pub fn handler(
    public_key_result: Result<ApiKey, ApiError>,
    body: Json<RequestBody>,
) -> Result<Json<ResponseBody>, ErrorBody> {
    // We need to unwrap public_key_result first because if it fails, that means
    // the user is not authorized and the handler will short circuit to throwing
    // an error.
    let public_key = public_key_result?.public_key;

    let answer = engine::eval(&body.calculation, &body.variables);

    match answer {
        Ok(answer) => Ok(Json(ResponseBody { answer })),
        Err(error) => Err(ErrorBody::from(error)),
    }
}
