use crate::auth;
use crate::db::DB;
use crate::model::{self, Message, Role, User, Visit};
use async_graphql::futures_util::TryStreamExt;
use mongodb::bson::Document;
use mongodb::bson::{doc, oid::ObjectId};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    pub email: String,
    pub phone: String,
    pub name: String,
    pub profile_pic: Option<String>,
    pub role: Role,
    pub sub: String,
    pub access_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserUpdate {
    pub sub: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub profile_pic: Option<String>,
    pub role: Option<Role>,
    pub access_code: Option<String>,
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
        if let Some(email) = &self.email {
            set.insert("email", email);
        }
        if let Some(sub) = &self.sub {
            set.insert("sub", sub);
        }
        if let Some(role) = self.role {
            set.insert(
                "role",
                mongodb::bson::to_bson(&role).expect("Could not convert to bson"),
            );
        }
        if let Some(profile_pic) = self.profile_pic.clone() {
            set.insert("profile_pic", profile_pic);
        }
        if let Some(access_code) = self.access_code.clone() {
            set.insert("access_code", access_code);
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
    pub sub: Option<String>,
    pub access_code: Option<String>,
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
        if let Some(access_code) = self.access_code.clone() {
            doc.insert("access_code", access_code);
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
        profile_pic: None,
        sub: user.sub.clone(),
        role: user.role,
        access_code: user.access_code.clone(),
    })
}

pub async fn remove_user(db: &DB, filter: UserFilter) -> Result<User, String> {
    let users_collection = db.client.collection::<User>("Users");
    let filter_doc = filter.to_doc();

    let users_result = users_collection
        .find_one_and_delete(filter_doc, None)
        .await
        .expect("Error removing user to database")
        .ok_or(format!("No member with id {:?} was found.", filter.id));

    let messages_collection = db.client.collection::<Message>("Messages");
    let creator_id = filter.id.unwrap().to_string();
    println!("creator id{creator_id}");
    let messages_result = messages_collection
        .delete_many(doc! {"creator_id": creator_id.clone()}, None)
        .await
        .expect("Error removing users messages");

    println!("{messages_result:?}");

    let messages_result2 = messages_collection
        .update_many(
            doc! {},
            doc! { "$pull": doc! {"comments": doc!{"creator_id": creator_id}}},
            None,
        )
        .await
        .expect("Error removing users messages");

    println!("{messages_result2:?}");
    // let _ = messages_collection.update_many(query, update, options)
    let visits_collection = db.client.collection::<Visit>("Visits");
    let visits_result = visits_collection
        .delete_many(doc! {"creator_id": filter.id.unwrap().to_string()}, None)
        .await
        .expect("Error removing users visits");
    println!("{visits_result:?}");

    users_result
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
        users.push(user);
    }
    Ok(users)
}

pub async fn update_user(db: &DB, filter: UserFilter, update: UserUpdate) -> Result<User, String> {
    let filter_doc = filter.to_doc();
    let update_doc = update.to_update_doc();
    let collection = db.client.collection::<User>("Users");
    let result = collection
        .find_one_and_update(filter_doc, update_doc, None)
        .await
        .map_err(|e| format!("Unable to update user in db {}", e))?
        .ok_or(format!(
            "No user found to update. Access code {:?} or User ID {:?} not correct.",
            filter.access_code, filter.id
        ))?;
    let update_user = User {
        sub: update.sub.unwrap_or(result.sub),
        email: update.email.unwrap_or(result.email),
        phone: update.phone.unwrap_or(result.phone),
        profile_pic: update.profile_pic.or(result.profile_pic),
        role: update.role.unwrap_or(result.role),
        access_code: update.access_code.unwrap_or(result.access_code),
        name: update.name.unwrap_or(result.name),
        _id: result._id,
    };
    Ok(update_user)
}
