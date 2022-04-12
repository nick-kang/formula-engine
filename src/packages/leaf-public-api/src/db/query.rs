use log::{error, trace};
use postgres::Client;
use uuid::Uuid;

use crate::error::api::ApiError;

use super::types::PublicKeyType;

#[derive(Debug)]
pub struct GetPrivateKeyHashResponse {
    pub private_key_hash: String,
    pub key_type: PublicKeyType,
    pub public_key: String,
}

pub fn get_private_key_hash(
    client: &mut Client,
    public_key: &str,
) -> Result<GetPrivateKeyHashResponse, ApiError> {
    let rows = match Uuid::parse_str(public_key) {
        Ok(parsed_public_key) => match client.query(
            "
            SELECT g.private_key_hash
                 , g.type
              FROM app_hidden_fn.get_auth_context (
                public_key => $1
              ) g;
          ",
            &[&parsed_public_key],
        ) {
            Ok(rows) => rows,
            Err(error) => {
                error!("Problem fetching api key: {:#?}", error);
                return Err(ApiError::Unknown(String::from(
                    "Unable to fetch private key",
                )));
            }
        },
        Err(_) => return Err(ApiError::InvalidApiKey),
    };

    trace!("Private key hash response: {:#?}", rows);

    match rows.first() {
        None => Err(ApiError::InvalidApiKey),
        Some(row) => {
            // Should be safe because PG will throw error if you select an incorrect column
            let private_key_hash: &str = row.get(0);
            let key_type: PublicKeyType = row.get(1);
            Ok(GetPrivateKeyHashResponse {
                private_key_hash: private_key_hash.to_string(),
                key_type,
                public_key: public_key.to_string(),
            })
        }
    }
}
