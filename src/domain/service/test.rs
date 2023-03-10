use crate::domain::model::test::Test;
use crate::infrastructure::{
    repository::{interface::IRepository, test_repositpry::TestRepository},
    unit_of_work::IUnitOfWork,
};
use anyhow::Result;
use sqlx::Database;
use std::marker::PhantomData;
use ulid::Ulid;

pub struct TestService<'a, DB: Database + Sync + Send, UnitOfWork: IUnitOfWork<'a, DB>> {
    uow: &'a UnitOfWork,
    database: PhantomData<DB>,
}

impl<'a, DB, UnitOfWork> TestService<'a, DB, UnitOfWork>
where
    DB: Database + Sync + Send,
    UnitOfWork: IUnitOfWork<'a, DB> + Sync + Send,
{
    pub fn new(unit_of_work: &'a UnitOfWork) -> Self {
        Self {
            uow: unit_of_work,
            database: PhantomData,
        }
    }
    pub async fn execute(&self) -> Result<()> {
        let repo = TestRepository::new(self.uow);
        let test = Test {
            id: Ulid::new().to_string(),
            name: "test".to_string(),
        };
        let test2 = Test {
            id: Ulid::new().to_string(),
            name: "test2".to_string(),
        };
        repo.save(&test).await?;
        repo.save(&test2).await?;
        Ok(())
    }
}
