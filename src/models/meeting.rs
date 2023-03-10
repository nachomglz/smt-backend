use crate::utils;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use rocket::{self, http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::{config::Pool, utils::responders::Response};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meeting {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    /// Real duration of the meeting
    duration: u16,
    /// Date and time when the meeting started
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    date_utc: DateTime<Utc>,
}

#[rocket::post("/new", format = "json", data = "<meeting>")]
pub async fn new(
    db_pool: &State<Pool>,
    meeting: Json<Meeting>,
) -> Result<Response<Meeting>, Status> {
    let collection = utils::db::get_collection::<Meeting>(db_pool, "meetings").await;

    let mut new_meeting = meeting.0.clone();

    let result = collection.insert_one(&new_meeting, None).await;

    match result {
        Ok(result) => {
            let id = result.inserted_id;
            new_meeting.id = Some(id.as_object_id().unwrap());
            Ok(Response::Created(Json(new_meeting)))
        }
        Err(error) => {
            eprintln!("[INSERT][MEETING] ~ {}", error);
            Err(Status::InternalServerError)
        }
    }
}
