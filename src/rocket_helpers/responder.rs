use rocket::{
    http::Status,
    response::{self, Responder},
    serde::json::Json,
    Request,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct AppResponse<T> {
    pub status: Status,
    pub message: T,
}

#[derive(Responder)]
#[response(content_type = "json")]
pub struct ResponseEnvelope<T> {
    pub inner: (Status, T),
}

// impl<T> ResponseEnvelope<T> {
//     pub fn new(_status: Status, _message: T) -> Self {
//         Self {
//             inner: (_status, Json(_message)),
//         }
//     }
// }
