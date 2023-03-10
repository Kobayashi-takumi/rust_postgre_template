use anyhow::Result;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use sqlx::{database::Database, pool::Pool, postgres::Postgres, PgPool, Transaction};
use std::env::var;

struct Config {
    pub url: String,
    pub max_connections: u32,
}

static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config {
        url: var("DATABASE_URL").expect("DB_HOST must be set"),
        max_connections: var("DB_MAX_CONNECTIONS")
            .expect("DB_MAX_CONNECTIONS must be set")
            .parse::<u32>()
            .expect("DB_MAX_CONNECTIONS must be u32 type"),
    }
});

#[derive(Clone)]
pub struct PgHandler {
    pool: PgPool,
}

#[async_trait::async_trait]
pub trait IHandler<'a, T: Database> {
    async fn new() -> Result<Self>
    where
        Self: Sized + Sync + Send;
    fn pool(&self) -> &Pool<T>;
    async fn transaction(&self) -> Result<Transaction<'a, T>>;
}

#[async_trait::async_trait]
impl<'a> IHandler<'a, Postgres> for PgHandler {
    async fn new() -> Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(CONFIG.max_connections)
            .connect(&CONFIG.url)
            .await?;
        Ok(Self { pool })
    }
    fn pool(&self) -> &PgPool {
        &self.pool
    }
    async fn transaction(&self) -> Result<Transaction<'a, Postgres>> {
        let tran = self.pool.begin().await?;
        Ok(tran)
    }
}
