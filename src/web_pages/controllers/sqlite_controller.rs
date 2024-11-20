use rocket::{
    http::{Method, Status},
    response::status,
    serde::json::Json,
};
use rocket_cors::{AllowedOrigins, CorsOptions};

use crate::{
    rocket_helpers::responder::{DummyResponse, DummyResponse2, ResponseEnvelope},
    sqlite_helpers::sqlite_connection::SqliteConnection,
};

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

#[get("/welcome")]
fn welcome() -> Json<ResponseEnvelope<String>> {
    match SqliteConnection::first_run_check() {
        Ok(sql_conn) => {
            if *sql_conn.is_exist() == true {
                return Json(ResponseEnvelope::<String> {
                    status: Status::Ok,
                    message: "true".to_string(),
                });
            } else {
                return Json(ResponseEnvelope::<String> {
                    status: Status::Ok,
                    message: "false".to_string(),
                });
            }
        }
        Err(error) => {
            return Json(ResponseEnvelope::<String> {
                status: Status::InternalServerError,
                message: error.to_string(),
            });
        }
    }
}

#[launch]
pub fn rocket() -> _ {
    let exact = ["http://localhost:55555"];
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&exact))
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 58555)))
        .mount("/sqlite", routes![hello, get_username, welcome])
        .attach(cors)
}
