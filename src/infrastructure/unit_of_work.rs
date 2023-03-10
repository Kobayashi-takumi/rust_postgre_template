use anyhow::Result;
use sqlx::{Database, Postgres, Transaction};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("処理に失敗しました。")]
    ProcessFail,
}

#[async_trait::async_trait]
pub trait IUnitOfWork<'a, DB: Database + Sync + Send> {
    fn new(transaction: Transaction<'static, Postgres>) -> Self;
    async fn commit(self) -> Result<()>;
    async fn rollback(self) -> Result<()>;
    fn transaction(&self) -> &Arc<tokio::sync::Mutex<Transaction<'static, Postgres>>>;
}

pub struct PgUnitOfWork {
    pub transaction: Arc<tokio::sync::Mutex<Transaction<'static, Postgres>>>,
}

#[async_trait::async_trait]
impl<'a> IUnitOfWork<'a, Postgres> for PgUnitOfWork {
    fn new(transaction: Transaction<'static, Postgres>) -> Self {
        Self {
            transaction: Arc::new(tokio::sync::Mutex::new(transaction)),
        }
    }

    async fn commit(self) -> Result<()> {
        match Arc::try_unwrap(self.transaction) {
            Ok(tran) => {
                tran.into_inner().commit().await?;
                Ok(())
            }
            _ => Err(Error::ProcessFail.into()),
        }
    }

    async fn rollback(self) -> Result<()> {
        match Arc::try_unwrap(self.transaction) {
            Ok(tran) => {
                tran.into_inner().rollback().await?;
                Ok(())
            }
            _ => Err(Error::ProcessFail.into()),
        }
    }
    fn transaction(&self) -> &Arc<tokio::sync::Mutex<Transaction<'static, Postgres>>> {
        &self.transaction
    }
}
