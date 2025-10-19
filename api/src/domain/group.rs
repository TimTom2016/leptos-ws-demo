use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,
    pub join_code: String,
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            avatar: None,
            created_at: Utc::now(),
            join_code: nanoid::nanoid!(8),
        }
    }
    pub fn new_with_avatar(name: String, avatar: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            avatar: Some(avatar),
            created_at: Utc::now(),
            join_code: nanoid::nanoid!(8),
        }
    }
}
