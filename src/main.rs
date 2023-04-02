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
use reqwest::StatusCode;

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/", rank = 2)]
fn index_get() -> Result<Json<ResponseData>, String> {
    let client = reqwest::blocking::Client::new();
    let response_result = client.get("http://viacep.com.br/ws/010")
        .send();
    match response_result {
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let body_result = response.text();
                    match body_result {
                        Ok(body) => {
                            let response_data = ResponseData { message: body };
                            Ok(Json(response_data))
                        }
                        Err(e) => Err(format!("Failed to get response body: {}", e))
                    }
                },
                StatusCode::NOT_FOUND => {
                    Err("URL não encontrada".to_string())
                },
                _ => {
                    Err(format!("Erro ao fazer a requisição: {}", response.status()))
                }
            }
        },
        Err(e) => Err(format!("Failed to send request: {}", e))
    }
}



#[derive(Serialize, Deserialize)]
struct RequestBody {
    service_type: String,
    data: Value,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
}

#[post("/", format = "json", data = "<data>")]
fn index_post(data: Json<RequestBody>) -> Json<Result<ResponseData, String>> {
    let client = reqwest::blocking::Client::new();
    let response_result = client.post("http://localhost:8083")
        .json(&data.0)
        .send();
    match response_result {
        Ok(response) => {
            let body_result = response.text();
            match body_result {
                Ok(body) => {
                    let response_data = ResponseData { message: body };
                    Json(Ok(response_data))
                }
                Err(e) => Json(Err(format!("Failed to get response body: {}", e)))
            }
        }
        Err(e) => Json(Err(format!("Failed to send request: {}", e)))
    }
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

