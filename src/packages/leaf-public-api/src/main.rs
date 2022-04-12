#[macro_use]
extern crate rocket;
extern crate dotenv;

use error::catcher;
use rocket_sync_db_pools::{database, postgres};

mod auth;
mod calculation;
mod config;
mod db;
mod error;

#[database("pg")]
struct PgDatabase(postgres::Client);

#[launch]
fn rocket() -> _ {
    config::load_config();
    env_logger::init();
    rocket::custom(db::postgres::get_config())
        .manage(db::redis::create_pool())
        .attach(PgDatabase::fairing())
        .register("/", catchers![catcher::handle_400, catcher::handle_default])
        .mount("/", routes![calculation::handler])
}
