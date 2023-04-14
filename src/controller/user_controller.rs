use crate::{model::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{serde::json::Json, State, http::Status};

#[post("/user", data = "<new_user>")]
pub async fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<path>")]
pub async fn get_user_by_id(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user_by_id(&id).await;
    match user_detail {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) =>  Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[delete("/user/<path>")]
pub async fn delete_user(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Usuário deletado com sucesso!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

