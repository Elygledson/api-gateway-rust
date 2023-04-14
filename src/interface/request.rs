use rocket::serde::{Serialize, Deserialize, json::serde_json};

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(rename = "type")]
    pub _type: String,
    pub action: String,
    pub content: serde_json::Value,
}