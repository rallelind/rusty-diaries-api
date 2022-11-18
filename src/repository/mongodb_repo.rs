
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use crate::models::diary_model::Diary;
use crate::models::config::Config;

pub struct MongoRepo {
    diary_collection: Collection<Diary>,
}

impl MongoRepo {
    pub fn init() -> Self {
        let config = Config::read_config();

        let client = Client::with_uri_str(config.mongo_connection_string).unwrap();
        let db = client.database("rustDB");
        let diary_collection: Collection<Diary> = db.collection("Diary");
        MongoRepo { diary_collection }
    }

    pub fn create_diary(&self, new_diary: Diary) -> Result<InsertOneResult, Error> {

        let new_doc = Diary {
            id: None,
            description: new_diary.description,
            title: new_diary.title,
            date: new_diary.date,
            updated_at: None,
        };

        let diary = self
            .diary_collection
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating diary");

        Ok(diary)
    }

    pub fn get_diary(&self, id: &String) -> Result<Diary, Error> {
        let obj_id = ObjectId::parse_str(id).expect("Wrong id provided");

        let filter = doc! {"_id": obj_id};
        let diary_detail = self
            .diary_collection
            .find_one(filter, None)
            .ok()
            .expect("Error getting the diary details");
        
        if !diary_detail.is_none() {
            Ok(diary_detail.unwrap())
        } else {
            Err(Error::DeserializationError { message: ("not found".to_string()) })
        }
    }

    pub fn update_diary(&self, id: &String, new_diary: Diary) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_diary.id,
                    "date": new_diary.date,
                    "title": new_diary.title,
                    "description": new_diary.description,
                    "updated_at": new_diary.updated_at
                },
        };
        let updated_doc = self
            .diary_collection
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating diary");
        Ok(updated_doc)
    }

    pub fn delete_diary(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let diary_detail = self 
            .diary_collection
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting the diary");
        
        Ok(diary_detail)
    }

}