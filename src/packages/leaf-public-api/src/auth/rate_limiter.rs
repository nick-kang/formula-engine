use crate::db::types::PublicKeyType;
use chrono::prelude::*;
use deadpool_redis::{redis::cmd, Connection};
use log::error;

// Implementation from https://redis.io/commands/INCR

#[derive(Debug)]
pub struct ShouldAllowResponse {
    pub should_allow: bool,
    key: String,
}

const TEST_EXPIRATION: i32 = 2592000; // 30 days in seconds
const PROD_EXPIRATION: i32 = 1; // 1 second

const TEST_LIMIT: i32 = 300; // 300 calls a month
const PROD_LIMIT: i32 = 10; // 10 calls a month

pub async fn should_allow(
    conn: &mut Connection,
    public_key: &str,
    public_key_type: &PublicKeyType,
) -> ShouldAllowResponse {
    let time_aware_public_key = build_time_aware_public_key(public_key, public_key_type);
    let current = match cmd("LLEN")
        .arg(&time_aware_public_key)
        .query_async::<_, i32>(conn)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            error!("Error fetching LLEN: {}", err);
            return ShouldAllowResponse {
                should_allow: false,
                key: time_aware_public_key,
            };
        }
    };

    match public_key_type {
        PublicKeyType::Test => ShouldAllowResponse {
            should_allow: current < TEST_LIMIT,
            key: time_aware_public_key,
        },
        PublicKeyType::Prod => ShouldAllowResponse {
            should_allow: current < PROD_LIMIT,
            key: time_aware_public_key,
        },
    }
}

pub async fn incr(
    conn: &mut Connection,
    input: &ShouldAllowResponse,
    public_key_type: &PublicKeyType,
) {
    let exists = match cmd("EXISTS")
        .arg(&input.key)
        .query_async::<_, bool>(conn)
        .await
    {
        Ok(exists) => exists,
        Err(error) => {
            error!(
                "Unable to call exists for key ({}) in rate limiter: {}",
                input.key, error
            );
            return;
        }
    };

    if exists {
        if let Err(error) = cmd("RPUSHX")
            .arg(&input.key)
            .arg(&input.key)
            .query_async::<_, ()>(conn)
            .await
        {
            error!(
                "Unable to call RPUSHX for key ({}) in rate limiter: {}",
                input.key, error
            );
        };
    } else {
        // MULTI
        //   RPUSH(key, key)
        //   EXPIRE(key, i32)
        // EXEC
        if let Err(error) = cmd("MULTI").query_async::<_, ()>(conn).await {
            error!(
                "Unable to call MULTI for key ({}) in rate limiter: {}",
                input.key, error
            );
            return;
        }

        if let Err(error) = cmd("RPUSH")
            .arg(&input.key)
            .arg(&input.key)
            .query_async::<_, ()>(conn)
            .await
        {
            error!(
                "Unable to call RPUSH for key ({}) in rate limiter: {}",
                input.key, error
            );
            if let Err(error) = cmd("DISCARD").query_async::<_, ()>(conn).await {
                error!(
                    "Unable to call DISCARD for key ({}) in rate limiter: {}",
                    input.key, error
                );
            };
            return;
        }

        if let Err(error) = cmd("EXPIRE")
            .arg(&input.key)
            .arg(build_expiration(public_key_type))
            .query_async::<_, ()>(conn)
            .await
        {
            error!(
                "Unable to call EXPIRE for key ({}) in rate limiter: {}",
                input.key, error
            );
            if let Err(error) = cmd("DISCARD").query_async::<_, ()>(conn).await {
                error!(
                    "Unable to call DISCARD for key ({}) in rate limiter: {}",
                    input.key, error
                );
            };
            return;
        }

        if let Err(error) = cmd("EXEC").query_async::<_, ()>(conn).await {
            error!(
                "Unable to call EXEC for key ({}) in rate limiter: {}",
                input.key, error
            );
            if let Err(error) = cmd("DISCARD").query_async::<_, ()>(conn).await {
                error!(
                    "Unable to call DISCARD for key ({}) in rate limiter: {}",
                    input.key, error
                );
            };
        };
    }
}

fn build_expiration(public_key_type: &PublicKeyType) -> i32 {
    match public_key_type {
        PublicKeyType::Test => TEST_EXPIRATION,
        PublicKeyType::Prod => PROD_EXPIRATION,
    }
}

fn build_time_aware_public_key(public_key: &str, public_key_type: &PublicKeyType) -> String {
    match public_key_type {
        // Month based rate limiter
        PublicKeyType::Test => {
            let current_date = chrono::Utc::now();
            let month = current_date.month();
            format!("{}-{}-test", public_key, month)
        }

        // Second based rate limiter
        PublicKeyType::Prod => {
            let current_time = Local::now();
            let second = current_time.second();
            format!("{}-{}-prod", public_key, second)
        }
    }
}
