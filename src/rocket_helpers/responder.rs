use rocket::http::Status;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseEnvelope<T> {
    pub status: Status,
    pub message: T,
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
