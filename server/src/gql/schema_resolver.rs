use async_graphql::{Context, Object};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, Uuid};

use crate::db::{MessageEntry, MessageFilter, UserEntry, UserFilter, VisitEntry, VisitUpdate};
use crate::{gql, auth};
use crate::model::{self, Reaction, Role, Visit};
use crate::model::{Message, User};

use super::{CommentData, MessageData, UserData, VisitData};

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        let res = a + b;
        println!("{}", res);
        res
    }

    async fn check_credentials(&self, credentials: String) -> String {
        let claims = auth::decode_jwt(credentials).await.expect("Error decoding token");
        format!("claims {:?}", claims)
    }

    async fn validate_account(&self, ctx: &Context<'_>, user_id: String, credentials: String) -> bool {
        true
    }

    async fn get_users(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        role: Option<Role>,
        sub: Option<String>,

    ) -> Vec<UserData> {
        let r = gql::Resolver::from_context(ctx).await;
        let filter = UserFilter {
            id: if let Some(id) = id {
                Some(ObjectId::parse_str(id).expect("error parsing objectid"))
            } else {
                None
            },
            name,
            phone,
            email,
            role,
            sub
        };
        let users = r
            .get_users(filter)
            .await
            .expect("Unable to get users from database");

        users.iter().map(|u| UserData(u.clone())).collect()
    }

    async fn get_messages(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        creator_id: Option<String>,
        start_time: Option<String>,
        end_time: Option<String>,
    ) -> Vec<MessageData> {
        let ar = gql::Resolver::from_context(ctx).await;

        
        let message_filter = MessageFilter {
            id: match id {
                Some(id) => Some(ObjectId::parse_str(id).unwrap()),
                None => None,
            },
            creator_id,
            start_time: if let Some(start) = start_time {
                Some(DateTime::parse_rfc3339_str(start).expect("unable to parse datetime"))
            } else {
                None
            },
            end_time: if let Some(end) = end_time {
                Some(DateTime::parse_rfc3339_str(end).expect("unable to parse datetime"))
            } else {
                None
            },
            ..MessageFilter::default()
        };
        let result = ar
            .get_messages(message_filter)
            .await
            .expect("Error retrieving messages from DB");
        result.iter().map(|m| MessageData(m.clone())).collect()
    }
    pub async fn get_visits(
        &self,
        ctx: &Context<'_>,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<Vec<VisitData>, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let s = start.map(|date| DateTime::parse_rfc3339_str(date).expect("Error parsing datetime to get visit"));
        let e = end.map(|date| DateTime::parse_rfc3339_str(date).expect("Error parsing datetime to get visit"));
        let result = r
            .get_visits(s, e)
            .await
            .expect("Failed to remove visit from resolver");

        Ok(result
            .iter()
            .map(|visit| VisitData(visit.clone()))
            .collect())
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn add_comment(&self, ctx: &Context<'_>, message_id: String, comment_content: String) -> Result<MessageData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let id = ObjectId::parse_str(message_id).expect("Unable to parse string to oid");
        let result = r.add_comment(id, comment_content).await.expect("");
        Ok(MessageData(result.clone()))

    }

    pub async fn remove_comment(&self, ctx: &Context<'_>, message_id: String, comment_id: String) -> Result<MessageData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let id = ObjectId::parse_str(message_id).expect("Unable to parse string to oid");
        let c_id = ObjectId::parse_str(comment_id).expect("Unable to parse string to oid");
        let result = r.remove_comment(id, c_id).await.expect("");
        Ok(MessageData(result.clone()))

    }
    pub async fn get_session(&self, ctx: &Context<'_>, credentials: String) -> Result<String, String> {
        let r = gql::Resolver::from_context(ctx).await;
        // take a credential token from oauth service
        let (sub, _) = auth::decode_jwt(credentials).await.map_err(|_| "Error occured decoding jwt")?;
        // check database to see if sub is there
        let user = r.get_users(UserFilter{sub: Some(sub.clone()), ..Default::default()}).await?.first().cloned();
        
        if user.is_none() {
            return Ok("MAKE_ACCOUNT".to_string());
        } else if !user.unwrap().is_active {
            return Ok("ACTIVATE_ACCOUNT".to_string());
        }
        // create session token
        let token = auth::create_token().await.map_err(|_| "Error creating token")?;

        // if user not there send token indicating to make an account
        // create account token

        Ok(token)
    }

    pub async fn add_member(
        &self,
        ctx: &Context<'_>,
        name: String,
        phone: String,
        credentials: String,
        
    ) -> Result<gql::UserData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let (sub, email) = auth::decode_jwt(credentials).await.map_err(|_| "Failed to get sub from token")?;
        let existing_user = r.get_users(UserFilter{sub: Some(sub.clone()), ..Default::default()}).await.map_err(|_| "Unable to get users")?;
        let u = UserEntry {
            name,
            email,
            phone,
            sub,
            role: Role::Member,
            is_active: false,
        };
        if existing_user.is_empty() {
            let result = r.add_user(&u).await;
            let user: UserData = gql::UserData(result.unwrap().clone());
            Ok(user)
        } else {
            Err("User already exists".to_string())
        }
    }

    pub async fn remove_member(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<gql::UserData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let result = r
            .remove_user(UserFilter {
                id: Some(ObjectId::parse_str(id).expect("Failed parsing Object Id")),
                ..UserFilter::default()
            })
            .await;
        let user = gql::UserData(result.expect("Failed to remove user"));
        Ok(user)
    }

    async fn activate_account(&self, ctx: &Context<'_>, id: String) -> Result<UserData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let result = r.activate_account(ObjectId::parse_str(id).expect("failed to parse")).await?;

        Ok(UserData (result))
    }

    async fn add_message(
        &self,
        ctx: &Context<'_>,
        creator_id: String,
        content: String,
    ) -> Result<MessageData, String> {
        let ar = gql::Resolver::from_context(ctx).await;
        let message_entry = MessageEntry {
            creator_id: creator_id.clone(),
            content: content.clone(),
            ..MessageEntry::default()
        };
        let message = ar
            .add_message(&message_entry)
            .await
            .expect("Failed to get messsage from resolver");
        Ok(MessageData(message))
    }

    pub async fn update_visit(
        &self,
        ctx: &Context<'_>,
        visit_id: String,
        creator_id: Option<String>,
        arrival: Option<String>,
        departure: Option<String>,
        posted_on: Option<String>,
        num_staying: Option<isize>,
    ) -> Result<VisitData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let result = r
            .update_visit(
                ObjectId::parse_str(visit_id).expect("Error parsing object id"),
                VisitUpdate {
                    creator_id,
                    arrival: if let Some(time) = arrival {
                        Some(DateTime::parse_rfc3339_str(time).expect("Error parsing datetime"))
                    } else {
                        None
                    },
                    departure: if let Some(time) = departure {
                        Some(DateTime::parse_rfc3339_str(time).expect("Error parsing datetime"))
                    } else {
                        None
                    },
                    posted_on: if let Some(time) = posted_on {
                        Some(DateTime::parse_rfc3339_str(time).expect("Error parsing datetime"))
                    } else {
                        None
                    },
                    num_staying,
                },
            )
            .await
            .expect("Failed to update visit from resolver");
        Ok(VisitData(result))
    }

    pub async fn add_visit(
        &self,
        ctx: &Context<'_>,
        creator_id: String,
        arrival: String,
        departure: String,
        num_staying: isize,
    ) -> Result<VisitData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let result = r
            .add_visit(&VisitEntry {
                creator_id,
                arrival: DateTime::parse_rfc3339_str(arrival).expect("Error parsing datetime"),
                departure: DateTime::parse_rfc3339_str(departure).expect("Error parsing datetime"),
                posted_on: DateTime::now(),
                num_staying,
            })
            .await
            .expect("Failed to add visit from resolver");
        Ok(VisitData(result))
    }
    pub async fn remove_visit(
        &self,
        ctx: &Context<'_>,
        visit_id: String,
    ) -> Result<VisitData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let result = r
            .remove_visit(ObjectId::parse_str(visit_id).expect("Error parsing object id"))
            .await
            .expect("Failed to remove visit from resolver");
        Ok(VisitData(result))
    }

    pub async fn update_message(
        &self,
        ctx: &Context<'_>,
        id: String,
        creator_id: String,
        content: Option<String>,
    ) -> Result<MessageData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let filter = MessageFilter {
            id: Some(ObjectId::parse_str(id).expect("Did not parse id")),
            creator_id: Some(creator_id),
            content,
            start_time: None,
            end_time: None,
            ..MessageFilter::default()
        };
        let result = r.update_message(filter).await;
        let message = MessageData(result.expect("failed to retrieve Message"));
        Ok(message)
    }

    pub async fn remove_message(
        &self,
        ctx: &Context<'_>,
        message_id: String
    ) -> Result<MessageData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let id = ObjectId::parse_str(message_id).expect("Did not parse id");
        let result = r.remove_message(id).await.expect("Failed to remove message");
        Ok(MessageData (result.clone()))
    }
}
