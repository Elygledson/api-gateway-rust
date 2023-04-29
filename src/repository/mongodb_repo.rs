use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::{DeleteResult, InsertOneResult},
    Database,
};

use crate::model::user_model::User;

pub struct MongoRepo {
    pub(crate) db: Database,
}

impl MongoRepo {
    pub fn init(db: Database) -> Self {
        MongoRepo { db }
    }

    pub async fn create_user(&self, user: User) -> mongodb::error::Result<InsertOneResult> {
        let collection = self.db.collection::<User>("user");
        let data = User {
            id: None,
            name: user.name,
            email: user.email,
            password: user.password,
        };
        let result = collection.insert_one(data, None).await?;
        Ok(result)
    }

    pub async fn get_user_by_id(&self, id: &String) -> mongodb::error::Result<Option<User>> {
        let collection = self.db.collection::<User>("user");
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = collection.find_one(filter, None).await?;
        Ok(user_detail)
    }

    pub async fn get_user_by_email(&self, email: &String) -> mongodb::error::Result<Option<User>> {
        let collection = self.db.collection::<User>("user");
        let filter = doc! {"email": email};
        let user_detail = collection.find_one(filter, None).await?;
        Ok(user_detail)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let collection = self.db.collection::<User>("user");
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = collection
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Erro ao tentar deletar");
        Ok(user_detail)
    }
}
