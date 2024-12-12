use std::thread;

use log::info;
use tokio::runtime::Runtime;
use web_pages::controllers::sqlite_controller::rocket;

mod rocket_helpers;
mod sqlite_helpers;
mod web_pages;

#[macro_use]
extern crate rocket;
#[rocket::main]
async fn main() {
    // let _start = rocket().launch().await;
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    // Create new thread
    thread::spawn(|| {
        // Create new Tokio runtime
        info!("Start thread!");
        let rt = Runtime::new().unwrap();

        // Create async function
        rt.block_on(async {
            let _start = rocket().launch().await;
        });
    });

    loop {}
    // Another code
}
