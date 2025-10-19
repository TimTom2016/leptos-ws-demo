use sqlx_migrator::error::Error;
use sqlx_migrator::operation::Operation;
use sqlx_migrator::vec_box;

use crate::migrations::m0000_setup::SetupMigration;

pub(crate) struct ChatOperation;
pub(crate) struct ChatMigration;

#[async_trait::async_trait]
impl Operation<sqlx::Sqlite> for ChatOperation {
    // Up migration: create chat-related tables
    async fn up(&self, connection: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        // Groups table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS groups (
                id         BLOB NOT NULL PRIMARY KEY,
                name       VARCHAR(64) NOT NULL UNIQUE,
                avatar_url TEXT,
                created_at DATETIME NOT NULL
            );",
        )
        .execute(&mut *connection)
        .await?;

        // Group membership table (many-to-many: users <-> groups)
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS group_members (
                group_id   BLOB NOT NULL,
                user_id    BLOB NOT NULL,
                joined_at  DATETIME NOT NULL,
                PRIMARY KEY (group_id, user_id),
                FOREIGN KEY (group_id) REFERENCES groups(id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            );",
        )
        .execute(&mut *connection)
        .await?;

        // Messages table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS messages (
                id         BLOB NOT NULL PRIMARY KEY,
                group_id   BLOB NOT NULL,
                user_id    BLOB NOT NULL,
                content    TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                FOREIGN KEY (group_id) REFERENCES groups(id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            );",
        )
        .execute(&mut *connection)
        .await?;

        Ok(())
    }

    // Down migration: drop chat-related tables
    async fn down(&self, connection: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE IF EXISTS messages")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS group_members")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS groups")
            .execute(&mut *connection)
            .await?;
        Ok(())
    }
}

sqlx_migrator::sqlite_migration!(
    ChatMigration,
    "main",
    "chat",
    vec_box![SetupMigration],
    vec_box![ChatOperation]
);
