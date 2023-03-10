use anyhow::Result;
use sqlx::{Database, Pool};

#[async_trait::async_trait]
pub trait IQuery<'a, DB, Request = (), Response = ()>
where
    DB: Database + Sync + Send,
    Request: Sync + Send,
    Response: Sync + Send,
{
    fn new(pool: &'a Pool<DB>) -> Self
    where
        Self: Sized + Sync + Send;
    async fn fetch(&self, request: &Request) -> Result<Response>;
}
