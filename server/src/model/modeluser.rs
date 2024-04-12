use async_graphql::Enum;
use mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Default)]
pub enum Role {
    Admin,
    Owner,
    #[default]
    Member,
}

impl std::string::ToString for Role {
    fn to_string(&self) -> String {
        match self {
           Role::Admin => String::from("Admin"),
           Role::Owner => String::from("Owner"),
           Role::Member => String::from("Member")
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    #[serde(with = "hex_string_as_object_id")]
    pub _id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub profile_pic: Option<String>,
    pub role: Role,
    pub sub: String,
    pub access_code: String,
}
