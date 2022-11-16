use test_modules::api::diary_api;
use test_modules::initiate_api::initiate;

use rocket::local::blocking::Client;
use rocket::http::Status;
use rocket::uri;

fn initiate_rocket_client() -> Client {

    let client = Client::tracked(initiate()).expect("valid rocket instance");

    client
}

#[test]
fn test_get_diary() {
    let client = initiate_rocket_client();
}