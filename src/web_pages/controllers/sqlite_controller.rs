use rocket::{http::Status, serde::json::Json};

use crate::rocket_helpers::responder::{AppResponse, ResponseEnvelope};

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world from sqlite!"
}

#[get("/get_username")]
fn get_username() -> ResponseEnvelope<String> {
    ResponseEnvelope{inner: (Status::BadGateway, "Hello Ari".to_string())}
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 58555)))
        .mount("/sqlite", routes![hello, get_username])
}
