use crate::config::Pool;
use crate::utils::responders::Response;
use mongodb::bson::{doc, oid::ObjectId};
use rocket::State;
use rocket::{http::Status, serde::json::Json};
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
pub async fn new(db_pool: &State<Pool>, team: Json<Team>) -> Result<Response<Team>, Status> {
    let db = db_pool.get().await.unwrap();
    let collection = db.default_database().unwrap().collection::<Team>("teams");
    let user_collection = db.default_database().unwrap().collection::<User>("users");

    let mut new_team: Team = team.0.clone();
    let mut new_team_users: Vec<ObjectId> = vec![];

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

    let result = collection.insert_one(&new_team, None).await.unwrap();
    new_team.id = Some(result.inserted_id.as_object_id().unwrap());

    Ok(Response::Created(Json(new_team)))
}

#[rocket::get("/<team_id>")]
pub async fn get(db_pool: &State<Pool>, team_id: String) -> Result<Response<Team>, Status> {
    let db = db_pool.get().await.unwrap();
    let collection = db.default_database().unwrap().collection::<Team>("teams");

    let team_id: ObjectId = match ObjectId::parse_str(team_id) {
        Ok(result) => result,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

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
        Some(team) => Ok(Response::Success(Json(team))),
        None => Err(Status::NotFound),
    }
}
