use mongodb::{Client, Database};
use rocket::fairing::AdHoc;
use std::env;
use dotenv::dotenv;

use crate::repository::mongodb_repo::MongoRepo;

pub async fn init() -> AdHoc {
    AdHoc::on_ignite("Connect to MongoDB cluster", |rocket| async {
        match connect().await {
            Ok(database) => rocket.manage(MongoRepo::init(database)),
            Err(error) => {
                panic!("Cannot connect to MDB instance:: {:?}", error)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Database> {
    dotenv().ok();
    let uri = match env::var("MONGOURI") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Erro ao carregar variável de ambiente"),
    };
    let client = Client::with_uri_str(uri).await.unwrap();
    Ok(client.database("gateway"))
}