use crate::infrastructure::unit_of_work::IUnitOfWork;
use anyhow::Result;
use sqlx::Database;

#[async_trait::async_trait]
pub trait IService<'a, DB, UnitOfWork, Request, Response = ()>
where
    DB: Database + Sync + Send,
    UnitOfWork: IUnitOfWork<'a, DB> + Sync + Send,
    Request: Sync + Send,
    Response: Sync + Send,
{
    fn new(unit_of_work: &'a UnitOfWork) -> Self
    where
        Self: Sized;
    async fn execute(&self, request: &Request) -> Result<Response>;
}
