use async_graphql::Enum;
use mongodb::bson::{oid::ObjectId, serde_helpers::hex_string_as_object_id, DateTime};
use serde::{Deserialize, Serialize};
#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reaction {
    ThumbsUp,
    Heart,
    Smile,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Comment {
    pub _id: String,
    pub creator_id: String,
    pub content: String,
    pub reactions: Vec<Reaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(with = "hex_string_as_object_id")]
    pub _id: String,
    pub creator_id: String,
    pub content: String,
    pub comments: Vec<Comment>,
    pub reactions: Vec<Reaction>,
    pub seen_by: Vec<String>,
    pub posted_on: DateTime,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            _id: ObjectId::new().to_string(),
            creator_id: "".to_string(),
            content: "".to_string(),
            comments: Vec::<Comment>::new(),
            reactions: Vec::<Reaction>::new(),
            seen_by: Vec::<String>::new(),
            posted_on: DateTime::now(),
        }
    }
}
