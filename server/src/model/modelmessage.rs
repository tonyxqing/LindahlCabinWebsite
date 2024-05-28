use async_graphql::Enum;
use mongodb::bson::{oid::ObjectId, serde_helpers::hex_string_as_object_id, DateTime};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    #[serde(with = "hex_string_as_object_id")]
    pub _id: String,
    pub creator_id: String,
    pub content: String,
    pub posted_on: DateTime,
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            _id: ObjectId::new().to_string(),
            creator_id: "".to_string(),
            content: "".to_string(),
            posted_on: DateTime::now(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(with = "hex_string_as_object_id")]
    pub _id: String,
    pub creator_id: String,
    pub content: String,
    pub comments: Vec<Comment>,
    pub posted_on: DateTime,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            _id: ObjectId::new().to_string(),
            creator_id: "".to_string(),
            content: "".to_string(),
            comments: Vec::<Comment>::new(),
            posted_on: DateTime::now(),
        }
    }
}
