use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub message: String,
    pub token: String,
}