use super::team::Team;
use crate::utils::db::get_collection;
use crate::{config::Pool, utils::responders::Response};
use bson::{doc, oid::ObjectId};
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

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

#[rocket::post("/", format = "json", data = "<meeting_config>")]
pub async fn create(
    db_pool: &State<Pool>,
    meeting_config: Json<MeetingConfig>,
) -> Result<Response<MeetingConfig>, Status> {
    let collection = get_collection::<MeetingConfig>(db_pool, "meeting_configs").await;
    let team_collection = get_collection::<Team>(db_pool, "teams").await;

    let mut new_config = meeting_config.0.clone();

    // Check if the team_id exists
    let team_id = match ObjectId::parse_str(&new_config.team_id) {
        Ok(id) => id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let result = team_collection
        .find_one(
            doc! {
                "_id": team_id
            },
            None,
        )
        .await
        .unwrap();

    println!(
        "result: {:?} \n filter: {:?}",
        result,
        doc! {
            "_id": team_id
        }
    );

    if let Some(_) = result {
        let result = collection.insert_one(&new_config, None).await;

        match result {
            Ok(result) => {
                new_config.id = Some(result.inserted_id.as_object_id().unwrap());
                Ok(Response::Created(Json(new_config)))
            }
            Err(_) => Err(Status::InternalServerError),
        }
    } else {
        Err(Status::NotFound)
    }
}

#[rocket::get("/<meeting_config_id>")]
pub async fn get(
    db_pool: &State<Pool>,
    meeting_config_id: String,
) -> Result<Response<MeetingConfig>, Status> {
    let collection = get_collection::<MeetingConfig>(db_pool, "meeting_configs").await;

    let meeting_config_id = match ObjectId::parse_str(meeting_config_id) {
        Ok(id) => id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let result = collection
        .find_one(doc! { "_id": meeting_config_id }, None)
        .await
        .unwrap();

    match result {
        Some(meeting) => Ok(Response::Success(Json(meeting))),
        None => Err(Status::NotFound),
    }
}
