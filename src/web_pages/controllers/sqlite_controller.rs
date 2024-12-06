use rocket::{
    http::{Method, Status},
    response::status,
    serde::json::Json,
};
use rocket_cors::{AllowedOrigins, CorsOptions};

use crate::{
    rocket_helpers::responder::{
        ApiStatus, DummyResponse, DummyResponse2, ResponseEnvelope, ZakatType,
    },
    sqlite_helpers::sqlite_connection::SqliteConnection,
};

use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world from sqlite!"
}

// #[get("/get_username")]
// fn get_username() -> Json<ResponseEnvelope<String>> {
//     // Serialize DummyResponse into a String
//     let dummy_response = DummyResponse {
//         id: 1,
//         name: "Ari".to_string(),
//         dum: DummyResponse2 {
//             id: 2,
//             name: "Pepe".to_string(),
//         },
//     };

//     // Convert the DummyResponse to a JSON string
//     let serialized_message = serde_json::to_string(&dummy_response).unwrap(); // Serialize DummyResponse to String

//     // Return the response with the serialized string as the message
//     Json(ResponseEnvelope::<String> {
//         status: ApiStatus::Ok,
//         message: serialized_message,
//         data: None,
//     })
// }

#[get("/welcome")]
fn welcome() -> Json<ResponseEnvelope<String>> {
    match SqliteConnection::first_run_check() {
        Ok(sql_conn) => {
            if *sql_conn.is_exist() {
                Json(ResponseEnvelope {
                    status: ApiStatus::Ok,
                    message: "true".to_string(),
                    data: Some("Welcome to the application zakat calculator!".to_string()),
                })
            } else {
                Json(ResponseEnvelope::<String> {
                    status: ApiStatus::Ok,
                    message: "false".to_string(),
                    data: None,
                })
            }
        }
        Err(error) => Json(ResponseEnvelope::<String> {
            status: ApiStatus::InternalServerError,
            message: error.to_string(),
            data: None,
        }),
    }
}

use rocket_okapi::{openapi, openapi_get_routes};

#[openapi]
#[get("/list-zakat")]
fn get_zakat_types() -> Json<ResponseEnvelope<Vec<ZakatType>>> {
    let zakat_types = vec![
        ZakatType {
            id: 1,
            name: "Zakat Mal".to_string(),
        },
        ZakatType {
            id: 2,
            name: "Zakat Profesi".to_string(),
        },
        ZakatType {
            id: 3,
            name: "Zakat Emas, Perak, dan Logam Mulia".to_string(),
        },
        ZakatType {
            id: 4,
            name: "Zakat Pendapatan dan Jasa".to_string(),
        },
        ZakatType {
            id: 5,
            name: "Zakat Perniagaan".to_string(),
        },
        ZakatType {
            id: 6,
            name: "Zakat Pertambangan".to_string(),
        },
        ZakatType {
            id: 7,
            name: "Zakat Pertanian, Perkebunan, dan Kehutanan - Individu".to_string(),
        },
        ZakatType {
            id: 8,
            name: "Zakat Perusahaan".to_string(),
        },
        ZakatType {
            id: 9,
            name: "Zakat Peternakan dan Perikanan".to_string(),
        },
        ZakatType {
            id: 10,
            name: "Zakat Rikaz (Harta Temuan Seperti Harta Karun)".to_string(),
        },
        ZakatType {
            id: 11,
            name: "Zakat Uang dan Surat Berharga Lainnya".to_string(),
        },
    ];

    Json(ResponseEnvelope {
        status: ApiStatus::Ok,
        message: "List of Zakat retrieved successfully.".to_string(),
        data: Some(zakat_types),
    })
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
        .mount("/sqlite", openapi_get_routes![get_zakat_types])
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/sqlite/openapi.json".to_string(),
                ..Default::default()
            }),
        )
        .attach(cors)
}
