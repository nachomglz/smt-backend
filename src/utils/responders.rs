use rocket::{response::Responder, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub enum ResponseStatus {
    Success,
    NotAuthorized,
    NotFound,
}

#[derive(Serialize, Clone)]
pub struct CustomResponse<T: Serialize + Clone> {
    status_code: u16,
    status: String,
    data: Option<T>,
}

impl<T: Serialize + Clone> CustomResponse<T> {
    pub fn new() -> CustomResponseBuilder<T> {
        CustomResponseBuilder {
            status_code: None,
            status: None,
            data: None,
        }
    }
    pub fn respond(&self) -> Response<Self> {
        match self.status_code {
            200_u16 => Response::Success(Json(self.clone())),
            404_u16 => Response::NotFound(Json(self.clone())),
            401_u16 => Response::NotAuthorized(Json(self.clone())),
            _ => Response::UnknownError(Json(self.clone())),
        }
    }
}

pub struct CustomResponseBuilder<T: Serialize + Clone> {
    status_code: Option<u16>,
    status: Option<String>,
    data: Option<T>,
}

impl<T: Serialize + Clone> CustomResponseBuilder<T> {
    pub fn status(&mut self, status: ResponseStatus) -> &mut Self {
        self.status = match status {
            ResponseStatus::Success => {
                self.status_code = Some(200);
                Some("success".to_owned())
            }
            ResponseStatus::NotAuthorized => {
                self.status_code = Some(401);
                Some("fail".to_owned())
            }
            ResponseStatus::NotFound => {
                self.status_code = Some(404);
                Some("fail".to_owned())
            }
        };
        self
    }

    pub fn data(&mut self, data: Option<T>) -> &mut Self {
        self.data = data;
        self
    }

    pub fn build(&mut self) -> CustomResponse<T> {
        CustomResponse {
            status_code: self.status_code.unwrap(),
            status: self.status.clone().unwrap_or("fail".to_owned()),
            data: self.data.clone(),
        }
    }
}

#[derive(Responder)]
pub enum Response<T> {
    #[response(status = 200, content_type = "json")]
    Success(Json<T>),
    #[response(status = 401, content_type = "json")]
    NotAuthorized(Json<T>),
    #[response(status = 404, content_type = "json")]
    NotFound(Json<T>),
    #[response(status = 500, content_type = "json")]
    UnknownError(Json<T>),
}
