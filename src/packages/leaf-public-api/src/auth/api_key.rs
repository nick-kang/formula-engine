use super::{rate_limiter, verify};
use crate::{db, PgDatabase};
use log::error;
use std::str::FromStr;

use crate::error::api::ApiError;
use http_auth_basic::Credentials;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;

#[derive(Debug)]
pub struct ApiKey {
    pub public_key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get PG and Redis from guards
        let db_outcome = req.guard::<PgDatabase>().await;
        let redis_outcome = req.guard::<&State<deadpool_redis::Pool>>().await;

        if !db_outcome.is_success() {
            return Outcome::Failure((
                Status::InternalServerError,
                ApiError::Unknown(String::from("Unable to access postgres")),
            ));
        }

        if !redis_outcome.is_success() {
            return Outcome::Failure((
                Status::InternalServerError,
                ApiError::Unknown(String::from("Unable to access redis")),
            ));
        }

        // Extract Postgres connection
        let pg = db_outcome.unwrap();

        // Extract Redis connection
        let redis_state = redis_outcome.unwrap();
        let mut redis_connection = match redis_state.get().await {
            Ok(connection) => connection,
            Err(error) => {
                error!("Unable to get Redis pool: {}", error);
                return Outcome::Failure((
                    Status::InternalServerError,
                    ApiError::Unknown(String::from("Unable to get Redis pool")),
                ));
            }
        };

        // Decode credentials from Authorization header
        let credentials = match req.headers().get_one("Authorization") {
            Some(auth_header) => match Credentials::from_str(auth_header) {
                Ok(credentials) => credentials,
                Err(_) => {
                    return Outcome::Failure((Status::BadRequest, ApiError::MalformedAuthHeader))
                }
            },
            None => return Outcome::Failure((Status::BadRequest, ApiError::MissingApiKey)),
        };

        let public_key = credentials.user_id;
        let private_key = credentials.password;

        // Get auth context from database
        let response = match pg
            .run(move |client| db::query::get_private_key_hash(client, &public_key))
            .await
        {
            Err(err) => return Outcome::Failure((Status::BadRequest, err)),
            Ok(response) => response,
        };

        // Check private key is valid
        let is_valid_credentials = verify::sha512(&private_key, &response.private_key_hash);

        if !is_valid_credentials {
            return Outcome::Failure((Status::Unauthorized, ApiError::InvalidApiKey));
        }

        // Check if we should rate limit
        let should_allow_response = rate_limiter::should_allow(
            &mut redis_connection,
            &response.public_key,
            &response.key_type,
        )
        .await;

        if !should_allow_response.should_allow {
            return Outcome::Failure((Status::TooManyRequests, ApiError::TooManyRequests));
        }

        // Increment rate limiter
        rate_limiter::incr(
            &mut redis_connection,
            &should_allow_response,
            &response.key_type,
        )
        .await;

        Outcome::Success(ApiKey {
            public_key: response.public_key,
        })
    }
}
