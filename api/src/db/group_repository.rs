use crate::domain::group::Group;
use crate::domain::message::Message;
use crate::{Pool, domain::group_member::GroupMember};
use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct GroupRepository {
    pub pool: Pool,
}

pub struct GroupWithLastMessage {
    pub group: Group,
    pub last_message: Option<Message>,
}

impl GroupRepository {
    pub fn new(pool: Pool) -> Self {
        GroupRepository { pool }
    }

    pub async fn create_group(&self, group: Group) -> Result<Uuid, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO groups (id, name, avatar_url, created_at, join_code) VALUES (?1, ?2, ?3, ?4, ?5)",
            group.id,
            group.name,
            group.avatar,
            group.created_at,
            group.join_code
        )
        .execute(&self.pool)
        .await?;
        Ok(group.id)
    }

    pub async fn get_group_by_id(&self, id: Uuid) -> Result<Group, sqlx::Error> {
        let record = sqlx::query!(
            "SELECT id as 'id: uuid::Uuid', name, avatar_url, join_code, created_at as 'created_at: chrono::DateTime<chrono::Utc>' FROM groups WHERE id = ?1",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Group {
            id: record.id,
            name: record.name,
            avatar: record.avatar_url,
            created_at: record.created_at,
            join_code: record.join_code.unwrap_or_default(),
        })
    }

    pub async fn get_group_by_name(&self, name: String) -> Result<Group, sqlx::Error> {
        let record = sqlx::query!(
            "SELECT id as 'id: uuid::Uuid', avatar_url, join_code, name, created_at as 'created_at: chrono::DateTime<chrono::Utc>' FROM groups WHERE name = ?1",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Group {
            id: record.id,
            name: record.name,
            avatar: record.avatar_url,
            created_at: record.created_at,
            join_code: record.join_code.unwrap_or_default(),
        })
    }

    pub async fn list_groups(&self) -> Result<Vec<Group>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT id as 'id: uuid::Uuid', avatar_url, join_code, name, created_at as 'created_at: chrono::DateTime<chrono::Utc>' FROM groups"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| Group {
                id: record.id,
                name: record.name,
                avatar: record.avatar_url,
                created_at: record.created_at,
                join_code: record.join_code.unwrap_or_default(),
            })
            .collect())
    }

    pub async fn add_member(&self, group_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
        let joined_at = Utc::now();
        sqlx::query!(
            "INSERT INTO group_members (group_id, user_id, joined_at) VALUES (?1, ?2, ?3)",
            group_id,
            user_id,
            joined_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn remove_member(&self, group_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM group_members WHERE group_id = ?1 AND user_id = ?2",
            group_id,
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_members(&self, group_id: Uuid) -> Result<Vec<GroupMember>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT group_id as 'group_id: uuid::Uuid', user_id as 'user_id: uuid::Uuid', joined_at as 'joined_at: chrono::DateTime<chrono::Utc>' FROM group_members WHERE group_id = ?1",
            group_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| GroupMember {
                group_id: record.group_id,
                user_id: record.user_id,
                joined_at: record.joined_at,
            })
            .collect())
    }

    pub async fn list_user_groups(&self, user_id: Uuid) -> Result<Vec<Group>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT g.id as 'id: uuid::Uuid', g.name, g.avatar_url, g.created_at as 'created_at: chrono::DateTime<chrono::Utc>', g.join_code \
             FROM groups g \
             JOIN group_members gm ON g.id = gm.group_id \
             WHERE gm.user_id = ?1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| Group {
                id: record.id,
                name: record.name,
                avatar: record.avatar_url,
                created_at: record.created_at,
                join_code: record.join_code.unwrap_or_default(),
            })
            .collect())
    }

    pub async fn list_user_groups_with_last_message(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<GroupWithLastMessage>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT
                g.id AS "group_id: uuid::Uuid",
                g.name,
                g.avatar_url,
                g.join_code,
                g.created_at AS "group_created_at: chrono::DateTime<chrono::Utc>",
                m.id AS "message_id: uuid::Uuid",
                m.group_id AS "message_group_id: uuid::Uuid",
                m.user_id AS "message_user_id: uuid::Uuid",
                m.content AS message_content,
                m.created_at AS "message_created_at: chrono::DateTime<chrono::Utc>"
            FROM groups g
            JOIN group_members gm ON g.id = gm.group_id
            LEFT JOIN messages m
                ON m.id = (
                    SELECT id
                    FROM messages
                    WHERE group_id = g.id
                    ORDER BY created_at DESC
                    LIMIT 1
                )
            WHERE gm.user_id = ?1;

               "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| GroupWithLastMessage {
                group: Group {
                    id: record.group_id,
                    name: record.name,
                    avatar: record.avatar_url,
                    created_at: record.group_created_at,
                    join_code: record.join_code.unwrap_or_default(),
                },
                last_message: record.message_id.map(|id| Message {
                    id,
                    group_id: record.message_group_id.unwrap(),
                    user_id: record.message_user_id.unwrap(),
                    content: record.message_content.unwrap(),
                    created_at: record.message_created_at.unwrap(),
                }),
            })
            .collect())
    }

    pub async fn get_by_join_code(&self, join_code: &str) -> Result<Option<Group>, sqlx::Error> {
        let record = sqlx::query!(
            r#"SELECT id AS "group_id: Uuid", name, avatar_url, created_at AS "group_created_at: DateTime<Utc>", join_code
            FROM groups
            WHERE join_code = ?1"#,
            join_code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|record| Group {
            id: record.group_id,
            name: record.name,
            avatar: record.avatar_url,
            created_at: record.group_created_at,
            join_code: record.join_code.unwrap_or_default(),
        }))
    }

    pub async fn is_member(&self, group_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let record = sqlx::query!(
                r#"SELECT COUNT(*) AS "count: i32" FROM group_members WHERE group_id = ?1 AND user_id = ?2"#,
                group_id,
                user_id
            )
            .fetch_one(&self.pool)
            .await?;
        Ok(record.count > 0)
    }
}
