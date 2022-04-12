use crate::config;
use deadpool_redis::{Manager, Pool};
use log::info;

pub fn create_pool() -> Pool {
    let host = config::get_var("REDIS_HOST");
    let port = config::get_var("REDIS_PORT");

    let connection_string = format!("redis://{host}:{port}", host = host, port = port,);

    let mgr = match Manager::new(connection_string) {
        Ok(manager) => manager,
        Err(error) => panic!("Unable to create Redis manager: {}", error),
    };

    match Pool::builder(mgr).build() {
        Ok(pool) => {
            info!("Successfully connected to Redis");
            pool
        }
        Err(error) => panic!("Unable to create Redis pool: {}", error),
    }
}
