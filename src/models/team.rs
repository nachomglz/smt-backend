use crate::config::Pool;
use crate::utils::responders::{CustomResponse, Response, ResponseStatus};
use mongodb::bson::{doc, oid::ObjectId};
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Team {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    users: Vec<ObjectId>,
}

#[rocket::post("/new", format = "json", data = "<team>")]
pub async fn new(db_pool: &State<Pool>, team: Json<Team>) -> Response<CustomResponse<Team>> {
    let db = db_pool.get().await.unwrap();
    let collection = db.default_database().unwrap().collection::<Team>("teams");
    let user_collection = db.default_database().unwrap().collection::<User>("users");

    let mut new_team: Team = team.0.clone();
    let mut new_team_users: Vec<ObjectId> = vec![];

    // Check if users exists, if so, add them
    for provided_user in team.0.clone().users {
        let exists = user_collection
            .find_one(doc! { "_id": provided_user }, None)
            .await
            .unwrap();
        if let Some(user) = exists {
            new_team_users.push(user.get_id());
        }
    }

    new_team.users = new_team_users;

    // insert new team
    let result = collection.insert_one(&new_team, None).await;
    match result {
        Ok(result) => {
            new_team.id = Some(result.inserted_id.as_object_id().unwrap());
            CustomResponse::new()
                .data(Some(new_team))
                .status(ResponseStatus::Success)
                .build()
                .respond()
        }
        Err(_) => CustomResponse::new()
            .status(ResponseStatus::NotFound)
            .build()
            .respond(),
    }
}

#[rocket::get("/<team_id>")]
pub async fn get(db_pool: &State<Pool>, team_id: String) -> Response<CustomResponse<Team>> {
    let db = db_pool.get().await.unwrap();
    let collection = db.default_database().unwrap().collection::<Team>("teams");

    let team_id: Option<ObjectId> = match ObjectId::parse_str(team_id) {
        Ok(result) => Some(result),
        Err(_) => None,
    };

    match team_id {
        Some(team_id) => {
            let result = collection
                .find_one(
                    doc! {
                        "_id": team_id
                    },
                    None,
                )
                .await
                .unwrap();
            match result {
                Some(team) => CustomResponse::new()
                    .status(ResponseStatus::Success)
                    .data(Some(team))
                    .build()
                    .respond(),
                None => CustomResponse::new()
                    .status(ResponseStatus::NotFound)
                    .build()
                    .respond(),
            }
        }
        None => CustomResponse::new()
            .status(ResponseStatus::NotFound)
            .build()
            .respond(),
    }
}
