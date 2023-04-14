use rocket::serde::{Serialize, json::serde_json};


#[derive(Serialize)]
pub struct ResponseData {
    pub status: u16,
    pub message: String,
    pub content: Option<serde_json::Value>,
}
