use std::env;

pub fn load_config() {
    match env::var("NODE_ENV") {
        Ok(val) => {
            if val != "production" {
                println!("NODE_ENV is {}. Loading .env file.", val);
                load_env_file()
            }
        }
        Err(_) => {
            println!("NODE_ENV not set. Loading .env file.");
            load_env_file();
        }
    }
}

fn load_env_file() {
    let env_path = env::current_dir()
        .map(|a| a.join("packages/leaf-public-api/.env"))
        .unwrap();
    println!("PATH {:#?}", env_path);
    match dotenv::from_path(env_path) {
        Ok(_) => (),
        Err(err) => println!("Failed to load: {:#?}", err),
    }
}

pub fn get_var(key: &str) -> String {
    match env::var(key) {
        Ok(value) => value,
        Err(error) => {
            panic!("{} env variable not found. \n {:#?}", key, error);
        }
    }
}
