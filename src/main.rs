// import Rocket
#[macro_use] 
extern crate rocket;
extern crate lazy_static;
// add our routes and services modules
mod routes;
mod interface;
// import our routes
use routes::handler::{method_get, method_post, method_delete};

// start the web server and mount our get route at "/api". Can replace /api with anything
// or just leave it as "/" as the default location
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![method_get, method_post, method_delete])
}