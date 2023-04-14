// import Rocket
#[macro_use] 
extern crate rocket;
extern crate lazy_static;

//add the modules
mod interface;
mod controller; 
mod model;
mod repository;
mod database;

// import our routes
use controller::redirect_controller::{method_post,method_get, method_delete};
use controller::user_controller::{create_user, get_user_by_id, delete_user};
use database::config::{init};
// start the web server and mount our get route at "/api". Can replace /api with anything
// or just leave it as "/" as the default location
#[launch]
async fn rocket() -> _ {
    rocket::build().attach(init().await).mount("/", routes![create_user, get_user_by_id, delete_user, method_post, method_get, method_delete])
}