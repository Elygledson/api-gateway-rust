use bcrypt::verify;
use rocket::{http::Status, serde::json::Json, State};
use validator::Validate;

use crate::middleware::jwt::generate_token;
use crate::repository::mongodb_repo::MongoRepo;
use crate::interfaces::login::LoginDto;
use crate::interfaces::response::ResponseData;
use crate::interfaces::token_response::TokenResponse;



#[post("/auth/login", data = "<login>")]
pub async fn login(
    db: &State<MongoRepo>,
    login: Json<LoginDto>,
) -> Result<Json<TokenResponse>, Json<ResponseData>> {
    login.validate().map_err(|e| ResponseData {
        status: Status::BadRequest.code,
        message: e.to_string(),
        content: None,
    })?;
    match db.get_user_by_email(&login.email).await {
        Ok(Some(user)) => {
            if verify(&login.password, &user.password).unwrap_or(false) {
                Ok(Json(TokenResponse {
                    message: "Login efetuado com sucesso!".to_string(),
                    token: generate_token(user),
                }))
            } else {
                Err(Json(ResponseData {
                    status: Status::BadRequest.code,
                    message: "Credênciais inválidas".to_string(),
                    content: None
                }))
            }
        }
        Ok(None) => Err(Json(ResponseData {
            status: Status::NotFound.code,
            message: "Usuário não encontrado.".to_string(),
            content: None
        })),
        Err(_) => Err(Json(ResponseData {
            status: Status::InternalServerError.code,
            message: "".to_string(),
            content: None
        })),
    }
}
