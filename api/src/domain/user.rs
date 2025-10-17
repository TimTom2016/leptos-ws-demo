use axum_session_auth::{Authentication, HasPermission};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashSet;
use uuid::Uuid;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub permissions: HashSet<String>,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        let permissions = HashSet::new();

        Self {
            id: Uuid::new_v4(),
            username,
            password,
            permissions,
        }
    }

    pub fn with_permissions(
        id: Uuid,
        username: String,
        password: String,
        permissions: HashSet<String>,
    ) -> Self {
        Self {
            id,
            username,
            password,
            permissions,
        }
    }
}

#[async_trait::async_trait]
impl Authentication<User, Uuid, SqlitePool> for User {
    async fn load_user(userid: Uuid, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
        use crate::db::UserRepository;
        let Some(pool) = pool else {
            return Err(anyhow::anyhow!("Database pool is not available"));
        };
        let repo = UserRepository { pool: pool.clone() };

        let user: User = repo.get_by_id(userid).await?;
        return Ok(user);
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}

#[async_trait::async_trait]
impl HasPermission<SqlitePool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&SqlitePool>) -> bool {
        self.permissions.contains(perm)
    }
}
