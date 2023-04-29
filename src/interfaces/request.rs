use rocket::serde::json::Value;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(rename = "type")]
    pub _type: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Value>,
}
