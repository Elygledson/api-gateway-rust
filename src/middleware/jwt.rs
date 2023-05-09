use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::env;

use crate::model::user_model::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub iat: usize,
    pub exp: usize,
    pub id: ObjectId,
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(token) => {
                if let Some(t) = token.strip_prefix("Bearer ") {
                    if let Ok(token_data) = decode_token(t.to_string()) {
                        return Outcome::Success(token_data.claims);
                    }
                }
                Outcome::Failure((Status::Unauthorized, "Token inválido".to_string()))
            }
            None => Outcome::Failure((Status::Unauthorized, "Token não encontrado".to_string())),
        }
    }
}

pub fn generate_token(user: User) -> String {
    let payload = UserToken {
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        id: user.id.unwrap(),
        email: user.email,
    };
    let jwt_secret: String =
        env::var("JWT_SECRET").expect("Variável de ambiente JWT_SECRET não definida.");
    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap()
}

fn decode_token(token: String) -> Result<TokenData<UserToken>, String> {
    let jwt_secret = env::var("JWT_SECRET").expect("Variável de ambiente JWT_SECRET não definida.");
    match jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => Ok(token_data),
        Err(err) => Err(format!("Erro ao decodificar o token: {}", err)),
    }
}
