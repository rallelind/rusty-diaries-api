
use mongodb::bson::DateTime;
use mongodb::results::InsertOneResult;
use rocket::local::asynchronous::LocalResponse;
use rocket::serde::json::Json;
use test_modules::initiate_api::initiate;
use test_modules::models::diary_model::Diary;
use rocket::serde::Deserialize;

use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};
use test_modules::models::encrypt::Encryption;

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

    let good_response_body = good_response.into_json::<Diary>().unwrap();

    let Diary { title, description, updated_at, date, .. } = good_response_body;

    assert_eq!(title, Encryption::decrypt("s+ZhSR4QBbxuN7Ys+mcy8w==".to_string()));
    assert_eq!(description, Encryption::decrypt("b28Ru7ppUm0USSqWpOhzl5S7kzh8o7ODXFeuY68K/JQ=".to_string()));
    assert_eq!(date.unwrap().to_string(), "2022-11-16 22:24:55.467 +00:00:00".to_string());
    assert_eq!(updated_at, None);
}

#[test]
fn test_post_diary() {

    let client = initiate_rocket_client();

    let response = client.post("/api/diary")
        .body(r##"{
            "title": "post title",
            "description": "test post description"
        }"##)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);    

}