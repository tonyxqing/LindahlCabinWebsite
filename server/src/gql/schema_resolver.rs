use std::collections::{BTreeMap, HashMap};

use async_graphql::{Context, Object, Json, Data};
use chrono::{Datelike, NaiveDate};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, Uuid};

use crate::db::{MessageEntry, MessageFilter, UserEntry, UserFilter, VisitEntry, VisitUpdate};
use crate::{gql, auth};
use crate::model::{self, Reaction, Role, Visit};
use crate::model::{Message, User};

use super::{CommentData, MessageData, UserData, VisitData};

fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
}

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        let res = a + b;
        println!("{}", res);
        res
    }

    async fn make_ledger(&self, ctx: &Context<'_>, date: String) -> Result<BTreeMap<String, Visit>, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let current_date = DateTime::parse_rfc3339_str(date).map_err(|e| e.to_string())?;
        let datetime = current_date.to_chrono();
        let end_of_month = last_day_of_month(datetime.year(), datetime.month());
        let mut ledger: BTreeMap<String, Visit> = BTreeMap::new();
        let visits: Vec<Visit> = r.get_visits(Some(DateTime::from_chrono(datetime.with_day(1).unwrap())), Some(DateTime::from_chrono(datetime.with_day(29).unwrap()))).await?;
        println!("{:?}", visits);
        for i in 0..end_of_month.day0() + 1 {
            let date = datetime.with_day0(i).unwrap().date_naive();
            if !visits.is_empty() {
                ledger.insert(date.to_string(), visits[0].clone());
            }
        }
        
        Ok(ledger)
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
    ) -> Result<Vec<UserData>, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let filter = UserFilter {
            id: id.map(|id| ObjectId::parse_str(id).map_err(|e| e.to_string())).transpose()?,
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

        Ok(users.iter().map(|u| UserData(u.clone())).collect())
    }

    async fn get_messages(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        creator_id: Option<String>,
        start_time: Option<String>,
        end_time: Option<String>,
    ) -> Result<Vec<MessageData>, String> {
        let ar = gql::Resolver::from_context(ctx).await;

        
        let message_filter = MessageFilter {
            id: id.map(|id| ObjectId::parse_str(id).map_err(|e| e.to_string())).transpose()?,
            creator_id,
            content: None,
            start_time: start_time.map(|start| DateTime::parse_rfc3339_str(start).map_err(|e| e.to_string())).transpose()?,
            end_time: end_time.map(|end| DateTime::parse_rfc3339_str(end).map_err(|e| e.to_string())).transpose()?,
        };

        let result = ar
            .get_messages(message_filter)
            .await
            .expect("Error retrieving messages from DB");
        Ok(result.iter().map(|m| MessageData(m.clone())).collect())
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

fn parse_date(date: Option<String>) -> Result<Option<DateTime>, String> {
    Ok(date.map(|d| DateTime::parse_rfc3339_str(&d).map_err(|e| e.to_string()))
        .transpose()?)
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
        let visit_update = VisitUpdate {
            creator_id,
            arrival: parse_date(arrival)?,
            departure: parse_date(departure)?,
            posted_on: if let Some(time) = posted_on {
                Some(DateTime::parse_rfc3339_str(time).expect("Error parsing datetime"))
            } else {
                None
            },
            num_staying,
        };
        let result = r
            .update_visit(
                ObjectId::parse_str(visit_id).expect("Error parsing object id"),
                visit_update
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
