
mod util;
mod routes;
mod config;
mod repository;
mod io_manager;
pub mod model;
mod tests;

#[macro_use] extern crate rocket;

use routes::get_file;
use routes::create_empty;
use routes::update_file;
use routes::delete_file;

#[launch]
fn rocket() -> _ {
    rocket::build()
            .mount("/api/v1/", routes![get_file, create_empty, update_file, delete_file])
}
