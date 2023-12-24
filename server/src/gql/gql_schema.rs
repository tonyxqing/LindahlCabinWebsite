use crate::model::{self, Reaction};
use async_graphql::Object;
use mongodb::bson::Uuid;

pub struct UserData(pub model::User);

#[Object]
impl UserData {
    async fn id(&self) -> String {
        self.0._id.clone()
    }
    async fn name(&self) -> &str {
        &self.0.name
    }
    async fn email(&self) -> &str {
        &self.0.email
    }
    async fn phone(&self) -> &str {
        &self.0.phone
    }
    async fn role(&self) -> &model::Role {
        &self.0.role
    }
}

pub struct VisitData(pub model::Visit);
#[Object]
impl VisitData {
    async fn id(&self) -> String {
        self.0._id.clone()
    }
    async fn creator_id(&self) -> &str {
        &self.0.creator_id
    }
    async fn arrival(&self) -> String {
        self.0.arrival.to_string()
    }
    async fn departure(&self) -> String {
        self.0.departure.to_string()
    }
    async fn posted_on(&self) -> String {
        self.0.posted_on.to_string()
    }
    async fn num_staying(&self) -> isize {
        self.0.num_staying
    }
}

pub struct CommentData(pub model::Comment);
#[Object]
impl CommentData {
    async fn id(&self) -> &str {
        &self.0._id
    }
    async fn creator_id(&self) -> &str {
        &self.0.creator_id
    }
    async fn content(&self) -> &str {
        &self.0.content
    }
    async fn reactions(&self) -> Vec<Reaction> {
        self.0.reactions.clone()
    }
}
pub struct MessageData(pub model::Message);
#[Object]
impl MessageData {
    async fn id(&self) -> &str {
        &self.0._id
    }
    async fn creator_id(&self) -> &str {
        &self.0.creator_id
    }
    async fn comments(&self) -> Vec<CommentData> {
        self.0
            .comments
            .iter()
            .map(|d| CommentData(d.clone()))
            .collect()
    }
    async fn content(&self) -> &str {
        &self.0.content
    }
    async fn reactions(&self) -> Vec<Reaction> {
        self.0.reactions.clone()
    }
    async fn seen_by(&self) -> Vec<String> {
        self.0.seen_by.clone()
    }
    async fn posted_on(&self) -> String {
        self.0.posted_on.to_string()
    }
}
