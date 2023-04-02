use crate::utils::db::get_collection;
use crate::{config::Pool, utils::responders::Response};
use bson::doc;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use rocket::{self, http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meeting {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    /// Real duration of the meeting in seconds (max 65535)
    duration: u16,
    /// Id of the Meeting Configuration associated
    config_id: Option<ObjectId>,
    /// Date and time when the meeting started
    #[serde(with = "ts_milliseconds")]
    date_utc: DateTime<Utc>,
}

#[rocket::post("/", format = "json", data = "<meeting>")]
pub async fn create(
    db_pool: &State<Pool>,
    meeting: Json<Meeting>,
) -> Result<Response<Meeting>, Status> {
    let collection = get_collection::<Meeting>(db_pool, "meetings").await;

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

#[rocket::get("/<meeting_id>")]
pub async fn get(db_pool: &State<Pool>, meeting_id: String) -> Result<Response<Meeting>, Status> {
    let collection = get_collection::<Meeting>(db_pool, "meetings").await;

    let meeting_id = match ObjectId::parse_str(meeting_id) {
        Ok(id) => id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let result = collection
        .find_one(
            doc! {
                "_id": meeting_id
            },
            None,
        )
        .await
        .unwrap();

    match result {
        Some(meeting) => Ok(Response::Success(Json(meeting))),
        None => Err(Status::NotFound),
    }
}
