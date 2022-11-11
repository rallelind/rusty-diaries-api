use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Diary {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub date: Option<DateTime>,
    pub title: String,
    pub description: String,
}