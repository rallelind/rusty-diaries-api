use test_modules::initiate_api::initiate;

use rocket::local::blocking::Client;
use rocket::http::Status;

fn initiate_rocket_client() -> Client {

    let client = Client::tracked(initiate()).expect("valid rocket instance");

    client
}

#[test]
fn test_get_diary() {
    let client = initiate_rocket_client();
    let good_response = client.get("/api/diary/63756337cdb871616aa71fc0").dispatch();
    let bad_response = client.get("/api/diary/hejsa").dispatch();
    let not_found_document = client.get("/api/diary/111111111111111111111111").dispatch();

    assert_eq!(good_response.status(), Status::Ok);
    assert_eq!(bad_response.status(), Status::BadRequest);
    assert_eq!(not_found_document.status(), Status::new(404));
}