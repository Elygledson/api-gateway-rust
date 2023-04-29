use lazy_static::lazy_static;
use reqwest::StatusCode;
use rocket::serde::json::{Json, Value};
use std::collections::HashMap;

use crate::interfaces::request::RequestBody;
use crate::interfaces::response::ResponseData;

lazy_static! {
    static ref URL_CACHE: HashMap<&'static str, String> = {
        let mut map = HashMap::new();
        map.insert(
            "publicações",
            "https://jsonplaceholder.typicode.com".to_string(),
        );
        map
    };
}

#[delete("/", format = "application/json", data = "<req>")]
pub async fn method_delete(req: Json<RequestBody>) -> Result<Json<ResponseData>, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}",
        URL_CACHE
            .get(&req.0._type.as_str())
            .ok_or("URL não encontrada".to_string())?,
        req.0.action
    );
    let response_result = client.delete(url).send().await;
    match response_result {
        Ok(response) => {
            let status_code = response.status();
            match response.status() {
                StatusCode::OK => Ok(Json(ResponseData {
                    status: status_code.as_u16(),
                    message: "Requisição feita com sucesso".to_string(),
                    content: None,
                })),
                StatusCode::NOT_FOUND => Err(format!("URL não encontrada: {}", response.status())),
                _ => Err(format!("Erro ao fazer a requisição: {}", response.status())),
            }
        }
        Err(e) => Err(format!("Erro ao fazer a requisição: {}", e)),
    }
}

#[get("/", format = "application/json", data = "<req>")]
pub async fn method_get(req: Json<RequestBody>) -> Result<Json<ResponseData>, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}",
        URL_CACHE
            .get(&req.0._type.as_str())
            .ok_or("URL não encontrada".to_string())?,
        req.0.action
    );
    match client.get(url).send().await {
        Ok(response) => {
            let status_code = response.status();
            match response.status() {
                StatusCode::OK => {
                    let json = response
                        .json::<Value>()
                        .await
                        .map_err(|e| format!("{}", e))?;
                    Ok(Json(ResponseData {
                        status: status_code.as_u16(),
                        message: "Requisição feita com sucesso".to_string(),
                        content: Some(json),
                    }))
                }
                StatusCode::NOT_FOUND => Err(format!("URL não encontrada: {}", response.status())),
                _ => Err(format!("Erro ao fazer a requisição: {}", response.status())),
            }
        }
        Err(e) => Err(format!("Erro ao fazer a requisição: {}", e)),
    }
}

#[post("/", format = "application/json", data = "<req>")]
pub async fn method_post(req: Json<RequestBody>) -> Result<Json<ResponseData>, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}",
        URL_CACHE
            .get(&req.0._type.as_str())
            .ok_or("URL não encontrada".to_string())?,
        req.0.action
    );
    match client.post(url).json(&req.0.content).send().await {
        Ok(response) => {
            let status_code = response.status();
            match response.status() {
                StatusCode::CREATED => {
                    let response_data = ResponseData {
                        status: status_code.as_u16(),
                        message: "Requisição feita com sucesso".to_string(),
                        content: None,
                    };
                    Ok(Json(response_data))
                }
                StatusCode::NOT_FOUND => Err(format!("URL não encontrada: {}", response.status())),
                _ => Err(format!("Erro ao fazer a requisição: {}", response.status())),
            }
        }
        Err(e) => Err(format!("Erro ao fazer a requisição: {}", e)),
    }
}
