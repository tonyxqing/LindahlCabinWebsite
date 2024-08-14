use std::collections::{BTreeMap, HashMap};
use std::default;

use async_graphql::{Context, Object, SimpleObject};
use chrono::{prelude::*, Months};
use chrono::{Datelike, NaiveDate, NaiveTime, Timelike, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{de, DateTime, Uuid};
use serde::Serialize;

use crate::auth::AuthResult;
use crate::db::{
    MessageEntry, MessageFilter, UserEntry, UserFilter, UserUpdate, VisitEntry, VisitUpdate,
};
use crate::model::{self, Role, Visit};
use crate::model::{Message, User};
use crate::{auth, gql};

use super::{CommentData, MessageData, UserData, VisitData};
use rand::prelude::*;
fn random_code(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut result: String = String::with_capacity(length);
    for i in 0..length {
        let rand_num: usize = rng.gen();
        let idx = rand_num % charset.len();
        result.push(charset[idx] as char);
    }
    result
}
fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, 1)
        .unwrap()
        .checked_add_months(Months::new(1))
        .unwrap()
        .pred_opt()
        .unwrap()
}
#[derive(SimpleObject, Serialize)]
pub struct LedgerVisit {
    id: String,
    arrival: String,
    departure: String,
    posted_on: String,
    creator_id: String,
    profile_pic: String,
    name: String,
    num_staying: isize,
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

    async fn make_ledger(
        &self,
        ctx: &Context<'_>,
        date: String,
    ) -> Result<BTreeMap<String, Vec<LedgerVisit>>, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let current_date = DateTime::parse_rfc3339_str(date).map_err(|e| e.to_string())?;
        let datetime = current_date.to_chrono();
        let end_of_month =
            last_day_of_month(datetime.year(), datetime.month()).and_time(NaiveTime::default());
        let mut ledger: BTreeMap<String, Vec<LedgerVisit>> = BTreeMap::new();
        let mut memoizedUsers: HashMap<String, User> = HashMap::new();
        let visits: Vec<Visit> = r
            .get_visits(
                Some(DateTime::from_chrono(datetime.with_day(1).unwrap())),
                Some(DateTime::from_chrono(
                    Utc.from_local_datetime(&end_of_month).unwrap(),
                )),
            )
            .await?;
        for i in 0..end_of_month.day0() + 1 {
            let date = datetime.with_day0(i).unwrap().date_naive();
            let time = NaiveTime::default();

            let mut contained_visits: Vec<LedgerVisit> = Vec::new();
            if !visits.is_empty() {
                for visit in visits.iter() {
                    let datetime = date.and_time(time);
                    if visit.arrival.to_chrono() <= Utc.from_utc_datetime(&datetime)
                        && Utc.from_utc_datetime(&datetime) <= visit.departure.to_chrono()
                    {
                        let user = if memoizedUsers.contains_key(visit.creator_id.as_str()) {
                            memoizedUsers
                                .get(visit.creator_id.as_str())
                                .unwrap()
                                .to_owned()
                        } else {
                            let u = r
                                .get_users(UserFilter {
                                    id: ObjectId::parse_str(visit.creator_id.clone()).ok(),
                                    ..Default::default()
                                })
                                .await?
                                .first()
                                .unwrap()
                                .clone();
                            memoizedUsers.insert(visit.creator_id.clone(), u.clone());
                            u
                        };
                        contained_visits.push(LedgerVisit {
                            id: visit._id.to_string(),
                            arrival: visit
                                .arrival
                                .try_to_rfc3339_string()
                                .expect("arrival was not able to convert to rfc3339 string"),
                            profile_pic: user.profile_pic.clone().unwrap(),
                            name: user.name.clone(),
                            departure: visit
                                .departure
                                .try_to_rfc3339_string()
                                .expect("departure was not able to convert to rfc 3339 string"),
                            posted_on: visit.posted_on.to_string(),
                            creator_id: visit.creator_id.to_string(),
                            num_staying: visit.num_staying,
                        });
                    }
                }
                ledger.insert(date.to_string(), contained_visits);
            }
        }

