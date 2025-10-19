use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupMember {
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

impl GroupMember {
    pub fn new(group_id: Uuid, user_id: Uuid, joined_at: DateTime<Utc>) -> Self {
        Self {
            group_id,
            user_id,
            joined_at,
        }
    }
}
