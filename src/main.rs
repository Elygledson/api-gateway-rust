#![feature(proc_macro_hygiene, decl_macro)]

use rocket::config::{Config,Environment};

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, Header};
use rocket::http::hyper::header::{Authorization, Basic};
use rocket::fairing::AdHoc;
use std::collections::HashSet;

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/", rank = 2)]
fn index_get() -> &'static str {
    "This is the API gateway, please use POST method to call the microservice"
}

// Define rota "/" com o método POST e com o parâmetro "data" para receber dados no corpo da requisição
#[post("/", data = "<data>")]
fn index_post(data: String) -> &'static str {
    // Cria um cliente HTTP utilizando a biblioteca reqwest
    let client = reqwest::blocking::Client::new();
    
    // Envia uma requisição POST com os dados recebidos na rota para a URL do microserviço
    let response = client.post("http://localhost:8083")
        .body(data)
        .send()
        .unwrap();
    
    // Retorna o corpo da resposta do microserviço
    let body = response.text().unwrap();
    return Box::leak(body.into_boxed_str());
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

