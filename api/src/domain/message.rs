use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(group_id: Uuid, user_id: Uuid, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            group_id,
            user_id,
            content,
            created_at: Utc::now(),
        }
    }
}
