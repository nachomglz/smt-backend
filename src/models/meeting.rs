use crate::utils;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use rocket::{self, http::Status, State};
use serde::{Deserialize, Serialize};

use crate::{config::Pool, utils::responders::Response};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meeting {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    /// Real duration of the meeting
    duration: u16,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    date_utc: DateTime<Utc>,
}

#[rocket::post("/new")]
pub async fn new(db_pool: &State<Pool>) -> Result<Response<Meeting>, Status> {
    let collection = utils::db::get_collection::<Meeting>(db_pool, "meetings").await;
    todo!()
}
