use std::env;

use test_modules::models::config::{Config};

#[test]
fn test_read_mongo_env() {
    env::set_var("MONGO_DB_CONNECTION_STRING", "mongouri");

    let mongo_uri = Config::read_config();

    assert_eq!(mongo_uri.mongo_connection_string, "mongouri");
}