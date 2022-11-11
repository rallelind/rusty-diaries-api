
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, DateTime},
    results::{ InsertOneResult},
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
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let diary_detail = self
            .diary_collection
            .find_one(filter, None)
            .ok()
            .expect("Error getting the diary details");
        
        Ok(diary_detail.unwrap())
    }

}