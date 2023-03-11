use crate::config::Pool;
use crate::utils::{db::get_collection, responders::Response};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use rocket::State;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Team {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    users: Option<Vec<ObjectId>>,
}

#[rocket::post("/", format = "json", data = "<team>")]
pub async fn create(db_pool: &State<Pool>, team: Json<Team>) -> Result<Response<Team>, Status> {
    let collection = get_collection::<Team>(db_pool, "teams").await;
    let user_collection = get_collection::<User>(db_pool, "users").await;

    let mut new_team: Team = team.0.clone();
    let mut new_team_users: Vec<ObjectId> = vec![];

    if let Some(users) = &new_team.users {
        for provided_user in users {
            let exists_in_db = user_collection
                .find_one(doc! { "_id": provided_user }, None)
                .await
                .unwrap();
            let exists_in_list = new_team_users.contains(&provided_user);
            if let Some(user) = exists_in_db {
                if !exists_in_list {
                    new_team_users.push(user.get_id());
                }
            }
        }

        new_team.users = Some(new_team_users);
    }

    let result = collection.insert_one(&new_team, None).await.unwrap();
    new_team.id = Some(result.inserted_id.as_object_id().unwrap());

    Ok(Response::Created(Json(new_team)))
}

#[rocket::get("/<team_id>")]
pub async fn get(db_pool: &State<Pool>, team_id: String) -> Result<Response<Team>, Status> {
    let collection = get_collection::<Team>(db_pool, "teams").await;

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

#[rocket::put("/<team_id>", format = "json", data = "<team>")]
pub async fn update(
    db_pool: &State<Pool>,
    team_id: String,
    team: Json<Team>,
) -> Result<Response<Team>, Status> {
    let collection = get_collection::<Team>(db_pool, "teams").await;

    let team_id = match ObjectId::parse_str(team_id) {
        Ok(team_id) => team_id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let opts = FindOneAndUpdateOptions::builder()
        .return_document(Some(ReturnDocument::After))
        .build();

    let result = collection
        .find_one_and_update(
            doc! { "_id": team_id },
            doc! { "$set": { "name": team.0.name } },
            opts,
        )
        .await
        .unwrap();

    match result {
        Some(new_team) => Ok(Response::Success(Json(new_team))),
        None => Err(Status::NotFound),
    }
}

#[rocket::get("/<team_id>/users")]
pub async fn get_users(db_pool: &State<Pool>, team_id: String) -> Result<Json<Vec<User>>, Status> {
    let collection = get_collection::<Team>(db_pool, "teams").await;

    let team_id = match ObjectId::parse_str(team_id) {
        Ok(id) => id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let team = collection
        .find_one(doc! { "_id": team_id }, None)
        .await
        .unwrap();

    match team {
        Some(team) => {
            let user_collection = get_collection::<User>(db_pool, "users").await;
            let mut users = Vec::<User>::new();
            let users_id: Vec<ObjectId> = team.users.unwrap_or(Vec::<ObjectId>::new());

            // get users from users id's
            for id in users_id {
                let user = user_collection
                    .find_one(doc! { "_id": id }, None)
                    .await
                    .unwrap()
                    .unwrap();
                users.push(user);
            }
            Ok(Json(users))
        }
        None => Err(Status::NotFound),
    }
}
