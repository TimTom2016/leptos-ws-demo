use crate::Pool;
use setup::SetupMigration;
use sqlx_migrator::{Info as _, Migrate as _, Plan};

mod setup;
type Migrator = sqlx_migrator::Migrator<sqlx::Postgres>;
fn setup_migrator() -> Result<Migrator, sqlx_migrator::Error> {
    let mut migrator = Migrator::default();
    migrator.add_migration(Box::new(SetupMigration))?;
    Ok(migrator)
}

pub async fn migrate(pool: &mut Pool) -> Result<(), sqlx_migrator::Error> {
    let mut connection = pool.acquire().await?;
    let migrator = setup_migrator()?;
    migrator.run(&mut *connection, &Plan::apply_all()).await?;
    Ok(())
}
