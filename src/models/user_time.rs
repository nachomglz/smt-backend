use crate::{
    config::Pool,
    utils::{db::get_collection, responders::Response},
    models::{user::User, meeting::Meeting},
};
use bson::{doc, oid::ObjectId};
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserTime {
    #[serde(skip_serializing_if = "Option::is_none")]
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
    let meeting_exists = meeting_collection.find_one(doc! {
        "_id": &new_user_time.meeting_id
    }, None).await.unwrap().is_some();

    let user_exists = user_collection.find_one(doc! {
        "_id": &new_user_time.user_id
    }, None).await.unwrap().is_some();

    // if they both exist, create the new user_time document
    if meeting_exists && user_exists {
        match collection.insert_one(&new_user_time, None).await {
            Ok(result) => {
                let id = result.inserted_id.as_object_id().unwrap();
                new_user_time.id = Some(id);
                Ok(Response::Success(Json(new_user_time)))
            }
            Err(_) => Err(Status::InternalServerError)
        }
    } else {
        Err(Status::NotFound)
    }

}


