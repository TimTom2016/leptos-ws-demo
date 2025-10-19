use sqlx_migrator::error::Error;
use sqlx_migrator::operation::Operation;
use sqlx_migrator::vec_box;

use crate::migrations::m0001_chat::ChatMigration;

pub(crate) struct JoinCodeOperation;
pub(crate) struct JoinCodeMigration;

#[async_trait::async_trait]
impl Operation<sqlx::Sqlite> for JoinCodeOperation {
    // Up migration: add join_code column to groups and create unique index
    async fn up(&self, connection: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        // Add join_code column if it doesn't exist
        sqlx::query("ALTER TABLE groups ADD COLUMN join_code VARCHAR(8);")
            .execute(&mut *connection)
            .await?;

        // Generate and set unique join codes for existing groups
        let groups: Vec<(Vec<u8>,)> =
            sqlx::query_as("SELECT id FROM groups WHERE join_code IS NULL OR join_code = ''")
                .fetch_all(&mut *connection)
                .await?;

        for (group_id,) in groups {
            let mut join_code = generate_join_code();
            // Ensure uniqueness
            loop {
                let exists: Option<(i64,)> =
                    sqlx::query_as("SELECT 1 FROM groups WHERE join_code = ?1 LIMIT 1")
                        .bind(&join_code)
                        .fetch_optional(&mut *connection)
                        .await?;
                if exists.is_none() {
                    break;
                }
                join_code = generate_join_code();
            }
            sqlx::query("UPDATE groups SET join_code = ?1 WHERE id = ?2")
                .bind(&join_code)
                .bind(&group_id)
                .execute(&mut *connection)
                .await?;
        }

        // Create unique index on join_code (if not already unique by column constraint)
        sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS idx_groups_join_code ON groups(join_code);")
            .execute(&mut *connection)
            .await?;

        Ok(())
    }

    // Down migration: remove join_code column and index
    async fn down(&self, connection: &mut sqlx::SqliteConnection) -> Result<(), Error> {
        // Drop the index
        sqlx::query("DROP INDEX IF EXISTS idx_groups_join_code")
            .execute(&mut *connection)
            .await?;
        sqlx::query("ALTER TABLE groups DROP COLUMN join_code;")
            .execute(&mut *connection)
            .await?;

        Ok(())
    }
}

// Generates an 8-letter join code using nanoid crate (alphanumeric, URL-safe)
fn generate_join_code() -> String {
    use nanoid::nanoid;
    nanoid!(8)
}

sqlx_migrator::sqlite_migration!(
    JoinCodeMigration,
    "main",
    "join_code",
    vec_box![ChatMigration],
    vec_box![JoinCodeOperation]
);
