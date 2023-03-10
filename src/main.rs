mod domain;
mod infrastructure;

use anyhow::Result;
use domain::service::{interface::IService, test::TestService};
use infrastructure::database::{IHandler, PgHandler};
use infrastructure::unit_of_work::{IUnitOfWork, PgUnitOfWork};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let db = PgHandler::new().await?;
    //
    // Migrate
    //
    sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await?
        .run(db.pool())
        .await?;
    let transaction = db.transaction().await?;
    let uow = PgUnitOfWork::new(transaction);
    let service = TestService::new(&uow);
    service.execute(&()).await?;
    uow.commit().await?;
    Ok(())
}
