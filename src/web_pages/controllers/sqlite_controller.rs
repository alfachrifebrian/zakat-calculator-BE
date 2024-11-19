#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world from sqlite!"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().configure(rocket::Config::figment().merge(("port", 58555))).mount("/sqlite", routes![hello])
}