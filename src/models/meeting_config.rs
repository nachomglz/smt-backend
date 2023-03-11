use super::team::Team;
use crate::utils::db::get_collection;
use crate::{config::Pool, utils::responders::Response};
use bson::{doc, oid::ObjectId};
use mongodb::bson::to_bson;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
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
    /// Team Id as ObjectId
    team_id: Option<ObjectId>,
    /// Team Id as String
    #[serde(skip_serializing)]
    team_id_str: Option<String>,
    /// Time in seconds that the meeting should last at maximum
    desired_duration: i64,
    /// Name of the meeting (ex: Pandora Daily)
    meeting_name: String,
    /// Description of the meeting
    description: String,
    /// Type of the meeting (RETRO | DAILY)
    meeting_type: String,
}

#[rocket::post("/", format = "json", data = "<meeting_config>")]
pub async fn create(
    db_pool: &State<Pool>,
    meeting_config: Json<MeetingConfig>,
) -> Result<Response<MeetingConfig>, Status> {
    let collection = get_collection::<MeetingConfig>(db_pool, "meeting_configs").await;
    let team_collection = get_collection::<Team>(db_pool, "teams").await;

    let mut new_config = meeting_config.0.clone();
    if let Some(new_team_id) = &new_config.team_id_str {
        new_config.team_id = Some(match ObjectId::parse_str(new_team_id) {
            Ok(id) => id,
            Err(_) => return Err(Status::UnprocessableEntity),
        });

        // Check if the team with the `team_id` provided exists
        let result = team_collection
            .find_one(
                doc! {
                    "_id": new_config.team_id.unwrap()
                },
                None,
            )
            .await
            .unwrap();

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
    } else {
        Err(Status::UnprocessableEntity)
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

#[rocket::put("/<meeting_config_id>", format = "json", data = "<meeting_config>")]
pub async fn update(
    db_pool: &State<Pool>,
    meeting_config_id: String,
    meeting_config: Json<MeetingConfig>,
) -> Result<Response<MeetingConfig>, Status> {
    let collection = get_collection::<MeetingConfig>(db_pool, "meeting_configs").await;
    let team_collection = get_collection::<Team>(db_pool, "teams").await;

    let mut new_meeting_config = meeting_config.0.clone();

    let meeting_config_id = match ObjectId::parse_str(meeting_config_id) {
        Ok(id) => id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    // get team_id as object_id
    new_meeting_config.team_id = Some(
        match ObjectId::parse_str(&new_meeting_config.team_id_str.unwrap()) {
            Ok(id) => id,
            Err(_) => return Err(Status::UnprocessableEntity),
        },
    );

    // check if that team exists
    let team_exists: bool = team_collection
        .find_one(doc! { "_id": &new_meeting_config.team_id.unwrap() }, None)
        .await
        .unwrap()
        .is_some();

    if team_exists {
        let result = collection
            .find_one_and_update(
                doc! { "_id": meeting_config_id },
                doc! {
                    "$set": {
                        "team_id": &new_meeting_config.team_id.unwrap(),
                        "desired_duration": to_bson(&new_meeting_config.desired_duration).unwrap(),
                        "meeting_name": &new_meeting_config.meeting_name,
                        "description": &new_meeting_config.description,
                        "meeting_type": &new_meeting_config.meeting_type
                    }
                },
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            )
            .await;

        match result {
            Ok(result) => match result {
                Some(new_config) => Ok(Response::Success(Json(new_config))),
                None => Err(Status::NotFound),
            },
            Err(error) => {
                eprintln!("[UPDATE][MEETING_CONFIG] ~ {}", error);
                Err(Status::InternalServerError)
            }
        }
    } else {
        Err(Status::NotFound)
    }
}
