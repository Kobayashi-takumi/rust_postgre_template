use super::super::unit_of_work::IUnitOfWork;
use super::interface::IRepository;
use crate::domain::model::test::Test;
use anyhow::Result;
use sqlx::Database;
use std::marker::PhantomData;

pub struct TestRepository<'a, DB: Database + Sync + Send, UnitOfWork: IUnitOfWork<'a, DB>> {
    uow: &'a UnitOfWork,
    database: PhantomData<DB>,
}

#[async_trait::async_trait]
impl<'a, DB, UnitOfWork> IRepository<'a, DB, UnitOfWork, Test>
    for TestRepository<'a, DB, UnitOfWork>
where
    DB: Database + Sync + Send,
    UnitOfWork: IUnitOfWork<'a, DB> + Sync + Send,
{
    fn new(unit_of_work: &'a UnitOfWork) -> Self {
        Self {
            uow: unit_of_work,
            database: PhantomData,
        }
    }
    async fn save(&self, model: &Test) -> Result<()> {
        sqlx::query(
            "insert into test (id, name) values ($1, $2) on conflict (id) do update set name=$2 returning *",
        )
        .bind(model.id.clone())
        .bind(model.name.clone())
        .execute(&mut *self.uow.transaction().lock().await)
        .await?;
        Ok(())
    }
}
