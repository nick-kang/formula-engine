use crate::config;
use rocket::figment::{
    util::map,
    value::{Map, Value},
    Figment,
};

pub fn get_config() -> Figment {
    let user = config::get_var("DB_USER");
    let password = config::get_var("DB_PASSWORD");
    let host = config::get_var("DB_HOST");
    let port = config::get_var("DB_PORT");
    let db = config::get_var("DB");

    let connection_string = format!(
        "postgres://{user}:{password}@{host}:{port}/{db}",
        user = user,
        password = password,
        host = host,
        port = port,
        db = db
    );

    let pg: Map<_, Value> = map! {
      "url" => connection_string.into()
    };

    rocket::Config::figment().merge(("databases", map!["pg" => pg]))
}
