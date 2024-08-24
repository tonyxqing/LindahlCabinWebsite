use crate::gql;
use crate::{
    db::UserFilter,
    model::{self},
};
use async_graphql::{Context, Object};
use mongodb::bson::oid::ObjectId;
use serde::Serialize;

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
    async fn profile_pic(&self) -> Option<String> {
        self.0.profile_pic.clone()
    }
    async fn sub(&self) -> &str {
        &self.0.sub
    }
    async fn access_code(&self) -> &str {
        &self.0.access_code
    }
}

#[derive(Serialize)]
pub struct VisitData(pub model::Visit);
#[Object]
impl VisitData {
    async fn id(&self) -> String {
        self.0._id.clone()
    }
    async fn creator_id(&self) -> &str {
        &self.0.creator_id
    }
    async fn arrival(&self) -> Result<String, String> {
        Ok(self
            .0
            .arrival
            .try_to_rfc3339_string()
            .map_err(|e| e.to_string())?)
    }
    async fn departure(&self) -> Result<String, String> {
        Ok(self
            .0
            .departure
            .try_to_rfc3339_string()
            .map_err(|e| e.to_string())?)
    }
    async fn posted_on(&self) -> Result<String, String> {
        Ok(self
            .0
            .posted_on
            .try_to_rfc3339_string()
            .map_err(|e| e.to_string())?)
    }
    async fn num_staying(&self) -> isize {
        self.0.num_staying
    }
    async fn profile_pic(&self, ctx: &Context<'_>) -> Result<Option<String>, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let user = r
            .get_users(UserFilter {
                id: ObjectId::parse_str(self.0.creator_id.clone()).ok(),
                ..Default::default()
            })
            .await?
            .first()
            .expect("issue getting user for visit")
            .clone();
        Ok(user.profile_pic.clone())
    }
    async fn name(&self, ctx: &Context<'_>) -> Result<String, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let user = r
            .get_users(UserFilter {
                id: ObjectId::parse_str(self.0.creator_id.clone()).ok(),
                ..Default::default()
            })
            .await?
            .first()
            .expect("issue getting user for visit")
            .clone();
        Ok(user.name.clone())
    }
    async fn email(&self, ctx: &Context<'_>) -> Result<String, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let user = r
            .get_users(UserFilter {
                id: ObjectId::parse_str(self.0.creator_id.clone()).ok(),
                ..Default::default()
            })
            .await?
            .first()
            .expect("issue getting user for visit")
            .clone();
        Ok(user.email.clone())
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
    async fn name(&self, ctx: &Context<'_>) -> Result<String, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let user = r
            .get_users(UserFilter {
                id: ObjectId::parse_str(self.0.creator_id.clone()).ok(),
                ..Default::default()
            })
            .await?
            .first()
            .expect("issue getting user for message")
            .clone();
        Ok(user.name.clone())
    }
    async fn profile_pic(&self, ctx: &Context<'_>) -> Result<String, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let user = r
            .get_users(UserFilter {
                id: ObjectId::parse_str(self.0.creator_id.clone()).ok(),
                ..Default::default()
            })
            .await?
            .first()
            .expect("problem getting pic for message")
            .clone();
        Ok(user
            .profile_pic
            .expect("problem getting profilepic for message")
            .clone())
    }
    async fn content(&self) -> &str {
        &self.0.content
    }
    async fn posted_on(&self) -> String {
        self.0.posted_on.to_string()
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
    async fn name(&self, ctx: &Context<'_>) -> Result<String, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let user = r
            .get_users(UserFilter {
                id: ObjectId::parse_str(self.0.creator_id.clone()).ok(),
                ..Default::default()
            })
            .await?
            .first()
            .expect("issue getting user for message")
            .clone();
        Ok(user.name.clone())
    }
    async fn profile_pic(&self, ctx: &Context<'_>) -> Result<String, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let user = r
            .get_users(UserFilter {
                id: ObjectId::parse_str(self.0.creator_id.clone()).ok(),
                ..Default::default()
            })
            .await?
            .first()
            .expect("problem getting pic for message")
            .clone();
        Ok(user
            .profile_pic
            .expect("problem getting profilepic for message")
            .clone())
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
    async fn posted_on(&self) -> String {
        self.0.posted_on.to_string()
    }
}
