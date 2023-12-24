use crate::db::DB;
use crate::model::{self, Role, User};
use async_graphql::futures_util::TryStreamExt;
use mongodb::bson::Document;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    pub email: String,
    pub phone: String,
    pub name: String,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserFilter {
    pub id: Option<ObjectId>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub name: Option<String>,
    pub role: Option<Role>,
}

impl UserFilter {
    fn to_doc(&self) -> Document {
        let mut doc = doc! {};

        if let Some(id) = self.id {
            doc.insert("_id", id);
        }
        if let Some(name) = &self.name {
            doc.insert("name", name);
        }
        if let Some(email) = &self.email {
            doc.insert("email", email);
        }
        if let Some(phone) = &self.phone {
            doc.insert("phone", phone);
        }
        if let Some(role) = self.role {
            doc.insert(
                "role",
                mongodb::bson::to_bson::<Role>(&role).expect("failed to convert role to bson"),
            );
        }
        doc.to_owned()
    }
}

pub async fn add_user(db: &DB, user: &UserEntry) -> Result<User, String> {
    let collection = db.client.collection::<UserEntry>("Users");
    let result = collection
        .insert_one(user, None)
        .await
        .expect("Error adding user to database");

    Ok(User {
        _id: result.inserted_id.to_string(),
        name: user.name.clone(),
        email: user.email.clone(),
        phone: user.phone.clone(),
        role: user.role,
    })
}

pub async fn remove_user(db: &DB, filter: UserFilter) -> Result<User, String> {
    let collection = db.client.collection::<User>("Users");

    let filter_doc = filter.to_doc();

    let result = collection
        .find_one_and_delete(filter_doc, None)
        .await
        .expect("Error removing user to database")
        .unwrap();
    Ok(result)
}

pub async fn get_users(db: &DB, filter: UserFilter) -> Result<Vec<User>, String> {
    let collection = db.client.collection::<User>("Users");

    let filter_doc = filter.to_doc();

    let mut cursor = collection
        .find(filter_doc, None)
        .await
        .expect("Failed getting cursor for collection");
    let mut users = Vec::<User>::new();
    while let Some(user) = cursor.try_next().await.expect("Users") {
        println!("User: {:?}", &user);
        users.push(user);
    }
    Ok(users)
}
