mod api;
mod models;
mod repository;

#[macro_use] 
extern crate rocket;

use api::diary_api::{create_diary, get_diary, update_diary, delete_diary};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/api", routes![create_diary, get_diary, update_diary, delete_diary])
}