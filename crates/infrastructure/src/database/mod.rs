mod postgres;
mod mongodb;

use anyhow::Result;
pub use self::postgres::PgUserRepository;
pub use self::mongodb::MongoUserRepository;

pub struct DatabaseManager {
    pg_repo: PgUserRepository,
    mongo_repo: MongoUserRepository,
}

impl DatabaseManager {
    pub async fn new(pg_url: &str, mongo_url: &str) -> Result<Self> {
        let pg_repo = PgUserRepository::new(pg_url).await?;
        let mongo_repo = MongoUserRepository::new(mongo_url).await?;

        Ok(Self {
            pg_repo,
            mongo_repo,
        })
    }

    pub fn get_pg_repo(&self) -> &PgUserRepository {
        &self.pg_repo
    }

    pub fn get_mongo_repo(&self) -> &MongoUserRepository {
        &self.mongo_repo
    }
}