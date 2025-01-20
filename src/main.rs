
mod routes;
pub mod model;
pub mod util;
mod config;

#[macro_use] extern crate rocket;

use routes::index;
use routes::post_file;
use routes::get_file;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
            .mount("/", routes![index])
            .mount("/api/v1/", routes![post_file, get_file])
}
