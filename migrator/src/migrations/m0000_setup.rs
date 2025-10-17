use sqlx_migrator::error::Error;
use sqlx_migrator::operation::Operation;
use sqlx_migrator::vec_box;

pub(crate) struct SetupOperation;
pub(crate) struct SetupMigration;
#[async_trait::async_trait]
impl Operation<sqlx::Sqlite> for SetupOperation {
    // Up function runs apply migration
    async fn up(&self, connection: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS users (
                id         BLOB NOT NULL PRIMARY KEY ,
                username   VARCHAR(32) NOT NULL UNIQUE,
                password   VARCHAR(128) NOT NULL,
                created_at DATETIME NOT NULL
            );
            ",
        )
        .execute(&mut *connection)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS user_permissions (
            user_id  UUID NOT NULL,
            token    VARCHAR(32) NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );",
        )
        .execute(&mut *connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE user_permissions")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE users")
            .execute(&mut *connection)
            .await?;
        Ok(())
    }
}

sqlx_migrator::sqlite_migration!(
    SetupMigration,
    "main",
    "setup",
    vec_box![],
    vec_box![SetupOperation]
);
