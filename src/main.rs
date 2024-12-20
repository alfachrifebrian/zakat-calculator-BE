use std::thread;

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

    // Create new thread
    thread::spawn(|| {
        // Create new Tokio runtime
        let rt = Runtime::new().unwrap();

        // Create async function
        rt.block_on(async {
            let _start = rocket().launch().await;
        });
    });

    loop {}
    // Another code
}
