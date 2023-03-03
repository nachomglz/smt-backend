use rocket::{response::Responder, serde::json::Json};
use serde::Serialize;

#[derive(Responder)]
pub enum Response<T: Serialize + Clone> {
    #[response(status = 200, content_type = "json")]
    Success(Json<T>),
    #[response(status = 201, content_type = "json")]
    Created(Json<T>),
}
