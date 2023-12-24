use mongodb::bson::{DateTime, serde_helpers::hex_string_as_object_id};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Visit {
    #[serde(with = "hex_string_as_object_id")]
    pub _id: String,
    pub arrival: DateTime,
    pub departure: DateTime,
    pub posted_on: DateTime,
    pub creator_id: String,
    pub num_staying: isize,
}