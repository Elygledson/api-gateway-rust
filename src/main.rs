// import Rocket
#[macro_use]
extern crate rocket;
extern crate lazy_static;

//add the modules
mod controllers;
mod database;
mod interfaces;
mod middleware;
mod model;
mod repository;

// import our routes
use controllers::auth_controller::login;
use controllers::redirect_controller::{method_delete, method_get, method_post};
use controllers::user_controller::{create_user, delete_user, get_user_by_id};
use database::config::init;
// start the web server and mount our get route at "/api". Can replace /api with anything
// or just leave it as "/" as the default location
#[launch]
async fn rocket() -> _ {
    rocket::build().attach(init().await).mount(
        "/",
        routes![
            login,
            create_user,
            get_user_by_id,
            delete_user,
            method_post,
            method_get,
            method_delete
        ],
    )
}
