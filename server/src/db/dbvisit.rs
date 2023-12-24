use crate::{db, model::Visit};
use actix_web::cookie::time::Date;
use async_graphql::futures_util::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct VisitEntry {
    pub creator_id: String,
    pub arrival: DateTime,
    pub posted_on: DateTime,
    pub departure: DateTime,
    pub num_staying: isize,
}

pub async fn add_visit(db: &db::DB, visit: &VisitEntry) -> Result<Visit, String> {
    let collection = db.client.collection::<VisitEntry>("Visits");
    let result = collection
        .insert_one(visit, None)
        .await
        .expect("Failed to add visit to db");
    let visit = Visit {
        _id: result.inserted_id.to_string(),
        creator_id: visit.creator_id.clone(),
        arrival: visit.arrival,
        departure: visit.departure,
        posted_on: visit.posted_on,
        num_staying: visit.num_staying,
    };
    Ok(visit)
}

pub struct VisitUpdate {
    pub creator_id: Option<String>,
    pub arrival: Option<DateTime>,
    pub posted_on: Option<DateTime>,
    pub departure: Option<DateTime>,
    pub num_staying: Option<isize>,
}

pub async fn update_visit(
    db: &db::DB,
    visit_id: ObjectId,
    visit: VisitUpdate,
) -> Result<Visit, String> {
    let collection = db.client.collection::<Visit>("Visits");
    let filter = doc! {"_id": visit_id};
    let mut update = doc! {};
    if let Some(creator_id) = visit.creator_id {
        update.insert("creator_id", creator_id);
    }
    if let Some(arrival) = visit.arrival {
        update.insert("arrival", arrival);
    }
    if let Some(departure) = visit.departure {
        update.insert("departure", departure);
    }
    if let Some(posted_on) = visit.posted_on {
        update.insert("posted_on", posted_on);
    }
    if let Some(num_staying) = &visit.num_staying {
        update.insert(
            "num_staying",
            mongodb::bson::to_bson(num_staying).expect("error converting to bson"),
        );
    }
    let result = collection
        .find_one_and_update(filter, doc! {"$set": update}, None)
        .await
        .expect("Error updating visit in db");
    Ok(result.unwrap())
}

pub async fn remove_visit(db: &db::DB, visit_id: ObjectId) -> Result<Visit, String> {
    let collection = db.client.collection::<Visit>("Visits");
    let filter = doc! {"_id": visit_id};
    let result = collection
        .find_one_and_delete(filter, None)
        .await
        .expect("Failed to delete visit from db");
    Ok(result.unwrap())
}

pub async fn get_visits(
    db: &db::DB,
    start: Option<DateTime>,
    end: Option<DateTime>,
) -> Result<Vec<Visit>, String> {
    let collection = db.client.collection::<Visit>("Visits");
    let mut cursor = collection
        .find(doc! {}, None)
        .await
        .expect("Failed to get visits from db");
    let mut visits = Vec::new();
    while let Some(visit) = cursor
        .try_next()
        .await
        .expect("Error advancing visit cursor")
    {
        visits.push(visit);
    }
    Ok(visits)
}
