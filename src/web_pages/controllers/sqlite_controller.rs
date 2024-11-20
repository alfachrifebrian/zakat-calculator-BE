use rocket::{http::Status, serde::json::Json};

use crate::rocket_helpers::responder::{DummyResponse, DummyResponse2, ResponseEnvelope};

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world from sqlite!"
}

#[get("/get_username")]
fn get_username() -> Json<ResponseEnvelope<DummyResponse>> {
    Json(ResponseEnvelope::<DummyResponse> {
        status: Status::Ok,
        message: DummyResponse {
            id: 1,
            name: "Ari".to_string(),
            dum: DummyResponse2 {
                id: 2,
                name: "Pepe".to_string(),
            },
        },
    })
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 58555)))
        .mount("/sqlite", routes![hello, get_username])
}
