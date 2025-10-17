use crate::domain::user::User;
use crate::{Pool, auth::auth};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Pool,
}

impl UserRepository {
    pub fn new(pool: Pool) -> Self {
        UserRepository { pool }
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}

impl UserRepository {
    pub async fn get_by_id(&self, id: Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query!(
            r#"SELECT id as "id: uuid::Uuid", username, password FROM users WHERE id = ?1"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        let sql_user_perms = sqlx::query_as::<_, crate::auth::SqlPermissionTokens>(
            "SELECT token FROM user_permissions WHERE user_id = ?1;",
        )
        .bind(user.id)
        .fetch_all(&self.pool)
        .await
        .ok();

        Ok(User {
            id: user.id,
            username: user.username,
            password: user.password,
            permissions: if let Some(user_perms) = sql_user_perms {
                user_perms
                    .into_iter()
                    .map(|x| x.token)
                    .collect::<HashSet<String>>()
            } else {
                HashSet::<String>::new()
            },
        })
    }
    pub async fn create(&self, data: User) -> Result<Uuid, sqlx::Error> {
        let now = chrono::Utc::now();
        sqlx::query!(
            "INSERT INTO users (id,username, password,created_at) VALUES (?1,?2,?3,?4)",
            data.id,
            data.username,
            data.password,
            now
        )
        .execute(&self.pool)
        .await?;
        Ok(data.id)
    }
    async fn update(&self, id: Uuid, data: User) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query!(
            r#"UPDATE users SET username = ?1, password = ?2 WHERE id = ?3 RETURNING id as "id: uuid::Uuid""#,
            data.username,
            data.password,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(id.id)
    }
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE id = ?1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

impl UserRepository {
    pub async fn get_by_username(&self, username: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query!(
            r#"SELECT id as "id: uuid::Uuid", username, password FROM users WHERE username = ?1"#,
            username
        )
        .fetch_one(&self.pool)
        .await?;
        let sql_user_perms = sqlx::query_as::<_, crate::auth::SqlPermissionTokens>(
            "SELECT token FROM user_permissions WHERE user_id = ?1;",
        )
        .bind(user.id)
        .fetch_all(&self.pool)
        .await
        .ok();

        Ok(User {
            id: user.id,
            username: user.username,
            password: user.password,
            permissions: if let Some(user_perms) = sql_user_perms {
                user_perms
                    .into_iter()
                    .map(|x| x.token)
                    .collect::<HashSet<String>>()
            } else {
                HashSet::<String>::new()
            },
        })
    }

    pub async fn update_permissions(&self, data: User) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Get current permissions from database
        let current_permissions = sqlx::query_as::<_, SqlPermissionTokens>(
            "SELECT token FROM user_permissions WHERE user_id = ?1",
        )
        .bind(data.id)
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(|p| p.token)
        .collect::<HashSet<String>>();

        // Calculate permissions to add and remove
        let new_permissions = &data.permissions;
        let to_add = new_permissions.difference(&current_permissions);
        let to_remove = current_permissions.difference(new_permissions);

        // Remove permissions that are no longer needed
        for perm in to_remove {
            sqlx::query!(
                "DELETE FROM user_permissions WHERE user_id = ?1 AND token = ?2",
                data.id,
                perm
            )
            .execute(&mut *tx)
            .await?;
        }

        // Add new permissions
        for perm in to_add {
            sqlx::query!(
                "INSERT INTO user_permissions (user_id, token) VALUES (?1, ?2)",
                data.id,
                perm
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        let _ = auth().await.inspect(|v| v.cache_clear_user(data.id));
        Ok(())
    }
}
