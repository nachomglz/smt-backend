use crate::utils::db::get_collection;
use bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::{config::Pool, utils::responders::Response};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeetingType {
    RETRO,
    DAILY,
}

impl MeetingType {
    fn value(&self) -> &str {
        match self {
            MeetingType::DAILY => "DAILY",
            MeetingType::RETRO => "RETRO",
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MeetingConfig {
    /// Meeting DB Id
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    /// Team Id
    team_id: String,
    /// Time in seconds that the meeting should last at maximum
    desired_duration: u16,
    /// Name of the meeting (ex: Pandora Daily)
    meeting_name: String,
    /// Description of the meeting
    description: String,
    /// Type of the meeting (RETRO | DAILY)
    meeting_type: MeetingType,
}

#[rocket::post("/new", format = "json", data = "<meeting_config>")]
pub async fn new(
    db_pool: &State<Pool>,
    meeting_config: Json<MeetingConfig>,
) -> Result<Response<MeetingConfig>, Status> {
    let collection = get_collection::<MeetingConfig>(db_pool, "meeting_configs").await;

    let mut new_config = meeting_config.0.clone();

    let result = collection.insert_one(&new_config, None).await;

    match result {
        Ok(result) => {
            new_config.id = Some(result.inserted_id.as_object_id().unwrap());
            Ok(Response::Created(Json(new_config)))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
