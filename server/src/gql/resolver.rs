use crate::db::{
    self, MessageEntry, MessageFilter, UserEntry, UserFilter, VisitEntry, VisitUpdate,
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

    pub async fn remove_user(&self, filter: UserFilter) -> Result<User, String> {
        db::remove_user(&self.db, filter).await
    }

    pub async fn update_message(&self, filter: MessageFilter) -> Result<Message, String> {
        db::update_message(&self.db, filter).await
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
}
