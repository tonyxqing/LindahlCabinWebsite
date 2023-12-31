use crate::db;
use crate::model::{Comment, Message, Reaction};
use async_graphql::futures_util::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct CommentEntry {
    pub _id: ObjectId,
    pub creator_id: String,
    pub content: String,
    pub reactions: Vec<Reaction>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageEntry {
    pub creator_id: String,
    pub content: String,
    pub comments: Vec<Comment>,
    pub reactions: Vec<Reaction>,
    pub seen_by: Vec<String>,
    pub posted_on: DateTime,
}

impl Default for MessageEntry {
    fn default() -> Self {
        Self {
            creator_id: "".to_string(),
            content: "".to_string(),
            comments: Vec::<Comment>::new(),
            reactions: Vec::<Reaction>::new(),
            seen_by: Vec::<String>::new(),
            posted_on: DateTime::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MessageFilter {
    pub id: Option<ObjectId>,
    pub creator_id: Option<String>,
    pub content: Option<String>,
    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
}

impl MessageFilter {
    fn to_doc(&self) -> Document {
        let mut doc = doc! {};
        let mut predicate = doc! {};
        if let Some(id) = self.id {
            doc.insert("_id", id);
        }

        if let Some(creator_id) = &self.creator_id {
            doc.insert("creator_id", creator_id);
        }

        if let Some(start_time) = &self.start_time {
            predicate.insert("$gte", start_time);
        }

        if let Some(end_time) = &self.end_time {
            predicate.insert("$lt", end_time);
        }
        if self.end_time.is_some() || self.start_time.is_some() {
            doc.insert("posted_on", predicate);
        }

        doc.to_owned()
    }

    fn to_update_doc(&self) -> Document {
        let mut doc = doc! {};
        let mut set = doc! {};
        if let Some(content) = &self.content {
            set.insert("content", content);
        }

        doc.insert("$set", set);
        doc
    }
}

pub async fn add_message(db: &db::DB, message: &MessageEntry) -> Result<Message, String> {
    let collection = db.client.collection::<MessageEntry>("Messages");
    let result = collection
        .insert_one(message, None)
        .await
        .expect("Error adding message to the db");
    let message = Message {
        _id: result.inserted_id.to_string(),
        creator_id: message.creator_id.clone(),
        comments: message.comments.clone(),
        content: message.content.clone(),
        posted_on: message.posted_on.clone(),
        reactions: message.reactions.clone(),
        seen_by: message.seen_by.clone(),
    };

    Ok(message)
}

pub async fn get_messages(db: &db::DB, filter: MessageFilter) -> Result<Vec<Message>, String> {
    let collection = db.client.collection::<Message>("Messages");
    let filter_doc = filter.to_doc();
    let mut cursor = collection
        .find(filter_doc, None)
        .await
        .expect("Error adding message to db");
    let mut messages = Vec::<Message>::new();
    while let Some(message) = cursor.try_next().await.expect("Error parsing") {
        messages.push(message);
    }
    Ok(messages)
}

pub async fn update_message(db: &db::DB, filter: MessageFilter) -> Result<Message, String> {
    let collection = db.client.collection::<Message>("Messages");
    let filter_doc = filter.to_doc();
    let update_doc = filter.to_update_doc();

    let result = collection
        .find_one_and_update(
            filter_doc,
            update_doc,
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        )
        .await;
    Ok(result.expect("Failed to update message in db").unwrap())
}

pub async fn remove_message(db: &db::DB, message_id: ObjectId) -> Result<Message, String> {
    let collection = db.client.collection::<Message>("Messages");
    let result = collection.find_one_and_delete(doc! {"_id": message_id}, None).await.expect("Failed to remove message from db");
    result.map_or(Err("Failed".to_string()), |message| Ok(message))
}

pub async fn add_comment(db: &db::DB, message_id: ObjectId, comment_content: String) -> Result<Message, String> {
    let collection = db.client.collection::<Message>("Messages");
    let comment = CommentEntry {
        content: comment_content,
        ..Default::default()
    };
    let result = collection.find_one_and_update(doc! {"_id": message_id}, doc!{"$push": doc! {"comments": mongodb::bson::to_bson(&comment).expect("Failed to convert comment to bson") }}, None).await.expect("Failed to add comment to message in db");
    result.map_or(Err("Could not update with comment in db".to_string()), |message| Ok(message.clone()))
}

pub async fn remove_comment(db: &db::DB, message_id: ObjectId, comment_id: ObjectId) -> Result<Message, String> {
    let collection = db.client.collection::<Message>("Messages");
    let result = collection.find_one_and_update(doc! {"_id": message_id}, doc!{"$pull": doc! {"comments": doc!{"_id": comment_id}}}, None).await.expect("Failed to remove comment on message in db");
    result.map_or(Err("Could not update with comment in db".to_string()), |message| Ok(message.clone()))
}