use crate::{
    config::Pool,
    models::{meeting::Meeting, user::User},
    utils::{db::get_collection, responders::Response},
};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use bson::{doc, oid::ObjectId, to_bson};
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserTime {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    /// User Id
    user_id: Option<ObjectId>,
    /// String version of User Id
    #[serde(skip_serializing)]
    user_id_str: Option<String>,
    /// Meeting Id
    meeting_id: Option<ObjectId>,
    /// String version of Meeting Id
    #[serde(skip_serializing)]
    meeting_id_str: Option<String>,
    /// Time in seconds (max: 65000 [8h])
    time: u16,
}

#[rocket::post("/", format = "json", data = "<user_time>")]
pub async fn create(
    db_pool: &State<Pool>,
    user_time: Json<UserTime>,
) -> Result<Response<UserTime>, Status> {
    let collection = get_collection::<UserTime>(db_pool, "user_times").await;
    let user_collection = get_collection::<User>(db_pool, "users").await;
    let meeting_collection = get_collection::<Meeting>(db_pool, "meetings").await;

    let mut new_user_time = user_time.0.clone();
    let user_id_str: &str = match &new_user_time.user_id_str {
        Some(id) => id,
        None => return Err(Status::UnprocessableEntity),
    };
    let meeting_id_str: &str = match &new_user_time.meeting_id_str {
        Some(id) => id,
        None => return Err(Status::UnprocessableEntity),
    };

    // parse ids to objectids
    new_user_time.user_id = match ObjectId::parse_str(user_id_str) {
        Ok(id) => Some(id),
        Err(_) => return Err(Status::UnprocessableEntity),
    };
    new_user_time.meeting_id = match ObjectId::parse_str(meeting_id_str) {
        Ok(id) => Some(id),
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    // Check if both user and meeting exist
    let meeting_exists = meeting_collection
        .find_one(
            doc! {
                "_id": &new_user_time.meeting_id
            },
            None,
        )
        .await
        .unwrap()
        .is_some();

    let user_exists = user_collection
        .find_one(
            doc! {
                "_id": &new_user_time.user_id
            },
            None,
        )
        .await
        .unwrap()
        .is_some();

    // if they both exist, create the new user_time document
    if meeting_exists && user_exists {
        match collection.insert_one(&new_user_time, None).await {
            Ok(result) => {
                let id = result.inserted_id.as_object_id().unwrap();
                new_user_time.id = Some(id);
                Ok(Response::Created(Json(new_user_time)))
            }
            Err(_) => Err(Status::InternalServerError),
        }
    } else {
        Err(Status::NotFound)
    }
}

#[rocket::get("/<user_time_id>")]
pub async fn get(
    db_pool: &State<Pool>,
    user_time_id: String,
) -> Result<Response<UserTime>, Status> {
    let collection = get_collection::<UserTime>(db_pool, "user_times").await;

    let user_time_id = match ObjectId::parse_str(user_time_id) {
        Ok(id) => id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let user_time = collection
        .find_one(
            doc! {
                "_id": user_time_id
            },
            None,
        )
        .await
        .unwrap();

    match user_time {
        Some(user_time) => Ok(Response::Success(Json(user_time))),
        None => Err(Status::NotFound),
    }
}

#[rocket::put("/<user_time_id>", format = "json", data = "<user_time>")]
pub async fn update(
    db_pool: &State<Pool>,
    user_time_id: String,
    user_time: Json<UserTime>,
) -> Result<Response<UserTime>, Status> {
    let collection = get_collection::<UserTime>(db_pool, "user_times").await;
    let user_collection = get_collection::<User>(db_pool, "users").await;
    let meeting_collection = get_collection::<Meeting>(db_pool, "meetings").await;

    let mut new_user_time = user_time.0.clone();

    // 1. parse user_time_id and check if it exists
    let user_time_id = match ObjectId::parse_str(user_time_id) {
        Ok(id) => id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let exists_user_time = collection.find_one(doc! { "_id": user_time_id }, None).await.unwrap().is_some();

    if exists_user_time {
        // parse the user and meeting id to objectd's
        new_user_time.user_id = match ObjectId::parse_str(user_time.0.user_id_str.unwrap()) {
            Ok(id) => Some(id),
            Err(_) => return Err(Status::UnprocessableEntity),
        };
        new_user_time.meeting_id = match ObjectId::parse_str(user_time.0.meeting_id_str.unwrap()) {
            Ok(id) => Some(id),
            Err(_) => return Err(Status::UnprocessableEntity),
        };

        let user_exists = user_collection.find_one(doc! { "_id": new_user_time.user_id.unwrap() }, None).await.unwrap().is_some();
        let meeting_exists = meeting_collection.find_one(doc! { "_id": new_user_time.meeting_id.unwrap() }, None).await.unwrap().is_some();

        if user_exists && meeting_exists {
            // if both exists, update
            let opts = FindOneAndUpdateOptions::builder()
                .return_document(Some(ReturnDocument::After))
                .build();

            let result = collection.find_one_and_update(
                    doc! {
                        "_id": user_time_id
                    },
                    doc! {
                        "$set": {
                            "user_id": new_user_time.user_id.unwrap(),
                            "meeting_id": new_user_time.meeting_id.unwrap(),
                            "time": to_bson(&new_user_time.time).unwrap()
                        }
                    },
                    opts
            ).await.unwrap();

            match result {
                Some(res) => Ok(Response::Success(Json(res))),
                None => Err(Status::NotFound),
            }

        } else {
            Err(Status::NotFound)
        }
    } else {
        Err(Status::NotFound)
    }

}

#[rocket::delete("/<user_time_id>")]
pub async fn delete(db_pool: &State<Pool>, user_time_id: String) -> Result<Response<UserTime>, Status> {
    let collection = get_collection::<UserTime>(db_pool, "user_times").await;

    let user_time_id = match ObjectId::parse_str(user_time_id) {
        Ok(id) => id,
        Err(_) => return Err(Status::UnprocessableEntity)
    };

    let result = collection
        .find_one_and_delete(doc! { "_id": user_time_id }, None)
        .await
        .unwrap();

    match result {
        Some(user_time) => Ok(Response::Success(Json(user_time))),
        None => Err(Status::Conflict)
    }

}
