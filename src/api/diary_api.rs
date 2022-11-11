use crate::{models::diary_model::Diary, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use mongodb::bson::DateTime;
use rocket::{http::Status, serde::json::Json, State};

#[post("/diary", data = "<new_diary>")]
pub fn create_diary(
    db: &State<MongoRepo>,
    new_diary: Json<Diary>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Diary {
        id: None,
        description: new_diary.description.to_owned(),
        date: Some(DateTime::now().to_owned()),
        title: new_diary.title.to_owned(),
        updated_at: None,
    };
    print!("{:?}", data);
    let user_detail = db.create_diary(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