        Ok(ledger)
    }

    async fn check_credentials(&self, credentials: String) -> String {
        let claims = auth::decode_jwt(credentials)
            .await
            .expect("Error decoding token");
        format!("claims {:?}", claims)
    }

    async fn validate_account(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        credentials: String,
    ) -> bool {
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
        access_code: Option<String>,
    ) -> Result<Vec<UserData>, String> {
        AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner))?;

        let auth = ctx
            .data::<AuthResult>()
            .map_err(|e| "Error occurred retrieving account from token".to_string())?;
        println!("{:?}", auth);
        let r = gql::Resolver::from_context(ctx).await;
        let filter = UserFilter {
            id: id
                .map(|id| ObjectId::parse_str(id).map_err(|e| e.to_string()))
                .transpose()?,
            name,
            phone,
            email,
            role,
            sub,
            access_code,
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
            id: id
                .map(|id| ObjectId::parse_str(id).map_err(|e| e.to_string()))
                .transpose()?,
            creator_id,
            content: None,
            start_time: start_time
                .map(|start| DateTime::parse_rfc3339_str(start).map_err(|e| e.to_string()))
                .transpose()?,
            end_time: end_time
                .map(|end| DateTime::parse_rfc3339_str(end).map_err(|e| e.to_string()))
                .transpose()?,
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
        let s = start.map(|date| {
            DateTime::parse_rfc3339_str(date).expect("Error parsing datetime to get visit")
        });
        let e = end.map(|date| {
            DateTime::parse_rfc3339_str(date).expect("Error parsing datetime to get visit")
        });
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
    Ok(date
        .map(|d| DateTime::parse_rfc3339_str(&d).map_err(|e| e.to_string()))
        .transpose()?)
}

