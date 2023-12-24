use crate::db;
use crate::model::{Comment, Message, Reaction};
use async_graphql::futures_util::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use serde::{Deserialize, Serialize};

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
    pub comment_id: Option<String>,
    pub content: Option<String>,
    pub start_time: Option<DateTime>,
    pub end_time: Option<DateTime>,
    pub reaction: Option<Reaction>,
    pub comment: Option<String>,
}

impl MessageFilter {
    fn to_doc(&self) -> Document {
        let mut doc = doc! {};
        let mut predicate = doc! {};
        if let Some(id) = self.id {
            doc.insert("_id", id);
        }

        if let Some(creator_id) = &self.creator_id {
            if self.comment.is_none() && self.reaction.is_none() {
                doc.insert("creator_id", creator_id);
            }
        }
        if let Some(comment_id) = &self.comment_id {
            doc.insert("comments", doc! { "_id": comment_id});
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
        let mut push = doc! {};
        if let Some(content) = &self.content {
            set.insert("content", content);
        }
        if let Some(c) = &self.comment {
            let comment = Comment {
                _id: ObjectId::new().to_string(),
                creator_id: self.creator_id.clone().unwrap(),
                content: c.to_string(),
                reactions: vec![],
            };
            push.insert(
                "comments",
                mongodb::bson::to_bson(&comment).expect("Error parsing to bson"),
            );
        }
        if self.comment_id.is_none() {
            if let Some(reaction) = self.reaction {
                push.insert(
                    "reactions",
                    mongodb::bson::to_bson(&reaction).expect("Error parsing to bson"),
                );
            }
        } else {
            if let Some(reaction) = self.reaction {
                set.insert("comments", doc! {"$push": doc! { "reactions": mongodb::bson::to_bson(&reaction).expect("Error parsing to bson")}});
            }
        }
        doc.insert("$set", set);
        doc.insert("$push", push);
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
