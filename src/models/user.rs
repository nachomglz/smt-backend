use crate::utils::db::get_collection;
use mongodb::bson::oid::ObjectId;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{self, bson::doc};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::config::Pool;
use crate::utils::responders::Response;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    email: String,
}

impl User {
    pub fn get_id(self) -> ObjectId {
        self.id.unwrap()
    }
}

#[derive(Debug)]
enum UserError {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequestBody {
    email: Option<String>,
}

#[rocket::post("/login", format = "json", data = "<email>")]
pub async fn login(
    db_pool: &State<Pool>,
    email: Json<LoginRequestBody>,
) -> Result<Response<User>, Status> {
    let collection = get_collection::<User>(db_pool, "users").await;

    let user = collection
        .find_one(doc! {"email": email.0.email}, None)
        .await
        .unwrap();

    match user {
        Some(user) => Ok(Response::Success(Json(user))),
        None => Err(Status::Unauthorized),
    }
}

#[rocket::post("/signup", format = "json", data = "<user>")]
pub async fn signup(db_pool: &State<Pool>, user: Json<User>) -> Result<Response<User>, Status> {
    let mut new_user = user.0.clone();
    let collection = get_collection::<User>(db_pool, "users").await;

    // check if the email is already registered
    let registered: bool = collection
        .find_one(doc! { "email": &new_user.email}, None)
        .await
        .unwrap()
        .is_some();

    if registered {
        return Err(Status::Conflict);
    }

    let result = collection.insert_one(&new_user, None).await.unwrap();

    new_user.id = Some(result.inserted_id.as_object_id().unwrap());

    Ok(Response::Created(Json(new_user)))
}

#[rocket::get("/<user_id>", format = "json")]
pub async fn get(db_pool: &State<Pool>, user_id: String) -> Result<Response<User>, Status> {
    let collection = get_collection::<User>(db_pool, "users").await;

    let user_id = match ObjectId::parse_str(user_id) {
        Ok(user_id) => user_id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let user = collection
        .find_one(doc! { "_id": user_id }, None)
        .await
        .unwrap();

    match user {
        Some(user) => Ok(Response::Success(Json(user))),
        None => Err(Status::NotFound),
    }
}

#[rocket::put("/<user_id>", format = "json", data = "<user>")]
pub async fn update(
    db_pool: &State<Pool>,
    user_id: String,
    user: Json<User>,
) -> Result<Response<User>, Status> {
    let collection = get_collection::<User>(db_pool, "users").await;

    let user_id = match ObjectId::parse_str(user_id) {
        Ok(user_id) => user_id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let opts = FindOneAndUpdateOptions::builder()
        .return_document(Some(ReturnDocument::After))
        .build();

    let result = collection
        .find_one_and_update(
            doc! { "_id": user_id },
            doc! { "$set": { "name": user.0.name } },
            opts,
        )
        .await
        .unwrap();

    match result {
        Some(new_user) => Ok(Response::Success(Json(new_user))),
        None => Err(Status::NotFound),
    }
}

#[rocket::delete("/<user_id>")]
pub async fn delete(db_pool: &State<Pool>, user_id: String) -> Result<Response<User>, Status> {
    let collection = get_collection::<User>(db_pool, "users").await;

    let user_id = match ObjectId::parse_str(user_id) {
        Ok(user_id) => user_id,
        Err(_) => return Err(Status::UnprocessableEntity),
    };

    let result = collection
        .find_one_and_delete(doc! { "_id": user_id }, None)
        .await
        .unwrap();

    match result {
        Some(user) => Ok(Response::Success(Json(user))),
        None => Err(Status::Conflict),
    }
}
