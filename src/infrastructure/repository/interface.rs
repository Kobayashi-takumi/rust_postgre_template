use super::super::unit_of_work::IUnitOfWork;
use anyhow::Result;
use sqlx::Database;

#[async_trait::async_trait]
pub trait IRepository<'a, DB, UnitOfWork, Model, Response = ()>
where
    DB: Database + Sync + Send,
    UnitOfWork: IUnitOfWork<'a, DB> + Sync + Send,
    Model: Sync + Send,
    Response: Sync + Send,
{
    fn new(unit_of_work: &'a UnitOfWork) -> Self
    where
        Self: Sized + Sync + Send;
    async fn save(&self, model: &Model) -> Result<Response>;
}
