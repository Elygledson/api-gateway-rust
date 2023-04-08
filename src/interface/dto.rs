use rocket::serde::{Serialize, Deserialize, json::serde_json};

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(rename = "type")]
    pub _type: String,
    pub action: String,
    pub content: serde_json::Value,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub status: u16,
    pub content: Option<serde_json::Value>,
}
