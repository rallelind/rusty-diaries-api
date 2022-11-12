use std::{env};

pub struct Config {
    pub mongo_connection_string: String,
}

impl Config {
    pub fn read_config() -> Config {
        dotenv::dotenv().expect("Failed to read .env file");

        let mongo_connection_string = env::var("MONGO_DB_CONNECTION_STRING").expect("Could not read mongo connection key");

        Config { mongo_connection_string }
    }
}