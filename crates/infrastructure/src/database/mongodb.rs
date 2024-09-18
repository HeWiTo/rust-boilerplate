use mongodb::{Client, options::ClientOptions};
use anyhow::Result;
use async_trait::async_trait;
use core::entities::User;
use core::repositories::UserRepository;
use mongodb::bson::{doc, Document};

pub struct MongoUserRepository {
    client: Client,
}

impl MongoUserRepository {
    pub async fn new(database_url: &str) -> Result<Self> {
        let client_options = ClientOptions::parse(database_url).await?;
        let client = Client::with_options(client_options)?;

        Ok(Self { client })
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn create_user(&self, user: &user) -> Result<()> {
        let collection = self.client.database("saas_db").collection("users");
        let doc = doc! {
            "id": &user.id,
            "email": &user.email,
            "password_hash": &user.password_hash,
        };
        collection.insert_one(doc, None).await?;
        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let collection = self.client.database("saas_db").collection("users");
        let filter = doc! { "email": email };
        let result = collection.find_one(filter, None).await?;

        match result {
            Some(doc) => {
                let user = User {
                    id: doc.get_str("id")?.to_string(),
                    email: doc.get_str("email")?.to_string(),
                    password_hash: doc.get_str("password_hash")?.to_string(),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    // Implement other methods...
}