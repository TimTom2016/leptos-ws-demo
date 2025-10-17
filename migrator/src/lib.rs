use sqlx::{Pool, Sqlite};
use sqlx_migrator::{Info as _, Migrate as _, Migrator, Plan};
mod migrations;

#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
    #[error("Failed to acquire database connection")]
    AcquireIssue,
    #[error("Failed to add migrations")]
    MigrationAdd,
    #[error("Failed to run migrations")]
    RunMigrate,
}

pub async fn migrate(pool: &mut Pool<Sqlite>) -> Result<(), MigrateError> {
    let mut migrator = Migrator::default();
    let mut conn = pool
        .acquire()
        .await
        .map_err(|v| MigrateError::AcquireIssue)?;
    migrator
        .add_migrations(migrations::migrations())
        .map_err(|v| MigrateError::MigrationAdd)?;
    migrator
        .run(&mut *conn, &Plan::apply_all())
        .await
        .inspect_err(|v| log::error!("Migration error: {:?}", v))
        .map_err(|v| MigrateError::RunMigrate)?;
    Ok(())
}
