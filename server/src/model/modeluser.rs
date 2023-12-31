use async_graphql::Enum;
use mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Default)]
pub enum Role {
    Owner,
    #[default]
    Member,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    #[serde(with = "hex_string_as_object_id")]
    pub _id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub role: Role,
    pub is_active: bool,
    pub sub: String,
}
