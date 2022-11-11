use crate::{models::diary_model::Diary, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::DateTime, results::InsertOneResult, bson::oid::ObjectId};
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

#[get("/diary/<path>")]
pub fn get_diary(db: &State<MongoRepo>, path: String) -> Result<Json<Diary>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let diary_details = db.get_diary(&id);
    match diary_details {
        Ok(diary) => Ok(Json(diary)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/diary/<path>", data = "<new_diary>")]
pub fn update_diary(
    db: &State<MongoRepo>,
    path: String,
    new_diary: Json<Diary>,
) -> Result<Json<Diary>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let data = Diary {
        id: Some(ObjectId::parse_str(&id).unwrap().to_owned()),
        title: new_diary.title.to_owned(),
        description: new_diary.description.to_owned(),
        date: None,
        updated_at: Some(DateTime::now().to_owned()),
    };

    print!("{:?}", data);
    print!("{:?}", id);


    let update_result = db.update_diary(&id, data);

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_diary_info = db.get_diary(&id);
                return match updated_diary_info {
                    Ok(diary) => Ok(Json(diary)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        } 
        Err(_) => Err(Status::InternalServerError),
    }
}
