use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::Result;
use async_trait::async_trait;
use core::entities::User;
use core::repositories::UserRepository;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create_user(&self, user: &User) -> Result<()> {
        sqlx::query!(
            "INSERT INTO users () VALUES ($1, $2, $3)",
            user.id,
            user.email,
            user.password_hash
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(User)
    }

    // Implement other methods...
}