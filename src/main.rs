#![feature(proc_macro_hygiene, decl_macro)]

use rocket::config::{Config,Environment};

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, Header};
use rocket::http::hyper::header::{Authorization, Basic};
use rocket::fairing::AdHoc;
use serde_json::Value;
use std::collections::HashSet;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/", rank = 2)]
fn index_get() -> &'static str {
    "This is the API gateway, please use POST method to call the microservice"
}



#[derive(Serialize, Deserialize)]
struct RequestData {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
}

#[post("/", format = "json", data = "<data>")]
fn index_post(data: Json<RequestData>) -> Json<ResponseData> {
    let client = reqwest::blocking::Client::new();
    let response = client.post("http://localhost:8083")
        .json(&data.0)
        .send()
        .unwrap();
    let body = response.text().unwrap();
    let response_data = ResponseData { message: body };
    return Json(response_data);
}




fn main() {
    let config = Config::build(Environment::Staging)
                                .address("127.0.0.1")
                                .port(8082)
                                .finalize()
                                .unwrap();

    rocket::custom(config)
            .mount("/hello", routes![hello])
            .mount("/", routes![index_get,index_post])
            .launch();
}

