use super::interface::IQuery;
use crate::domain::model::test::Test;
use anyhow::Result;
use sqlx::{Database, Pool, Postgres};
pub struct TestQuery<'a, DB: Database + Sync + Send> {
    pool: &'a Pool<DB>,
}

#[async_trait::async_trait]
impl<'a> IQuery<'a, Postgres, String, Test> for TestQuery<'a, Postgres> {
    fn new(pool: &'a Pool<Postgres>) -> Self
    where
        Self: Sized + Sync + Send,
    {
        Self { pool }
    }
    async fn fetch(&self, request: &String) -> Result<Test> {
        let res: Test = sqlx::query_as("SELECT * from test where id = $1")
            .bind(request)
            .fetch_one(self.pool)
            .await?;
        Ok(res)
    }
}
