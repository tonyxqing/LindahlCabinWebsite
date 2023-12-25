use async_graphql::{Context, Object};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, Uuid};

use crate::db::{MessageEntry, MessageFilter, UserEntry, UserFilter, VisitEntry, VisitUpdate};
use crate::gql;
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

    async fn get_users(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        role: Option<Role>,
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
    pub async fn add_member(
        &self,
        ctx: &Context<'_>,
        name: String,
        email: String,
        phone: String,
        role: model::Role,
    ) -> Result<gql::UserData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let u = UserEntry {
            name,
            email,
            phone,
            role,
        };
        let result = r.add_user(&u).await;
        let user: UserData = gql::UserData(result.unwrap().clone());
        Ok(user)
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
        comment_id: Option<String>,
        content: Option<String>,
        comment: Option<String>,
        reaction: Option<Reaction>,
    ) -> Result<MessageData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let filter = MessageFilter {
            id: Some(ObjectId::parse_str(id).expect("Did not parse id")),
            creator_id: Some(creator_id),
            comment_id,
            comment,
            content,
            reaction,
            start_time: None,
            end_time: None,
            ..MessageFilter::default()
        };
        let result = r.update_message(filter).await;
        let message = MessageData(result.expect("failed to retrieve Message"));
        Ok(message)
    }
}
