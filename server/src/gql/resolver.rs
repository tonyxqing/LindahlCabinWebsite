use crate::db::{
    self, MessageEntry, MessageFilter, UserEntry, UserFilter, VisitEntry, VisitUpdate, UserUpdate,
};
use crate::model::{Message, User, Visit};
use async_graphql::Context;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use std::sync::Arc;

pub struct Resolver {
    pub db: db::DB,
}

impl Resolver {
    pub async fn new() -> Self {
        Self {
            db: db::DB::new().await,
        }
    }

    pub async fn from_context(ctx: &Context<'_>) -> Arc<Resolver> {
        ctx.data::<Arc<Resolver>>()
            .expect("Could not find resolver in context")
            .clone()
    }
    pub async fn get_messages(&self, filter: MessageFilter) -> Result<Vec<Message>, String> {
        db::get_messages(&self.db, filter).await
    }
    pub async fn add_message(&self, message: &MessageEntry) -> Result<Message, String> {
        db::add_message(&self.db, message).await
    }

    pub async fn get_users(&self, filter: UserFilter) -> Result<Vec<User>, String> {
        db::get_users(&self.db, filter).await
    }

    pub async fn add_user(&self, user: &UserEntry) -> Result<User, String> {
        db::add_user(&self.db, user).await
    }

    pub async fn update_user(&self, filter: UserFilter, update: UserUpdate) -> Result<User, String> {
        db::update_user(&self.db, filter, update).await
    }

    pub async fn remove_user(&self, filter: UserFilter) -> Result<User, String> {
        db::remove_user(&self.db, filter).await
    }

    pub async fn update_message(&self, filter: MessageFilter) -> Result<Message, String> {
        db::update_message(&self.db, filter).await
    }

    pub async fn activate_account(&self, id: ObjectId) -> Result<User, String> {
        db::update_user(&self.db, UserFilter { id: Some(id), ..Default::default() }, UserUpdate { ..Default::default()}).await
    }
    pub async fn get_visits(
        &self,
        start: Option<DateTime>,
        end: Option<DateTime>,
    ) -> Result<Vec<Visit>, String> {
        db::get_visits(&self.db, start, end).await
    }

    pub async fn add_visit(&self, visit: &VisitEntry) -> Result<Visit, String> {
        db::add_visit(&self.db, visit).await
    }

    pub async fn update_visit(
        &self,
        visit_id: ObjectId,
        visit: VisitUpdate,
    ) -> Result<Visit, String> {
        db::update_visit(&self.db, visit_id, visit).await
    }

    pub async fn remove_visit(&self, visit_id: ObjectId) -> Result<Visit, String> {
        db::remove_visit(&self.db, visit_id).await
    }

    pub async fn remove_message(&self, message_id: ObjectId) -> Result<Message, String> {
        db::remove_message(&self.db, message_id).await
    }
    
    pub async fn add_comment(&self, message_id: ObjectId, comment_content: String) -> Result<Message, String> {
        db::add_comment(&self.db, message_id, comment_content).await
    }

    pub async fn remove_comment(&self, message_id: ObjectId, comment_id: ObjectId) -> Result<Message, String> {
        db::remove_comment(&self.db, message_id, comment_id).await
    }
}
