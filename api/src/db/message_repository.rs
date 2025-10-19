use crate::Pool;
use crate::domain::group::Group;
use crate::domain::group_member::GroupMember;
use crate::domain::message::Message;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct MessageRepository {
    pub pool: Pool,
}

impl MessageRepository {
    pub fn new(pool: Pool) -> Self {
        MessageRepository { pool }
    }

    pub async fn create(&self, message: Message) -> Result<Uuid, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO messages (id, group_id, user_id, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            message.id,
            message.group_id,
            message.user_id,
            message.content,
            message.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(message.id)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Message, sqlx::Error> {
        let record = sqlx::query!(
            r#"SELECT id as "id: uuid::Uuid", group_id as "group_id: uuid::Uuid", user_id as "user_id: uuid::Uuid", content, created_at as "created_at: chrono::DateTime<chrono::Utc>" FROM messages WHERE id = ?1"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Message {
            id: record.id,
            group_id: record.group_id,
            user_id: record.user_id,
            content: record.content,
            created_at: record.created_at,
        })
    }

    pub async fn get_by_group(&self, group_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        let records = sqlx::query!(
            r#"SELECT id as "id: uuid::Uuid", group_id as "group_id: uuid::Uuid", user_id as "user_id: uuid::Uuid", content, created_at as "created_at: chrono::DateTime<chrono::Utc>" FROM messages WHERE group_id = ?1 ORDER BY created_at ASC"#,
            group_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| Message {
                id: record.id,
                group_id: record.group_id,
                user_id: record.user_id,
                content: record.content,
                created_at: record.created_at,
            })
            .collect())
    }

    pub async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        let records = sqlx::query!(
            r#"SELECT id as "id: uuid::Uuid", group_id as "group_id: uuid::Uuid", user_id as "user_id: uuid::Uuid", content, created_at as "created_at: chrono::DateTime<chrono::Utc>" FROM messages WHERE user_id = ?1 ORDER BY created_at ASC"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| Message {
                id: record.id,
                group_id: record.group_id,
                user_id: record.user_id,
                content: record.content,
                created_at: record.created_at,
            })
            .collect())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM messages WHERE id = ?1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_by_group_paginated(
        &self,
        group_id: Uuid,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let records = sqlx::query!(
            r#"SELECT id as "id: uuid::Uuid", group_id as "group_id: uuid::Uuid", user_id as "user_id: uuid::Uuid", content, created_at as "created_at: chrono::DateTime<chrono::Utc>"
            FROM messages
            WHERE group_id = ?1
            ORDER BY created_at DESC
            LIMIT ?2 OFFSET ?3"#,
            group_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|record| Message {
                id: record.id,
                group_id: record.group_id,
                user_id: record.user_id,
                content: record.content,
                created_at: record.created_at,
            })
            .collect())
    }
}