async fn get_sub_and_email_from_credentials(
    credentials: String,
) -> Result<(String, String, String, Option<String>), String> {
    let (sub, email, name, picture) = auth::decode_jwt(credentials)
        .await
        .map_err(|_| "Failed to get sub from token")?;
    //    let existing_user = r
    //        .get_users(UserFilter {
    //            sub: Some(sub.clone()),
    //            ..Default::default()
    //        })
    //        .await
    //        .map_err(|_| "Unable to get users")?;
    //    if !existing_user.is_empty() {
    //        let result = r.add_user(&u).await;
    //        let user: UserData = gql::UserData(result.unwrap().clone());
    //        Ok(user)
    //    } else {
    //        Err("User already exists".to_string())
    //    }

    Ok((sub, email, name, picture))
}
pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn add_comment(
        &self,
        ctx: &Context<'_>,
        message_id: String,
        comment_content: String,
    ) -> Result<MessageData, String> {
        let creator_id = AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner)).or(AuthResult::from_context(ctx, Role::Member))?;
        let r = gql::Resolver::from_context(ctx).await;
        let id = ObjectId::parse_str(message_id).expect("Unable to parse string to oid");
        let result = r
            .add_comment(id, comment_content, creator_id)
            .await
            .expect("");
        Ok(MessageData(result.clone()))
    }

    pub async fn remove_comment(
        &self,
        ctx: &Context<'_>,
        message_id: String,
        comment_id: String,
    ) -> Result<MessageData, String> {
        let creator_id = AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner)).or(AuthResult::from_context(ctx, Role::Member))?;
        let r = gql::Resolver::from_context(ctx).await;

        let id = ObjectId::parse_str(message_id).expect("Unable to parse string to oid");
        let c_id = ObjectId::parse_str(comment_id).expect("Unable to parse string to oid");
        let result = r.remove_comment(id, c_id, creator_id).await.expect("Failed to remove comment");
        Ok(MessageData(result.clone()))
    }
    pub async fn get_session(
        &self,
        ctx: &Context<'_>,
        credentials: String,
    ) -> Result<String, String> {
        let r = gql::Resolver::from_context(ctx).await;
        // take a credential token from oauth service
        let (sub, _, _, picture) = auth::decode_jwt(credentials)
            .await
            .map_err(|_| "Error occured decoding jwt")?;
        // check database to see if sub is there
        let user = r
            .get_users(UserFilter {
                sub: Some(sub.clone()),
                ..Default::default()
            })
            .await?
            .first()
            .cloned();

        if user.is_none() {
            return Ok("MAKE_ACCOUNT".to_string());
        }
        // create session token
        let u = user.unwrap();
        let update = r
            .update_user(
                UserFilter {
                    sub: Some(sub.clone()),
                    ..Default::default()
                },
                UserUpdate {
                    profile_pic: picture.clone(),
                    ..Default::default()
                },
            )
            .await?;
        let token = auth::create_token(picture.unwrap(), u._id.to_string(), u.role.to_string(), u.name.to_string())
            .await
            .map_err(|_| "Error creating token")?;

        // if user not there send token indicating to make an account
        // create account token

        Ok(token)
    }

    pub async fn add_member(&self, ctx: &Context<'_>) -> Result<gql::UserData, String> {
        AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner))?;
        let r = gql::Resolver::from_context(ctx).await;
        let u = UserEntry {
            name: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            sub: "".to_string(),
            role: Role::Member,
            access_code: random_code(8),
            profile_pic: None,
        };

        let result = r.add_user(&u).await;
        let user: UserData = gql::UserData(result.unwrap().clone());
        Ok(user)
    }

    pub async fn register_member(
        &self,
        ctx: &Context<'_>,
        access_code: String,
        credentials: String,
    ) -> Result<String, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let filter = UserFilter {
            access_code: Some(access_code.clone()),
            ..Default::default()
        };
        let (sub, email, name, picture) = get_sub_and_email_from_credentials(credentials).await?;
        println!("sub: {sub}, email: {email}");

        let exists = r
            .get_users(UserFilter {
                sub: Some(sub.clone()),
                ..Default::default()
            })
            .await?;
        if !exists.is_empty() {
            return Err("Account already exists. Cannot create a new one.".to_string());
        }

        let account = r.get_users(filter).await?;
        println!("found user {:?}", account);
        if account.is_empty() {
            return Err("Invalid Access Code".to_string());
        }
        r.update_user(
            UserFilter {
                id: Some(mongodb::bson::oid::ObjectId::parse_str(account[0]._id.clone()).unwrap()),
                ..Default::default()
            },
            UserUpdate {
                access_code: Some("".to_string()),
                sub: Some(sub),
                name: Some(name),
                profile_pic: picture.clone(),
                role: Some(Role::Member),
                email: Some(email),
                phone: Some("".to_string()),
            },
        )
        .await?;
        let token = auth::create_token(
            picture.unwrap(),
            account.first().unwrap()._id.to_string(),
            account.first().unwrap().role.to_string(),
            account.first().unwrap().name.to_string(),
        )
        .await;
        Ok(token.unwrap())
    }

    pub async fn update_member(
        &self,
        ctx: &Context<'_>,
        filter_id: Option<String>,
        filter_access_code: Option<String>,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        role: Option<Role>,
        access_code: Option<String>,
    ) -> Result<UserData, String> {
        AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner))?;
        let r = gql::Resolver::from_context(ctx).await;
        let filter = UserFilter {
            id: filter_id.clone().map(|id| ObjectId::parse_str(id).unwrap()),
            access_code: filter_access_code.clone(),
            ..Default::default()
        };

        let update = UserUpdate {
            sub: None,
            profile_pic: None,
            name,
            email,
            phone,
            role,
            access_code,
        };

        let user = r.update_user(filter, update).await?;
        Ok(UserData(user))
    }

    pub async fn remove_member(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<gql::UserData, String> {
        AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner))?;
        let r = gql::Resolver::from_context(ctx).await;
        println!("{id}");
        let result = r
            .remove_user(UserFilter {
                id: Some(ObjectId::parse_str(id).expect("Failed parsing Object Id")),
                ..UserFilter::default()
            })
            .await?;
        let user = gql::UserData(result);
        Ok(user)
    }

    async fn activate_account(&self, ctx: &Context<'_>, id: String) -> Result<UserData, String> {
        let r = gql::Resolver::from_context(ctx).await;
        let result = r
            .activate_account(ObjectId::parse_str(id).expect("failed to parse"))
            .await?;

        Ok(UserData(result))
    }

    async fn add_message(
        &self,
        ctx: &Context<'_>,
        content: String,
    ) -> Result<MessageData, String> {
        let creator_id = AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner)).or(AuthResult::from_context(ctx, Role::Member))?;
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
        posted_on: Option<String>,
        num_staying: Option<isize>,
        arrival: Option<String>,
        departure: Option<String>
    ) -> Result<VisitData, String> {
        let creator_id = AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner)).or(AuthResult::from_context(ctx, Role::Member))?;
        let r = gql::Resolver::from_context(ctx).await;
        let visit_update = VisitUpdate {
            creator_id: Some(creator_id),
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
                visit_update,
            )
            .await
            .expect("Failed to update visit from resolver");
        Ok(VisitData(result))
    }

    pub async fn add_visit(
        &self,
        ctx: &Context<'_>,
        arrival: String,
        departure: String,
        num_staying: isize,
    ) -> Result<VisitData, String> { 
        let creator_id = AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner)).or(AuthResult::from_context(ctx, Role::Member))?;
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
        let creator_id = AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner)).or(AuthResult::from_context(ctx, Role::Member))?;
        let r = gql::Resolver::from_context(ctx).await;
        let v_id = ObjectId::parse_str(visit_id).expect("Error parsing object id");
        let result = r
            .remove_visit(v_id, creator_id)
            .await
            .expect("Failed to remove visit from resolver");
        Ok(VisitData(result))
    }

    pub async fn update_message(
        &self,
        ctx: &Context<'_>,
        id: String,
        content: Option<String>,
    ) -> Result<MessageData, String> {
        let creator_id = AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner)).or(AuthResult::from_context(ctx, Role::Member))?;
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
        message_id: String,
    ) -> Result<MessageData, String> {
        let creator_id = AuthResult::from_context(ctx, Role::Admin).or(AuthResult::from_context(ctx, Role::Owner)).or(AuthResult::from_context(ctx, Role::Member))?;
        let r = gql::Resolver::from_context(ctx).await;
        let id = ObjectId::parse_str(message_id).expect("Did not parse id");
        let result = r
            .remove_message(id, creator_id)
            .await
            .expect("Failed to remove message");
        Ok(MessageData(result.clone()))
    }
}
