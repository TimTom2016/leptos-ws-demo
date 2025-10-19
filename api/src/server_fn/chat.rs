use std::collections::HashMap;

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct SentChatMessage {
    pub text: String,
    pub time: DateTime<Utc>,
    pub username: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub enum ChatChannelMessages {
    NewMessage(SentChatMessage),
}

#[server]
pub async fn publish_message(group_id: String, message: String) -> Result<(), ServerFnError> {
    use crate::AppState;
    use crate::domain::message::Message;
    use uuid::Uuid;
    let state = use_context::<AppState>().expect("AppState not found");
    use crate::auth::get_user;
    let user = get_user().await?;
    let Some(user) = user else {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    };
    let Ok(group_id_uuid) = group_id.parse() else {
        return Err(ServerFnError::ServerError("Invalid group id".to_string()));
    };
    let message = Message::new(group_id_uuid, user.id, message);
    let message_id = state.message_repository.create(message.clone()).await?;
    let Ok(new_messages) = leptos_ws::ChannelSignal::<ChatChannelMessages>::new(&group_id) else {
        return Err(ServerFnError::ServerError(
            "Failed to create channel signal".to_string(),
        ));
    };
    new_messages.send_message(ChatChannelMessages::NewMessage(SentChatMessage {
        text: message.content,
        time: message.created_at,
        username: user.username,
    }));
    Ok(())
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct ChatMessage {
    pub text: String,
    pub time: DateTime<Utc>,
    pub sender: ChatSender,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum ChatSender {
    Sent,
    Received(String),
}

#[server]
pub async fn fetch_messages(
    group_id: String,
    offset: i64,
    limit: i64,
) -> Result<Vec<ChatMessage>, ServerFnError> {
    use crate::AppState;
    use crate::domain::message::Message;
    use uuid::Uuid;
    let state = use_context::<AppState>().expect("AppState not found");
    use crate::auth::get_user;
    let user = get_user().await?;
    let Some(user) = user else {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    };
    let Ok(group_id_uuid) = group_id.parse() else {
        return Err(ServerFnError::ServerError("Invalid group id".to_string()));
    };

    // Membership check
    let is_member = state
        .group_repository
        .is_member(group_id_uuid, user.id)
        .await?;
    if !is_member {
        return Err(ServerFnError::ServerError("Forbidden".to_string()));
    }

    let messages = state
        .message_repository
        .get_by_group_paginated(group_id_uuid, offset, limit)
        .await?;

    // You may need to fetch usernames for each message.user_id if not available in Message
    // For now, assuming you can get username from user_id (pseudo-code)
    let mut result = Vec::new();
    let mut username_cache: HashMap<Uuid, String> = HashMap::new();

    for msg in messages {
        // Replace this with actual username lookup if needed
        let username = if let Some(name) = username_cache.get(&msg.user_id) {
            name.clone()
        } else {
            // Fetch username from DB
            let user = state.user_repository.get_by_id(msg.user_id).await?;
            username_cache.insert(msg.user_id, user.username.clone());
            user.username
        };
        result.push(ChatMessage {
            text: msg.content,
            time: msg.created_at,
            sender: if username == user.username {
                ChatSender::Sent
            } else {
                ChatSender::Received(username)
            },
        });
    }
    Ok(result)
}
