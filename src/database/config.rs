use dotenv::dotenv;
use mongodb::{Client, Database};
use rocket::fairing::AdHoc;
use std::env;

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
    let uri = env::var("MONGOURI").expect("Variável de ambiente MONGOURI não definida.");
    let client = Client::with_uri_str(uri).await.unwrap();
    Ok(client.database("gateway"))
}
