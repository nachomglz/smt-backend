use mongodb::bson::Bson;
use mongodb::options::{FindOneOptions, InsertOneOptions};
use mongodb::{self, bson::doc};
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::utils::responders::Response;
use crate::{
    config::Pool,
    utils::responders::{CustomResponse, ResponseStatus},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: Option<Bson>,
    name: String,
    email: String,
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
) -> Response<CustomResponse<User>> {
    let db = db_pool.get().await.unwrap();
    let collection = db.default_database().unwrap().collection::<User>("users");
    let find_options = FindOneOptions::builder().build();

    let user = collection
        .find_one(doc! {"email": email.0.email}, find_options)
        .await
        .unwrap();

    match user {
        Some(user) => CustomResponse::<User>::new()
            .data(Some(user))
            .status(ResponseStatus::Success)
            .build()
            .respond(),
        None => CustomResponse::new()
            .status(ResponseStatus::NotAuthorized)
            .build()
            .respond(),
    }
}

#[rocket::post("/signup", format = "json", data = "<user>")]
pub async fn signup(db_pool: &State<Pool>, user: Json<User>) -> Response<CustomResponse<User>> {
    let mut new_user = user.0.clone();

    let db = db_pool.get().await.unwrap();
    let collection = db.default_database().unwrap().collection::<User>("users");
    let options = InsertOneOptions::builder().build();
    let result = collection.insert_one(&new_user, options).await.unwrap();

    new_user.id = Some(result.inserted_id);

    CustomResponse::new()
        .data(Some(new_user))
        .status(ResponseStatus::Success)
        .build()
        .respond()
}
