use bcrypt::{hash, DEFAULT_COST};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use validator::Validate;

use crate::interfaces::response::ResponseData;
use crate::middleware::jwt::UserToken;
use crate::model::user_model::User;
use crate::repository::mongodb_repo::MongoRepo;

#[post("/user", data = "<user>")]
pub async fn create_user(
    db: &State<MongoRepo>,
    user: Json<User>,
) -> Result<Json<InsertOneResult>, Json<ResponseData>> {
    let mut data = User {
        id: None,
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        password: user.password.to_owned(),
    };

    if let Ok(Some(_)) = db.get_user_by_email(&data.email).await {
        return Err(Json(ResponseData {
            status: Status::Conflict.code,
            message: "Email já cadastrado!".to_string(),
            content: None,
        }));
    }

    data.validate().map_err(|e| ResponseData {
        status: Status::BadRequest.code,
        message: e.to_string(),
        content: None,
    })?;

    let hashed_password = hash(&user.password, DEFAULT_COST).map_err(|_| ResponseData {
        status: Status::InternalServerError.code,
        message: "".to_string(),
        content: None,
    })?;

    data.password = hashed_password;
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => {
            let error = ResponseData {
                status: Status::InternalServerError.code,
                message: "Error ao criar usuário".to_string(),
                content: None,
            };
            Err(Json(error))
        }
    }
}

#[get("/user/<path>")]
pub async fn get_user_by_id(
    token: UserToken,
    db: &State<MongoRepo>,
    path: String,
) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    println!("{:?}", token);
    let user_detail = db.get_user_by_id(&id).await;
    match user_detail {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/user/<path>")]
pub async fn delete_user(db: &State<MongoRepo>, path: String) -> Result<(), Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(());
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
