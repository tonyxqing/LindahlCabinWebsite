use crate::auth;
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
    pub is_active: bool,
    pub sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserUpdate {
    pub phone: Option<String>,
    pub name: Option<String>,
    pub role: Option<Role>,
    pub is_active: Option<bool>,
}

impl UserUpdate {
    fn to_update_doc(&self) -> Document {
        let mut set = doc! {};

        if let Some(phone) = &self.phone {
            set.insert("phone", phone);
        }
        if let Some(name) = &self.name {
            set.insert("name", name);
        }
        if let Some(role) = self.role {
            set.insert("role", mongodb::bson::to_bson(&role).expect("Could not convert to bson"));
        }
        if let Some(is_active) = self.is_active {
            set.insert("is_active", is_active);
        }

        doc! {"$set": set}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserFilter {
    pub id: Option<ObjectId>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub name: Option<String>,
    pub role: Option<Role>,
    pub sub: Option<String>
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
        if let Some(sub) = &self.sub {
            doc.insert("sub", sub);
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
        is_active: user.is_active.clone(),
        sub: user.sub.clone(),
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
        .find( filter_doc, None)
        .await
        .expect("Failed getting cursor for collection");
    let mut users = Vec::<User>::new();
    while let Some(user) = cursor.try_next().await.expect("Users") {
        println!("User: {:?}", &user);
        users.push(user);
    }
    Ok(users)
}

pub async fn update_user(db: &DB, filter: UserFilter, update: UserUpdate) -> Result<User, String> {
    let filter_doc = filter.to_doc();
    let update_doc = update.to_update_doc();
    let collection = db.client.collection::<User>("Users");
    let result = collection.find_one_and_update(filter_doc, update_doc, None).await.map_err(|e| format!("Unable to update user in db {}", e))?;

    result.ok_or("No user found to update".to_string())

}