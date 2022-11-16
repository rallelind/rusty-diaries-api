use crate::{api, repository};

use rocket::{routes, Build, Rocket};

use api::diary_api::{create_diary, get_diary, update_diary, delete_diary};
use repository::mongodb_repo::MongoRepo;

pub fn initiate() -> Rocket<Build> {
    let db = MongoRepo::init();
        rocket::build()
            .manage(db)
            .mount("/api", routes![create_diary, get_diary, update_diary, delete_diary])
}