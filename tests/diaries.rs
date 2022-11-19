
use test_modules::initiate_api::initiate;
use test_modules::models::diary_model::Diary;

use test_modules::models::encrypt::Encryption;

use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};

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

#[test]
fn test_post_diary() {
    

    let client = initiate_rocket_client();

    let response = client.post("/api/diary")
        .header(ContentType::JSON)
        .body(r##"{
            "title": "post title",
            "description": "test post description"
        }"##)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_json::<Diary>();

    assert_eq!(response_body.unwrap().title, "post title");
}