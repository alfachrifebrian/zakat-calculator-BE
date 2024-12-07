use rocket::http::Status;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum ApiStatus {
    Ok,
    NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
    Forbidden,
    Conflict,
}

impl From<Status> for ApiStatus {
    fn from(status: Status) -> Self {
        match status.code {
            200 => ApiStatus::Ok,
            404 => ApiStatus::NotFound,
            500 => ApiStatus::InternalServerError,
            400 => ApiStatus::BadRequest,
            401 => ApiStatus::Unauthorized,
            403 => ApiStatus::Forbidden,
            409 => ApiStatus::Conflict,
            _ => ApiStatus::InternalServerError,
        }
    }
}

impl From<ApiStatus> for Status {
    fn from(api_status: ApiStatus) -> Self {
        match api_status {
            ApiStatus::Ok => Status::Ok,
            ApiStatus::NotFound => Status::NotFound,
            ApiStatus::InternalServerError => Status::InternalServerError,
            ApiStatus::BadRequest => Status::BadRequest,
            ApiStatus::Unauthorized => Status::Unauthorized,
            ApiStatus::Forbidden => Status::Forbidden,
            ApiStatus::Conflict => Status::Conflict,
        }
    }
}

#[derive(Serialize, JsonSchema)]
pub struct ResponseEnvelope<T: JsonSchema> {
    pub status: ApiStatus,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, JsonSchema)]
pub struct ZakatType {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize)]
pub struct DummyResponse {
    pub id: i32,
    pub name: String,
    pub dum: DummyResponse2,
}

#[derive(Serialize)]
pub struct DummyResponse2 {
    pub id: i32,
    pub name: String,
}
