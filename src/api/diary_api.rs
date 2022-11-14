use crate::{models::diary_model::Diary, repository::mongodb_repo::MongoRepo, models::encrypt::Encryption};
use mongodb::{bson::DateTime, results::InsertOneResult, bson::oid::ObjectId};
use rocket::{http::Status, serde::json::Json, State};


#[post("/diary", data = "<new_diary>")]
pub fn create_diary(
    db: &State<MongoRepo>,
    new_diary: Json<Diary>,
) -> Result<Json<InsertOneResult>, Status> {

    let encrypted_description = Encryption::encrypt(new_diary.description.to_owned().to_string());

    let data = Diary {
        id: None,
        description: encrypted_description,
        date: Some(DateTime::now().to_owned()),
        title: new_diary.title.to_owned(),
        updated_at: None,
    };
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
        Ok(mut diary) => {
            let decrypted_description = Encryption::decrypt(diary.description.to_string());

            diary.description = decrypted_description;
            Ok(Json(diary))
        },
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

    let encrypted_description = Encryption::encrypt(new_diary.description.to_owned().to_string());

    println!("{:?}", encrypted_description);

    let data = Diary {
        id: Some(ObjectId::parse_str(&id).unwrap().to_owned()),
        title: new_diary.title.to_owned(),
        description: encrypted_description,
        date: None,
        updated_at: Some(DateTime::now().to_owned()),
    };

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

#[delete("/diary/<path>")]
pub fn delete_diary(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let result = db.delete_diary(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Diary was successfully deleted"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError)
    }
}