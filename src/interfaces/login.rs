use rocket::serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 100))]
    pub password: String,
}
